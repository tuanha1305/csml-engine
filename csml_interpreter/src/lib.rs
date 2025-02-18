pub mod data;
pub mod error_format;
pub mod fold_bot;
pub mod interpreter;
pub mod linter;
pub mod parser;

pub use data::csml_logs;
pub use interpreter::components::load_components;
pub use parser::step_checksum::get_step;

use interpreter::{interpret_scope, json_to_literal};
use parser::parse_flow;

use data::ast::{Expr, Flow, InstructionScope, Interval};
use data::context::get_hashmap_from_mem;
use data::error_info::ErrorInfo;
use data::event::Event;
use data::literal::create_error_info;
use data::message_data::MessageData;
use data::msg::MSG;
use data::CsmlResult;
use data::{csml_bot::CsmlBot, CsmlFlow};
use data::{Context, Data, Position, STEP_LIMIT};
use error_format::*;
use fold_bot::fold_bot as fold;
use linter::{linter::lint_bot, FlowToValidate};
use parser::ExitCondition;

use std::collections::HashMap;
use std::env;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn execute_step(
    step: &str,
    flow: &Flow,
    mut data: &mut Data,
    sender: &Option<mpsc::Sender<MSG>>,
) -> MessageData {
    // stop execution if step_count >= STEP_LIMIT in order to avoid infinite loops
    if *data.step_count >= STEP_LIMIT {
        let msg_data = Err(gen_error_info(
            Position::new(
                Interval::new_as_u32(0, 0, 0, None, None),
                &data.context.flow,
            ),
            format!("{}, stop at step {}", ERROR_STEP_LIMIT, step),
        ));

        return MessageData::error_to_message(msg_data, sender);
    }

    let mut msg_data = match flow
        .flow_instructions
        .get(&InstructionScope::StepScope(step.to_owned()))
    {
        Some(Expr::Scope { scope, .. }) => {
            *data.step_count += 1;
            interpret_scope(scope, &mut data, &sender)
        }
        _ => Err(gen_error_info(
            Position::new(
                Interval::new_as_u32(0, 0, 0, None, None),
                &data.context.flow,
            ),
            format!("[{}] {}", step, ERROR_STEP_EXIST),
        )),
    };

    if let Ok(msg_data) = &mut msg_data {
        match &mut msg_data.exit_condition {
            Some(condition) if *condition == ExitCondition::Goto => {
                msg_data.exit_condition = None;
            }
            Some(_) => (),
            // if no goto at the end of the scope end conversation
            None => {
                msg_data.exit_condition = Some(ExitCondition::End);
                data.context.step = "end".to_string();
                MSG::send(
                    &sender,
                    MSG::Next {
                        flow: None,
                        step: Some("end".to_owned()),
                        bot: None,
                    },
                );
            }
        }
    }

    MessageData::error_to_message(msg_data, sender)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_steps_from_flow(bot: CsmlBot) -> HashMap<String, Vec<String>> {
    csml_logs::init_logger();

    let mut result = HashMap::new();

    for flow in bot.flows.iter() {
        if let Ok(parsed_flow) = parse_flow(&flow.content, &flow.name) {
            let mut vec = vec![];

            for instruction_type in parsed_flow.flow_instructions.keys() {
                if let InstructionScope::StepScope(step_name, ..) = instruction_type {
                    vec.push(step_name.to_owned());
                }
            }
            result.insert(flow.name.to_owned(), vec);
        }
    }
    result
}

pub fn validate_bot(bot: &CsmlBot) -> CsmlResult {
    csml_logs::init_logger();

    let mut flows = vec![];
    let mut modules = vec![];
    let mut errors = Vec::new();
    let mut imports = Vec::new();

    for flow in bot.flows.iter() {
        match parse_flow(&flow.content, &flow.name) {
            Ok(ast_flow) => {
                for (scope, ..) in ast_flow.flow_instructions.iter() {
                    if let InstructionScope::ImportScope(import_scope) = scope {
                        imports.push(import_scope.clone());
                    }
                }

                // flows.insert(flow.name.to_owned(), ast_flow);
                flows.push(FlowToValidate {
                    flow_name: flow.name.to_owned(),
                    ast: ast_flow,
                    raw_flow: &flow.content,
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    if let Some(ref mods) = bot.modules {
        for module in mods.iter() {
            if let Some(flow) = &module.flow {
                match parse_flow(&flow.content, &flow.name) {
                    Ok(ast_flow) => {
                        modules.push(FlowToValidate {
                            flow_name: flow.name.to_owned(),
                            ast: ast_flow,
                            raw_flow: &flow.content,
                        });
                    }
                    Err(error) => {
                        errors.push(error);
                    }
                }
            }
        }
    }

    let mut warnings = vec![];
    // only use the linter if there is no error in the paring otherwise the linter will catch false errors
    if errors.is_empty() {
        lint_bot(
            &flows,
            &modules,
            &mut errors,
            &mut warnings,
            &bot.native_components,
            &bot.default_flow,
        );
    }

    CsmlResult::new(
        FlowToValidate::get_flows(flows),
        FlowToValidate::get_flows(modules),
        warnings,
        errors,
    )
}

pub fn fold_bot(bot: &CsmlBot) -> String {
    csml_logs::init_logger();

    let mut flows = vec![];
    let mut modules = vec![];
    let mut errors = Vec::new();
    let mut imports = Vec::new();

    for flow in bot.flows.iter() {
        match parse_flow(&flow.content, &flow.name) {
            Ok(ast_flow) => {
                for (scope, ..) in ast_flow.flow_instructions.iter() {
                    if let InstructionScope::ImportScope(import_scope) = scope {
                        imports.push(import_scope.clone());
                    }
                }

                // flows.insert(flow.name.to_owned(), ast_flow);
                flows.push(FlowToValidate {
                    flow_name: flow.name.to_owned(),
                    ast: ast_flow,
                    raw_flow: &flow.content,
                });
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    if let Some(ref mods) = bot.modules {
        for module in mods.iter() {
            if let Some(flow) = &module.flow {
                match parse_flow(&flow.content, &flow.name) {
                    Ok(ast_flow) => {
                        modules.push(FlowToValidate {
                            flow_name: flow.name.to_owned(),
                            ast: ast_flow,
                            raw_flow: &flow.content,
                        });
                    }
                    Err(error) => {
                        errors.push(error);
                    }
                }
            }
        }
    }

    let mut warnings = vec![];
    // only use the fold if there is no error in the paring otherwise the linter will catch false errors

    fold(
        &flows,
        &modules,
        &mut errors,
        &mut warnings,
        &bot.native_components,
        &bot.default_flow,
    )
}

fn get_flows(bot: &CsmlBot) -> (HashMap<String, Flow>, HashMap<String, Flow>) {
    match &bot.bot_ast {
        Some(bot) => {
            let base64decoded = base64::decode(&bot).unwrap();
            bincode::deserialize(&base64decoded[..]).unwrap()
        }
        None => {
            let bot = validate_bot(&bot);

            let flows = match bot.flows {
                Some(flows) => flows,
                None => HashMap::new(),
            };

            let extern_flows = match bot.extern_flows {
                Some(extern_flows) => extern_flows,
                None => HashMap::new(),
            };

            (flows, extern_flows)
        }
    }
}

pub fn search_for_modules(bot: &mut CsmlBot) -> Result<(), String> {
    let default_auth = env::var("MODULES_AUTH").ok();
    let default_url = env::var("MODULES_URL").ok();

    if let Some(ref mut modules) = bot.modules {
        for module in modules.iter_mut() {
            if module.flow.is_some() {
                // module already downloaded
                continue;
            }

            let request = match (&module.url, &default_url) {
                (Some(url), _) => {
                    let request = ureq::get(url);
                    match &module.auth {
                        Some(auth) => {
                            let authorization =
                                format!("Basic {}", base64::encode(auth.as_bytes()));

                            request.set("Authorization", &authorization)
                        }
                        _ => request,
                    }
                }
                (None, Some(url)) => {
                    let request = ureq::get(url);

                    match &default_auth {
                        Some(auth) => {
                            let authorization =
                                format!("Basic {}", base64::encode(auth.as_bytes()));

                            request.set("Authorization", &authorization)
                        }
                        _ => request,
                    }
                }
                _ => {
                    return Err(format!(
                        "missing url in order to get module [{}]",
                        module.name
                    ));
                }
            };

            match request.call() {
                Ok(response) => {
                    let flow_content = match response.into_string() {
                        Ok(flow) => flow,
                        Err(_) => return Err(format!("invalid module {}", module.name)),
                    };

                    module.flow = Some(CsmlFlow {
                        id: module.name.clone(),
                        name: module.name.clone(),
                        content: flow_content,
                        commands: vec![],
                    });
                }
                Err(error) => return Err(error.to_string()),
            }
        }
    }

    Ok(())
}

pub fn interpret(
    bot: CsmlBot,
    mut context: Context,
    event: Event,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    csml_logs::init_logger();

    let mut msg_data = MessageData::default();

    let mut flow = context.flow.to_owned();
    let mut step = context.step.to_owned();

    let mut step_count = 0;

    let mut step_vars = match &context.hold {
        Some(hold) => get_hashmap_from_mem(&hold.step_vars, &flow),
        None => HashMap::new(),
    };

    let native = match bot.native_components {
        Some(ref obj) => obj.to_owned(),
        None => serde_json::Map::new(),
    };

    let custom = match bot.custom_components {
        Some(serde_json::Value::Object(ref obj)) => obj.to_owned(),
        _ => serde_json::Map::new(),
    };

    let (flows, extern_flows) = get_flows(&bot);

    let env = match bot.env {
        Some(env) => json_to_literal(&env, Interval::default(), &flow).unwrap(),
        None => data::primitive::PrimitiveNull::get_literal(Interval::default()),
    };

    let mut previous_info = match &context.hold {
        Some(hold) => match &hold.previous {
            Some(previous) => Some(previous.clone()),
            None => None,
        },
        None => None,
    };

    while msg_data.exit_condition.is_none() {
        let ast = match flows.get(&flow) {
            Some(result) => result.to_owned(),
            None => {
                let error_message = format!("flow '{}' does not exist in this bot", flow);
                let error_info = create_error_info(&error_message, Interval::default());

                return MessageData::error_to_message(
                    Err(ErrorInfo {
                        position: Position {
                            flow: flow.clone(),
                            interval: Interval::default(),
                        },
                        message: error_message,
                        additional_info: Some(error_info),
                    }),
                    &sender,
                );
            }
        };

        // if the target flow dose not contains a 'start' flow change the target to the default_flow
        if step == "start"
            && ast
                .flow_instructions
                .get(&InstructionScope::StepScope(step.to_owned()))
                .is_none()
        {
            flow = bot.default_flow.clone();
            continue;
        }

        let mut data = Data::new(
            &flows,
            &extern_flows,
            &ast,
            bot.default_flow.clone(),
            &mut context,
            &event,
            &env,
            vec![],
            0,
            &mut step_count,
            step_vars,
            previous_info.clone(),
            &custom,
            &native,
        );

        msg_data = msg_data + execute_step(&step, &ast, &mut data, &sender);
        previous_info = data.previous_info.clone();
        flow = data.context.flow.to_string();
        step = data.context.step.to_string();
        // add reset loops index
        step_vars = HashMap::new();
    }

    msg_data
}

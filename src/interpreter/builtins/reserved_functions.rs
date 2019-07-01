use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::parser::{ast::{Expr, Literal, ReservedFunction}, tokens::*};
use crate::interpreter:: {
    message::*,
    variable_handler::*,
    builtins::*,
    data::Data
};

pub fn remember(name: String, value: String) -> MessageType {
    MessageType::Assign{name, value}
}

pub fn typing(args: &HashMap<String, Literal>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => return Ok(MessageType::Msg(Message::new(value, name))),
            None        => return Err("Builtin Typing bad argument type".to_owned())
        }
    }
    return Err("Builtin Typing bad number of argument".to_owned())
}

pub fn wait(args: &HashMap<String, Literal>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("Numeric") {
            Some(value) => return Ok(MessageType::Msg(Message::new(value, name))),
            None        => return Err("Builtin Typing bad argument type".to_owned())
        }
    }
    return Err("Builtin Typing bad number of argument".to_owned())
}

pub fn text(args: &HashMap<String, Literal>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => return Ok(MessageType::Msg(Message::new(value, name))),
            None        => return Err("Builtin Typing bad argument type".to_owned())
        }
    }
    return Err("Builtin Typing bad number of argument".to_owned())
}

pub fn img(args: &HashMap<String, Literal>, name: String) -> Result<MessageType, String> {
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => return Ok(MessageType::Msg(Message::new(value, name))),
            None        => return Err("Builtin Typing bad argument type".to_owned())
        }
    }
    return Err("Builtin Typing bad number of argument".to_owned())
}

pub fn url(args: &HashMap<String, Literal>, name: String) -> Result<MessageType, String>{
    if args.len() == 1 {
        match args.get("String") {
            Some(value) => return Ok(MessageType::Msg(Message::new(value, name))),
            None        => return Err("Builtin Typing bad argument type".to_owned())
        }
    }
    return Err("Builtin Typing bad number of argument".to_owned())
}

pub fn one_of(args: &HashMap<String, Literal>, elem_type: String, data: &mut Data) -> Result<MessageType, String> {
    let lit = &args.values().nth(rand::thread_rng().gen_range(0, args.len())).expect("error in get one_of");
    return Ok(MessageType::Msg(Message::new(lit, elem_type)));
}

fn parse_quickbutton(val: Literal, buttton_type: Literal, accepts: &mut Vec<Literal>) -> Literal {
    let mut button_value = HashMap::new();

    accepts.push(val.clone());

    button_value.insert("title".to_owned(), val.clone());
    button_value.insert("buttton_type".to_owned(), Literal::ArrayLiteral(vec![buttton_type]));
    button_value.insert("accept".to_owned(), val.clone());
    button_value.insert("key".to_owned(), val.clone());
    button_value.insert("value".to_owned(), val.clone());
    button_value.insert("payload".to_owned(), val);

    Literal::ObjectLiteral{ name: "button".to_owned(), value: button_value}
}

fn match_buttons(buttons: &mut Vec<Literal>, button_type: &Literal, accepts: &mut Vec<Literal>, name: &str, expr: &Literal, data: &mut Data) -> Result<bool, String> {
    match (name, expr.borrow()) {
        (BUTTON, Literal::ArrayLiteral(expr_vec))   => {
            for elem in expr_vec.iter() {
                buttons.push(
                    parse_quickbutton(elem.clone(), button_type.clone(), accepts)
                );
            }
        }
        _                                   => return Err("bad Button Type".to_owned())
    }

    Ok(true)
}

pub fn question(args: &HashMap<String, Literal>, name: String, data: &mut Data) -> Result<MessageType, String> {
    let mut question_value = HashMap::new();

    let expr_title = args.get("title").expect("error in question");
    let button_type = args.get("button_type").expect("error in question");
    let expr_buttons = args.get("buttons").expect("error in question");

    let mut buttons: Vec<Literal> = vec![];
    let mut accepts: Vec<Literal> = vec![];

    if let Literal::ArrayLiteral(array) = expr_buttons {
        for button in array.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Normal(name, expr)) = button {
            match_buttons(&mut buttons, &button_type, &mut accepts, &name, &expr, data)?;
        }
        // else { WARNING bad element }
        }
    }


    question_value.insert("title".to_owned(), Literal::StringLiteral(get_var_from_ident(expr_title, data)?.to_string()));
    question_value.insert("accepts".to_owned(), Literal::ArrayLiteral(accepts));
    question_value.insert("buttons".to_owned(), Literal::ArrayLiteral(buttons));

    Ok(MessageType::Msg(
        Message {
            content_type: name.to_lowercase(),
            content: Literal::ObjectLiteral{name: "question".to_owned(), value: question_value}
        }
    ))
}

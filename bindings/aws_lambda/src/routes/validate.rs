use csml_engine::{validate_bot, CsmlResult};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};

use lambda_runtime::{error::HandlerError};

use crate::{LambdaResponse};


#[derive(Debug, Serialize, Deserialize)]
struct ValidateBotResponse {
    valid: bool,
    errors: Vec<ValidationError>,
}

impl ValidateBotResponse {
    fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidationError {
    flow: String,
    step: String,
    line: u32,
    column: u32,
    message: String,
}

pub fn handler(body: CsmlBot) -> Result<LambdaResponse, HandlerError> {
    let response = match validate_bot(body.clone()) {
        CsmlResult {
            flows: _,
            warnings: _,
            errors: None,
        } => {
            ValidateBotResponse::new()
        }

        CsmlResult {
        flows: _,
        warnings: _,
        errors: Some(errors),
        } => {
            let mut errors_array = Vec::new();
            for (_, error_info) in errors.iter().enumerate() {
                errors_array.push(ValidationError {
                    flow: error_info.position.flow.clone(),
                    step: error_info.position.step.clone(),
                    line: error_info.position.interval.line,
                    column: error_info.position.interval.column,
                    message: error_info.message.clone(),
                })
            }
            ValidateBotResponse {
                valid: false,
                errors: errors_array,
            }
        }
    };

    Ok(LambdaResponse {
        lambda_request: serde_json::json!(response),
    })
}

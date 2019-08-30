use reqwest::{ClientBuilder, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE}};
use serde_json::{Value, map::Map};
use std::{env, collections::HashMap};
use crate::parser::{ast::Literal,};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, builtins::*, json_to_rust::json_to_literal};

// default #############################################################################

fn parse_api(args: &HashMap<String, Literal>, data: &mut Data) -> Result<(String, Map<String, Value>), ErrorInfo> {
    let mut map: Map<String, Value> = Map::new();

    if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("fn_id") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    } else if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("default") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    }

    let sub_map = create_submap(&["fn_id", "default"], &args)?;
    let client = client_to_json(&data.memory.client);

    map.insert("data".to_owned(), Value::Object(sub_map));
    map.insert("client".to_owned(), Value::Object(client));
    Ok((data.memory.fn_endpoint.to_string(), map))
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let api_key = match env::var("FN_X_API_KEY") {
        Ok(key) => HeaderValue::from_str(&key).unwrap(),
        Err(_e) => HeaderValue::from_str("PoePoe").unwrap()
    };

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    headers.insert("X-Api-Key", api_key);
    headers
}

pub fn api(args: HashMap<String, Literal>, interval: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args, data)?;
    let client = ClientBuilder::new()
            .use_rustls_tls()
            .build().unwrap();

    match client.post(&http_arg)
        .headers(construct_headers())
        .json(&map).send() {

        Ok(ref mut arg) => match &arg.text() {
            Ok(text) => {
                let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                if let Some(value) = json.get("data") {
                    match json_to_literal(value) {
                        Ok(val) => Ok(val),
                        Err(string) => Err(
                            ErrorInfo {
                                message: string,
                                interval,
                            }
                        )
                    }
                } else {
                    Ok(Literal::null())
                }
            }
            Err(_e) => {
                Err(ErrorInfo{
                    message: "Error in parsing reqwest result".to_owned(),
                    interval
                })
            }
        },
        Err(_e) => {
            Err(ErrorInfo{
                message: "Error in reqwest post".to_owned(),
                interval
            })
        }
    }
}
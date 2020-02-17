use csmlinterpreter::data::{Client, ContextJson, Event, MessageData};
use serde_json::{json, map::Map, Value};

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
pub fn gen_context(
    past: serde_json::Value,
    current: serde_json::Value,
    metadata: serde_json::Value,
    retries: i64,
    is_initial_step: bool,
) -> ContextJson {
    ContextJson {
        past,
        current,
        metadata,
        retries,
        is_initial_step,
        client: Client {
            bot_id: "none".to_owned(),
            channel_id: "none".to_owned(),
            user_id: "none".to_owned(),
        },
        fn_endpoint: "none".to_owned(),
    }
}

#[allow(dead_code)]
pub fn gen_event(event: &str) -> Event {
    Event {
        content_type: "payload".to_lowercase(),
        content: event.to_owned(),
        metadata: json!(null),
    }
}

// #[allow(dead_code)]
// pub fn gen_memory(value: Literal) -> MemoryType {
//     MemoryType {
//         created_at: "Today".to_owned(),
//         step_id: None,
//         flow_id: None,
//         conversation_id: None,
//         value,
//     }
// }

pub fn message_to_jsonvalue(result: MessageData) -> Value {
    let mut message: Map<String, Value> = Map::new();
    let mut vec = vec![];
    let mut memories = vec![];

    for msg in result.messages.iter() {
        vec.push(msg.to_owned().message_to_json());
    }

    if let Some(mem) = result.memories {
        for elem in mem.iter() {
            let mut map = Map::new();
            map.insert("key".to_owned(), json!(elem.key.to_owned()));
            map.insert("value".to_owned(), elem.value.to_owned());
            //TODO: UPDATE
            // map.insert(elem.key.to_owned(), elem.value.to_owned());
            memories.push(json!(map));
        }
    }

    message.insert("memories".to_owned(), Value::Array(memories));
    message.insert("messages".to_owned(), Value::Array(vec));
    message.insert(
        "next_flow".to_owned(),
        match serde_json::to_value(result.next_flow) {
            Ok(val) => val,
            _ => json!(null),
        },
    );
    message.insert(
        "next_step".to_owned(),
        match serde_json::to_value(result.next_step) {
            Ok(val) => val,
            _ => json!(null),
        },
    );

    Value::Object(message)
}

// metadata.insert(
//     "firstname".to_owned(),
//     gen_memory("firstname", Value::String("Alexis".to_owned()) )
// );
// metadata.insert(
//     "tutu".to_owned(),
//     gen_memory("tutu", Value::String("toto".to_owned()))
// );

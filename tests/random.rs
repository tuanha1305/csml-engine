mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/built-in/random.csml".to_owned()).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    interpret(&text, step, context, &event, None, None, None)
}

#[test]
fn ok_random() {
    let msg = format_message(None, "start");

    let v: Value = message_to_jsonvalue(msg);

    let float = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if float < 0.0 || float > 1.0 {
        panic!("Random fail {}", float);
    }
}

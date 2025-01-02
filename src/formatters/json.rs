use crate::ast::Node;
use serde_json::{json, Value};

pub fn render(ast: Vec<Node>) -> String {
    let mut values: Vec<Value> = vec![];

    for node in ast {
        values.push(json!({
            "name": node.name,
            "type": node.r#type,
            "oldValue": node.old_value,
            "newValue": node.new_value,
            "children": node.children
        }));
    }

    let res = Value::from(values);
    serde_json::to_string_pretty(&res).unwrap()
}

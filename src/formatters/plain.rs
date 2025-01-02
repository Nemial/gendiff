use crate::ast::{Node, NodeType};
use serde_json::Value;

pub fn render(ast: Vec<Node>) -> String {
    build(ast, "")
}

pub fn build(ast: Vec<Node>, path: &str) -> String {
    let filtered_ast = ast.into_iter().filter(|node| node.r#type != NodeType::Unchanged);
    let mut result: Vec<String> = vec![];

    for node in filtered_ast {
        let current_path = format!("{path}{}", node.name);
        let old_value = stringify(node.old_value);
        let new_value = stringify(node.new_value);

        match node.r#type {
            NodeType::Object => {
                let new_path = format!("{current_path}.");
                result.push(build(node.children, new_path.as_str()));
            }
            NodeType::Changed => result.push(format!(
                "Property '{current_path}' was changed. From '{old_value}' to '{new_value}'"
            )),
            NodeType::Added => result.push(format!(
                "Property '{current_path}' was added with value: '{new_value}'"
            )),
            NodeType::Removed => result.push(format!("Property '{current_path}' was removed")),
            NodeType::Unchanged => panic!("Unsupported type node {:?}", node.r#type),
        }
    }

    result.join("\n")
}

pub fn stringify(value: Option<Value>) -> String {
    match value {
        Some(val) => {
            if val.is_object() {
                "complex value".to_string()
            } else {
                val.as_str().unwrap().to_string()
            }
        }
        None => String::new(),
    }
}

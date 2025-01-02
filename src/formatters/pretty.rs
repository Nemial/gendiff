use crate::ast::{Node, NodeType};
use serde_json::Value;

const COUNT_INDENT: usize = 4;

pub fn render(ast: Vec<Node>) -> String {
    let data = build(ast, 1);
    format!("{{\n{data}\n}}")
}

fn build(ast: Vec<Node>, depth: usize) -> String {
    let mut result = vec![];

    for node in ast {
        let name = node.name;
        let indent_size = depth * COUNT_INDENT;
        let indent = " ".repeat(indent_size);
        let short_indent_size = indent_size - 2;
        let short_indent = " ".repeat(short_indent_size);

        let old_value = stringify(node.old_value, depth);
        let new_value = stringify(node.new_value, depth);
        match node.r#type {
            NodeType::Object => {
                let new_depth = depth + 1;
                let children = build(node.children, new_depth);

                result.push(format!("{indent}{name}: {{\n{children}\n{indent}}}"));
            }
            NodeType::Unchanged => result.push(format!("{indent}{name}: {old_value}")),
            NodeType::Changed => result.push(format!(
                "{short_indent}+ {name}: {new_value}\n{short_indent}- {name}: {old_value}"
            )),
            NodeType::Added => result.push(format!("{short_indent}+ {name}: {new_value}")),
            NodeType::Removed => result.push(format!("{short_indent}- {name}: {old_value}")),
        }
    }

    result.join("\n")
}

fn stringify(item: Option<Value>, depth: usize) -> String {
    match item {
        Some(value) => match value {
            Value::Object(obj) => {
                let indent_size = depth * COUNT_INDENT;
                let indent = " ".repeat(indent_size);

                let value_indent_size = indent_size + COUNT_INDENT;
                let value_indent = " ".repeat(value_indent_size);

                let mut result: Vec<String> = vec!["{".to_string()];

                for (key, val) in obj {
                    result.push(format!("{value_indent}{key}: {val}"));
                }

                result.push(format!("{indent}}}"));

                result.join("\n")
            }
            _ => value.to_string(),
        },
        None => String::new(),
    }
}

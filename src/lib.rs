use clap::ValueEnum;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::path::PathBuf;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum, Debug)]
pub enum Format {
    Json,
}

#[derive(Debug, Serialize)]
enum NodeType {
    Object,
    Added,
    Removed,
    Changed,
    Unchanged,
}

#[derive(Debug, Serialize)]
struct Node {
    name: String,
    r#type: NodeType,
    old_value: Option<Value>,
    new_value: Option<Value>,
    children: Vec<Node>,
}

impl Node {
    fn make(
        name: String,
        r#type: NodeType,
        old_value: Option<Value>,
        new_value: Option<Value>,
        children: Vec<Node>,
    ) -> Self {
        Self {
            name,
            r#type,
            old_value,
            new_value,
            children,
        }
    }
}

pub fn start(format: Format, first_file: &PathBuf, second_file: &PathBuf) {
    let parsed_file1 = get_content(first_file);
    let parsed_file2 = get_content(second_file);
    let diff = gen_diff(format, &parsed_file1, &parsed_file2);

    println!("{diff}");
}

fn get_content(file: &PathBuf) -> Value {
    match file.extension() {
        Some(ext) => {
            let file_content: Vec<u8> = std::fs::read(file).unwrap();

            match ext.to_str() {
                Some("json") => serde_json::from_slice(&file_content).unwrap(),
                Some("yaml" | "yml") => serde_yaml::from_slice(&file_content).unwrap(),
                _ => panic!("{ext:?}: unsupported extension"),
            }
        }
        None => panic!("{file:?}: undefined extension"),
    }
}

fn gen_diff(format: Format, file_data1: &Value, file_data2: &Value) -> String {
    let ast = build_ast(file_data1, file_data2);

    render(format, ast)
}

fn build_ast(file_content1: &Value, file_content2: &Value) -> Vec<Node> {
    let mut ast = vec![];
    let file_data1 = file_content1.as_object().unwrap();
    let file_data2 = file_content2.as_object().unwrap();
    let keys = get_keys(Box::from([file_data1, file_data2]));

    for key in &keys {
        if !file_data2.contains_key(key) {
            ast.push(Node::make(
                key.to_owned(),
                NodeType::Removed,
                Some(file_data1.get(key).unwrap().to_owned()),
                None,
                vec![],
            ));
        } else if !file_data1.contains_key(key) {
            ast.push(Node::make(
                key.to_owned(),
                NodeType::Added,
                None,
                Some(file_data2.get(key).unwrap().to_owned()),
                vec![],
            ));
        } else {
            let file1_value = file_data1.get(key).unwrap();
            let file2_value = file_data2.get(key).unwrap();

            if file1_value.is_object() && file2_value.is_object() {
                let children = build_ast(file1_value, file2_value);
                ast.push(Node::make(
                    key.to_owned(),
                    NodeType::Object,
                    None,
                    None,
                    children,
                ));
            } else if file1_value == file2_value {
                ast.push(Node::make(
                    key.to_owned(),
                    NodeType::Unchanged,
                    None,
                    None,
                    vec![],
                ));
            } else {
                ast.push(Node::make(
                    key.to_owned(),
                    NodeType::Changed,
                    Some(file1_value.to_owned()),
                    Some(file2_value.to_owned()),
                    vec![],
                ));
            }
        }
    }

    ast
}

fn get_keys(contents: Box<[&Map<String, Value>]>) -> Vec<String> {
    let mut keys: Vec<String> = vec![];

    for content in contents {
        for (key, _) in content {
            if !keys.contains(key) {
                keys.push(key.to_owned());
            }
        }
    }

    keys
}

fn render(format: Format, ast: Vec<Node>) -> String {
    match format {
        Format::Json => render_json(ast),
    }
}

fn render_json(ast: Vec<Node>) -> String {
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

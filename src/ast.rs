use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, Serialize)]
pub enum NodeType {
    Object,
    Added,
    Removed,
    Changed,
    Unchanged,
}
#[derive(Debug, Serialize)]
pub struct Node {
    pub name: String,
    pub r#type: NodeType,
    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
    pub children: Vec<Node>,
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

pub fn build(file_content1: &Value, file_content2: &Value) -> Vec<Node> {
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
                let children = build(file1_value, file2_value);
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

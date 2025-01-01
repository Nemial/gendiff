mod ast;
mod formatters;

use crate::ast::build;
use crate::formatters::render;
use clap::ValueEnum;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum, Debug)]
pub enum Format {
    Json,
}

pub fn start(format: Format, first_file: &PathBuf, second_file: &PathBuf) {
    let parsed_file1 = get_content(first_file);
    let parsed_file2 = get_content(second_file);
    let diff = gen_diff(format, &parsed_file1, &parsed_file2);

    println!("{diff}");
}

fn gen_diff(format: Format, file_data1: &Value, file_data2: &Value) -> String {
    let ast = build(file_data1, file_data2);

    render(format, ast)
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

#[cfg(test)]
mod tests {
    use crate::{gen_diff, get_content, Format};
    use std::fs::read;
    use std::path::PathBuf;

    #[test]
    fn check_json_format() {
        let file1 = PathBuf::from("fixtures/before.json");
        let file2 = PathBuf::from("fixtures/after.json");
        let expected =
            String::from_utf8(read(PathBuf::from("fixtures/json_expected.json")).unwrap()).unwrap();

        assert_eq!(
            expected,
            gen_diff(Format::Json, &get_content(&file1), &get_content(&file2))
        );
    }
}

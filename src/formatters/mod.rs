use crate::ast::Node;
use crate::Format;

mod json;
mod pretty;
mod plain;

pub fn render(format: Format, ast: Vec<Node>) -> String {
    match format {
        Format::Json => json::render(ast),
        Format::Pretty => pretty::render(ast),
        Format::Plain => plain::render(ast),
    }
}

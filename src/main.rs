use clap::Parser;
use gendiff::{start, Format};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(long, value_enum, default_value_t = Format::Json)]
    format: Format,
    first_file: PathBuf,
    second_file: PathBuf,
}

fn main() {
    let app = App::parse();

    start(app.format, &app.first_file, &app.second_file);
}

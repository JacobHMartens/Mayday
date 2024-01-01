use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_parser=validate_target_path)]
    pub target_path: String,
}

pub fn get_args() -> Args {
    return Args::parse();
}

fn validate_target_path(path: &str) -> Result<String, String> {
    let _path = PathBuf::from(path);
    if !_path.is_dir() {
        return Err(format!("Invalid path: Expected path to a Cargo directory. Got {}.", path));
    }
    // if !(_path.is_file() && path.ends_with(".rs")) {
    //     return Err(format!("Invalid file path: Expected path to a Rust file. Got {}.", path));
    // }
    return Ok(path.to_string());
}

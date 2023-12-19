use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to a crate's lib.rs or main.rs file
    #[arg(value_parser = validate_path)]
    path: String
}

pub fn compiler_args() -> Vec<String> {
    let cli_args = Args::parse();
    return vec![cli_args.path];
}

fn validate_path(path: &str) -> Result<String, String> {
    let _path = PathBuf::from(path);
    if !(_path.is_file() && (path.ends_with("main.rs") || _path.ends_with("lib.rs"))) {
        return Err(format!("Invalid file path: Expected path to lib.rs or main.rs. Got {}", path));
    }
    return Ok(path.to_string());
}
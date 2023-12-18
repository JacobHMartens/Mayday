use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to a crate's lib.rs or main.rs file
    #[arg(value_parser = validate_path)]
    path: String
}

pub fn get_all_args() -> Vec<String> {
    let cli_args = Args::parse();
    return vec![cli_args.path];
}

fn validate_path(path: &str) -> Result<String, String> {
    if !(PathBuf::from(path).is_file() && (path.ends_with("main.rs") || path.ends_with("lib.rs"))) {
        return Err(format!("Invalid file path: Expected path to lib.rs or main.rs. Got {}", path));
    }
    return Ok(path.to_string());
}
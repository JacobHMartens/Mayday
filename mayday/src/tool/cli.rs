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
    // TODO: Cargo check path
    
    return Ok(path.to_string());
}

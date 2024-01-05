use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short='P', long="crate-path", default_value=".\\")]
    pub crate_path: String,
    #[arg(raw=true)]
    pub cargo_args: Vec<String>
}

pub fn get_args() -> Args {
    return Args::parse();
}
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(raw=true)]
    pub cargo_args: Vec<String>
    
}

pub fn get_args() -> Args {
    return Args::parse();
}
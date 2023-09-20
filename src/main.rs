mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use crate::args::Args;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    Ok(())
}

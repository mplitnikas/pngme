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
    // println!("{:?}", args);

    match args {
        Args::Encode(encode_args) => println!("encode with {:?}", encode_args),
        Args::Decode(decode_args) => println!("decode with {:?}", decode_args),
        Args::Remove(remove_args) => println!("remove with {:?}", remove_args),
        Args::Print(print_args) => println!("print with {:?}", print_args),
    }
    Ok(())
}

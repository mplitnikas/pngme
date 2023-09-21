mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use crate::args::Args;
use crate::commands::{decode, encode, print, remove};
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match args {
        Args::Encode(encode_args) => encode(encode_args)?,
        Args::Decode(decode_args) => decode(decode_args)?,
        Args::Remove(remove_args) => remove(remove_args)?,
        Args::Print(print_args) => print(print_args)?,
    }
    Ok(())
}

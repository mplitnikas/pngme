use crate::args::*;
use crate::png::{Chunk, ChunkType, Png};
use crate::{Error, Result};
use std::fs;
use std::path::Path;

// pub fn encode(args: EncodeArgs) -> Result<()> {}
//
// pub fn decode(args: DecodeArgs) -> Result<()> {}
//
// pub fn remove(args: RemoveArgs) -> Result<()> {}

pub fn print(args: PrintArgs) -> Result<()> {
    let contents: &[u8] = &from_file(args.path)?;
    let png = Png::try_from(contents)?;
    println!("{}", png);
    Ok(())
}

fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let contents = fs::read(path)?;
    Ok(contents)
}

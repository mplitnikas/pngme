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
    let png = from_file(args.path)?;
    println!("{}", png);
    Ok(())
}

fn from_file<P: AsRef<Path>>(path: P) -> Result<Png> {
    let contents: &[u8] = &fs::read(path)?;
    let png = Png::try_from(contents)?;
    Ok(png)
}

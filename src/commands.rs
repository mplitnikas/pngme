use crate::args::*;
use crate::png::{Chunk, ChunkType, Png};
use crate::{Error, Result};
use std::fs;
use std::path::Path;

// pub fn encode(args: EncodeArgs) -> Result<()> {}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = from_file(args.path)?;
    let res = png.chunk_by_type(&args.chunk_type);

    if let Some(chunk) = res {
        println!("{}", String::from_utf8_lossy(chunk.data()));
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = from_file(&args.path)?;
    let res = png.chunk_by_type(&args.chunk_type);

    if let Some(_chunk) = res {
        let removed = png.remove_chunk(&args.chunk_type)?;
        to_file(args.path, png)?;
        println!("removed chunk: {}", removed);
    } else {
        println!("chunk type not found: {}", args.chunk_type);
    }
    Ok(())
}

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

fn to_file<P: AsRef<Path>>(path: P, png: Png) -> Result<()> {
    fs::write(path, png.as_bytes())?;
    Ok(())
}

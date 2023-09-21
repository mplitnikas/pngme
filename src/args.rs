use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Args {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub path: String,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub path: String,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub path: String,
}

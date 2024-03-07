use clap::Parser;
use std::{error::Error, ffi::OsString, fs};

#[derive(Parser)]
#[command(author="Harsh", version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub pattern: String,

    #[arg(short, long)]
    pub file_path: OsString,
}

#[allow(dead_code)]
pub fn parse_file(args: Args) -> Result<String, Box<dyn Error>> {
    let file_content: String = fs::read_to_string(args.file_path)?;

    Ok(file_content)
}

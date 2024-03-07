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
pub fn parse_file(file_path: &OsString) -> Result<String, Box<dyn Error>> {
    let file_content: String = fs::read_to_string(file_path)?;

    Ok(file_content)
}

#[allow(dead_code)]
pub fn search(pattern: &str, file_content: &str) -> Vec<String> {
    let mut matched_lines: Vec<String> = Vec::new();
    for line in file_content.lines() {
        if line.contains(pattern) {
            matched_lines.push(line.to_string());
        }
    }
    return matched_lines;
}

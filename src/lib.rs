use clap::Parser;
use std::{error::Error, ffi::OsString, fs};

/// Search for PATTERNS in each FILE.
#[derive(Parser)]
#[command(author="Harsh", version, about, long_about = None)]
pub struct Args {
    pub pattern: String,

    pub file_path: OsString,

    /// ignore case distinctions in patterns and data
    #[arg(short, long)]
    pub ignore_case: bool,
}

pub fn parse_file(file_path: &OsString) -> Result<String, Box<dyn Error>> {
    let file_content: String = fs::read_to_string(file_path)?;

    Ok(file_content)
}

pub fn search(pattern: &str, file_content: &str) -> Vec<String> {
    let mut matched_lines: Vec<String> = Vec::new();
    for line in file_content.lines() {
        if line.contains(pattern) {
            matched_lines.push(line.to_string());
        }
    }
    return matched_lines;
}

pub fn search_ignore_case(pattern: &str, file_content: &str) -> Vec<String> {
    let mut matched_lines: Vec<String> = Vec::new();
    let pattern = pattern.to_lowercase();
    for line in file_content.lines() {
        if line.to_lowercase().contains(&pattern) {
            matched_lines.push(line.to_string());
        }
    }
    return matched_lines;
}

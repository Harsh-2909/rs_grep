use clap::{Parser, Subcommand};
use std::{ffi::OsString, fs};

#[derive(Parser)]
#[command(author="Harsh", version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    file_path: OsString, // Update the type here
}

fn main() {
    let cli = Args::parse();

    let file = fs::read_to_string(cli.file_path).expect("Invalid file path or file not found!");
    println!("File content: {}", file);
}

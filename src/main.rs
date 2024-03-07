use clap::Parser;
use rs_grep::*;
use std::process;

fn main() {
    let args = Args::parse();

    let file_content = parse_file(args).unwrap_or_else(|err| {
        println!("Error while opening file: {err}");
        process::exit(1);
    });

    println!("File content:\n{}", file_content);
}

use clap::Parser;
use rs_grep::*;
use std::process;

fn main() {
    let args = Args::parse();

    let file_content = parse_file(&args.file_path).unwrap_or_else(|err| {
        println!("Error while opening file: {err}");
        process::exit(1);
    });

    let output: Vec<String> = if args.ignore_case {
        search_ignore_case(&args.pattern, &file_content)
    } else {
        search(&args.pattern, &file_content)
    };
    for line in output.iter() {
        println!("{line}");
    }
}

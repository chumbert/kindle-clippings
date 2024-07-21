use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use kindle_clippings::{export_entry, parse_clippings};

fn main() -> io::Result<()> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    let entries = parse_clippings(&content);
    for e in entries {
        println!("{}", export_entry(e));
    }

    Ok(())
}
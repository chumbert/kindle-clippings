use std::{io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{Parser};
use kindle_clippings::{Entry, export_entry, parse_clippings};

mod cli_args;
use cli_args::Cli;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file = File::open(args.clippings_file)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    let entries = parse_clippings(&content);

    let filtered_entries: Vec<Entry> = entries.into_iter()
        .filter(|e|e.author_contains(&(args.author.clone().unwrap_or("".to_string()))))
        .filter(|e| e.title().contains(&(args.title.clone().unwrap_or("".to_string()))))
        .collect()
    ;
    for e in filtered_entries {
        println!("{}", export_entry(e));
        println!("\n---\n");
    }

    Ok(())
}
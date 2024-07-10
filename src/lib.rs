mod parser;

use std::fs::File;
use std::io::{self, BufReader};
use std::env;

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

    let entries = parser::parse_clippings(reader);

    for entry in &entries {
        println!("{:?}", entry);
    }

    Ok(())
}

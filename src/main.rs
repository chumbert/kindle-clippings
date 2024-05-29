use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;
use regex::Regex;

#[derive(Debug)]
struct Entry {
    title: String,
    author: String,
    action: String,
    page: Option<u32>,
    location: String,
    date: String,
    content: Option<String>,
}

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

    let mut entries = Vec::new();
    let mut lines = reader.lines().map(|l| l.unwrap().trim().to_string()).peekable();

    let re_title_author = Regex::new(r"^(.*) \((.*)\)$").unwrap();
    let re_action_line = Regex::new(
        r"^- Your (\w+) on page (\d+)? \| location ([\d-]+) \| Added on (.*)$"
    ).unwrap();

    while lines.peek().is_some() {
        // Skip separator line
        while lines.peek().is_some() && lines.peek().unwrap().starts_with("==========") {
            lines.next();
        }

        if lines.peek().is_none() {
            break;
        }

        let title_author_line = lines.next().unwrap();
        let caps = re_title_author.captures(&title_author_line).unwrap();
        let title = caps.get(1).unwrap().as_str().to_string();
        let author = caps.get(2).unwrap().as_str().to_string();

        let action_line = lines.next().unwrap();
        let caps = re_action_line.captures(&action_line).unwrap();
        let action = caps.get(1).unwrap().as_str().to_string();
        let page = caps.get(2).map(|m| m.as_str().parse::<u32>().unwrap());
        let location = caps.get(3).unwrap().as_str().to_string();
        let date = caps.get(4).unwrap().as_str().to_string();

        let mut content = None;
        if action == "Highlight" {
            let mut quote_lines = Vec::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with("==========") {
                    break;
                }
                quote_lines.push(lines.next().unwrap());
            }
            content = Some(quote_lines.join(" "));
        }

        entries.push(Entry {
            title,
            author,
            action,
            page,
            location,
            date,
            content,
        });
    }

    for entry in &entries {
        println!("{:?}", entry);
    }

    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
pub struct Entry {
    title: String,
    author: String,
    action: String,
    page: Option<u32>,
    location: String,
    date: String,
    content: Option<String>,
}

pub enum ParsingError {
    MalformedLine,
}

const TITLE_AUTHOR_REGEX: Regex = Regex::new(r"^(.*) \((.*)\)$").unwrap();
const ACTION_LINE_REGEX: Regex = Regex::new(
    r"^- Your (\w+) on page (\d+)? \| location ([\d-]+) \| Added on (.*)$"
).unwrap();

pub fn parse_clippings(reader: BufReader<File>) -> Result<Vec<Entry>, ParsingError> {
    let mut entries = Vec::new();
    let mut lines = reader.lines().map(|l| l.unwrap().trim().to_string()).peekable();

    while lines.peek().is_some() {
        // Skip separator line
        while lines.peek().is_some() && lines.peek().unwrap().starts_with("==========") {
            lines.next();
        }

        if lines.peek().is_none() {
            break;
        }

        let title_author_line = lines.next().unwrap();
        let caps = TITLE_AUTHOR_REGEX.captures(&title_author_line).unwrap();
        let title = caps.get(1).unwrap().as_str().to_string();
        let author = caps.get(2).unwrap().as_str().to_string();

        let action_line = lines.next().unwrap();
        let caps = ACTION_LINE_REGEX.captures(&action_line).unwrap();
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

    Ok(entries)
}
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Entry {
    title: String,
    author: String,
    action: String,
    pub page: Option<u32>,
    location: String,
    date: String,
    content: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
impl Entry {
    #[wasm_bindgen(constructor)]
    pub fn new(title: String, author: String, action: String, page: Option<u32>, location: String, date: String, content: Option<String>) -> Entry {
        Entry { title, author, action, page, location, date, content }
    }
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String { self.title.clone() }

    #[wasm_bindgen(getter)]
    pub fn author(&self) -> String { self.author.clone() }

    #[wasm_bindgen(getter)]
    pub fn action(&self) -> String { self.action.clone() }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String { self.date.clone() }

    #[wasm_bindgen(getter)]
    pub fn location(&self) -> String { self.location.clone() }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Option<String> { self.content.clone() }
}

#[wasm_bindgen]
pub enum ParsingError {
    MalformedLine,
}

lazy_static! {
    static ref TITLE_AUTHOR_REGEX: Regex = Regex::new(r"^(.*) \((.*)\)$").unwrap();
    static ref ACTION_LINE_REGEX: Regex = Regex::new(
    r"^- Your (\w+) on page (\d+)? \| location ([\d-]+) \| Added on (.*)$"
).unwrap();

}

#[wasm_bindgen]
pub fn parse_clippings(content: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut lines = content.lines().map(|l| l.trim().to_string()).peekable();

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

    entries
}

pub struct Template {
    tags: HashMap<String, Box<dyn Fn(&Entry) -> String>>,
    template: String,
}

impl Template {
    pub fn template(&self) -> String { self.template.clone() }

    pub fn default() -> Template {
        Template {
            tags: HashMap::from([(
                String::from("{author}"),
                Box::from(|e: &Entry| e.author.clone()) as Box<dyn Fn(&Entry) -> String>
            )]),
            template: String::from("Author: {author}")
        }
    }

    pub fn format(self, entry: Entry) -> String {
        let mut result = self.template;
        for(tag, extractor) in self.tags {
            result = result.replace(tag.as_str(), extractor(&entry).as_str())
        }

        result
    }
}
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
pub struct Entry {
    title: String,
    author: Option<String>,
    action: String,
    page: Option<String>,
    location: Option<String>,
    date: String,
    content: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
impl Entry {
    #[wasm_bindgen(constructor)]
    pub fn new(title: String, author: Option<String>, action: String, page: Option<String>, location: Option<String>, date: String, content: Option<String>) -> Entry {
        Entry { title, author, action, page, location, date, content }
    }
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String { self.title.clone() }

    #[wasm_bindgen(getter)]
    pub fn author(&self) -> Option<String> { self.author.clone() }

    #[wasm_bindgen(getter)]
    pub fn action(&self) -> String { self.action.clone() }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String { self.date.clone() }

    #[wasm_bindgen(getter)]
    pub fn page(&self) -> Option<String> { self.page.clone() }

    #[wasm_bindgen(getter)]
    pub fn location(&self) -> Option<String> { self.location.clone() }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Option<String> { self.content.clone() }
}

#[wasm_bindgen]
pub enum ParsingError {
    MalformedLine,
}

lazy_static! {
    static ref TITLE_AUTHOR_REGEX: Regex = Regex::new(r"^(?P<title>.*)(?:\s+)(?:\((?P<author>.*)\))?$").unwrap();
    static ref ACTION_LINE_REGEX: Regex = Regex::new(
        r"^- Your (?P<action>\w+) (on page (?P<page>(\d+(-\d+)?)|(\w+))|at location (?P<location>\d+(-\d+)?)) (\| location (?<locationGroup>\d+(-\d+)?) )?\| Added on (?P<date>.+)$"
).unwrap();

}

#[wasm_bindgen]
pub fn parse_clippings(content: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut lines = content.lines().map(|l| l.trim().to_string()).peekable();

    while lines.peek().is_some() {
        println!("{}", lines.peek().unwrap());
        // Skip separator line
        while lines.peek().is_some() && lines.peek().unwrap().starts_with("==========") {
            lines.next();
        }

        if lines.peek().is_none() {
            break;
        }

        let title_author_line = lines.next().unwrap();
        println!("title: {}", title_author_line);
        let caps = match(TITLE_AUTHOR_REGEX.captures(&title_author_line)) {
            Some(author_title_match) => { author_title_match }
            None => Regex::new("(?P<title>.*)").unwrap().captures(&title_author_line).unwrap()

        };
        let title = caps.name("title").unwrap().as_str().to_string();
        let author = caps.name("author").map_or_else(|| None, |x| Some(x.as_str().to_string()));

        let action_line = lines.next().unwrap();
        println!("action: {}", action_line);
        let caps = match(ACTION_LINE_REGEX.captures(&action_line)) {
            Some(action_match) => { action_match }
            None => panic!("Can't parse action line: {}", action_line)
        };
        let action = caps["action"].to_string();
        let page = caps.name("page").map_or_else(|| None, |x| Some(x.as_str().to_string()));
        let location = caps.name("locationGroup").map_or_else(|| None, |x| Some(x.as_str().to_string()));
        let date = caps["date"].to_string();

        let mut content = None;
        if action == "Highlight" || action == "Note" {
            let mut quote_lines = Vec::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with("==========") {
                    break;
                }
                quote_lines.push(lines.next().unwrap());
            }
            content = Some(quote_lines.join(" "));
        }

        if action == "Bookmark" {
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with("==========") {
                    break;
                }
                lines.next();
            }
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
            tags: HashMap::from([
                (
                    String::from("{action}"),
                    Box::from(|e: &Entry| e.action.clone()) as Box<dyn Fn(&Entry) -> String>
                ),
                (
                    String::from("{author}"),
                    Box::from(|e: &Entry| e.author.clone().unwrap_or("Unknown author".to_string())) as Box<dyn Fn(&Entry) -> String>
                ),
                (
                    String::from("{title}"),
                    Box::from(|e: &Entry| e.title.clone()) as Box<dyn Fn(&Entry) -> String>
                ),
                (
                    String::from("{content}"),
                    Box::from(|e: &Entry| e.content.clone().unwrap_or("".to_string())) as Box<dyn Fn(&Entry) -> String>
                ),
            ]),
            template: String::from("[{action}] Author: {author}, Title: {title}\n{content}")
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

#[wasm_bindgen(js_name="exportEntry")]
pub fn export_entry(e: Entry) -> String {
    Template::default().format(e)
}

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

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
    pub fn new(
        title: String,
        author: Option<String>,
        action: String,
        page: Option<String>,
        location: Option<String>,
        date: String,
        content: Option<String>,
    ) -> Entry {
        Entry {
            title,
            author,
            action,
            page,
            location,
            date,
            content,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn action(&self) -> String {
        self.action.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.date.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn page(&self) -> Option<String> {
        self.page.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn location(&self) -> Option<String> {
        self.location.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    pub fn author_contains(&self, sequence: &str) -> bool {
        match self.author.clone() {
            None => false,
            Some(author) => author.contains(sequence),
        }
    }
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

/// Parses clipping file content
///
/// ```
/// use kindle_clippings::parsing::parse_clippings;
/// let content = "The Grapes of Wrath (John Steinbeck)\n- Your Highlight on page 12453252 | Added on Friday, 5 July 2024 09:55:52\nHow can we live without our lives ? How will we know it's us without our past ? No. Leave it. Burn it.\n";
/// let parsed = parse_clippings(content);
/// assert_eq!(1, parsed.len());
/// let entry = parsed.get(0).unwrap();
/// assert_eq!(Some("John Steinbeck".to_string()), entry.author());
/// assert_eq!("The Grapes of Wrath".to_string(), entry.title());
/// assert_eq!(Some("12453252".to_string()), entry.page());
/// assert_eq!(Some("How can we live without our lives ? How will we know it's us without our past ? No. Leave it. Burn it.".to_string()), entry.content());
/// assert_eq!("Friday, 5 July 2024 09:55:52".to_string(), entry.date());
/// assert_eq!(None, entry.location());
/// ```
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

        let (title, author) = parse_title_author(&lines.next().unwrap());
        let (action, page, location, date) = parse_action_line(&lines.next().unwrap());

        let mut content = None;
        if action == "Highlight" || action == "Note" {
            let mut quote_lines = Vec::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with("==========") {
                    break;
                }
                quote_lines.push(lines.next().unwrap());
            }
            content = Some(quote_lines.join(" ").trim().to_string());
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

fn parse_title_author(line: &str) -> (String, Option<String>) {
    let caps = match TITLE_AUTHOR_REGEX.captures(line) {
        Some(author_title_match) => author_title_match,
        None => Regex::new("(?P<title>.*)")
            .unwrap()
            .captures(line)
            .unwrap(),
    };
    let title = caps.name("title").unwrap().as_str().to_string();
    let author = caps
        .name("author")
        .map_or_else(|| None, |x| Some(x.as_str().to_string()));

    (title, author)
}

fn parse_action_line(line: &str) -> (String, Option<String>, Option<String>, String) {
    let caps = match ACTION_LINE_REGEX.captures(line) {
        Some(action_match) => action_match,
        None => panic!("Can't parse action line: {}", line),
    };
    let action = caps["action"].to_string();
    let page = caps
        .name("page").map(|x| x.as_str().to_string());
    
    let only_location = caps.name("location").map(|x| x.as_str().to_string());

    let location = caps
        .name("locationGroup").map(|x| x.as_str().to_string());
    
    let date = caps["date"].to_string();

    (action, page, only_location.or(location), date)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_get_first(content: &str) -> Entry {
        let parsed = parse_clippings(content);
        assert_eq!(1, parsed.len());
        parsed.into_iter().next().unwrap()
    }

    #[test]
    fn test_parse_entry_without_author() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 12453252 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(None, entry.author());
        assert_eq!("The Grapes of Wrath".to_string(), entry.title());
    }

    #[test]
    fn test_parse_action_line_only_location() {
        let content = "The Grapes of Wrath \n- Your Highlight at location 3867-3869 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(None, entry.page());
        assert_eq!(Some("3867-3869".to_string()), entry.location());
    }

    #[test]
    fn test_parse_action_line_page_and_location() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 14 | location 214-215 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(Some("14".to_string()), entry.page());
        assert_eq!(Some("214-215".to_string()), entry.location());
    }

    #[test]
    fn test_parse_action_line_only_page() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 14 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(Some("14".to_string()), entry.page());
        assert_eq!(None, entry.location());
    }

    #[test]
    fn test_parse_action_line_highlight() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 14 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!("Highlight".to_string(), entry.action());
    }

    #[test]
    fn test_parse_action_line_bookmark() {
        let content = "The Grapes of Wrath \n- Your Bookmark on page 14 | Added on Friday, 5 July 2024 09:55:52\n\n";
        let entry = parse_and_get_first(content);
        assert_eq!("Bookmark".to_string(), entry.action());
    }

    #[test]
    fn test_parse_action_line_note() {
        let content = "The Grapes of Wrath \n- Your Note on page 14 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!("Note".to_string(), entry.action());
    }

    #[test]
    fn test_parse_content_highlight() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 12453252 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(Some("Small for convenience.".to_string()), entry.content());
    }

    #[test]
    fn test_parse_content_note() {
        let content = "The Grapes of Wrath \n- Your Highlight on page 12453252 | Added on Friday, 5 July 2024 09:55:52\n\nSmall for convenience.\n";
        let entry = parse_and_get_first(content);
        assert_eq!(Some("Small for convenience.".to_string()), entry.content());
    }

    #[test]
    fn test_parse_content_bookmark() {
        let content = "The Grapes of Wrath \n- Your Bookmark on page 12453252 | Added on Friday, 5 July 2024 09:55:52\n\n";
        let entry = parse_and_get_first(content);
        assert_eq!(None, entry.content());
    }
}
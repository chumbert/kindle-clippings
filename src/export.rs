use crate::parsing::Entry;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

type AttributeExtractor = Box<dyn Fn(&Entry) -> String>;
pub struct Template {
    tags: HashMap<String, AttributeExtractor>,
    template: String,
}

impl Template {
    pub fn template(&self) -> String {
        self.template.clone()
    }

    pub fn new() -> Template {
        Template {
            tags: HashMap::from([
                (
                    String::from("{action}"),
                    Box::from(|e: &Entry| e.action().clone()) as Box<dyn Fn(&Entry) -> String>,
                ),
                (
                    String::from("{author}"),
                    Box::from(|e: &Entry| {
                        e.author().clone().unwrap_or("Unknown author".to_string())
                    }) as Box<dyn Fn(&Entry) -> String>,
                ),
                (
                    String::from("{title}"),
                    Box::from(|e: &Entry| e.title().clone()) as Box<dyn Fn(&Entry) -> String>,
                ),
                (
                    String::from("{content}"),
                    Box::from(|e: &Entry| e.content().clone().unwrap_or("".to_string()))
                        as Box<dyn Fn(&Entry) -> String>,
                ),
            ]),
            template: String::from("{content}\n\n*{author} - {title}*"),
        }
    }

    pub fn format(self, entry: Entry) -> String {
        let mut result = self.template;
        for (tag, extractor) in self.tags {
            result = result.replace(tag.as_str(), extractor(&entry).as_str())
        }

        result
    }
}

#[wasm_bindgen(js_name = "exportEntry")]
pub fn export_entry(e: Entry) -> String {
    Template::new().format(e)
}


#[cfg(test)]
mod test_export {
    use super::*;
    use crate::parsing::Entry;

    #[test]
    fn test_export() {
        let entry = Entry::new(
            "The Grapes of Wrath".to_string(),
            Some("John Steinbeck".to_string()),
            "Highlight".to_string(),
            None,
            None,
            "Friday, 5 July 2024 09:55:52".to_string(),
            Some("Small for convenience.".to_string()),
        );

        let exported = export_entry(entry);

        assert_eq!("Small for convenience.\n\n*John Steinbeck - The Grapes of Wrath*".to_string(), exported);
    }

    #[test]
    fn test_export_default_author_and_content() {
        let entry = Entry::new(
            "The Graves of Wraiths".to_string(),
            None,
            "Highlight".to_string(),
            None,
            None,
            "Friday, 5 July 2024 09:55:52".to_string(),
            None,
        );

        let exported = export_entry(entry);

        assert_eq!("\n\n*Unknown author - The Graves of Wraiths*".to_string(), exported);
    }
}
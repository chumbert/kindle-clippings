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

    pub fn debug_template() -> Template {
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
    Template::debug_template().format(e)
}

mod parser;
mod utils;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    let entries = parser::parse_clippings(&content);

    Ok(())
}

#[wasm_bindgen]
pub fn parse_file_content(content: &str) -> Box<[JsValue]> {
    let ret =parser::parse_clippings(content);

    console_log!("{}", content);

    let v = ret.iter()
        .map(|x| serde_wasm_bindgen::to_value(x).unwrap())
        .collect::<Vec<JsValue>>();

    console_log!("{:?}", v);


    ret.iter()
        .map(|x| serde_wasm_bindgen::to_value(x).unwrap())
        .collect::<Vec<JsValue>>()
        .into_boxed_slice()
}

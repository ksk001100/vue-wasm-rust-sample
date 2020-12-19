use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pulldown_cmark(source_text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(source_text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
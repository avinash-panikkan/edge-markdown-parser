pub mod parser;

use wasm_bindgen::prelude::*;

/// WebAssembly entry point that delegates directly to our internal parser.
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    parser::parse_markdown(input)
}
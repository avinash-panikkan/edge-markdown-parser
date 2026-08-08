use wasm_bindgen::prelude::*;

/// Core parsing function exposed to JavaScript/WebAssembly
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    let mut html_output = String::new();

    for line in input.lines() {
        if let Some(stripped) = line.strip_prefix("# ") {
            html_output.push_str(&format!("<h1>{}</h1>\n", stripped));
        } else {
            html_output.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_h1() {
        let markdown = "# Hello Edge";
        let expected = "<h1>Hello Edge</h1>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }
}
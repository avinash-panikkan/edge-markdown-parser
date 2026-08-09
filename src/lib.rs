use wasm_bindgen::prelude::*;

/// Helper function to parse inline styles (Bold ** and Italic *)
fn parse_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Parse Bold (**text**)
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let actual_end = start + 2 + end;
            let content = &result[start + 2..actual_end];
            let replacement = format!("<b>{}</b>", content);
            result.replace_range(start..actual_end + 2, &replacement);
        } else {
            break; // Unmatched opening **, stop parsing bold
        }
    }

    // Parse Italic (*text*)
    while let Some(start) = result.find('*') {
        if let Some(end) = result[start + 1..].find('*') {
            let actual_end = start + 1 + end;
            let content = &result[start + 1..actual_end];
            let replacement = format!("<i>{}</i>", content);
            result.replace_range(start..actual_end + 1, &replacement);
        } else {
            break; // Unmatched opening *, stop parsing italic
        }
    }

    result
}

/// Core parsing function exposed to JavaScript/WebAssembly
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    let mut html_output = String::new();

    for line in input.lines() {
        if let Some(stripped) = line.strip_prefix("# ") {
            let parsed_inline = parse_inline(stripped);
            html_output.push_str(&format!("<h1>{}</h1>\n", parsed_inline));
        } else {
            let parsed_inline = parse_inline(line);
            html_output.push_str(&format!("<p>{}</p>\n", parsed_inline));
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

    #[test]
    fn test_parse_bold_and_italic() {
        let markdown = "# Hello **Bold** and *Italic*";
        let expected = "<h1>Hello <b>Bold</b> and <i>Italic</i></h1>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }
}
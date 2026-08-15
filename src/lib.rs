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

/// Helper function to detect heading level (1 to 6) and return (level, content)
fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let hash_count = line.chars().take_while(|&c| c == '#').count();

    if hash_count >= 1 && hash_count <= 6 {
        let remainder = &line[hash_count..];
        if let Some(content) = remainder.strip_prefix(' ') {
            return Some((hash_count, content));
        }
    }

    None
}

/// Helper function to check if a line is an unordered list item (* or -)
fn parse_list_item(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if let Some(content) = trimmed.strip_prefix("* ") {
        Some(content)
    } else if let Some(content) = trimmed.strip_prefix("- ") {
        Some(content)
    } else {
        None
    }
}

/// Core parsing function exposed to JavaScript/WebAssembly
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    let mut html_output = String::new();
    let mut in_list = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            if in_list {
                html_output.push_str("</ul>\n");
                in_list = false;
            }
            continue;
        }

        if let Some(item_content) = parse_list_item(line) {
            if !in_list {
                html_output.push_str("<ul>\n");
                in_list = true;
            }
            let parsed_inline = parse_inline(item_content);
            html_output.push_str(&format!("<li>{}</li>\n", parsed_inline));
        } else {
            // Close list if previously inside one
            if in_list {
                html_output.push_str("</ul>\n");
                in_list = false;
            }

            if let Some((level, heading_content)) = parse_heading(line) {
                let parsed_inline = parse_inline(heading_content);
                html_output.push_str(&format!("<h{}>{}</h{}>\n", level, parsed_inline, level));
            } else {
                let parsed_inline = parse_inline(line);
                html_output.push_str(&format!("<p>{}</p>\n", parsed_inline));
            }
        }
    }

    // Close any dangling list at the end of input
    if in_list {
        html_output.push_str("</ul>\n");
    }

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_headings_h1_to_h6() {
        let markdown = "# Level 1\n## Level 2\n### Level 3\n###### Level 6";
        let expected = "<h1>Level 1</h1>\n<h2>Level 2</h2>\n<h3>Level 3</h3>\n<h6>Level 6</h6>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }

    #[test]
    fn test_heading_with_inline_styles() {
        let markdown = "## Heading with **Bold** and *Italic*";
        let expected = "<h2>Heading with <b>Bold</b> and <i>Italic</i></h2>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }

    #[test]
    fn test_unordered_list() {
        let markdown = "* First item\n* Second item with **Bold**\n- Third item with *Italic*";
        let expected = "<ul>\n<li>First item</li>\n<li>Second item with <b>Bold</b></li>\n<li>Third item with <i>Italic</i></li>\n</ul>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }

    #[test]
    fn test_list_surrounded_by_paragraphs() {
        let markdown = "Intro text\n* List item\nOutro text";
        let expected = "<p>Intro text</p>\n<ul>\n<li>List item</li>\n</ul>\n<p>Outro text</p>\n";
        assert_eq!(parse_markdown(markdown), expected);
    }
}
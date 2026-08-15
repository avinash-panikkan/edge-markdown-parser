/// Scans and parses inline elements: Images, Links, Bold, and Italics.
pub(crate) fn parse_inline(text: &str) -> String {
    let mut result = text.to_string();

    // 1. Parse Images: ![alt](url) -> <img src="url" alt="alt" />
    while let Some(start) = result.find("![") {
        if let Some(mid) = result[start..].find("](") {
            let actual_mid = start + mid;
            if let Some(end) = result[actual_mid..].find(')') {
                let actual_end = actual_mid + end;
                let alt = &result[start + 2..actual_mid];
                let url = &result[actual_mid + 2..actual_end];
                let replacement = format!("<img src=\"{}\" alt=\"{}\" />", url, alt);
                result.replace_range(start..=actual_end, &replacement);
                continue;
            }
        }
        break;
    }

    // 2. Parse Links: [text](url) -> <a href="url">text</a>
    while let Some(start) = result.find('[') {
        if let Some(mid) = result[start..].find("](") {
            let actual_mid = start + mid;
            if let Some(end) = result[actual_mid..].find(')') {
                let actual_end = actual_mid + end;
                let text_content = &result[start + 1..actual_mid];
                let url = &result[actual_mid + 2..actual_end];
                let replacement = format!("<a href=\"{}\">{}</a>", url, text_content);
                result.replace_range(start..=actual_end, &replacement);
                continue;
            }
        }
        break;
    }

    // 3. Parse Bold (**text**)
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let actual_end = start + 2 + end;
            let content = &result[start + 2..actual_end];
            let replacement = format!("<b>{}</b>", content);
            result.replace_range(start..actual_end + 2, &replacement);
        } else {
            break;
        }
    }

    // 4. Parse Italic (*text*)
    while let Some(start) = result.find('*') {
        if let Some(end) = result[start + 1..].find('*') {
            let actual_end = start + 1 + end;
            let content = &result[start + 1..actual_end];
            let replacement = format!("<i>{}</i>", content);
            result.replace_range(start..actual_end + 1, &replacement);
        } else {
            break;
        }
    }

    result
}
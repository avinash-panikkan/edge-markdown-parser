/// Scans and parses inline styles like Bold (**text**) and Italic (*text*).
pub(crate) fn parse_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Parse Bold (**text**)
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

    // Parse Italic (*text*)
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
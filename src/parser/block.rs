/// Detects heading level (1 to 6) and returns (level, content).
pub(crate) fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let hash_count = line.chars().take_while(|&c| c == '#').count();
    if (1..=6).contains(&hash_count) {
        let remainder = &line[hash_count..];
        if let Some(content) = remainder.strip_prefix(' ') {
            return Some((hash_count, content));
        }
    }
    None
}

/// Detects if a line is an unordered list item (* or -) and returns content.
pub(crate) fn parse_list_item(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if let Some(content) = trimmed.strip_prefix("* ") {
        Some(content)
    } else if let Some(content) = trimmed.strip_prefix("- ") {
        Some(content)
    } else {
        None
    }
}
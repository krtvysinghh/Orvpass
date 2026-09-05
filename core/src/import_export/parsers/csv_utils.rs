pub fn unescape_csv_field(field: &str) -> String {
    let trimmed = field.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].replace("\"\"", "\"")
    } else {
        trimmed.to_string()
    }
}

pub fn detect_csv_delimiter(content: &str) -> char {
    let comma_count = content.chars().take(500).filter(|c| *c == ',').count();
    let semi_count = content.chars().take(500).filter(|c| *c == ';').count();
    let tab_count = content.chars().take(500).filter(|c| *c == '\t').count();
    if tab_count > comma_count && tab_count > semi_count {
        '\t'
    } else if semi_count > comma_count {
        ';'
    } else {
        ','
    }
}

use comfy_table::{presets::UTF8_FULL, Table};

pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(headers);
    for row in rows {
        table.add_row(row);
    }
    table.to_string()
}

pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    format!("{}...", &s[..max_len.saturating_sub(3)])
}

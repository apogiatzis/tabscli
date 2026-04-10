use comfy_table::{Cell, Color, ContentArrangement, Table, presets::UTF8_FULL_CONDENSED};

use crate::cli::OutputFormat;
use crate::model::tab::Tab;

pub fn format_tabs(tabs: &[Tab], format: &OutputFormat) -> String {
    match format {
        OutputFormat::Table => format_table(tabs),
        OutputFormat::Json => format_json(tabs),
        OutputFormat::Csv => format_csv(tabs),
        OutputFormat::Plain => format_plain(tabs),
    }
}

fn format_table(tabs: &[Tab]) -> String {
    let show_browser = tabs.iter().any(|t| t.browser.is_some());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic);

    if show_browser {
        table.set_header(vec![
            Cell::new("#").fg(Color::DarkCyan),
            Cell::new("Browser").fg(Color::DarkCyan),
            Cell::new("Title").fg(Color::DarkCyan),
            Cell::new("Domain").fg(Color::DarkCyan),
            Cell::new("URL").fg(Color::DarkCyan),
        ]);
    } else {
        table.set_header(vec![
            Cell::new("#").fg(Color::DarkCyan),
            Cell::new("Title").fg(Color::DarkCyan),
            Cell::new("Domain").fg(Color::DarkCyan),
            Cell::new("URL").fg(Color::DarkCyan),
        ]);
    }

    for (i, tab) in tabs.iter().enumerate() {
        let domain = tab.domain();
        let title = truncate(&tab.title, 50);
        let url = truncate(&tab.url, 60);
        if show_browser {
            let browser = tab.browser.as_deref().unwrap_or("");
            table.add_row(vec![
                Cell::new(i + 1),
                Cell::new(browser).fg(Color::Cyan),
                Cell::new(title),
                Cell::new(domain).fg(Color::Green),
                Cell::new(url).fg(Color::DarkGrey),
            ]);
        } else {
            table.add_row(vec![
                Cell::new(i + 1),
                Cell::new(title),
                Cell::new(domain).fg(Color::Green),
                Cell::new(url).fg(Color::DarkGrey),
            ]);
        }
    }

    table.to_string()
}

fn format_json(tabs: &[Tab]) -> String {
    serde_json::to_string_pretty(tabs).unwrap_or_else(|_| "[]".to_string())
}

fn format_csv(tabs: &[Tab]) -> String {
    let mut lines = vec!["index,title,domain,url".to_string()];
    for (i, tab) in tabs.iter().enumerate() {
        lines.push(format!(
            "{},{},{},{}",
            i + 1,
            csv_escape(&tab.title),
            csv_escape(&tab.domain()),
            csv_escape(&tab.url),
        ));
    }
    lines.join("\n")
}

fn format_plain(tabs: &[Tab]) -> String {
    tabs.iter()
        .map(|t| format!("{}\t{}", t.title, t.url))
        .collect::<Vec<_>>()
        .join("\n")
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max.saturating_sub(3)).collect();
        format!("{}...", truncated)
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

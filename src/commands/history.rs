use std::collections::HashSet;

use anyhow::Result;
use chrono::Utc;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, ContentArrangement, Table};

use crate::chrome::Browser;
use crate::store::filesystem;

pub async fn run(
    browser: &Browser,
    limit: usize,
    restore: Option<String>,
    diff: Option<Vec<String>>,
) -> Result<()> {
    if let Some(snapshot_id) = restore {
        return restore_snapshot(browser, &snapshot_id).await;
    }

    if let Some(ids) = diff {
        if ids.len() == 2 {
            return diff_snapshots(&ids[0], &ids[1]);
        }
        anyhow::bail!("--diff requires exactly two snapshot IDs");
    }

    // List snapshots
    let snapshots = filesystem::list_snapshots(limit)?;
    if snapshots.is_empty() {
        println!("No snapshots. Use `tabs snapshot` to create one.");
        return Ok(());
    }

    let now = Utc::now();
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").fg(Color::DarkCyan),
            Cell::new("Tabs").fg(Color::DarkCyan),
            Cell::new("Label").fg(Color::DarkCyan),
            Cell::new("Age").fg(Color::DarkCyan),
        ]);

    for s in &snapshots {
        let age = format_age(now.signed_duration_since(s.created_at));
        table.add_row(vec![
            Cell::new(&s.id),
            Cell::new(s.tab_count),
            Cell::new(s.label.as_deref().unwrap_or("")).fg(Color::Green),
            Cell::new(age).fg(Color::DarkGrey),
        ]);
    }

    println!("{}", table);
    println!("\n{} snapshot(s)", snapshots.len());
    Ok(())
}

async fn restore_snapshot(browser: &Browser, id: &str) -> Result<()> {
    let snapshot = filesystem::load_snapshot(id)?;

    println!("Restoring snapshot {} ({} tabs)...", id, snapshot.tab_count);

    let mut opened = 0;
    for tab in &snapshot.tabs {
        match browser.open_tab(&tab.url).await {
            Ok(_) => opened += 1,
            Err(e) => eprintln!("Failed to open '{}': {}", tab.url, e),
        }
    }

    println!("Opened {} tab(s).", opened);
    Ok(())
}

fn diff_snapshots(id1: &str, id2: &str) -> Result<()> {
    let snap1 = filesystem::load_snapshot(id1)?;
    let snap2 = filesystem::load_snapshot(id2)?;

    let urls1: HashSet<&str> = snap1.tabs.iter().map(|t| t.url.as_str()).collect();
    let urls2: HashSet<&str> = snap2.tabs.iter().map(|t| t.url.as_str()).collect();

    let removed: Vec<_> = snap1
        .tabs
        .iter()
        .filter(|t| !urls2.contains(t.url.as_str()))
        .collect();
    let added: Vec<_> = snap2
        .tabs
        .iter()
        .filter(|t| !urls1.contains(t.url.as_str()))
        .collect();

    println!("Diff: {} → {}", id1, id2);
    println!();

    if !removed.is_empty() {
        println!("Removed ({}):", removed.len());
        for tab in &removed {
            println!("  - {} — {}", tab.title, tab.domain());
        }
    }

    if !added.is_empty() {
        println!("Added ({}):", added.len());
        for tab in &added {
            println!("  + {} — {}", tab.title, tab.domain());
        }
    }

    if removed.is_empty() && added.is_empty() {
        println!("No differences.");
    }

    Ok(())
}

fn format_age(duration: chrono::Duration) -> String {
    let secs = duration.num_seconds();
    if secs < 60 {
        format!("{}s ago", secs)
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    }
}

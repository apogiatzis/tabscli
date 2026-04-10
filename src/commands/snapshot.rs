use anyhow::Result;
use chrono::Utc;

use crate::chrome::Browser;
use crate::model::session::Snapshot;
use crate::model::tab::SavedTab;
use crate::store::filesystem;

pub async fn run(browser: &Browser, label: Option<String>) -> Result<()> {
    create_snapshot(browser, label).await?;
    Ok(())
}

pub async fn create_snapshot(browser: &Browser, label: Option<String>) -> Result<()> {
    let tabs = browser.list_tabs().await?;
    let now = Utc::now();
    let id = now.format("%Y%m%dT%H%M%S").to_string();

    let saved_tabs: Vec<SavedTab> = tabs.iter().map(SavedTab::from).collect();
    let tab_count = saved_tabs.len();

    let snapshot = Snapshot {
        id: id.clone(),
        label: label.clone(),
        tabs: saved_tabs,
        tab_count,
        created_at: now,
    };

    filesystem::save_snapshot(&snapshot)?;
    let label_str = label.map(|l| format!(", label: \"{}\"", l)).unwrap_or_default();
    println!("Snapshot saved: {} ({} tabs{})", id, tab_count, label_str);
    Ok(())
}

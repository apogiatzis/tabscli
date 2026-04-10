use std::path::Path;

use anyhow::Result;
use chrono::Utc;

use crate::chrome::Browser;
use crate::model::session::Session;
use crate::model::tab::SavedTab;
use crate::store::filesystem;

pub async fn run(
    browser: &Browser,
    name: Option<String>,
    file: Option<String>,
    tags: Vec<String>,
    overwrite: bool,
) -> Result<()> {
    let tabs = browser.list_tabs().await?;
    let saved_tabs: Vec<SavedTab> = tabs.iter().map(SavedTab::from).collect();
    let now = Utc::now();

    match file {
        Some(path) => {
            if !overwrite && Path::new(&path).exists() {
                anyhow::bail!("File '{}' already exists. Use --overwrite to replace it.", path);
            }
            let session_name = name.unwrap_or_else(|| {
                Path::new(&path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed")
                    .to_string()
            });
            let session = Session {
                name: session_name,
                tags,
                tabs: saved_tabs,
                created_at: now,
                updated_at: now,
            };
            let json = serde_json::to_string_pretty(&session)?;
            std::fs::write(&path, json)?;
            println!("Saved {} tabs to {}", tabs.len(), path);
        }
        None => {
            let name = name.ok_or_else(|| anyhow::anyhow!("Provide a session name or --file <path>"))?;
            if !overwrite && filesystem::load_session(&name).is_ok() {
                anyhow::bail!(
                    "Session '{}' already exists. Use --overwrite to replace it.",
                    name
                );
            }
            let session = Session {
                name: name.clone(),
                tags,
                tabs: saved_tabs,
                created_at: now,
                updated_at: now,
            };
            filesystem::save_session(&session)?;
            println!("Session '{}' saved ({} tabs)", name, tabs.len());
        }
    }

    Ok(())
}

use anyhow::Result;

use crate::cli::SelectAction;
use crate::chrome::Browser;
use crate::tui::picker;

pub async fn run(browser: &Browser, action: SelectAction, multi: bool) -> Result<()> {
    let tabs = browser.list_tabs().await?;

    if tabs.is_empty() {
        println!("No tabs found.");
        return Ok(());
    }

    let selected = picker::run_picker(tabs, multi)?;

    if selected.is_empty() {
        println!("No tabs selected.");
        return Ok(());
    }

    match action {
        SelectAction::Activate => {
            if let Some(tab) = selected.last() {
                browser.activate_tab(&tab.id).await?;
                println!("Activated: {}", tab.title);
            }
        }
        SelectAction::Close => {
            for tab in &selected {
                browser.close_tab(&tab.id).await?;
            }
            println!("Closed {} tab(s).", selected.len());
        }
        SelectAction::Save => {
            println!("Enter session name: ");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name)?;
            let name = name.trim().to_string();
            if name.is_empty() {
                println!("Cancelled.");
                return Ok(());
            }

            let saved_tabs = selected
                .iter()
                .map(crate::model::tab::SavedTab::from)
                .collect();
            let now = chrono::Utc::now();
            let session = crate::model::session::Session {
                name: name.clone(),
                tags: vec![],
                tabs: saved_tabs,
                created_at: now,
                updated_at: now,
            };
            crate::store::filesystem::save_session(&session)?;
            println!("Session '{}' saved ({} tabs)", name, selected.len());
        }
    }

    Ok(())
}

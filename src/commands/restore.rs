use anyhow::Result;

use crate::chrome::Browser;
use crate::model::session::Session;
use crate::store::filesystem;

pub async fn run(
    browser: &Browser,
    name: Option<String>,
    file: Option<String>,
    new_window: bool,
) -> Result<()> {
    let session = match (name, file) {
        (_, Some(path)) => {
            let json = std::fs::read_to_string(&path)
                .map_err(|_| anyhow::anyhow!("Could not read file: {}", path))?;
            let session: Session = serde_json::from_str(&json)
                .map_err(|e| anyhow::anyhow!("Invalid session JSON: {}", e))?;
            session
        }
        (Some(name), None) => filesystem::load_session(&name)?,
        (None, None) => anyhow::bail!("Provide a session name or --file <path>"),
    };

    println!(
        "Restoring session '{}' ({} tabs)...",
        session.name,
        session.tabs.len()
    );

    let window_id = if new_window {
        Some(browser.new_window().await?)
    } else {
        None
    };

    let mut opened = 0;
    for (i, tab) in session.tabs.iter().enumerate() {
        let result = if let Some(ref wid) = window_id {
            if i == 0 {
                // First tab: navigate the new window's blank tab instead of creating another
                browser.open_tab_in_window(&tab.url, wid).await
            } else {
                browser.open_tab_in_window(&tab.url, wid).await
            }
        } else {
            browser.open_tab(&tab.url).await
        };

        match result {
            Ok(_) => opened += 1,
            Err(e) => eprintln!("Failed to open '{}': {}", tab.url, e),
        }
    }

    println!(
        "Opened {} tab(s){}.",
        opened,
        if new_window { " in new window" } else { "" }
    );
    Ok(())
}

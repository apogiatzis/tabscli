pub mod applescript;
pub mod cdp;
pub mod types;

use crate::model::tab::Tab;
use anyhow::Result;

pub enum Browser {
    AppleScript(applescript::AppleScriptClient),
    Cdp(cdp::CdpClient),
}

impl Browser {
    /// Create an AppleScript browser backend targeting the given app.
    pub fn applescript(app_name: &str) -> Self {
        Browser::AppleScript(applescript::AppleScriptClient::new(app_name))
    }

    /// Create a CDP browser backend.
    pub fn cdp(port: u16) -> Self {
        Browser::Cdp(cdp::CdpClient::new(port))
    }

    pub async fn list_tabs(&self) -> Result<Vec<Tab>> {
        match self {
            Browser::AppleScript(c) => c.list_tabs().await,
            Browser::Cdp(c) => c.list_tabs().await,
        }
    }

    pub async fn close_tab(&self, id: &str) -> Result<()> {
        match self {
            Browser::AppleScript(c) => c.close_tab(id).await,
            Browser::Cdp(c) => c.close_tab(id).await,
        }
    }

    pub async fn activate_tab(&self, id: &str) -> Result<()> {
        match self {
            Browser::AppleScript(c) => c.activate_tab(id).await,
            Browser::Cdp(c) => c.activate_tab(id).await,
        }
    }

    pub async fn open_tab(&self, url: &str) -> Result<()> {
        match self {
            Browser::AppleScript(c) => c.open_tab(url).await,
            Browser::Cdp(c) => {
                c.open_tab(url).await?;
                Ok(())
            }
        }
    }

    /// Create a new browser window and return its ID.
    pub async fn new_window(&self) -> Result<String> {
        match self {
            Browser::AppleScript(c) => c.new_window().await,
            Browser::Cdp(_) => {
                anyhow::bail!("new_window not supported with CDP backend")
            }
        }
    }

    /// Open a tab in a specific window.
    pub async fn open_tab_in_window(&self, url: &str, window_id: &str) -> Result<()> {
        match self {
            Browser::AppleScript(c) => c.open_tab_in_window(url, window_id).await,
            Browser::Cdp(_) => self.open_tab(url).await,
        }
    }
}

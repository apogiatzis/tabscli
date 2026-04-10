use anyhow::{Context, Result};
use reqwest::Client;

use super::types::CdpTab;
use crate::model::tab::Tab;

pub struct CdpClient {
    base_url: String,
    client: Client,
}

impl CdpClient {
    pub fn new(port: u16) -> Self {
        Self {
            base_url: format!("http://localhost:{}", port),
            client: Client::new(),
        }
    }

    /// List all open browser tabs (filters to type "page").
    pub async fn list_tabs(&self) -> Result<Vec<Tab>> {
        let url = format!("{}/json", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Chrome DevTools. Is Chrome running with --remote-debugging-port?")?;

        let cdp_tabs: Vec<CdpTab> = resp
            .json()
            .await
            .context("Failed to parse Chrome DevTools response")?;

        let tabs = cdp_tabs
            .into_iter()
            .filter(|t| t.target_type == "page")
            .enumerate()
            .map(|(idx, t)| Tab::from_cdp(t, idx))
            .collect();

        Ok(tabs)
    }

    /// Close a tab by its CDP target ID.
    pub async fn close_tab(&self, id: &str) -> Result<()> {
        let url = format!("{}/json/close/{}", self.base_url, id);
        self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to close tab {}", id))?;
        Ok(())
    }

    /// Activate (focus) a tab by its CDP target ID.
    pub async fn activate_tab(&self, id: &str) -> Result<()> {
        let url = format!("{}/json/activate/{}", self.base_url, id);
        self.client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to activate tab {}", id))?;
        Ok(())
    }

    /// Open a new tab with the given URL.
    pub async fn open_tab(&self, tab_url: &str) -> Result<CdpTab> {
        let url = format!("{}/json/new?{}", self.base_url, tab_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to open new tab")?;

        let tab: CdpTab = resp.json().await.context("Failed to parse new tab response")?;
        Ok(tab)
    }
}

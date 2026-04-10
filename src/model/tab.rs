use serde::{Deserialize, Serialize};
use url::Url;

use crate::chrome::types::CdpTab;

/// A browser tab.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub index: usize,
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
}

impl Tab {
    pub fn from_cdp(cdp: CdpTab, index: usize) -> Self {
        Self {
            id: cdp.id,
            index,
            title: cdp.title,
            url: cdp.url,
            browser: None,
        }
    }

    /// Extract the domain (hostname) from the tab URL.
    pub fn domain(&self) -> String {
        Url::parse(&self.url)
            .ok()
            .and_then(|u| u.host_str().map(String::from))
            .unwrap_or_default()
    }
}

/// A saved tab (no ephemeral CDP ID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTab {
    pub title: String,
    pub url: String,
}

impl From<&Tab> for SavedTab {
    fn from(tab: &Tab) -> Self {
        Self {
            title: tab.title.clone(),
            url: tab.url.clone(),
        }
    }
}

impl SavedTab {
    pub fn domain(&self) -> String {
        Url::parse(&self.url)
            .ok()
            .and_then(|u| u.host_str().map(String::from))
            .unwrap_or_default()
    }
}

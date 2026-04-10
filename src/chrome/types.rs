use serde::Deserialize;

/// Raw tab/target as returned by Chrome DevTools Protocol /json endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CdpTab {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub target_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub dev_tools_frontend_url: String,
    #[serde(default)]
    pub web_socket_debugger_url: String,
    #[serde(default)]
    pub favicon_url: Option<String>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::tab::SavedTab;

/// A user-saved named collection of tabs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub tabs: Vec<SavedTab>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A point-in-time snapshot of all open tabs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Timestamp-based ID (e.g. "20260407T151200").
    pub id: String,
    pub label: Option<String>,
    pub tabs: Vec<SavedTab>,
    pub tab_count: usize,
    pub created_at: DateTime<Utc>,
}

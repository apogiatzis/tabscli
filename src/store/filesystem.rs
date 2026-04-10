use anyhow::{Context, Result};
use std::fs;

use crate::config;
use crate::model::session::{Session, Snapshot};

/// Validate that a name is safe for use as a filename (no path traversal).
fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Name cannot be empty");
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        anyhow::bail!(
            "Invalid name '{}': must not contain '/', '\\', or '..'",
            name
        );
    }
    Ok(())
}

/// Save a named session to disk.
pub fn save_session(session: &Session) -> Result<()> {
    validate_name(&session.name)?;
    let dir = config::sessions_dir()?;
    let path = dir.join(format!("{}.json", session.name));
    let json = serde_json::to_string_pretty(session)?;
    fs::write(&path, json).context("Failed to write session file")?;
    Ok(())
}

/// Load a named session from disk.
pub fn load_session(name: &str) -> Result<Session> {
    validate_name(name)?;
    let dir = config::sessions_dir()?;
    let path = dir.join(format!("{}.json", name));
    let json = fs::read_to_string(&path).context(format!("Session '{}' not found", name))?;
    let session: Session = serde_json::from_str(&json)?;
    Ok(session)
}

/// List all saved sessions (metadata only).
pub fn list_sessions() -> Result<Vec<SessionMeta>> {
    let dir = config::sessions_dir()?;
    let mut sessions = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(session) = serde_json::from_str::<Session>(&json) {
                    sessions.push(SessionMeta {
                        name: session.name,
                        tags: session.tags,
                        tab_count: session.tabs.len(),
                        created_at: session.created_at,
                        updated_at: session.updated_at,
                    });
                }
            }
        }
    }
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}

/// Delete a saved session.
pub fn delete_session(name: &str) -> Result<()> {
    validate_name(name)?;
    let dir = config::sessions_dir()?;
    let path = dir.join(format!("{}.json", name));
    fs::remove_file(&path).context(format!("Session '{}' not found", name))?;
    Ok(())
}

/// Save a snapshot to disk.
pub fn save_snapshot(snapshot: &Snapshot) -> Result<()> {
    let dir = config::snapshots_dir()?;
    let path = dir.join(format!("{}.json", snapshot.id));
    let json = serde_json::to_string_pretty(snapshot)?;
    fs::write(&path, json).context("Failed to write snapshot file")?;
    Ok(())
}

/// Load a snapshot by ID.
pub fn load_snapshot(id: &str) -> Result<Snapshot> {
    let dir = config::snapshots_dir()?;
    let path = dir.join(format!("{}.json", id));
    let json = fs::read_to_string(&path).context(format!("Snapshot '{}' not found", id))?;
    let snapshot: Snapshot = serde_json::from_str(&json)?;
    Ok(snapshot)
}

/// List snapshots, newest first.
pub fn list_snapshots(limit: usize) -> Result<Vec<SnapshotMeta>> {
    let dir = config::snapshots_dir()?;
    let mut snapshots = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(snap) = serde_json::from_str::<Snapshot>(&json) {
                    snapshots.push(SnapshotMeta {
                        id: snap.id,
                        label: snap.label,
                        tab_count: snap.tab_count,
                        created_at: snap.created_at,
                    });
                }
            }
        }
    }
    snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    snapshots.truncate(limit);
    Ok(snapshots)
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SessionMeta {
    pub name: String,
    pub tags: Vec<String>,
    pub tab_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct SnapshotMeta {
    pub id: String,
    pub label: Option<String>,
    pub tab_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

use anyhow::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

/// Return the data directory for tabs-cli.
/// Creates it if it doesn't exist.
pub fn data_dir() -> Result<PathBuf> {
    let proj = ProjectDirs::from("", "", "tabs-cli")
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    let dir = proj.data_dir().to_path_buf();
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Return the sessions subdirectory.
pub fn sessions_dir() -> Result<PathBuf> {
    let dir = data_dir()?.join("sessions");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Return the snapshots subdirectory.
pub fn snapshots_dir() -> Result<PathBuf> {
    let dir = data_dir()?.join("snapshots");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default)]
struct Config {
    sessions_dir: Option<PathBuf>,
    snapshots_dir: Option<PathBuf>,
}

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "tabscli")
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

/// Load configuration from the XDG config directory.
/// Returns defaults if the config file doesn't exist.
fn load_config() -> Result<Config> {
    let proj = project_dirs()?;
    let config_path = proj.config_dir().join("config.toml");
    if config_path.exists() {
        let contents = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Resolve a path, expanding `~` to the home directory.
fn resolve_path(path: &Path) -> PathBuf {
    if let Ok(stripped) = path.strip_prefix("~")
        && let Some(home) = dirs_home()
    {
        return home.join(stripped);
    }
    path.to_path_buf()
}

fn dirs_home() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf())
}

/// Return the sessions directory.
/// Uses config override if set, otherwise defaults to `<data_dir>/sessions`.
pub fn sessions_dir() -> Result<PathBuf> {
    let config = load_config()?;
    let dir = match config.sessions_dir {
        Some(ref p) => resolve_path(p),
        None => project_dirs()?.data_dir().join("sessions"),
    };
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Return the snapshots directory.
/// Uses config override if set, otherwise defaults to `<data_dir>/snapshots`.
pub fn snapshots_dir() -> Result<PathBuf> {
    let config = load_config()?;
    let dir = match config.snapshots_dir {
        Some(ref p) => resolve_path(p),
        None => project_dirs()?.data_dir().join("snapshots"),
    };
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

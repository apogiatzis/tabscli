use anyhow::{Context, Result};
use tokio::process::Command;

use crate::model::tab::Tab;

pub struct AppleScriptClient {
    app_name: String,
}

impl AppleScriptClient {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn is_running(&self) -> bool {
        let script = format!(
            r#"tell application "System Events" to (name of processes) contains "{app}""#,
            app = self.app_name
        );
        run_osascript(&script)
            .await
            .map(|s| s.trim() == "true")
            .unwrap_or(false)
    }

    pub async fn list_tabs(&self) -> Result<Vec<Tab>> {
        if !self.is_running().await {
            anyhow::bail!("{} is not running", self.app_name);
        }
        let script = format!(
            r#"
tell application "{app}"
    set output to ""
    set wCount to count of windows
    repeat with wIdx from 1 to wCount
        set w to window wIdx
        set tabTitles to title of every tab of w
        set tabUrls to URL of every tab of w
        set tabIds to id of every tab of w
        set tCount to count of tabTitles
        repeat with i from 1 to tCount
            set output to output & (item i of tabIds) & "|||" & (item i of tabTitles) & "|||" & (item i of tabUrls) & linefeed
        end repeat
    end repeat
    return output
end tell
"#,
            app = self.app_name
        );
        let output = run_osascript(&script).await?;
        let mut tabs = Vec::new();
        for (idx, line) in output.lines().enumerate() {
            let parts: Vec<&str> = line.splitn(3, "|||").collect();
            if parts.len() == 3 {
                tabs.push(Tab {
                    id: parts[0].trim().to_string(),
                    index: idx,
                    title: parts[1].to_string(),
                    url: parts[2].to_string(),
                });
            }
        }
        Ok(tabs)
    }

    pub async fn close_tab(&self, id: &str) -> Result<()> {
        let script = format!(
            r#"
tell application "{app}"
    repeat with w in windows
        repeat with i from (count of tabs of w) to 1 by -1
            set t to tab i of w
            if (id of t as text) is "{id}" then
                close t
                return "ok"
            end if
        end repeat
    end repeat
    return "not found"
end tell
"#,
            app = self.app_name,
            id = id
        );
        let result = run_osascript(&script).await?;
        if result.trim() == "not found" {
            anyhow::bail!("Tab not found: {}", id);
        }
        Ok(())
    }

    pub async fn activate_tab(&self, id: &str) -> Result<()> {
        let script = format!(
            r#"
tell application "{app}"
    repeat with w in windows
        repeat with i from 1 to count of tabs of w
            if (id of tab i of w as text) is "{id}" then
                set active tab index of w to i
                set index of w to 1
                activate
                return "ok"
            end if
        end repeat
    end repeat
    return "not found"
end tell
"#,
            app = self.app_name,
            id = id
        );
        let result = run_osascript(&script).await?;
        if result.trim() == "not found" {
            anyhow::bail!("Tab not found: {}", id);
        }
        Ok(())
    }

    pub async fn open_tab(&self, url: &str) -> Result<()> {
        let escaped_url = sanitize_applescript_string(url);
        let script = format!(
            r#"
tell application "{app}"
    tell window 1
        make new tab with properties {{URL:"{url}"}}
    end tell
end tell
"#,
            app = self.app_name,
            url = escaped_url
        );
        run_osascript(&script).await?;
        Ok(())
    }

    pub async fn new_window(&self) -> Result<String> {
        let script = format!(
            r#"
tell application "{app}"
    set w to make new window
    return id of w as text
end tell
"#,
            app = self.app_name
        );
        let output = run_osascript(&script).await?;
        Ok(output.trim().to_string())
    }

    pub async fn open_tab_in_window(&self, url: &str, window_id: &str) -> Result<()> {
        let escaped_url = sanitize_applescript_string(url);
        let script = format!(
            r#"
tell application "{app}"
    repeat with w in windows
        if (id of w as text) is "{window_id}" then
            tell w to make new tab with properties {{URL:"{url}"}}
            return "ok"
        end if
    end repeat
end tell
"#,
            app = self.app_name,
            url = escaped_url,
            window_id = window_id
        );
        run_osascript(&script).await?;
        Ok(())
    }
}

/// Sanitize a string for safe interpolation into an AppleScript quoted string.
/// Escapes backslashes and double quotes to prevent injection.
fn sanitize_applescript_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

async fn run_osascript(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .await
        .context("Failed to run osascript. Is this macOS?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not running") || stderr.contains("No result was returned") {
            anyhow::bail!("Browser is not running. Please open it first.");
        }
        anyhow::bail!("osascript error: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

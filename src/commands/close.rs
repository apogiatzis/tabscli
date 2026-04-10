use std::collections::HashSet;
use std::io::{self, Write};

use anyhow::Result;
use regex::Regex;

use crate::chrome::Browser;
use crate::model::tab::Tab;

pub async fn run(
    browser: &Browser,
    pattern: Option<String>,
    domain: Option<String>,
    duplicates: bool,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    let tabs = browser.list_tabs().await?;

    let to_close: Vec<&Tab> = if duplicates {
        find_duplicates(&tabs)
    } else if let Some(ref p) = pattern {
        let re = Regex::new(p)?;
        tabs.iter()
            .filter(|t| re.is_match(&t.title) || re.is_match(&t.url))
            .collect()
    } else if let Some(ref d) = domain {
        let d_lower = d.to_lowercase();
        tabs.iter()
            .filter(|t| t.domain().to_lowercase().contains(&d_lower))
            .collect()
    } else {
        anyhow::bail!("Provide a pattern, --domain, or --duplicates flag");
    };

    if to_close.is_empty() {
        println!("No matching tabs found.");
        return Ok(());
    }

    println!("Tabs to close ({}):", to_close.len());
    for tab in &to_close {
        println!("  {} — {}", tab.title, tab.domain());
    }

    if dry_run {
        println!("\n(dry run — no tabs were closed)");
        return Ok(());
    }

    if !force {
        print!("\nClose {} tab(s)? [y/N] ", to_close.len());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Auto-snapshot before closing
    if let Err(e) =
        crate::commands::snapshot::create_snapshot(browser, Some("pre-close".to_string())).await
    {
        eprintln!("Warning: auto-snapshot failed: {}", e);
    }

    let mut closed = 0;
    for tab in &to_close {
        if let Err(e) = browser.close_tab(&tab.id).await {
            eprintln!("Failed to close '{}': {}", tab.title, e);
        } else {
            closed += 1;
        }
    }
    println!("Closed {} tab(s).", closed);
    Ok(())
}

fn find_duplicates(tabs: &[Tab]) -> Vec<&Tab> {
    let mut seen = HashSet::new();
    let mut dupes = Vec::new();
    for tab in tabs {
        if !seen.insert(&tab.url) {
            dupes.push(tab);
        }
    }
    dupes
}

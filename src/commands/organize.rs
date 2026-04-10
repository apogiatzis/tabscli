use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::chrome::Browser;
use crate::model::tab::Tab;

pub async fn run(
    browser: &Browser,
    by_domain: bool,
    deduplicate: bool,
    dry_run: bool,
) -> Result<()> {
    let tabs = browser.list_tabs().await?;

    if !by_domain && !deduplicate {
        anyhow::bail!("Specify --by-domain and/or --deduplicate");
    }

    if deduplicate {
        let mut seen = HashSet::new();
        let dupes: Vec<&Tab> = tabs.iter().filter(|t| !seen.insert(&t.url)).collect();

        if dupes.is_empty() {
            println!("No duplicate tabs found.");
        } else {
            println!("Duplicate tabs ({}):", dupes.len());
            for tab in &dupes {
                println!("  {} — {}", tab.title, tab.domain());
            }
            if !dry_run {
                for tab in &dupes {
                    browser.close_tab(&tab.id).await?;
                }
                println!("Closed {} duplicate tab(s).", dupes.len());
            } else {
                println!("(dry run — no tabs were closed)");
            }
        }
    }

    if by_domain {
        let mut groups: HashMap<String, Vec<&Tab>> = HashMap::new();
        for tab in &tabs {
            groups.entry(tab.domain()).or_default().push(tab);
        }

        println!("\nTabs by domain ({} domains):", groups.len());
        let mut domains: Vec<_> = groups.keys().collect();
        domains.sort();
        for domain in domains {
            let tabs = &groups[domain];
            println!("\n  {} ({} tabs):", domain, tabs.len());
            for tab in tabs {
                println!("    {}", tab.title);
            }
        }

        if dry_run {
            println!("\n(dry run — no changes applied)");
        }
    }

    Ok(())
}

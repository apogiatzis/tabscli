use anyhow::Result;

use crate::chrome::Browser;
use crate::cli::{OutputFormat, SortField};
use crate::model::tab::Tab;
use crate::output::format_tabs;

pub async fn run(
    browsers: &[Browser],
    format: OutputFormat,
    domain: Option<String>,
    search: Option<String>,
    sort: SortField,
) -> Result<()> {
    let mut tabs = crate::chrome::list_tabs_from_all(browsers).await?;

    // Filter by domain
    if let Some(ref d) = domain {
        let d_lower = d.to_lowercase();
        tabs.retain(|t| t.domain().to_lowercase().contains(&d_lower));
    }

    // Filter by search query (title or URL)
    if let Some(ref q) = search {
        let q_lower = q.to_lowercase();
        tabs.retain(|t| {
            t.title.to_lowercase().contains(&q_lower) || t.url.to_lowercase().contains(&q_lower)
        });
    }

    // Sort
    sort_tabs(&mut tabs, &sort);

    if tabs.is_empty() {
        println!("No tabs found.");
        return Ok(());
    }

    println!("{}", format_tabs(&tabs, &format));
    println!("\n{} tab(s)", tabs.len());
    Ok(())
}

fn sort_tabs(tabs: &mut [Tab], sort: &SortField) {
    match sort {
        SortField::Window => {} // Already in window order
        SortField::Title => {
            tabs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
        }
        SortField::Url => tabs.sort_by(|a, b| a.url.cmp(&b.url)),
        SortField::Domain => tabs.sort_by_key(|a| a.domain()),
    }
}

pub mod close;
pub mod history;
pub mod list;
pub mod organize;
pub mod restore;
pub mod save;
pub mod select;
pub mod sessions;
pub mod setup;
pub mod snapshot;

use crate::cli::{Cli, Commands};
use anyhow::Result;

pub async fn run(cli: Cli) -> Result<()> {
    let browser = cli.make_browser();

    match cli.command {
        Some(Commands::List {
            format,
            domain,
            search,
            sort,
            json,
        }) => {
            let fmt = if json {
                crate::cli::OutputFormat::Json
            } else {
                format
            };
            list::run(&browser, fmt, domain, search, sort).await
        }
        Some(Commands::Close {
            pattern,
            domain,
            duplicates,
            dry_run,
            force,
        }) => close::run(&browser, pattern, domain, duplicates, dry_run, force).await,
        Some(Commands::Select { action, multi }) => select::run(&browser, action, multi).await,
        Some(Commands::Save {
            name,
            file,
            tags,
            overwrite,
        }) => save::run(&browser, name, file, tags, overwrite).await,
        Some(Commands::Restore {
            name,
            file,
            new_window,
        }) => restore::run(&browser, name, file, new_window).await,
        Some(Commands::Sessions { command }) => sessions::run(command).await,
        Some(Commands::Organize {
            by_domain,
            deduplicate,
            dry_run,
        }) => organize::run(&browser, by_domain, deduplicate, dry_run).await,
        Some(Commands::Snapshot { label }) => snapshot::run(&browser, label).await,
        Some(Commands::History {
            limit,
            restore,
            diff,
        }) => history::run(&browser, limit, restore, diff).await,
        Some(Commands::Setup) => {
            setup::run();
            Ok(())
        }
        None => {
            list::run(
                &browser,
                crate::cli::OutputFormat::Table,
                None,
                None,
                crate::cli::SortField::Window,
            )
            .await
        }
    }
}

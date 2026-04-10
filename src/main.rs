mod chrome;
mod cli;
mod commands;
mod config;
mod model;
mod output;
mod store;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli).await
}

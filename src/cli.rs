use clap::{Parser, Subcommand, ValueEnum};

use crate::chrome::Browser;

#[derive(Parser)]
#[command(
    name = "tabscli",
    about = "Manage Chromium browser tabs from the terminal"
)]
#[command(version)]
pub struct Cli {
    /// Target browser
    #[arg(short, long, default_value = "chrome", global = true)]
    pub browser: BrowserTarget,

    /// Query all supported browsers
    #[arg(short, long, global = true)]
    pub all: bool,

    /// Communication backend: applescript (default, macOS) or cdp
    #[arg(long, default_value = "applescript", global = true)]
    pub backend: Backend,

    /// Chrome DevTools Protocol port (only used with --backend cdp)
    #[arg(long, default_value = "9222", global = true)]
    pub port: u16,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn make_browser(&self) -> Browser {
        match self.backend {
            Backend::Applescript => Browser::applescript(self.browser.app_name()),
            Backend::Cdp => Browser::cdp(self.port),
        }
    }

    pub fn make_browsers(&self) -> Vec<Browser> {
        if self.all {
            BrowserTarget::all()
                .iter()
                .map(|t| match self.backend {
                    Backend::Applescript => Browser::applescript(t.app_name()),
                    Backend::Cdp => Browser::cdp(self.port),
                })
                .collect()
        } else {
            vec![self.make_browser()]
        }
    }
}

#[derive(ValueEnum, Clone, Default)]
pub enum BrowserTarget {
    #[default]
    Chrome,
    Brave,
    Edge,
    Chromium,
}

impl BrowserTarget {
    pub fn app_name(&self) -> &str {
        match self {
            BrowserTarget::Chrome => "Google Chrome",
            BrowserTarget::Brave => "Brave Browser",
            BrowserTarget::Edge => "Microsoft Edge",
            BrowserTarget::Chromium => "Chromium",
        }
    }

    pub fn all() -> &'static [BrowserTarget] {
        &[
            BrowserTarget::Chrome,
            BrowserTarget::Brave,
            BrowserTarget::Edge,
            BrowserTarget::Chromium,
        ]
    }
}

#[derive(ValueEnum, Clone, Default)]
pub enum Backend {
    #[default]
    Applescript,
    Cdp,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all open tabs
    List {
        /// Output format
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,

        /// Filter by domain
        #[arg(short, long)]
        domain: Option<String>,

        /// Search title/URL
        #[arg(short, long)]
        search: Option<String>,

        /// Sort by field
        #[arg(long, default_value = "window")]
        sort: SortField,

        /// Shorthand for --format json
        #[arg(long)]
        json: bool,
    },

    /// Close tabs matching a pattern
    Close {
        /// Regex pattern to match against title/URL
        pattern: Option<String>,

        /// Close tabs from specific domain
        #[arg(short, long)]
        domain: Option<String>,

        /// Close duplicate URLs (keep first occurrence)
        #[arg(long)]
        duplicates: bool,

        /// Preview what would be closed
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Interactive fuzzy tab picker
    Select {
        /// Action on selection: activate, close, save
        #[arg(short, long, default_value = "activate")]
        action: SelectAction,

        /// Enable multi-select
        #[arg(short, long)]
        multi: bool,
    },

    /// Save current tabs as a named session or to a file
    Save {
        /// Session name
        name: Option<String>,

        /// Save to a JSON file instead of the data directory
        #[arg(long, short)]
        file: Option<String>,

        /// Tags for the session
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Overwrite existing session or file
        #[arg(long)]
        overwrite: bool,
    },

    /// Restore tabs from a saved session or file
    Restore {
        /// Session name (from saved sessions)
        name: Option<String>,

        /// Restore from a JSON session file
        #[arg(long, short)]
        file: Option<String>,

        /// Open tabs in a new window
        #[arg(long)]
        new_window: bool,
    },

    /// Manage saved sessions
    Sessions {
        #[command(subcommand)]
        command: Option<SessionCommands>,
    },

    /// Organize open tabs
    Organize {
        /// Group tabs by domain into separate windows
        #[arg(long)]
        by_domain: bool,

        /// Remove duplicate URLs
        #[arg(long)]
        deduplicate: bool,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },

    /// Save a point-in-time snapshot
    Snapshot {
        /// Optional label
        #[arg(long)]
        label: Option<String>,
    },

    /// View and manage snapshot history
    History {
        /// Number of snapshots to show
        #[arg(short = 'n', long, default_value = "20")]
        limit: usize,

        /// Restore tabs from a snapshot
        #[arg(long)]
        restore: Option<String>,

        /// Diff two snapshots (provide two IDs)
        #[arg(long, num_args = 2)]
        diff: Option<Vec<String>>,
    },

    /// Show setup instructions for Chrome DevTools Protocol
    Setup,
}

#[derive(Subcommand)]
pub enum SessionCommands {
    /// Delete a saved session
    Delete {
        /// Session name to delete
        name: String,
    },
}

#[derive(ValueEnum, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
    Plain,
}

#[derive(ValueEnum, Clone, Default)]
pub enum SortField {
    #[default]
    Window,
    Title,
    Url,
    Domain,
}

#[derive(ValueEnum, Clone, Default)]
pub enum SelectAction {
    #[default]
    Activate,
    Close,
    Save,
}

use anyhow::Result;
use comfy_table::{Cell, Color, ContentArrangement, Table, presets::UTF8_FULL_CONDENSED};

use crate::cli::SessionCommands;
use crate::store::filesystem;

pub async fn run(command: Option<SessionCommands>) -> Result<()> {
    match command {
        Some(SessionCommands::Delete { name }) => {
            filesystem::delete_session(&name)?;
            println!("Session '{}' deleted.", name);
            Ok(())
        }
        None => {
            let sessions = filesystem::list_sessions()?;
            if sessions.is_empty() {
                println!("No saved sessions. Use `tabs save <name>` to create one.");
                return Ok(());
            }

            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL_CONDENSED)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Name").fg(Color::DarkCyan),
                    Cell::new("Tabs").fg(Color::DarkCyan),
                    Cell::new("Tags").fg(Color::DarkCyan),
                    Cell::new("Updated").fg(Color::DarkCyan),
                ]);

            for s in &sessions {
                table.add_row(vec![
                    Cell::new(&s.name),
                    Cell::new(s.tab_count),
                    Cell::new(s.tags.join(", ")).fg(Color::Green),
                    Cell::new(s.updated_at.format("%Y-%m-%d %H:%M")),
                ]);
            }

            println!("{}", table);
            println!("\n{} session(s)", sessions.len());
            Ok(())
        }
    }
}

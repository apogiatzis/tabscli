# Architecture

## Module Structure

```
main.rs                    Entry point (tokio async runtime, clap parsing)
cli.rs                     CLI argument and subcommand definitions
config.rs                  Data directory management (XDG paths)

chrome/
  mod.rs                   Browser enum — backend abstraction layer
  applescript.rs           AppleScript backend (macOS, runs osascript)
  cdp.rs                   Chrome DevTools Protocol backend (HTTP, cross-platform)
  types.rs                 CDP JSON response types

model/
  mod.rs                   Re-exports
  tab.rs                   Tab and SavedTab types
  session.rs               Session and Snapshot types

store/
  mod.rs                   Re-exports
  filesystem.rs            JSON file I/O for sessions and snapshots

commands/
  mod.rs                   Central dispatcher (routes CLI subcommands)
  list.rs                  List tabs with filtering, sorting, formatting
  close.rs                 Close tabs by pattern/domain/duplicates
  select.rs                Launch interactive fuzzy picker
  save.rs                  Save tabs as named session or to file
  restore.rs               Restore tabs from session or file
  sessions.rs              List and delete saved sessions
  organize.rs              Deduplicate and group tabs by domain
  snapshot.rs              Create point-in-time snapshots
  history.rs               View, diff, and restore from snapshots
  setup.rs                 Print CDP setup instructions

output/
  mod.rs                   Re-exports
  formatter.rs             Output formatting (table, JSON, CSV, plain)

tui/
  mod.rs                   Re-exports
  picker.rs                Interactive fuzzy tab picker (ratatui + crossterm)
```

## Request Flow

```
main.rs
  |
  v
cli.rs (parse args)
  |
  v
commands/mod.rs (dispatch)
  |
  +---> chrome::Browser (fetch/mutate tabs)
  |       |-- AppleScript (osascript subprocess)
  |       |-- CDP (HTTP to localhost:PORT)
  |
  +---> model::{Tab, Session, Snapshot}
  |
  +---> store::filesystem (read/write JSON)
  |
  +---> tui::picker (interactive selection)
  |
  +---> output::formatter (render output)
```

## Backend Abstraction

The `Browser` enum wraps two backend implementations behind a common async interface:

| Method | AppleScript | CDP |
|--------|-------------|-----|
| `list_tabs()` | osascript, parses `id\|\|\|title\|\|\|url` delimited output | GET `/json`, filters `type == "page"` |
| `close_tab(id)` | osascript, iterates windows/tabs by ID | GET `/json/close/{id}` |
| `activate_tab(id)` | osascript, sets active tab index + window focus | GET `/json/activate/{id}` |
| `open_tab(url)` | osascript, `make new tab with properties` | PUT `/json/new?{url}` |
| `new_window()` | osascript, `make new window` | Not supported (falls back to open_tab) |
| `open_tab_in_window(url, window_id)` | osascript, targets specific window | Not supported (falls back to open_tab) |

Backend is selected at runtime via `--backend` flag. AppleScript is the default on macOS and requires no setup. CDP works cross-platform but requires Chrome launched with `--remote-debugging-port`.

## Data Models

### Tab (ephemeral, in-memory)

Represents a currently open browser tab. Carries a browser-assigned `id` used for close/activate operations. Not persisted directly.

### SavedTab (persistent)

A tab stripped of its ephemeral ID. Used in sessions and snapshots. Conversion via `From<&Tab>`.

### Session (user-created)

A named collection of saved tabs with optional tags and timestamps. Stored as `{name}.json` in the sessions directory. Supports create, overwrite, list, delete.

### Snapshot (automatic or manual)

A point-in-time capture of all open tabs. ID is a timestamp string (`YYYYMMDDTHHMMSS`). Automatically created before destructive operations (close, deduplicate). Supports manual creation with labels, listing, diffing two snapshots, and restoring.

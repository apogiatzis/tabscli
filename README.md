# tabscli

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fast CLI tool to manage Chrome browser tabs from the terminal. Built in Rust.

## Features

- **List** all open tabs with filtering, search, sorting, and multiple output formats
- **Close** tabs by regex pattern, domain, or duplicate detection (with dry-run preview)
- **Select** tabs interactively with a built-in fuzzy picker TUI
- **Save** tabs as named sessions or export to portable JSON files
- **Restore** sessions from saved names or JSON files, optionally in a new window
- **Organize** tabs by grouping by domain or removing duplicates
- **Snapshot** tab state for point-in-time recovery
- **History** with diff between snapshots and one-command restore
- **Auto-snapshots** before destructive operations (close, deduplicate)

## How It Works

tabscli uses **AppleScript** (macOS) to communicate with Chrome — no special flags, extensions, or configuration needed. Just open Chrome normally and use tabscli.

A **CDP backend** (Chrome DevTools Protocol) is also available for cross-platform use via `--backend cdp`, which requires Chrome to be launched with `--remote-debugging-port=9222`. Run `tabscli setup` for details.

## Installation

### Pre-built binaries (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/apogiatzis/tabscli/main/install.sh | bash
```

This auto-detects your OS and architecture, downloads the latest release, and installs to `~/.local/bin`.

### From source

```bash
cargo install --path .
```

## Quick Start

```bash
tabscli                    # list all open tabs
tabscli list --json        # JSON output
tabscli select             # fuzzy picker to switch tabs
tabscli save work          # save all tabs as "work" session
tabscli close --duplicates # close duplicate tabs
```

## Commands

### `tabscli list`

List all open tabs.

```bash
tabscli list                         # table format (default)
tabscli list --json                  # JSON output (shorthand for -f json)
tabscli list -f csv                  # CSV with header
tabscli list -f plain                # tab-separated: title\tURL

tabscli list --domain github.com     # filter by domain
tabscli list --search "pull request" # search title and URL
tabscli list --sort domain           # sort by: window, title, url, domain
```

### `tabscli close`

Close tabs matching criteria. Prompts for confirmation unless `--force` is used. Automatically takes a snapshot before closing.

```bash
tabscli close "stackoverflow"        # regex match on title/URL
tabscli close --domain reddit.com    # close all tabs from a domain
tabscli close --duplicates           # close duplicate URLs (keeps first)
tabscli close "old-project" --dry-run  # preview what would close
tabscli close --domain example.com --force  # skip confirmation
```

### `tabscli select`

Interactive fuzzy tab picker with a TUI interface.

```bash
tabscli select                       # pick a tab and activate it
tabscli select --action close        # pick a tab and close it
tabscli select --action save         # pick tabs and save as session
tabscli select --multi               # enable multi-select (Tab to toggle)
tabscli select --action close --multi  # multi-select and close
```

**Key bindings:**

| Key | Action |
|-----|--------|
| Type | Filter tabs (fuzzy search on title + URL) |
| `Up`/`Down` | Navigate |
| `Enter` | Confirm selection |
| `Tab` | Toggle selection (multi-select mode) |
| `Esc` / `Ctrl-C` | Cancel |

### `tabscli save`

Save current tabs as a named session or to a file.

```bash
tabscli save work                    # save to data directory as "work"
tabscli save work --tags dev,daily   # with tags
tabscli save work --overwrite        # overwrite existing session

tabscli save -f ~/backup.json        # save to a specific file
tabscli save -f ./tabs.json my-name  # custom session name in the file
```

### `tabscli restore`

Restore tabs from a saved session or file.

```bash
tabscli restore work                 # restore by name (into current window)
tabscli restore work --new-window    # restore into a new window

tabscli restore -f ~/backup.json     # restore from a file
tabscli restore -f ./tabs.json --new-window
```

### `tabscli sessions`

Manage saved sessions.

```bash
tabscli sessions                     # list all saved sessions
tabscli sessions delete old-session  # delete a session
```

### `tabscli organize`

Organize open tabs.

```bash
tabscli organize --deduplicate            # close duplicate URLs
tabscli organize --by-domain              # show tabs grouped by domain
tabscli organize --by-domain --dry-run    # preview without changes
tabscli organize --deduplicate --by-domain  # both
```

### `tabscli snapshot`

Save a point-in-time snapshot of all open tabs.

```bash
tabscli snapshot                          # auto-generated ID
tabscli snapshot --label "before cleanup" # with a label
```

### `tabscli history`

View and manage snapshot history.

```bash
tabscli history                      # list recent snapshots
tabscli history -n 50                # show last 50 snapshots

tabscli history --diff <id1> <id2>   # diff two snapshots (shows added/removed)
tabscli history --restore <id>       # restore all tabs from a snapshot
```

### `tabscli setup`

Show setup instructions for the CDP backend.

```bash
tabscli setup
```

## Session File Format

Sessions are stored as JSON and are portable across machines:

```json
{
  "name": "work",
  "tags": ["dev", "daily"],
  "tabs": [
    {
      "title": "GitHub",
      "url": "https://github.com"
    },
    {
      "title": "Hacker News",
      "url": "https://news.ycombinator.com"
    }
  ],
  "created_at": "2026-04-07T14:00:00Z",
  "updated_at": "2026-04-07T14:00:00Z"
}
```

You can create session files by hand and restore them with `tabscli restore -f`.

## Data Storage

Sessions and snapshots are stored as JSON files in platform-specific directories:

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/tabscli/` |
| Linux | `~/.local/share/tabscli/` |
| Windows | `%APPDATA%/tabscli/` |

```
tabscli/
  sessions/
    work.json
    research.json
  snapshots/
    20260407T151200.json
    20260407T140000.json
```

## Tab Recovery

tabscli provides three tiers of recovery:

1. **Auto-snapshots** — Every `close` and `organize --deduplicate` automatically snapshots current state before acting. If you accidentally close tabs, check `tabscli history`.

2. **Manual snapshots** — `tabscli snapshot --label "before refactor"` for intentional checkpoints.

3. **Named sessions** — `tabscli save work` / `tabscli restore work` for reusable tab collections.

**Recovery workflow:**

```bash
# "I accidentally closed 20 tabs!"
tabscli history                              # find the pre-close snapshot
tabscli history --diff <before> <after>      # see what was lost
tabscli history --restore <before-id> --new-window  # get them back
```

## Backends

| Backend | Platform | Setup | Capabilities |
|---------|----------|-------|-------------|
| `applescript` (default) | macOS | None — works with any running Chrome | List, close, activate, open, new window |
| `cdp` | All | Chrome must be started with `--remote-debugging-port` | List, close, activate, open |

Switch backends with the `--backend` global flag:

```bash
tabscli --backend cdp --port 9222 list
```

## Examples

```bash
# Morning routine: restore work tabs
tabscli restore work --new-window

# End of day: save and close everything
tabscli save work --overwrite
tabscli close ".*" --force

# Clean up: find and close duplicates
tabscli close --duplicates

# Export tabs for a colleague
tabscli save -f ./shared-tabs.json --tags project-x

# Quick tab switch without leaving the terminal
tabscli select

# See what changed since your last snapshot
tabscli history --diff 20260407T090000 20260407T170000
```

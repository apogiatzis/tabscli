# Commands

## Global Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--browser` | `chrome` | Target browser: chrome, brave, edge, chromium |
| `--backend` | `applescript` | Communication backend: applescript or cdp |
| `--port` | `9222` | CDP port (only used with `--backend cdp`) |

## `tabscli list`

List all open tabs with optional filtering, sorting, and format control.

| Flag | Description |
|------|-------------|
| `--format`, `-f` | Output format: table (default), json, csv, plain |
| `--json` | Shorthand for `--format json` |
| `--domain` | Filter tabs by domain |
| `--search` | Search title and URL |
| `--sort` | Sort by: window (default), title, url, domain |

Default command when no subcommand is given.

## `tabscli close`

Close tabs matching criteria. Prompts for confirmation unless `--force` is used. Automatically takes a snapshot before closing.

| Argument/Flag | Description |
|---------------|-------------|
| `<pattern>` | Regex pattern to match against title/URL |
| `--domain` | Close tabs from a specific domain |
| `--duplicates` | Close duplicate URLs (keeps first occurrence) |
| `--dry-run` | Preview what would be closed |
| `--force` | Skip confirmation prompt |

## `tabscli select`

Interactive fuzzy tab picker using a TUI interface.

| Flag | Description |
|------|-------------|
| `--action` | Action on selection: activate (default), close, save |
| `--multi` | Enable multi-select (Tab to toggle) |

Key bindings: Arrow keys to navigate, Enter to confirm, Tab to toggle selection (multi-mode), Esc/Ctrl-C to cancel.

## `tabscli save`

Save current tabs as a named session or to a file.

| Argument/Flag | Description |
|---------------|-------------|
| `<name>` | Session name |
| `--file`, `-f` | Save to a JSON file instead of the data directory |
| `--tags` | Comma-separated tags |
| `--overwrite` | Overwrite existing session or file |

## `tabscli restore`

Restore tabs from a saved session or file.

| Argument/Flag | Description |
|---------------|-------------|
| `<name>` | Session name to restore |
| `--file`, `-f` | Restore from a JSON file |
| `--new-window` | Open tabs in a new window |

## `tabscli sessions`

List saved sessions. Has a `delete` subcommand.

| Subcommand | Description |
|------------|-------------|
| (none) | List all saved sessions |
| `delete <name>` | Delete a saved session |

## `tabscli organize`

Organize open tabs.

| Flag | Description |
|------|-------------|
| `--deduplicate` | Close duplicate URLs |
| `--by-domain` | Show tabs grouped by domain |
| `--dry-run` | Preview changes without applying |

## `tabscli snapshot`

Create a point-in-time snapshot of all open tabs.

| Flag | Description |
|------|-------------|
| `--label` | Optional label for the snapshot |

## `tabscli history`

View and manage snapshot history.

| Flag | Description |
|------|-------------|
| `-n`, `--limit` | Number of snapshots to show (default: 20) |
| `--restore <id>` | Restore tabs from a snapshot |
| `--diff <id1> <id2>` | Diff two snapshots (shows added/removed) |

## `tabscli setup`

Print setup instructions for the CDP backend. No arguments.

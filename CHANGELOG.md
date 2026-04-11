# Changelog

## Unreleased

### Added
- `--all` / `-a` global flag to query tabs from all supported browsers (Chrome, Brave, Edge, Chromium)
- Browser name column in table output when using `--all`
- Browser running detection — AppleScript backend checks if a browser is running before sending commands, preventing accidental app launches

### Changed
- Linux release builds now use musl for fully static binaries (no glibc dependency)

## 0.1.2 - 2026-04-10

### Added
- Config file support (`~/.config/tabscli/config.toml`) for custom `sessions_dir` and `snapshots_dir` paths
- `~` expansion in config file paths
- Respects `$XDG_CONFIG_HOME` for config file location

## 0.1.1 - 2026-04-10

### Changed
- Renamed data directory from `tabs-cli` to `tabscli`

## 0.1.0 - 2026-04-10

Initial release.

### Added
- `list` command with table, JSON, CSV, and plain output formats
- Filtering by domain and search query, sorting by window/title/url/domain
- `close` command with regex pattern, domain, and duplicate matching
- Dry-run and force modes for close
- Auto-snapshot before destructive operations
- `select` command with interactive fuzzy picker TUI (single and multi-select)
- `save` / `restore` commands for named sessions and JSON file export
- `sessions` command to list and delete saved sessions
- `organize` command with deduplication and group-by-domain
- `snapshot` / `history` commands for point-in-time tab state capture, diffing, and restore
- `setup` command with CDP backend instructions
- AppleScript backend (macOS, default) and CDP backend (cross-platform)
- Support for Chrome, Brave, Edge, and Chromium browsers
- Install script (`install.sh`) with auto-detection of OS/architecture
- GitHub Actions CI (fmt, clippy, test, build) and release workflows
- MIT license

### Security
- AppleScript string sanitization to prevent injection via URLs
- Path traversal prevention for session names
- UTF-8 safe string truncation

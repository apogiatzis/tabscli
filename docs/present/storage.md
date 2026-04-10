# Storage

## Directory Layout

Data is stored as JSON files in platform-specific directories using the `directories` crate (XDG Base Directory spec):

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/tabs-cli/` |
| Linux | `~/.local/share/tabs-cli/` |
| Windows | `%APPDATA%/tabs-cli/` |

```
tabs-cli/
  sessions/
    work.json
    research.json
  snapshots/
    20260407T151200.json
    20260407T140000.json
```

Directories are created automatically on first use.

## Session Format

```json
{
  "name": "work",
  "tags": ["dev", "daily"],
  "tabs": [
    {
      "title": "GitHub",
      "url": "https://github.com"
    }
  ],
  "created_at": "2026-04-07T14:00:00Z",
  "updated_at": "2026-04-07T14:00:00Z"
}
```

Sessions are portable — they can be created by hand and restored with `tabscli restore -f`.

## Snapshot Format

```json
{
  "id": "20260407T151200",
  "label": "before cleanup",
  "tabs": [
    {
      "title": "GitHub",
      "url": "https://github.com"
    }
  ],
  "tab_count": 47,
  "created_at": "2026-04-07T15:12:00Z"
}
```

Snapshot IDs are timestamp-based (`YYYYMMDDTHHMMSS`), human-readable and sortable.

## Auto-Snapshots

The `close` and `organize --deduplicate` commands automatically create a snapshot (labelled `pre-close`) before performing destructive operations. This enables recovery via `tabscli history`.

## Name Validation

Session names are validated to prevent path traversal — names containing `/`, `\`, or `..` are rejected. Snapshot IDs are auto-generated and don't accept user input.

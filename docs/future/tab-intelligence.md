# Tab Intelligence

Features for understanding and managing tab usage patterns.

## `tabscli stats`

Show tab usage statistics.

```bash
tabscli stats                          # summary overview
tabscli stats --by-domain              # tab count per domain
tabscli stats --json                   # machine-readable output
```

Example output:
```
Open tabs: 47
Unique domains: 18
Most common: github.com (12), stackoverflow.com (8), docs.rs (5)
Duplicates: 3
```

## `tabscli stale`

Find tabs that haven't been visited recently. Uses snapshot history to detect tabs that have been open but untouched across multiple snapshots.

```bash
tabscli stale                          # tabs unchanged for 7+ days (default)
tabscli stale --days 3                 # tabs unchanged for 3+ days
tabscli stale --close                  # close stale tabs (with confirmation)
tabscli stale --close --dry-run        # preview what would be closed
```

## `tabscli group`

Create and manage Chrome tab groups from the CLI.

```bash
tabscli group create "Research" --search "arxiv"         # group matching tabs
tabscli group create "PRs" --url-contains "/pull/"       # group by URL pattern
tabscli group create "Work" --domain jira.com,github.com # group by domains
tabscli group list                                        # list existing groups
tabscli group ungroup "Research"                          # dissolve a group
```

Note: Chrome tab groups are only accessible via CDP, not AppleScript. This feature would require the CDP backend.

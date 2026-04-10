# Workflow Automation

Features for automating tab management and integrating with daily workflows.

## `tabscli switch <query>`

Non-interactive tab switch by fuzzy name match. Designed for keybindings and scripts where launching a TUI is not desired.

```bash
tabscli switch "pull request"          # activate the best-matching tab
tabscli switch "slack"                 # jump to Slack

# Bind to a hotkey in your window manager or tmux
bind-key t run-shell "tabscli switch 'jira'"
```

## `tabscli workspace`

Named tab sets tied to projects. Config-driven via TOML.

```toml
# ~/.config/tabscli/workspaces.toml
[backend]
urls = [
    "https://github.com/org/backend",
    "https://app.datadog.com/dashboard/backend",
    "https://linear.app/team/backend",
]

[frontend]
urls = [
    "https://github.com/org/frontend",
    "https://figma.com/file/design-system",
    "https://localhost:3000",
]
```

```bash
tabscli workspace start backend        # open all backend URLs in a new window
tabscli workspace stop backend         # save current state and close them
tabscli workspace list                 # show available workspaces
```

## `tabscli rules`

Auto-actions based on configurable rules.

```toml
# ~/.config/tabscli/rules.toml
[[rule]]
match = { domain = "reddit.com" }
action = { close_after = "30m" }

[[rule]]
match = { url_contains = "/pull/" }
action = { group = "PRs" }

[[rule]]
match = { domain = "docs.rs" }
action = { group = "Docs" }
```

```bash
tabscli rules apply                    # run rules once against current tabs
tabscli rules watch                    # continuously enforce rules
```

## `tabscli watch`

Stream tab open/close events as JSON lines. Useful for logging, triggering scripts, or building integrations.

```bash
tabscli watch                          # stream events to stdout
tabscli watch | jq 'select(.event == "opened")'  # filter for new tabs
tabscli watch --interval 5             # poll every 5 seconds
```

Output format:
```json
{"event": "opened", "url": "https://example.com", "title": "Example", "timestamp": "2026-04-10T12:00:00Z"}
{"event": "closed", "url": "https://old.com", "title": "Old Page", "timestamp": "2026-04-10T12:00:05Z"}
```

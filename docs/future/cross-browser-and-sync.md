# Cross-Browser & Sync

Features for working across browsers and machines.

## Firefox Support

Add Firefox as a backend via the Marionette protocol or native messaging.

```bash
tabscli --browser firefox list
tabscli --browser firefox close --duplicates
```

Requires Firefox to be started with `--marionette` or a companion browser extension for native messaging.

## Export & Import

Export is handled by `tabscli list --format` (see [piping-and-composability.md](piping-and-composability.md)) — formats like `html`, `markdown`, and `org` cover the export use case without a separate command.

Import would be a new `tabscli open -` command that reads URLs from stdin or a file:

```bash
tabscli list -f html > bookmarks.html  # export (cross-browser importable)
tabscli open - < urls.txt              # import URLs from a file
```

## `tabscli sync`

Sync sessions to a git repo or GitHub Gist for sharing across machines.

```bash
tabscli sync init --gist               # create a private gist for syncing
tabscli sync push                      # push current sessions to remote
tabscli sync pull                      # pull sessions from remote

tabscli sync init --repo git@github.com:user/tabs.git  # use a full repo
```

This enables a workflow where your tab sessions follow you across machines without relying on Chrome Sync.

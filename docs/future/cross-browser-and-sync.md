# Cross-Browser & Sync

Features for working across browsers and machines.

## Firefox Support

Add Firefox as a backend via the Marionette protocol or native messaging.

```bash
tabscli --browser firefox list
tabscli --browser firefox close --duplicates
```

Requires Firefox to be started with `--marionette` or a companion browser extension for native messaging.

## `tabscli export` / `tabscli import`

Portable HTML bookmark format for cross-browser compatibility.

```bash
tabscli export > tabs.html             # Netscape bookmark format (works in all browsers)
tabscli export --format markdown       # Markdown link list
tabscli export --format org            # Org-mode links

tabscli import tabs.html               # open tabs from bookmark file
```

The HTML bookmark format is universally supported — every browser can import it.

## `tabscli sync`

Sync sessions to a git repo or GitHub Gist for sharing across machines.

```bash
tabscli sync init --gist               # create a private gist for syncing
tabscli sync push                      # push current sessions to remote
tabscli sync pull                      # pull sessions from remote

tabscli sync init --repo git@github.com:user/tabs.git  # use a full repo
```

This enables a workflow where your tab sessions follow you across machines without relying on Chrome Sync.

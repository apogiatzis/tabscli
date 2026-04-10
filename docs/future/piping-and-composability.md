# Piping & Composability

Features to make tabscli a first-class citizen in shell pipelines.

## Extended `--format` options for `tabscli list`

Rather than adding separate commands, extend the existing `--format` flag on `tabscli list` with new output formats. Current formats: `table`, `json`, `csv`, `plain`.

New formats to add:

- **`urls`** — bare URL-per-line, ideal for piping
- **`markdown`** — Markdown link list (`- [title](url)`)
- **`org`** — Org-mode links (`- [[url][title]]`)
- **`html`** — Netscape bookmark HTML format (importable by all browsers)

```bash
tabscli list -f urls | xargs curl -I          # check HTTP status of all tabs
tabscli list -f urls | wc -l                   # count tabs
tabscli list -f urls | sort | uniq -d          # find duplicate URLs
tabscli list -f urls --domain github.com | pbcopy  # copy GitHub tabs to clipboard
tabscli list -f markdown > tabs.md             # export as Markdown
tabscli list -f html > bookmarks.html          # export as importable bookmarks
```

## `tabscli open -`

Read URLs from stdin, one per line.

```bash
cat urls.txt | tabscli open -          # open a list of URLs
pbpaste | tabscli open -               # open URLs from clipboard
tabscli list -f urls | grep github | tabscli open - --new-window  # reopen filtered tabs
```

## Shell Completions

Generate shell completions for bash, zsh, and fish.

```bash
tabscli completions zsh > ~/.zfunc/_tabscli
tabscli completions bash > /etc/bash_completion.d/tabscli
tabscli completions fish > ~/.config/fish/completions/tabscli.fish
```

This is straightforward to implement since clap provides `clap_complete` for auto-generating completions.

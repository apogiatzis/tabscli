# Piping & Composability

Features to make tabscli a first-class citizen in shell pipelines.

## `tabscli urls`

Bare URL-per-line output for easy piping.

```bash
tabscli urls | xargs curl -I          # check HTTP status of all tabs
tabscli urls | wc -l                   # count tabs
tabscli urls | sort | uniq -d          # find duplicate URLs
tabscli urls --domain github.com | pbcopy  # copy GitHub tabs to clipboard
```

## `tabscli open -`

Read URLs from stdin, one per line.

```bash
cat urls.txt | tabscli open -          # open a list of URLs
pbpaste | tabscli open -               # open URLs from clipboard
tabscli urls | grep github | tabscli open - --new-window  # reopen filtered tabs
```

## `tabscli filter`

Pipe-friendly filter that outputs matching tabs without side effects. Composable with other commands.

```bash
tabscli filter --domain reddit.com | tabscli close --stdin
tabscli filter --search "TODO" | tabscli save todo-tabs
```

## Shell Completions

Generate shell completions for bash, zsh, and fish.

```bash
tabscli completions zsh > ~/.zfunc/_tabscli
tabscli completions bash > /etc/bash_completion.d/tabscli
tabscli completions fish > ~/.config/fish/completions/tabscli.fish
```

This is straightforward to implement since clap provides `clap_complete` for auto-generating completions.

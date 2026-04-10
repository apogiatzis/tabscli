# Developer Workflows

Features tailored for software developers working in the terminal.

## `tabscli pr <number>`

Open the pull request page for the current git repo.

```bash
tabscli pr 42                          # opens github.com/<remote>/pull/42
tabscli pr 42 --files                  # open the files-changed view
tabscli pr                             # open the PR list for the current repo
```

Detects the remote URL from `git remote get-url origin` and constructs the appropriate GitHub/GitLab/Bitbucket URL.

## `tabscli repo`

Open the current git repo's remote URL in a browser tab.

```bash
tabscli repo                           # open repo homepage
tabscli repo --branch                  # open current branch
tabscli repo --actions                 # open CI/Actions page
tabscli repo --issues                  # open issues page
```

## `tabscli docs <package>`

Open documentation pages for packages/crates.

```bash
tabscli docs tokio                     # opens docs.rs/tokio
tabscli docs react --npm               # opens npmjs.com/package/react
tabscli docs requests --pypi           # opens pypi.org/project/requests
```

Auto-detects the ecosystem from the current project (Cargo.toml, package.json, requirements.txt) so the flag is optional.

## tmux / zellij Integration

Auto-save and restore tab sessions tied to terminal sessions.

```bash
# Save current tabs associated with a tmux session
tabscli save --tmux                    # uses current tmux session name

# Restore tabs when attaching to a tmux session
# Add to .tmux.conf:
set-hook -g session-created 'run-shell "tabscli restore $(tmux display -p \"#S\") --new-window 2>/dev/null"'
```

This links browser context to terminal context — when you switch tmux sessions, your tabs follow.

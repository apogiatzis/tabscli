# Design Decisions

## Enum-Based Backend Abstraction

The `Browser` enum wraps `AppleScriptClient` and `CdpClient` behind a common interface. This allows runtime backend selection via a CLI flag without dynamic dispatch overhead. Adding a new backend (e.g. Firefox Marionette) means adding a variant and implementing the same methods.

## Ephemeral vs Persistent Tab Models

`Tab` carries a browser-assigned ID needed for operations like close/activate. `SavedTab` drops the ID since it's meaningless after the browser session ends. This separation keeps the storage format clean and portable.

## JSON File Storage

Sessions and snapshots are stored as individual JSON files rather than a database. This keeps the tool dependency-free and makes the data inspectable, editable, and version-controllable by users.

## Timestamp-Based Snapshot IDs

Snapshot IDs use `YYYYMMDDTHHMMSS` format. They're human-readable, sortable as strings, and collision-proof at second granularity. No ID generation library or database needed.

## Auto-Snapshots Before Destructive Operations

The `close` command automatically snapshots before acting. This provides a safety net without requiring users to remember to snapshot manually. Failures are logged as warnings but don't block the operation.

## AppleScript as Default Backend

AppleScript requires no setup on macOS — it works with any running Chrome instance. CDP requires Chrome to be launched with a special flag, which is a barrier for casual use. AppleScript is the default; CDP is opt-in for cross-platform or advanced use.

## rustls Over OpenSSL

The `reqwest` dependency uses `rustls-tls` instead of the default `native-tls`. This avoids a system OpenSSL dependency, which simplifies cross-compilation (especially for aarch64-linux) and removes a common source of build failures in CI.

## Confirmation Before Destructive Actions

`close` and `organize --deduplicate` prompt for confirmation by default. `--force` skips the prompt, and `--dry-run` previews without acting. This protects against accidental tab loss.

## Clap Derive API

CLI parsing uses clap's derive macros rather than the builder API. This keeps command definitions declarative and co-located with the struct definitions, making the CLI self-documenting.

## AppleScript String Sanitization

URLs interpolated into AppleScript strings are sanitized by escaping backslashes and double quotes. This prevents injection attacks where a malicious URL in a session file could execute arbitrary shell commands via `do shell script`.

## Path Traversal Prevention

Session names are validated to reject `/`, `\`, and `..`. This prevents a crafted session name from writing or reading files outside the data directory.

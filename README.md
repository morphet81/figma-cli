# fcli

A command-line tool for interacting with the Figma REST API. Inspect files, export assets, manage comments, browse components and styles — all from your terminal.

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.70+)

### Install from source

```bash
git clone <repo-url> && cd figma-cli
./install.sh
```

This builds the release binary and copies it to `~/.local/bin/fcli`. Override the install location with `INSTALL_DIR`:

```bash
INSTALL_DIR=/usr/local/bin ./install.sh
```

Alternatively, install directly via Cargo:

```bash
cargo install --path .
```

## Authentication

fcli supports three authentication methods, checked in this order:

### 1. Environment variable (recommended for CI/scripts)

```bash
export FIGMA_ACCESS_TOKEN=figd_xxxxxxxxxxxx
```

When set, all commands use this token automatically. No login step needed.

### 2. OAuth 2.0 (recommended for interactive use)

```bash
fcli auth login
```

This prompts for your OAuth app's Client ID and Client Secret, opens your browser for Figma authorization, and stores the tokens locally. You need to [register an OAuth app](https://www.figma.com/developers/apps) on Figma first with `http://localhost:9876/callback` as a redirect URL.

### 3. Personal Access Token

```bash
fcli auth login --pat
```

Prompts for a PAT (input is hidden). Generate one from [Figma Settings > Security > Personal access tokens](https://www.figma.com/settings).

### Check status and log out

```bash
fcli auth status    # show current auth state
fcli auth logout    # clear stored tokens
```

## Passing Figma URLs

All commands that accept a file reference support two ways to pass it:

**By file key** (no quoting needed):
```bash
fcli file info yz9QHggkjM5NtzZYjzj4fY
```

**By URL using `--url`** (must be quoted due to `?` and `&` in URLs):
```bash
fcli file info --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design"
```

> **Why `--url`?** Figma URLs contain `?` and `&` which zsh/bash interpret as glob patterns and background operators. The `--url` flag makes it explicit that the value needs quoting.

When a URL contains a `node-id` parameter, it is automatically extracted and used by commands like `inspect` and `export`.

## Commands

### `fcli user me`

Show the currently authenticated user.

```bash
fcli user me
fcli user me --json
```

### `fcli file info`

Display file metadata (name, last modified, editor type, version).

```bash
fcli file info yz9QHggkjM5NtzZYjzj4fY
fcli file info --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design"
fcli file info yz9QHggkjM5NtzZYjzj4fY --json
```

### `fcli file inspect`

Print the document tree showing all layers, frames, and components.

```bash
# Inspect entire file (defaults to depth 3)
fcli file inspect yz9QHggkjM5NtzZYjzj4fY

# Control depth
fcli file inspect yz9QHggkjM5NtzZYjzj4fY --depth 5

# Inspect a specific node
fcli file inspect yz9QHggkjM5NtzZYjzj4fY --node-id 4657:25395

# Node ID is auto-extracted from URL
fcli file inspect --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design?node-id=4657-25395"

# Full JSON output
fcli file inspect yz9QHggkjM5NtzZYjzj4fY --json
```

Output format:
```
DOCUMENT My Design [0:0]
  CANVAS Page 1 [0:1]
    FRAME Header [1:2]
      TEXT Logo [1:3]
      INSTANCE NavBar [1:4]
    FRAME Content [2:1]
      ...
```

### `fcli file export`

Export nodes as images (PNG, JPG, SVG, or PDF).

```bash
# Export specific nodes
fcli file export yz9QHggkjM5NtzZYjzj4fY --ids 1:2,3:4

# Node ID auto-extracted from URL — no --ids needed
fcli file export --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design?node-id=4657-25395"

# Options
fcli file export yz9QHggkjM5NtzZYjzj4fY --ids 1:2 --format svg --scale 2 --output ./exports
```

| Option | Default | Description |
|--------|---------|-------------|
| `--ids` | from URL | Comma-separated node IDs |
| `--format` | `png` | `png`, `jpg`, `svg`, or `pdf` |
| `--scale` | `1` | Scale factor (0.01 to 4) |
| `--output` | `.` | Output directory |

### `fcli comments list`

List all comments on a file.

```bash
fcli comments list yz9QHggkjM5NtzZYjzj4fY
fcli comments list --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design" --json
```

### `fcli comments add`

Post a comment on a file.

```bash
fcli comments add yz9QHggkjM5NtzZYjzj4fY -m "Looks good, ship it!"
```

### `fcli comments delete`

Delete a comment by ID.

```bash
fcli comments delete yz9QHggkjM5NtzZYjzj4fY --comment-id 123456
```

### `fcli projects list`

List all projects in a team. The team ID can be found in the URL when viewing a team page in Figma.

```bash
fcli projects list 123456789
fcli projects list 123456789 --json
```

### `fcli projects files`

List files in a project.

```bash
fcli projects files 987654
fcli projects files 987654 --json
```

### `fcli components list`

List published components from a file or team library.

```bash
# From a file
fcli components list yz9QHggkjM5NtzZYjzj4fY
fcli components list --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design"

# From a team library
fcli components list 123456789 --team

# JSON output
fcli components list yz9QHggkjM5NtzZYjzj4fY --json
```

### `fcli styles list`

List published styles from a file or team library.

```bash
# From a file
fcli styles list yz9QHggkjM5NtzZYjzj4fY

# From a team library
fcli styles list 123456789 --team

# JSON output
fcli styles list yz9QHggkjM5NtzZYjzj4fY --json
```

### `fcli versions list`

List the version history of a file.

```bash
fcli versions list yz9QHggkjM5NtzZYjzj4fY
fcli versions list --url "https://www.figma.com/design/yz9QHggkjM5NtzZYjzj4fY/My-Design" --json
```

## JSON output

All list/info commands support `--json` for machine-readable output. This is useful for piping into `jq`, scripting, or integration with other tools:

```bash
fcli file info yz9QHggkjM5NtzZYjzj4fY --json | jq '.name'
fcli comments list yz9QHggkjM5NtzZYjzj4fY --json | jq '.[].message'
fcli components list yz9QHggkjM5NtzZYjzj4fY --json | jq '.[].name'
```

## Configuration

Tokens are stored at `~/.config/fcli/auth.json` (or the platform-equivalent config directory). This file is created by `fcli auth login` and removed by `fcli auth logout`.

The `FIGMA_ACCESS_TOKEN` environment variable always takes priority over stored tokens.

## Figma API scopes

When using OAuth, fcli requests the following scopes:

| Scope | Used by |
|-------|---------|
| `current_user:read` | `user me` |
| `file_content:read` | `file inspect`, `file export` |
| `file_metadata:read` | `file info` |
| `file_comments:read` | `comments list` |
| `file_comments:write` | `comments add`, `comments delete` |
| `file_versions:read` | `versions list` |
| `projects:read` | `projects list`, `projects files` |
| `team_library_content:read` | `components list --team`, `styles list --team` |
| `library_content:read` | `components list`, `styles list` |
| `library_assets:read` | component/style details |
| `webhooks:read` | webhook access |

When using a Personal Access Token, make sure it has the scopes matching the commands you need.

## License

MIT

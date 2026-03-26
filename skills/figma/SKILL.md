---
name: figma
version: 1.0.0
description: Interact with Figma designs through the REST API. Use when the user references a Figma URL, asks to inspect a design, export assets, review components/styles, read comments, or implement a design from Figma. Provides file inspection, asset export, component/style listing, and comment management.
argument-hint: "<figma URL or instruction referencing a Figma design>"
---

Interact with Figma designs using the `fcli` CLI tool based on `$ARGUMENTS`.

**Usage:**
- `/figma <Figma URL>` — Inspect the referenced design and summarize its structure
- `/figma <Figma URL> export the hero section` — Export specific nodes as images
- `/figma <Figma URL> list all components` — List published components in the file
- `/figma <Figma URL> what styles are used?` — List styles defined in the file
- `/figma <instruction referencing Figma>` — Perform any Figma-related task

**Instructions:**

1. **Check authentication.** Run `fcli auth status`. If not authenticated, tell the user to set `FIGMA_ACCESS_TOKEN` or run `fcli auth login`.

2. **Extract the Figma URL from `$ARGUMENTS`.**
   - Look for URLs matching `https://www.figma.com/(design|file|board)/...`
   - If no URL is found, ask the user to provide one.
   - Always pass the URL via `--url "..."` to avoid shell quoting issues.

3. **Determine what the user wants** and execute the appropriate workflow:

   - **Inspect / understand a design** → see [references/inspect.md](references/inspect.md)
   - **Export assets or screenshots** → see [references/export.md](references/export.md)
   - **List components or styles** → see [references/design-system.md](references/design-system.md)
   - **Work with comments** → see [references/comments.md](references/comments.md)
   - **Implement a design** → see [references/implement.md](references/implement.md)

4. **If the intent is ambiguous**, start by inspecting the design:
   ```bash
   fcli file inspect --url "<URL>" --depth 3
   ```
   Then summarize the structure and ask the user what they'd like to do.

5. **Always quote Figma URLs** in shell commands using double quotes.

## Quick reference

```bash
# Authentication
fcli auth status
fcli auth login --pat

# File info
fcli file info --url "<URL>"
fcli file info <FILE_KEY>

# Inspect document tree
fcli file inspect --url "<URL>"
fcli file inspect --url "<URL>" --depth 5
fcli file inspect --url "<URL>" --json
fcli file inspect <FILE_KEY> --node-id 4657:25395

# Export nodes as images
fcli file export --url "<URL>"                          # exports node from URL
fcli file export <FILE_KEY> --ids 1:2,3:4               # export specific nodes
fcli file export <FILE_KEY> --ids 1:2 --format svg      # as SVG
fcli file export <FILE_KEY> --ids 1:2 --scale 2         # at 2x
fcli file export <FILE_KEY> --ids 1:2 --output ./assets # to directory

# Comments
fcli comments list --url "<URL>"
fcli comments add <FILE_KEY> -m "message"
fcli comments delete <FILE_KEY> --comment-id <ID>

# Components and styles
fcli components list --url "<URL>"
fcli components list <TEAM_ID> --team
fcli styles list --url "<URL>"
fcli styles list <TEAM_ID> --team

# Projects
fcli projects list <TEAM_ID>
fcli projects files <PROJECT_ID>

# Version history
fcli versions list --url "<URL>"

# User
fcli user me
```

## Important notes

- All list commands support `--json` for machine-readable output. Prefer `--json` when you need to process the data programmatically.
- Figma URLs contain `?` and `&` which break in shells. Always use `--url "..."` with quotes, or pass a raw file key.
- When a Figma URL contains a `node-id` parameter, `fcli` automatically extracts and uses it for `inspect` and `export`.
- File keys can be extracted from any Figma URL: `https://www.figma.com/design/<FILE_KEY>/...`
- Node IDs in URLs use `-` (e.g., `4657-25395`) but the API uses `:` (e.g., `4657:25395`). `fcli` handles this conversion automatically.

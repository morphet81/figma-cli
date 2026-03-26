# Inspecting Figma designs

Use `fcli file inspect` to understand the structure of a Figma file. This is the starting point for most workflows.

## Get file metadata

```bash
fcli file info --url "<URL>"
```

Returns the file name, last modified date, editor type, and version.

## Inspect the document tree

```bash
# Tree view (default depth 3)
fcli file inspect --url "<URL>"

# Deeper tree
fcli file inspect --url "<URL>" --depth 10

# Full JSON structure (useful for programmatic analysis)
fcli file inspect --url "<URL>" --json
```

The tree output shows each node's type, name, and ID:
```
DOCUMENT My Design [0:0]
  CANVAS Page 1 [0:1]
    FRAME Header [1:2]
      TEXT Logo [1:3]
      INSTANCE NavBar [1:4]
    FRAME Hero Section [2:1]
```

## Inspect a specific node

When a URL contains a `node-id`, that node is inspected automatically:

```bash
fcli file inspect --url "https://www.figma.com/design/KEY/Name?node-id=4657-25395"
```

Or specify a node explicitly:

```bash
fcli file inspect <FILE_KEY> --node-id 4657:25395
```

## Get full JSON for analysis

Use `--json` to get the raw Figma document JSON. Pipe through `jq` to extract specific data:

```bash
# Get all text content in a node
fcli file inspect <FILE_KEY> --node-id 1:2 --json | jq '.. | .characters? // empty'

# Get all node types
fcli file inspect <FILE_KEY> --json | jq '.. | .type? // empty' | sort | uniq -c | sort -rn

# List all component instances
fcli file inspect <FILE_KEY> --json | jq '.. | select(.type? == "INSTANCE") | {name, id}'
```

## Workflow: understanding a design before implementation

1. Start with file info to understand what you're looking at:
   ```bash
   fcli file info --url "<URL>"
   ```

2. Inspect the top-level structure:
   ```bash
   fcli file inspect --url "<URL>" --depth 2
   ```

3. Dive into the specific section referenced by the URL:
   ```bash
   fcli file inspect --url "<URL>"
   ```

4. Export a screenshot to see the visual result:
   ```bash
   fcli file export --url "<URL>"
   ```

5. Check what components and styles are used:
   ```bash
   fcli components list --url "<URL>" --json
   fcli styles list --url "<URL>" --json
   ```

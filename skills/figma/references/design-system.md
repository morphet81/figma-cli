# Working with components and styles

Use `fcli components` and `fcli styles` to explore the design system defined in a Figma file or team library.

## List components in a file

```bash
fcli components list --url "<URL>"
fcli components list <FILE_KEY>
fcli components list <FILE_KEY> --json
```

## List components across a team

```bash
fcli components list <TEAM_ID> --team
fcli components list <TEAM_ID> --team --json
```

## List styles in a file

```bash
fcli styles list --url "<URL>"
fcli styles list <FILE_KEY>
fcli styles list <FILE_KEY> --json
```

## List styles across a team

```bash
fcli styles list <TEAM_ID> --team
fcli styles list <TEAM_ID> --team --json
```

## Workflow: audit the design system

1. List all published components:
   ```bash
   fcli components list <FILE_KEY> --json | jq '.meta.components[] | {name, description, key}'
   ```

2. List all styles (colors, typography, effects):
   ```bash
   fcli styles list <FILE_KEY> --json | jq '.meta.styles[] | {name, style_type, description}'
   ```

3. Cross-reference with the document tree to find usage:
   ```bash
   fcli file inspect <FILE_KEY> --json | jq '.. | select(.type? == "INSTANCE") | .componentId'
   ```

## Workflow: extract design tokens

1. Get all styles as JSON:
   ```bash
   fcli styles list <FILE_KEY> --json > styles.json
   ```

2. Get the full file JSON to read style property values:
   ```bash
   fcli file inspect <FILE_KEY> --json > document.json
   ```

3. Parse the JSON to extract color, typography, and spacing values for your codebase.

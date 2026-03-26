# Exporting assets from Figma

Use `fcli file export` to download images and assets from a Figma file.

## Export from a URL

When the URL contains a node-id, that specific node is exported:

```bash
fcli file export --url "https://www.figma.com/design/KEY/Name?node-id=4657-25395"
```

## Export specific nodes by ID

```bash
fcli file export <FILE_KEY> --ids 1:2
fcli file export <FILE_KEY> --ids 1:2,3:4,5:6
```

## Output formats

```bash
fcli file export <FILE_KEY> --ids 1:2 --format png   # default
fcli file export <FILE_KEY> --ids 1:2 --format svg
fcli file export <FILE_KEY> --ids 1:2 --format jpg
fcli file export <FILE_KEY> --ids 1:2 --format pdf
```

## Scale factor

```bash
fcli file export <FILE_KEY> --ids 1:2 --scale 1    # 1x (default)
fcli file export <FILE_KEY> --ids 1:2 --scale 2    # 2x (retina)
fcli file export <FILE_KEY> --ids 1:2 --scale 0.5  # half size
```

## Output directory

```bash
fcli file export <FILE_KEY> --ids 1:2 --output ./assets
```

## Workflow: export all top-level frames as images

1. List top-level frames:
   ```bash
   fcli file inspect <FILE_KEY> --depth 2 --json | jq '.document.children[].children[] | select(.type == "FRAME") | {name, id}'
   ```

2. Export them all:
   ```bash
   fcli file export <FILE_KEY> --ids <comma-separated-ids> --format png --scale 2 --output ./screenshots
   ```

## Workflow: export assets for implementation

1. Inspect the node to find specific elements (icons, images):
   ```bash
   fcli file inspect <FILE_KEY> --node-id 4657:25395 --depth 10 --json
   ```

2. Find exportable nodes (usually named with an export-friendly convention):
   ```bash
   fcli file inspect <FILE_KEY> --node-id 4657:25395 --json | jq '.. | select(.type? == "VECTOR" or .type? == "COMPONENT") | {name, id}'
   ```

3. Export as SVG for icons, PNG for images:
   ```bash
   fcli file export <FILE_KEY> --ids <icon-ids> --format svg --output ./icons
   fcli file export <FILE_KEY> --ids <image-ids> --format png --scale 2 --output ./images
   ```

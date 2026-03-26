# Implementing a Figma design

This reference describes the workflow for turning a Figma design into code. Use this when the user says "implement this design", "build this", "code this screen", or similar.

## Step 1: Understand the design

Inspect the design and export a screenshot to understand the visual layout:

```bash
fcli file info --url "<URL>"
fcli file inspect --url "<URL>" --depth 5
fcli file export --url "<URL>" --output /tmp
```

Review the document tree. Pay attention to:
- **FRAME** nodes — these map to container elements (`div`, `section`, `View`)
- **TEXT** nodes — extract the text content and font styles
- **INSTANCE** nodes — these are component instances, check what component they reference
- **VECTOR** / **GROUP** nodes — usually icons or illustrations, export as SVG

## Step 2: Extract design tokens

Get the styles to understand the design system:

```bash
fcli styles list --url "<URL>" --json
```

From the JSON output, extract:
- **Fill styles** → CSS colors / background colors
- **Text styles** → font-family, font-size, font-weight, line-height
- **Effect styles** → box-shadow, blur
- **Grid styles** → layout grids

## Step 3: Identify components

List components to understand reusable elements:

```bash
fcli components list --url "<URL>" --json
```

Map component names to your codebase. If a component already exists, reuse it. If not, create it.

## Step 4: Extract text content

Use JSON output to extract all text nodes and their content:

```bash
fcli file inspect --url "<URL>" --json | jq '.. | select(.type? == "TEXT") | {name, characters: .characters, style: .style}'
```

## Step 5: Export assets

Export icons, illustrations, and images:

```bash
# Icons as SVG
fcli file export <FILE_KEY> --ids <icon-node-ids> --format svg --output ./src/assets/icons

# Images as PNG at 2x
fcli file export <FILE_KEY> --ids <image-node-ids> --format png --scale 2 --output ./src/assets/images
```

## Step 6: Build the layout

Based on the document tree:
1. Map FRAME hierarchy to your component/HTML structure
2. Apply extracted design tokens (colors, typography, spacing)
3. Use exported assets for icons and images
4. Match the exported screenshot for visual accuracy

## Tips

- The Figma document tree closely mirrors the visual hierarchy. A FRAME inside a FRAME is like a `div` inside a `div`.
- Look at the `layoutMode` property in JSON to determine if a frame uses auto-layout (`HORIZONTAL` or `VERTICAL`), which maps directly to CSS flexbox.
- Use `itemSpacing`, `paddingTop/Right/Bottom/Left` from JSON for spacing values.
- Check `fills` array for background colors and gradients.
- Check `strokes` for borders.
- Check `effects` for shadows and blurs.
- `cornerRadius` maps to CSS `border-radius`.
- `opacity` maps to CSS `opacity`.

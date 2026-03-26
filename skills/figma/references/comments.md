# Working with comments

Use `fcli comments` to read and manage comments on a Figma file.

## List comments

```bash
fcli comments list --url "<URL>"
fcli comments list <FILE_KEY>
fcli comments list <FILE_KEY> --json
```

## Add a comment

```bash
fcli comments add <FILE_KEY> -m "This looks great!"
```

## Delete a comment

```bash
fcli comments delete <FILE_KEY> --comment-id <COMMENT_ID>
```

## Workflow: review a design

1. Open the file info to confirm you have the right file:
   ```bash
   fcli file info --url "<URL>"
   ```

2. Read all existing comments:
   ```bash
   fcli comments list --url "<URL>"
   ```

3. Inspect the specific area the user referenced:
   ```bash
   fcli file inspect --url "<URL>"
   ```

4. Add your feedback:
   ```bash
   fcli comments add <FILE_KEY> -m "The spacing between the header and hero section needs adjustment"
   ```

## Workflow: summarize design feedback

1. Fetch all comments as JSON:
   ```bash
   fcli comments list <FILE_KEY> --json
   ```

2. Parse and summarize the comments, grouping by author, resolved status, or topic.

use url::Url;

#[derive(Debug, Clone)]
pub struct ParsedFigmaUrl {
    pub file_key: String,
    pub node_id: Option<String>,
}

pub fn is_raw_file_key(input: &str) -> bool {
    !input.is_empty()
        && input
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

pub fn parse_figma_url(input: &str) -> Result<ParsedFigmaUrl, String> {
    let input = input.trim();

    if is_raw_file_key(input) {
        return Ok(ParsedFigmaUrl {
            file_key: input.to_string(),
            node_id: None,
        });
    }

    let url = Url::parse(input).map_err(|e| format!("Invalid URL: {e}"))?;

    let host = url.host_str().unwrap_or_default();
    if !host.contains("figma.com") {
        return Err(format!("Not a Figma URL: {host}"));
    }

    let segments: Vec<&str> = url
        .path_segments()
        .map(|s| s.collect())
        .unwrap_or_default();

    if segments.len() < 2 {
        return Err("URL does not contain a file key".to_string());
    }

    let kind = segments[0];
    if !["file", "design", "board"].contains(&kind) {
        return Err(format!("Unsupported Figma URL type: {kind}"));
    }

    let file_key = segments[1].to_string();

    let node_id = url
        .query_pairs()
        .find(|(k, _)| k == "node-id")
        .map(|(_, v)| v.replace('-', ":"));

    Ok(ParsedFigmaUrl { file_key, node_id })
}

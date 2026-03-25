use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::files::{get_file, get_file_metadata, get_file_nodes, get_images};

#[derive(Subcommand, Debug)]
pub enum FileCommands {
    /// Show file metadata
    Info {
        /// File key or Figma URL
        file: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show file document tree
    Inspect {
        /// File key or Figma URL
        file: String,
        /// Max tree depth
        #[arg(long)]
        depth: Option<u32>,
        /// Specific node ID to inspect
        #[arg(long)]
        node_id: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Export nodes as images
    Export {
        /// File key or Figma URL
        file: String,
        /// Comma-separated node IDs
        #[arg(long)]
        ids: String,
        /// Image format (png, jpg, svg, pdf)
        #[arg(long, default_value = "png")]
        format: String,
        /// Export scale
        #[arg(long, default_value_t = 1.0)]
        scale: f32,
        /// Output directory
        #[arg(long, default_value = ".")]
        output: String,
    },
}

pub async fn run(cmd: FileCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        FileCommands::Info { file, json } => info(&file, json).await,
        FileCommands::Inspect {
            file,
            depth,
            node_id,
            json,
        } => inspect(&file, depth, node_id.as_deref(), json).await,
        FileCommands::Export {
            file,
            ids,
            format,
            scale,
            output,
        } => export(&file, &ids, &format, scale, &output).await,
    }
}

async fn info(file: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;
    let meta = get_file_metadata(&client, &file_key).await?;
    let f = &meta.file;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "name": f.name,
            "last_touched_at": f.last_touched_at,
            "editor_type": f.editor_type,
            "version": f.version,
            "url": f.url,
            "role": f.role,
        }))?);
    } else {
        println!("{} {}", "Name:".bold(), f.name);
        if let Some(ref v) = f.last_touched_at {
            println!("{} {}", "Last modified:".bold(), v.dimmed());
        }
        if let Some(ref v) = f.editor_type {
            println!("{} {}", "Editor:".bold(), v);
        }
        if let Some(ref v) = f.version {
            println!("{} {}", "Version:".bold(), v.dimmed());
        }
        if let Some(ref v) = f.url {
            println!("{} {}", "URL:".bold(), v.dimmed());
        }
    }
    Ok(())
}

async fn inspect(
    file: &str,
    depth: Option<u32>,
    node_id: Option<&str>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;

    if let Some(nid) = node_id {
        let resp = get_file_nodes(&client, &file_key, nid, depth).await?;
        if json {
            println!("{}", serde_json::to_string_pretty(&resp.nodes)?);
        } else {
            if let serde_json::Value::Object(map) = &resp.nodes {
                for (_key, entry) in map {
                    if let Some(doc) = entry.get("document") {
                        print_tree(doc, 0, depth);
                    }
                }
            }
        }
    } else {
        let resp = get_file(&client, &file_key).await?;
        if json {
            println!("{}", serde_json::to_string_pretty(&resp.document)?);
        } else {
            print_tree(&resp.document, 0, depth);
        }
    }
    Ok(())
}

fn print_tree(node: &serde_json::Value, indent: u32, max_depth: Option<u32>) {
    if let Some(max) = max_depth {
        if indent > max {
            return;
        }
    }

    let node_type = node.get("type").and_then(|v| v.as_str()).unwrap_or("?");
    let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("");

    let prefix = "  ".repeat(indent as usize);
    println!("{prefix}{} {} {}", node_type.bold(), name, format!("[{id}]").dimmed());

    if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
        for child in children {
            print_tree(child, indent + 1, max_depth);
        }
    }
}

async fn export(
    file: &str,
    ids: &str,
    format: &str,
    scale: f32,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;

    let resp = get_images(&client, &file_key, ids, Some(format), Some(scale)).await?;

    if let Some(err) = &resp.err {
        return Err(format!("Figma API error: {err}").into());
    }

    tokio::fs::create_dir_all(output).await?;

    let http = reqwest::Client::new();
    for (node_id, url) in &resp.images {
        let Some(url) = url else {
            println!("{} {}", "Skipping".dimmed(), node_id);
            continue;
        };

        let safe_name = node_id.replace(':', "-");
        let filename = format!("{safe_name}.{format}");
        let path = std::path::Path::new(output).join(&filename);

        let bytes = http.get(url).send().await?.bytes().await?;
        tokio::fs::write(&path, &bytes).await?;
        println!("{} {}", "Saved".green(), path.display());
    }
    Ok(())
}

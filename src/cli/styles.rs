use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::components::{get_file_styles, get_team_styles};
use crate::api::types::StyleMeta;
use crate::utils::format_table;

#[derive(Subcommand, Debug)]
pub enum StylesCommands {
    /// List styles
    List {
        /// File key, team ID, or quoted Figma URL
        source: Option<String>,
        /// Figma URL (avoids shell quoting issues)
        #[arg(long)]
        url: Option<String>,
        /// Treat source as a team ID
        #[arg(long)]
        team: bool,
        #[arg(long)]
        json: bool,
    },
}

pub async fn run(cmd: StylesCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        StylesCommands::List { source, url, team, json } => {
            let input = super::require_file_arg(&source, &url)?;
            list(&input, team, json).await
        }
    }
}

async fn list(source: &str, team: bool, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;

    let styles: Vec<StyleMeta> = if team {
        get_team_styles(&client, source).await?.meta.styles
    } else {
        let file_key = super::resolve_file_key(source)?;
        get_file_styles(&client, &file_key).await?.meta.styles
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            styles.iter().map(|s| serde_json::json!({
                "key": s.key,
                "name": s.name,
                "style_type": s.style_type,
                "node_id": s.node_id,
                "description": s.description,
                "file_key": s.file_key,
                "updated_at": s.updated_at,
            })).collect::<Vec<_>>()
        ))?);
    } else if styles.is_empty() {
        println!("{}", "No styles found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = styles
            .iter()
            .map(|s| {
                vec![
                    s.name.clone(),
                    s.style_type.clone(),
                    s.node_id.clone(),
                    s.description.clone(),
                ]
            })
            .collect();
        println!(
            "{}",
            format_table(&["Name", "Type", "Node ID", "Description"], &rows)
        );
    }
    Ok(())
}

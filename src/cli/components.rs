use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::components::{get_file_components, get_team_components};
use crate::api::types::ComponentMeta;
use crate::utils::format_table;

#[derive(Subcommand, Debug)]
pub enum ComponentsCommands {
    /// List components
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

pub async fn run(cmd: ComponentsCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ComponentsCommands::List { source, url, team, json } => {
            let input = super::require_file_arg(&source, &url)?;
            list(&input, team, json).await
        }
    }
}

async fn list(source: &str, team: bool, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;

    let components: Vec<ComponentMeta> = if team {
        get_team_components(&client, source).await?.meta.components
    } else {
        let file_key = super::resolve_file_key(source)?;
        get_file_components(&client, &file_key).await?.meta.components
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            components.iter().map(|c| serde_json::json!({
                "key": c.key,
                "name": c.name,
                "node_id": c.node_id,
                "description": c.description,
                "file_key": c.file_key,
                "updated_at": c.updated_at,
            })).collect::<Vec<_>>()
        ))?);
    } else if components.is_empty() {
        println!("{}", "No components found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = components
            .iter()
            .map(|c| vec![c.name.clone(), c.node_id.clone(), c.description.clone()])
            .collect();
        println!(
            "{}",
            format_table(&["Name", "Node ID", "Description"], &rows)
        );
    }
    Ok(())
}

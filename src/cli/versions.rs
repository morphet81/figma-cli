use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::versions::get_file_versions;
use crate::utils::format_table;

#[derive(Subcommand, Debug)]
pub enum VersionsCommands {
    /// List file version history
    List {
        /// File key or Figma URL
        file: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

pub async fn run(cmd: VersionsCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        VersionsCommands::List { file, json } => list(&file, json).await,
    }
}

async fn list(file: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;
    let resp = get_file_versions(&client, &file_key).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            resp.versions.iter().map(|v| serde_json::json!({
                "id": v.id,
                "label": v.label,
                "description": v.description,
                "created_at": v.created_at,
                "user": v.user.handle,
            })).collect::<Vec<_>>()
        ))?);
    } else if resp.versions.is_empty() {
        println!("{}", "No versions found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = resp
            .versions
            .iter()
            .map(|v| {
                vec![
                    v.id.clone(),
                    v.label.clone().unwrap_or_default(),
                    v.user.handle.clone(),
                    v.created_at.clone(),
                    v.description.clone().unwrap_or_default(),
                ]
            })
            .collect();
        println!(
            "{}",
            format_table(
                &["ID", "Label", "User", "Created", "Description"],
                &rows,
            )
        );
    }
    Ok(())
}

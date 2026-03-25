use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::projects::{get_project_files, get_team_projects};
use crate::utils::format_table;

#[derive(Subcommand, Debug)]
pub enum ProjectsCommands {
    /// List projects in a team
    List {
        /// Team ID
        team_id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// List files in a project
    Files {
        /// Project ID
        project_id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

pub async fn run(cmd: ProjectsCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ProjectsCommands::List { team_id, json } => list(&team_id, json).await,
        ProjectsCommands::Files { project_id, json } => files(&project_id, json).await,
    }
}

async fn list(team_id: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let resp = get_team_projects(&client, team_id).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            resp.projects.iter().map(|p| serde_json::json!({
                "id": p.id,
                "name": p.name,
            })).collect::<Vec<_>>()
        ))?);
    } else if resp.projects.is_empty() {
        println!("{}", "No projects found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = resp
            .projects
            .iter()
            .map(|p| vec![p.id.clone(), p.name.clone()])
            .collect();
        println!("{}", format_table(&["ID", "Name"], &rows));
    }
    Ok(())
}

async fn files(project_id: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let resp = get_project_files(&client, project_id).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            resp.files.iter().map(|f| serde_json::json!({
                "key": f.key,
                "name": f.name,
                "last_modified": f.last_modified,
            })).collect::<Vec<_>>()
        ))?);
    } else if resp.files.is_empty() {
        println!("{}", "No files found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = resp
            .files
            .iter()
            .map(|f| vec![f.key.clone(), f.name.clone(), f.last_modified.clone()])
            .collect();
        println!(
            "{}",
            format_table(&["Key", "Name", "Last Modified"], &rows)
        );
    }
    Ok(())
}

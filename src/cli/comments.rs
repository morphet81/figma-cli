use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::comments::{delete_comment, get_comments, post_comment};
use crate::utils::format_table;

#[derive(Subcommand, Debug)]
pub enum CommentsCommands {
    /// List all comments on a file
    List {
        /// File key (or quoted Figma URL)
        file: Option<String>,
        /// Figma URL (avoids shell quoting issues)
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Add a comment to a file
    Add {
        /// File key (or quoted Figma URL)
        file: Option<String>,
        /// Figma URL (avoids shell quoting issues)
        #[arg(long)]
        url: Option<String>,
        /// Comment message
        #[arg(short, long)]
        message: String,
    },
    /// Delete a comment
    Delete {
        /// File key (or quoted Figma URL)
        file: Option<String>,
        /// Figma URL (avoids shell quoting issues)
        #[arg(long)]
        url: Option<String>,
        /// Comment ID to delete
        #[arg(long)]
        comment_id: String,
    },
}

pub async fn run(cmd: CommentsCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        CommentsCommands::List { file, url, json } => {
            let input = super::require_file_arg(&file, &url)?;
            list(&input, json).await
        }
        CommentsCommands::Add { file, url, message } => {
            let input = super::require_file_arg(&file, &url)?;
            add(&input, &message).await
        }
        CommentsCommands::Delete {
            file,
            url,
            comment_id,
        } => {
            let input = super::require_file_arg(&file, &url)?;
            delete(&input, &comment_id).await
        }
    }
}

async fn list(file: &str, json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;
    let resp = get_comments(&client, &file_key).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!(
            resp.comments.iter().map(|c| serde_json::json!({
                "id": c.id,
                "user": c.user.handle,
                "message": c.message,
                "created_at": c.created_at,
                "resolved_at": c.resolved_at,
            })).collect::<Vec<_>>()
        ))?);
    } else if resp.comments.is_empty() {
        println!("{}", "No comments found.".dimmed());
    } else {
        let rows: Vec<Vec<String>> = resp
            .comments
            .iter()
            .map(|c| {
                vec![
                    c.id.clone(),
                    c.user.handle.clone(),
                    c.message.clone(),
                    c.created_at.clone(),
                ]
            })
            .collect();
        println!(
            "{}",
            format_table(&["ID", "User", "Message", "Created"], &rows)
        );
    }
    Ok(())
}

async fn add(file: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;
    let comment = post_comment(&client, &file_key, message).await?;
    println!(
        "{} {}",
        "Comment added.".green(),
        format!("[id: {}]", comment.id).dimmed()
    );
    Ok(())
}

async fn delete(file: &str, comment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let file_key = super::resolve_file_key(file)?;
    delete_comment(&client, &file_key, comment_id).await?;
    println!("{}", "Comment deleted.".green());
    Ok(())
}

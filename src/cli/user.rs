use clap::Subcommand;
use colored::Colorize;

use crate::api::endpoints::users::get_me;

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// Show current user info
    Me {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

pub async fn run(cmd: UserCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        UserCommands::Me { json } => me(json).await,
    }
}

async fn me(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let client = super::get_client()?;
    let user = get_me(&client).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "id": user.id,
            "handle": user.handle,
            "email": user.email,
            "img_url": user.img_url,
        }))?);
    } else {
        println!("{} {}", "ID:".bold(), user.id);
        println!("{} {}", "Handle:".bold(), user.handle);
        println!("{} {}", "Email:".bold(), user.email);
        println!("{} {}", "Avatar:".bold(), user.img_url.dimmed());
    }
    Ok(())
}

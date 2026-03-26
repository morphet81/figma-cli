use clap::Subcommand;
use colored::Colorize;
use dialoguer::{Input, Password};

use crate::auth::{
    clear_tokens, get_auth_status, prompt_for_token, save_tokens, start_oauth_flow, validate_token,
    AuthTokens, OAuthConfig, TokenType,
};

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    /// Authenticate with Figma
    Login {
        /// Use OAuth 2.0 flow (opens browser)
        #[arg(long)]
        oauth: bool,
        /// Use Personal Access Token
        #[arg(long)]
        pat: bool,
    },
    /// Clear stored authentication tokens
    Logout,
    /// Show current authentication status
    Status,
}

pub async fn run(cmd: AuthCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        AuthCommands::Login { oauth, pat } => login(oauth, pat).await,
        AuthCommands::Logout => logout(),
        AuthCommands::Status => status(),
    }
}

async fn login(_oauth: bool, pat: bool) -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("FIGMA_ACCESS_TOKEN").is_ok() {
        println!("{}", "Already authenticated via FIGMA_ACCESS_TOKEN env var.".green());
        println!("{}", "Unset it first if you want to use a different auth method.".dimmed());
        return Ok(());
    }

    if pat {
        let token = prompt_for_token()?;
        if validate_token(&token).await? {
            let tokens = AuthTokens {
                access_token: token,
                refresh_token: None,
                expires_at: None,
                token_type: TokenType::Pat,
            };
            save_tokens(&tokens)?;
            println!("{}", "Successfully authenticated with PAT!".green());
        } else {
            println!("{}", "Invalid token. Authentication failed.".red());
        }
    } else {
        let client_id: String = Input::new()
            .with_prompt("Client ID")
            .interact_text()?;
        let client_secret: String = Password::new()
            .with_prompt("Client Secret")
            .interact()?;

        let config = OAuthConfig {
            client_id,
            client_secret,
            redirect_uri: "http://localhost:9876/callback".to_string(),
            scopes: vec![
                "current_user:read".into(),
                "file_content:read".into(),
                "file_comments:read".into(),
                "file_comments:write".into(),
                "file_metadata:read".into(),
                "file_versions:read".into(),
                "projects:read".into(),
                "team_library_content:read".into(),
                "library_content:read".into(),
                "library_assets:read".into(),
                "webhooks:read".into(),
            ],
        };

        let tokens = start_oauth_flow(&config).await?;
        save_tokens(&tokens)?;
        println!("{}", "Successfully authenticated via OAuth!".green());
    }
    Ok(())
}

fn logout() -> Result<(), Box<dyn std::error::Error>> {
    clear_tokens()?;
    println!("{}", "Logged out. Stored tokens cleared.".green());
    Ok(())
}

fn status() -> Result<(), Box<dyn std::error::Error>> {
    let status = get_auth_status();
    if status.authenticated {
        println!("{} {}", "Status:".bold(), "Authenticated".green());
        if status.from_env {
            println!("{} {}", "Source:".bold(), "FIGMA_ACCESS_TOKEN env var");
        } else if let Some(token_type) = &status.token_type {
            let label = match token_type {
                TokenType::Oauth => "OAuth",
                TokenType::Pat => "Personal Access Token",
            };
            println!("{} {}", "Type:".bold(), label);
        }
        if let Some(expires_at) = status.expires_at {
            let dt = chrono::DateTime::from_timestamp(expires_at, 0)
                .map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| "unknown".into());
            println!("{} {}", "Expires:".bold(), dt.dimmed());
        }
    } else {
        println!(
            "{} {}",
            "Status:".bold(),
            "Not authenticated".red()
        );
        println!(
            "{}",
            "Set FIGMA_ACCESS_TOKEN or run `fcli auth login` to authenticate.".dimmed()
        );
    }
    Ok(())
}

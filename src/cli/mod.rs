mod auth;
mod comments;
mod components;
mod file;
mod install;
mod projects;
mod styles;
mod user;
mod versions;

use clap::{Parser, Subcommand};

use crate::api::FigmaClient;
use crate::auth::{load_tokens, TokenType};

#[derive(Parser, Debug)]
#[command(name = "fcli", version, about = "CLI tool for interacting with Figma through the REST API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage authentication
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },
    /// User commands
    User {
        #[command(subcommand)]
        command: user::UserCommands,
    },
    /// File commands
    File {
        #[command(subcommand)]
        command: file::FileCommands,
    },
    /// Manage file comments
    Comments {
        #[command(subcommand)]
        command: comments::CommentsCommands,
    },
    /// Manage projects
    Projects {
        #[command(subcommand)]
        command: projects::ProjectsCommands,
    },
    /// Manage components
    Components {
        #[command(subcommand)]
        command: components::ComponentsCommands,
    },
    /// Manage styles
    Styles {
        #[command(subcommand)]
        command: styles::StylesCommands,
    },
    /// Manage file versions
    Versions {
        #[command(subcommand)]
        command: versions::VersionsCommands,
    },
    /// Install tools and integrations
    Install {
        #[command(subcommand)]
        command: install::InstallCommands,
    },
}

pub(crate) fn get_client() -> Result<FigmaClient, Box<dyn std::error::Error>> {
    if let Ok(token) = std::env::var("FIGMA_ACCESS_TOKEN") {
        return Ok(FigmaClient::new(token, false));
    }
    let tokens = load_tokens().ok_or("Not authenticated. Set FIGMA_ACCESS_TOKEN or run `fcli auth login`.")?;
    let is_oauth = tokens.token_type == TokenType::Oauth;
    Ok(FigmaClient::new(tokens.access_token, is_oauth))
}

pub(crate) fn require_file_arg(file: &Option<String>, url: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match (file, url) {
        (_, Some(u)) => Ok(u.clone()),
        (Some(f), None) => Ok(f.clone()),
        (None, None) => Err("Provide a file key or --url <figma-url>. Figma URLs must be passed via --url or quoted to prevent shell interpretation of ? and &.".into()),
    }
}

pub(crate) fn resolve_file_key(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(crate::utils::parse_figma_url(input)?.file_key)
}

pub(crate) fn resolve_file(input: &str) -> Result<crate::utils::ParsedFigmaUrl, Box<dyn std::error::Error>> {
    crate::utils::parse_figma_url(input).map_err(|e| e.into())
}

pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Auth { command } => auth::run(command).await,
        Commands::User { command } => user::run(command).await,
        Commands::File { command } => file::run(command).await,
        Commands::Comments { command } => comments::run(command).await,
        Commands::Projects { command } => projects::run(command).await,
        Commands::Components { command } => components::run(command).await,
        Commands::Styles { command } => styles::run(command).await,
        Commands::Versions { command } => versions::run(command).await,
        Commands::Install { command } => install::run(command).await,
    }
}

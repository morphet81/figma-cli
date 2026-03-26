use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Oauth,
    Pat,
}

#[derive(Debug)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub token_type: Option<TokenType>,
    pub expires_at: Option<i64>,
    pub from_env: bool,
}

fn auth_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("fcli").join("auth.json"))
}

pub fn save_tokens(tokens: &AuthTokens) -> Result<(), Box<dyn std::error::Error>> {
    let path = auth_file_path().ok_or("could not determine config directory")?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(tokens)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn load_tokens() -> Option<AuthTokens> {
    let path = auth_file_path()?;
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn clear_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let path = auth_file_path().ok_or("could not determine config directory")?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn get_auth_status() -> AuthStatus {
    if std::env::var("FIGMA_ACCESS_TOKEN").is_ok() {
        return AuthStatus {
            authenticated: true,
            token_type: Some(TokenType::Pat),
            expires_at: None,
            from_env: true,
        };
    }
    match load_tokens() {
        Some(tokens) => AuthStatus {
            authenticated: true,
            token_type: Some(tokens.token_type),
            expires_at: tokens.expires_at,
            from_env: false,
        },
        None => AuthStatus {
            authenticated: false,
            token_type: None,
            expires_at: None,
            from_env: false,
        },
    }
}

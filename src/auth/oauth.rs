use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::config::{AuthTokens, TokenType};

#[derive(Debug)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

pub fn generate_code_verifier() -> String {
    let mut buf = [0u8; 64];
    rand::rng().fill(&mut buf);
    URL_SAFE_NO_PAD.encode(buf)
}

pub fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}

pub fn build_auth_url(config: &OAuthConfig, code_challenge: &str, state: &str) -> String {
    let scopes = config.scopes.join(",");
    let params = [
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", config.redirect_uri.as_str()),
        ("scope", &scopes),
        ("state", state),
        ("response_type", "code"),
        ("code_challenge", code_challenge),
    ];
    url::Url::parse_with_params("https://www.figma.com/oauth", &params)
        .expect("failed to build auth URL")
        .to_string()
}

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: String,
}

pub async fn start_oauth_flow(
    config: &OAuthConfig,
) -> Result<AuthTokens, Box<dyn std::error::Error>> {
    let verifier = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);

    let mut state_bytes = [0u8; 32];
    rand::rng().fill(&mut state_bytes);
    let state: String = state_bytes.iter().map(|b| format!("{b:02x}")).collect();

    let auth_url = build_auth_url(config, &challenge, &state);

    let listener = TcpListener::bind("127.0.0.1:9876").await?;
    open::that(&auth_url)?;

    let (mut stream, _) = listener.accept().await?;

    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..n]);

    let request_line = request.lines().next().ok_or("empty HTTP request")?;
    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or("malformed HTTP request")?;

    let full_url = format!("http://localhost{path}");
    let parsed = url::Url::parse(&full_url)?;
    let params: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();

    let received_state = params.get("state").ok_or("missing state parameter")?;
    if *received_state != state {
        return Err("state mismatch — possible CSRF attack".into());
    }
    let code = params.get("code").ok_or("missing code parameter")?;

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h1>Authentication successful!</h1>\
        <p>You can close this tab.</p></body></html>";
    stream.write_all(response.as_bytes()).await?;
    stream.shutdown().await?;

    let credentials = STANDARD.encode(format!("{}:{}", config.client_id, config.client_secret));
    let body = format!(
        "redirect_uri={}&code={}&grant_type=authorization_code&code_verifier={}",
        urlencoded(&config.redirect_uri),
        urlencoded(code),
        urlencoded(&verifier),
    );

    let client = reqwest::Client::new();
    let token_resp: OAuthTokenResponse = client
        .post("https://api.figma.com/v1/oauth/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {credentials}"))
        .body(body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(AuthTokens {
        access_token: token_resp.access_token,
        refresh_token: Some(token_resp.refresh_token),
        expires_at: Some(Utc::now().timestamp() + token_resp.expires_in as i64),
        token_type: TokenType::Oauth,
    })
}

pub async fn refresh_access_token(
    config: &OAuthConfig,
    refresh_token: &str,
) -> Result<AuthTokens, Box<dyn std::error::Error>> {
    let credentials = STANDARD.encode(format!("{}:{}", config.client_id, config.client_secret));
    let body = format!("refresh_token={}", urlencoded(refresh_token));

    let client = reqwest::Client::new();
    let token_resp: OAuthTokenResponse = client
        .post("https://api.figma.com/v1/oauth/refresh")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {credentials}"))
        .body(body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(AuthTokens {
        access_token: token_resp.access_token,
        refresh_token: Some(token_resp.refresh_token),
        expires_at: Some(Utc::now().timestamp() + token_resp.expires_in as i64),
        token_type: TokenType::Oauth,
    })
}

fn urlencoded(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

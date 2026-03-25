use rpassword::prompt_password;

pub fn prompt_for_token() -> Result<String, Box<dyn std::error::Error>> {
    let token = prompt_password("Enter your Figma Personal Access Token: ")?;
    Ok(token)
}

pub async fn validate_token(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.figma.com/v1/me")
        .header("X-Figma-Token", token)
        .send()
        .await?;
    Ok(resp.status() == reqwest::StatusCode::OK)
}

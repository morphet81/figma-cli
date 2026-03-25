use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::time::Duration;

pub struct FigmaClient {
    http: reqwest::Client,
    base_url: String,
    access_token: String,
    is_oauth: bool,
}

impl FigmaClient {
    pub fn new(access_token: String, is_oauth: bool) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: "https://api.figma.com".to_string(),
            access_token,
            is_oauth,
        }
    }

    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if self.is_oauth {
            let val = format!("Bearer {}", self.access_token);
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&val).unwrap());
        } else {
            headers.insert(
                "X-Figma-Token",
                HeaderValue::from_str(&self.access_token).unwrap(),
            );
        }
        headers
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);
        let filtered: Vec<(&str, &str)> = params.iter().copied().filter(|(_, v)| !v.is_empty()).collect();

        let mut retries = 0u8;
        loop {
            let resp = self
                .http
                .get(&url)
                .headers(self.auth_headers())
                .query(&filtered)
                .send()
                .await?;

            if resp.status() == 429 && retries < 3 {
                let wait = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(5);
                tokio::time::sleep(Duration::from_secs(wait)).await;
                retries += 1;
                continue;
            }

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("API error {status}: {body}").into());
            }

            return Ok(resp.json::<T>().await?);
        }
    }

    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);

        let mut retries = 0u8;
        loop {
            let resp = self
                .http
                .post(&url)
                .headers(self.auth_headers())
                .json(body)
                .send()
                .await?;

            if resp.status() == 429 && retries < 3 {
                let wait = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(5);
                tokio::time::sleep(Duration::from_secs(wait)).await;
                retries += 1;
                continue;
            }

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("API error {status}: {body}").into());
            }

            return Ok(resp.json::<T>().await?);
        }
    }

    pub async fn delete(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);

        let mut retries = 0u8;
        loop {
            let resp = self
                .http
                .delete(&url)
                .headers(self.auth_headers())
                .send()
                .await?;

            if resp.status() == 429 && retries < 3 {
                let wait = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(5);
                tokio::time::sleep(Duration::from_secs(wait)).await;
                retries += 1;
                continue;
            }

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("API error {status}: {body}").into());
            }

            return Ok(());
        }
    }
}

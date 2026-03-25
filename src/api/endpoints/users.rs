use crate::api::client::FigmaClient;
use crate::api::types::GetMeResponse;

pub async fn get_me(client: &FigmaClient) -> Result<GetMeResponse, Box<dyn std::error::Error>> {
    client.get("/v1/me", &[]).await
}

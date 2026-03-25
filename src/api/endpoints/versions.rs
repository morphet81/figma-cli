use crate::api::client::FigmaClient;
use crate::api::types::GetFileVersionsResponse;

pub async fn get_file_versions(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetFileVersionsResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/files/{file_key}/versions"), &[])
        .await
}

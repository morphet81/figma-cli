use crate::api::client::FigmaClient;
use crate::api::types::{GetFileMetaResponse, GetFileNodesResponse, GetFileResponse, GetImagesResponse};

pub async fn get_file(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetFileResponse, Box<dyn std::error::Error>> {
    client.get(&format!("/v1/files/{file_key}"), &[]).await
}

pub async fn get_file_nodes(
    client: &FigmaClient,
    file_key: &str,
    ids: &str,
    depth: Option<u32>,
) -> Result<GetFileNodesResponse, Box<dyn std::error::Error>> {
    let depth_str = depth.map(|d| d.to_string()).unwrap_or_default();
    client
        .get(
            &format!("/v1/files/{file_key}/nodes"),
            &[("ids", ids), ("depth", &depth_str)],
        )
        .await
}

pub async fn get_file_metadata(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetFileMetaResponse, Box<dyn std::error::Error>> {
    client.get(&format!("/v1/files/{file_key}/meta"), &[]).await
}

pub async fn get_images(
    client: &FigmaClient,
    file_key: &str,
    ids: &str,
    format: Option<&str>,
    scale: Option<f32>,
) -> Result<GetImagesResponse, Box<dyn std::error::Error>> {
    let format_str = format.unwrap_or_default().to_string();
    let scale_str = scale.map(|s| s.to_string()).unwrap_or_default();
    client
        .get(
            &format!("/v1/images/{file_key}"),
            &[("ids", ids), ("format", &format_str), ("scale", &scale_str)],
        )
        .await
}

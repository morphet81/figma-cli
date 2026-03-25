use crate::api::client::FigmaClient;
use crate::api::types::{
    GetFileComponentsResponse, GetFileStylesResponse, GetTeamComponentsResponse,
    GetTeamStylesResponse,
};

pub async fn get_team_components(
    client: &FigmaClient,
    team_id: &str,
) -> Result<GetTeamComponentsResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/teams/{team_id}/components"), &[])
        .await
}

pub async fn get_file_components(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetFileComponentsResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/files/{file_key}/components"), &[])
        .await
}

pub async fn get_team_styles(
    client: &FigmaClient,
    team_id: &str,
) -> Result<GetTeamStylesResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/teams/{team_id}/styles"), &[])
        .await
}

pub async fn get_file_styles(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetFileStylesResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/files/{file_key}/styles"), &[])
        .await
}

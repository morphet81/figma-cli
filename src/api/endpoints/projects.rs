use crate::api::client::FigmaClient;
use crate::api::types::{GetProjectFilesResponse, GetTeamProjectsResponse};

pub async fn get_team_projects(
    client: &FigmaClient,
    team_id: &str,
) -> Result<GetTeamProjectsResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/teams/{team_id}/projects"), &[])
        .await
}

pub async fn get_project_files(
    client: &FigmaClient,
    project_id: &str,
) -> Result<GetProjectFilesResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/projects/{project_id}/files"), &[])
        .await
}

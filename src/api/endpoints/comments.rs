use crate::api::client::FigmaClient;
use crate::api::types::{Comment, GetCommentsResponse};

pub async fn get_comments(
    client: &FigmaClient,
    file_key: &str,
) -> Result<GetCommentsResponse, Box<dyn std::error::Error>> {
    client
        .get(&format!("/v1/files/{file_key}/comments"), &[])
        .await
}

pub async fn post_comment(
    client: &FigmaClient,
    file_key: &str,
    message: &str,
) -> Result<Comment, Box<dyn std::error::Error>> {
    let body = serde_json::json!({ "message": message });
    client
        .post(&format!("/v1/files/{file_key}/comments"), &body)
        .await
}

pub async fn delete_comment(
    client: &FigmaClient,
    file_key: &str,
    comment_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .delete(&format!("/v1/files/{file_key}/comments/{comment_id}"))
        .await
}

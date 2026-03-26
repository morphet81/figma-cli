use serde::Deserialize;
use std::collections::HashMap;

fn string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    pub handle: String,
    pub img_url: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetMeResponse {
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    pub handle: String,
    pub img_url: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct GetFileResponse {
    pub name: String,
    pub role: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    #[serde(rename = "editorType")]
    pub editor_type: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub document: serde_json::Value,
    #[serde(default)]
    pub components: serde_json::Value,
    #[serde(rename = "schemaVersion", default)]
    pub schema_version: i32,
    #[serde(default)]
    pub styles: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GetFileNodesResponse {
    pub name: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    #[serde(default)]
    pub nodes: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct FileMetaInfo {
    pub name: String,
    pub folder_name: Option<String>,
    pub last_touched_at: Option<String>,
    pub thumbnail_url: Option<String>,
    #[serde(rename = "editorType")]
    pub editor_type: Option<String>,
    pub version: Option<String>,
    pub role: Option<String>,
    pub link_access: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFileMetaResponse {
    pub file: FileMetaInfo,
}

#[derive(Debug, Deserialize)]
pub struct GetImagesResponse {
    pub err: Option<String>,
    #[serde(default)]
    pub images: HashMap<String, Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    #[serde(default)]
    pub message: String,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub user: User,
    pub order_id: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetCommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamProjectsResponse {
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFile {
    pub key: String,
    pub name: String,
    pub last_modified: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetProjectFilesResponse {
    pub files: Vec<ProjectFile>,
}

#[derive(Debug, Deserialize)]
pub struct ComponentMeta {
    pub key: String,
    pub file_key: String,
    pub node_id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub thumbnail_url: Option<String>,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ComponentsListMeta {
    pub components: Vec<ComponentMeta>,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamComponentsResponse {
    pub meta: ComponentsListMeta,
}

#[derive(Debug, Deserialize)]
pub struct GetFileComponentsResponse {
    pub meta: ComponentsListMeta,
}

#[derive(Debug, Deserialize)]
pub struct StyleMeta {
    pub key: String,
    pub file_key: String,
    pub node_id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub style_type: String,
    pub thumbnail_url: Option<String>,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct StylesListMeta {
    pub styles: Vec<StyleMeta>,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamStylesResponse {
    pub meta: StylesListMeta,
}

#[derive(Debug, Deserialize)]
pub struct GetFileStylesResponse {
    pub meta: StylesListMeta,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(deserialize_with = "string_or_number")]
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct GetFileVersionsResponse {
    pub versions: Vec<Version>,
}

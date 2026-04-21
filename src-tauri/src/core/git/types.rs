use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitStatusEntry {
    pub path: String,
    pub x: String,
    pub y: String,
    pub is_untracked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitStatusResponse {
    pub branch: Option<String>,
    pub remote_url: Option<String>,
    pub ahead_behind: Option<(usize, usize)>,
    pub staged: Vec<GitStatusEntry>,
    pub unstaged: Vec<GitStatusEntry>,
    pub untracked: Vec<GitStatusEntry>,
    pub is_clean: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitDiffResponse {
    pub diff: String,
    pub stat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitLogEntry {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitLogResponse {
    pub entries: Vec<GitLogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitOperationResult {
    pub success: bool,
    pub message: String,
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Highlight {
    pub id: String,
    pub user_id: String,
    pub article_id: String,
    pub content: String,
    pub note: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub start_container_path: String,
    pub end_container_path: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHighlightRequest {
    pub article_id: String,
    pub content: String,
    pub note: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub start_container_path: String,
    pub end_container_path: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHighlightRequest {
    pub note: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HighlightListResponse {
    pub highlights: Vec<Highlight>,
    pub total: i64,
}
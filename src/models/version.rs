use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleVersion {
    pub id: String,
    pub article_id: String,
    pub version_number: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub content_html: String,
    pub excerpt: String,
    pub cover_image_url: Option<String>,
    pub tags: Vec<String>,
    pub author_id: String,
    pub author_name: String,
    pub change_summary: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionDiff {
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleVersionComparison {
    pub version_a: ArticleVersion,
    pub version_b: ArticleVersion,
    pub diffs: Vec<VersionDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVersionRequest {
    pub article_id: String,
    pub change_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreVersionRequest {
    pub article_id: String,
    pub version_id: String,
}
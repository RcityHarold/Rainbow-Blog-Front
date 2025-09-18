use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BookmarkItem {
    pub id: String,
    pub article_id: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,

    // Article preview fields
    pub article_title: String,
    pub article_slug: String,
    pub article_excerpt: Option<String>,
    pub article_cover_image: Option<String>,
    pub article_reading_time: i32,
    pub author_name: String,
    pub author_username: String,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkListResponse {
    pub success: bool,
    pub data: Vec<BookmarkItem>,
}


use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub content_html: String,
    pub excerpt: String,
    pub cover_image_url: Option<String>,
    pub author: Author,
    pub publication: Option<Publication>,
    pub series: Option<Series>,
    pub tags: Vec<Tag>,
    pub status: String,
    pub is_paid_content: bool,
    pub is_featured: bool,
    pub reading_time: i32,
    pub word_count: i32,
    pub view_count: i32,
    pub clap_count: i32,
    pub comment_count: i32,
    pub bookmark_count: i32,
    pub share_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub is_bookmarked: bool,
    #[serde(default)]
    pub is_clapped: bool,
    #[serde(default)]
    pub user_clap_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleListResponse {
    pub articles: Vec<Article>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    pub current_page: i32,
    pub total_pages: i32,
    pub total_items: i32,
    pub items_per_page: i32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub publication_id: Option<String>,
    pub series_id: Option<String>,
    pub series_order: Option<i32>,
    pub is_paid_content: bool,
    pub tags: Vec<String>,
    pub save_as_draft: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub publication_id: Option<String>,
    pub series_id: Option<String>,
    pub series_order: Option<i32>,
    pub is_paid_content: Option<bool>,
    pub tags: Option<Vec<String>>,
}
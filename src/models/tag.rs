use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub color: Option<String>,
    pub article_count: i32,
    pub follower_count: i32,
    pub is_following: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
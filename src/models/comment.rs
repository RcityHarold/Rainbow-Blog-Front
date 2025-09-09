use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: String,
    pub article_id: String,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub is_author_response: bool,
    pub clap_count: i64,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentWithAuthor {
    #[serde(flatten)]
    pub comment: Comment,
    pub author_name: String,
    pub author_username: String,
    pub author_avatar: Option<String>,
    pub user_has_clapped: bool,
    pub replies: Vec<CommentWithAuthor>,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            id: String::new(),
            article_id: String::new(),
            author_id: String::new(),
            parent_id: None,
            content: String::new(),
            is_author_response: false,
            clap_count: 0,
            is_edited: false,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}
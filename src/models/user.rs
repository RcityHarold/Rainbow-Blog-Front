use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
            email: String::new(),
            display_name: None,
            bio: None,
            avatar_url: None,
            is_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub twitter_username: Option<String>,
    pub github_username: Option<String>,
    pub linkedin_url: Option<String>,
    pub facebook_url: Option<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub article_count: i32,
    pub total_claps_received: i32,
    pub is_verified: bool,
    #[serde(default)]
    pub is_suspended: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub profile: UserProfile,
    pub recent_articles: Vec<RecentArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentArticle {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub published_at: DateTime<Utc>,
    pub clap_count: i32,
    pub reading_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub articles_written: i32,
    pub comments_made: i32,
    pub claps_given: i32,
    pub claps_received: i32,
    pub followers: i32,
    pub following: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub twitter_username: Option<String>,
    pub github_username: Option<String>,
    pub linkedin_url: Option<String>,
    pub facebook_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserProfile>,
    pub pagination: super::article::Pagination,
}
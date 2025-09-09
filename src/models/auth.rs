use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

// Rainbow-Auth 实际返回的响应格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RainbowAuthResponse {
    pub token: String,
    pub user: RainbowAuthUser,
}

// Rainbow-Auth 的用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RainbowAuthUser {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub is_email_verified: Option<bool>,
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub has_password: bool,
    pub account_status: String,
    pub last_login_at: Option<i64>,
}

// 前端使用的统一响应格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub is_following: bool,
    pub created_at: DateTime<Utc>,
}
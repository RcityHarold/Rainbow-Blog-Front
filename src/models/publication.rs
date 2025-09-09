use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub logo_url: Option<String>,
    pub header_image_url: Option<String>,
    pub domain: Option<String>,
    pub social_links: Option<SocialLinks>,
    pub categories: Vec<String>,
    pub is_verified: bool,
    pub member_count: i32,
    pub article_count: i32,
    pub follower_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialLinks {
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublicationRequest {
    pub name: String,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub logo_url: Option<String>,
    pub header_image_url: Option<String>,
    pub categories: Vec<String>,
    pub social_links: Option<SocialLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublicationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub logo_url: Option<String>,
    pub header_image_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub social_links: Option<SocialLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicationMember {
    pub user: User,
    pub role: MemberRole,
    pub status: MemberStatus,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MemberRole {
    Owner,
    Editor,
    Writer,
    Contributor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MemberStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: MemberRole,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationListResponse {
    pub publications: Vec<Publication>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListResponse {
    pub members: Vec<PublicationMember>,
    pub total: i64,
    pub page: i32,
    pub total_pages: i32,
}
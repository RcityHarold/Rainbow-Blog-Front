use super::client::{ApiClient, ApiResult};
use crate::models::{
    user::{UserProfileResponse, UserStats, UpdateProfileRequest, UserListResponse},
    article::{ArticleListResponse, Article, Author, Pagination},
};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

// 创建用户扩展信息的请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateUserProfileRequest {
    pub auth_user_id: String,  // Rainbow-Auth 的用户 ID
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
}

pub struct UserService;

impl UserService {
    // 创建用户在 Blog 系统中的扩展信息（注册后调用）
    pub async fn create_user_profile(request: &CreateUserProfileRequest) -> ApiResult<()> {
        API_CLIENT.post("/blog/users/profile", request).await
    }
    
    pub async fn get_user_profile(username: &str) -> ApiResult<UserProfileResponse> {
        API_CLIENT.get(&format!("/blog/users/{}", username)).await
    }
    
    // 通过用户ID获取用户资料
    pub async fn get_user_profile_by_id(user_id: &str) -> ApiResult<UserProfileResponse> {
        API_CLIENT.get(&format!("/blog/users/by-id/{}", user_id)).await
    }
    
    pub async fn get_user_stats(username: &str) -> ApiResult<UserStats> {
        API_CLIENT.get(&format!("/blog/users/{}/stats", username)).await
    }
    
    pub async fn get_user_articles(
        username: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let mut url = format!("/blog/users/{}/articles", username);
        let mut query_params = vec![];
        
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        // 解析为原始列表，再转换为前端 ArticleListResponse
        let raw: crate::api::articles::RawArticleListResponse = API_CLIENT.get(&url).await?;
        let articles: Vec<Article> = raw.articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(),
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag { id: t.id, name: t.name, slug: t.slug }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0,
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0,
            share_count: 0,
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at,
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        Ok(ArticleListResponse { articles, pagination: raw.pagination })
    }
    
    // 通过用户ID获取用户文章
    pub async fn get_user_articles_by_id(
        user_id: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let mut url = format!("/blog/users/by-id/{}/articles", user_id);
        let mut query_params = vec![];
        
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        let raw: crate::api::articles::RawArticleListResponse = API_CLIENT.get(&url).await?;
        let articles: Vec<Article> = raw.articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(),
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag { id: t.id, name: t.name, slug: t.slug }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0,
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0,
            share_count: 0,
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at,
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        Ok(ArticleListResponse { articles, pagination: raw.pagination })
    }
    
    pub async fn get_current_user_profile() -> ApiResult<UserProfileResponse> {
        API_CLIENT.get("/blog/users/me").await
    }
    
    pub async fn update_profile(updates: &UpdateProfileRequest) -> ApiResult<UserProfileResponse> {
        API_CLIENT.put("/blog/users/me", updates).await
    }
    
    pub async fn follow_user(user_id: &str) -> ApiResult<()> {
        API_CLIENT.post(&format!("/blog/follows/user/{}/follow", user_id), &()).await
    }
    
    pub async fn unfollow_user(user_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/follows/user/{}/follow", user_id)).await
    }
    
    pub async fn get_followers(
        user_id: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<UserListResponse> {
        let mut url = format!("/blog/follows/user/{}/followers", user_id);
        let mut query_params = vec![];
        
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        API_CLIENT.get(&url).await
    }
    
    pub async fn get_following(
        user_id: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<UserListResponse> {
        let mut url = format!("/blog/follows/user/{}/following", user_id);
        let mut query_params = vec![];
        
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        API_CLIENT.get(&url).await
    }
    
    pub async fn is_following(user_id: &str) -> ApiResult<bool> {
        #[derive(serde::Deserialize)]
        struct Response {
            is_following: bool,
        }
        
        let response: Response = API_CLIENT
            .get(&format!("/blog/follows/user/{}/is-following", user_id))
            .await?;
        
        Ok(response.is_following)
    }
}

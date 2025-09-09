use super::client::{ApiClient, ApiResult};
use crate::models::{
    user::{UserProfileResponse, UserStats, UpdateProfileRequest, UserListResponse},
    article::ArticleListResponse,
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
        
        API_CLIENT.get(&url).await
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
        
        API_CLIENT.get(&url).await
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
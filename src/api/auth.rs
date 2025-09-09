use super::client::{ApiClient, ApiResult, ApiError};
use super::users::{UserService, CreateUserProfileRequest};
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest, User, RainbowAuthResponse, RainbowAuthUser};
use once_cell::sync::Lazy;
use chrono::Utc;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct AuthService;

impl AuthService {
    // 将 Rainbow-Auth 的响应转换为前端期望的格式
    fn convert_auth_response(auth_resp: RainbowAuthResponse) -> AuthResponse {
        let user = User {
            id: auth_resp.user.id.clone(),
            username: auth_resp.user.email.split('@').next().unwrap_or("user").to_string(), // 从 email 生成默认用户名
            email: auth_resp.user.email.clone(),
            full_name: None, // Rainbow-Auth 不返回这个字段
            bio: None,
            avatar_url: None,
            followers_count: 0,
            following_count: 0,
            created_at: auth_resp.user.created_at,
            updated_at: auth_resp.user.created_at,
        };
        
        AuthResponse {
            access_token: auth_resp.token,
            token_type: "Bearer".to_string(),
            user,
        }
    }
    
    pub async fn login(email: String, password: String) -> ApiResult<AuthResponse> {
        let request = LoginRequest { email, password };
        
        // 调用独立的 auth 服务
        let auth_response: RainbowAuthResponse = API_CLIENT.post("/auth/login", &request).await?;
        
        // 转换响应格式
        let response = Self::convert_auth_response(auth_response);
        
        // 保存token到localStorage
        ApiClient::set_token(&response.access_token);
        
        Ok(response)
    }
    
    pub async fn register(
        username: String,
        email: String,
        password: String,
        full_name: Option<String>,
    ) -> ApiResult<AuthResponse> {
        // Rainbow-Auth 只需要 email 和 password
        #[derive(serde::Serialize)]
        struct RainbowAuthRegisterRequest {
            email: String,
            password: String,
        }
        
        let request = RainbowAuthRegisterRequest {
            email: email.clone(),
            password,
        };
        
        // 调用独立的 auth 服务
        let auth_response: RainbowAuthResponse = API_CLIENT.post("/auth/register", &request).await?;
        
        // 转换响应格式
        let mut response = Self::convert_auth_response(auth_response);
        
        // 使用传入的 username 替换默认生成的
        response.user.username = username;
        response.user.full_name = full_name;
        
        // 保存token到localStorage
        ApiClient::set_token(&response.access_token);
        
        // 在 Rainbow-Blog 服务中创建用户的扩展信息
        let profile_request = CreateUserProfileRequest {
            auth_user_id: response.user.id.clone(),
            username: response.user.username.clone(),
            email: response.user.email.clone(),
            full_name: response.user.full_name.clone(),
        };
        
        // 尝试创建用户扩展信息（如果失败，不影响注册流程）
        if let Err(e) = UserService::create_user_profile(&profile_request).await {
            // 仅记录错误，不影响注册成功
            #[cfg(target_arch = "wasm32")]
            web_sys::console::error_1(&format!("Failed to create user profile: {}", e).into());
        }
        
        Ok(response)
    }
    
    pub async fn logout() -> ApiResult<()> {
        // 调用独立的 auth 服务登出接口
        let _ = API_CLIENT.post::<(), ()>("/auth/logout", &()).await;
        
        // 清除本地token
        ApiClient::clear_token();
        
        Ok(())
    }
    
    pub async fn get_current_user() -> ApiResult<User> {
        // 调用独立的 auth 服务获取当前用户信息
        let auth_user: RainbowAuthUser = API_CLIENT.get("/auth/me").await?;
        
        // 尝试从 Rainbow-Blog 服务获取用户的扩展信息
        match UserService::get_current_user_profile().await {
            Ok(profile_response) => {
                // 如果成功获取到扩展信息，使用完整的用户数据
                let profile = profile_response.profile;
                Ok(User {
                    id: auth_user.id,
                    username: profile.username,
                    email: auth_user.email,
                    full_name: profile.display_name,
                    bio: profile.bio,
                    avatar_url: profile.avatar_url,
                    followers_count: profile.follower_count,
                    following_count: profile.following_count,
                    created_at: profile.created_at,
                    updated_at: profile.created_at, // UserProfile 没有 updated_at，使用 created_at
                })
            }
            Err(_) => {
                // 如果获取扩展信息失败，返回基本信息
                Ok(User {
                    id: auth_user.id.clone(),
                    username: auth_user.email.split('@').next().unwrap_or("user").to_string(),
                    email: auth_user.email,
                    full_name: None,
                    bio: None,
                    avatar_url: None,
                    followers_count: 0,
                    following_count: 0,
                    created_at: auth_user.created_at,
                    updated_at: auth_user.created_at,
                })
            }
        }
    }
    
    pub async fn refresh_token() -> ApiResult<AuthResponse> {
        // 调用独立的 auth 服务刷新 token
        let auth_response: RainbowAuthResponse = API_CLIENT.post("/auth/refresh", &()).await?;
        
        // 转换响应格式
        let response = Self::convert_auth_response(auth_response);
        
        // 更新token
        ApiClient::set_token(&response.access_token);
        
        Ok(response)
    }
}
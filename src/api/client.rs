use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// 在 WASM 环境中，需要使用完整的 URL
#[cfg(target_arch = "wasm32")]
pub const API_BASE_URL: &str = "http://127.0.0.1:8000/api";

#[cfg(not(target_arch = "wasm32"))]
pub const API_BASE_URL: &str = "/api";
const TOKEN_KEY: &str = "auth_token";

// API 响应包装器
#[derive(Debug, Deserialize)]
struct ApiResponseWrapper<T> {
    success: bool,
    data: T,
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiClient {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let client = Client::builder()
            .build()
            .unwrap_or_else(|_| Client::new());
        
        #[cfg(not(target_arch = "wasm32"))]
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client }
    }
    
    pub fn get_token() -> Option<String> {
        LocalStorage::get::<String>(TOKEN_KEY).ok()
    }
    
    pub fn set_token(token: &str) {
        LocalStorage::set(TOKEN_KEY, token).ok();
    }
    
    pub fn clear_token() {
        LocalStorage::delete(TOKEN_KEY);
    }
    
    fn add_auth_header(&self, request: RequestBuilder) -> RequestBuilder {
        if let Some(token) = Self::get_token() {
            request.header("Authorization", format!("Bearer {}", token))
        } else {
            request
        }
    }
    
    async fn handle_response<T: for<'de> Deserialize<'de>>(response: Response) -> ApiResult<T> {
        let status = response.status();
        
        if status.is_success() {
            // First, get the response text for debugging
            let text = response.text().await.map_err(|e| ApiError {
                message: format!("Failed to read response: {}", e),
                status: status.as_u16(),
            })?;
            
            // Log response for debugging in development
            #[cfg(debug_assertions)]
            web_sys::console::log_1(&format!("API Response: {}", text).into());
            
            // Try to parse as wrapped response first
            match serde_json::from_str::<ApiResponseWrapper<T>>(&text) {
                Ok(wrapped) => {
                    if wrapped.success {
                        Ok(wrapped.data)
                    } else {
                        Err(ApiError {
                            message: "API returned success=false".to_string(),
                            status: status.as_u16(),
                        })
                    }
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    web_sys::console::error_1(&format!("Failed to parse wrapped response: {}", e).into());
                    
                    // If wrapped parsing fails, try direct parsing (for backward compatibility)
                    serde_json::from_str::<T>(&text).map_err(|e| ApiError {
                        message: format!("Failed to parse response: {}", e),
                        status: status.as_u16(),
                    })
                }
            }
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ApiError {
                message: error_msg,
                status: status.as_u16(),
            })
        }
    }
    
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> ApiResult<T> {
        let url = format!("{}{}", API_BASE_URL, path);
        let request = self.client.get(&url);
        let request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| ApiError {
            message: format!("Request failed: {}", e),
            status: 0,
        })?;
        
        Self::handle_response(response).await
    }
    
    pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> ApiResult<R> {
        let url = format!("{}{}", API_BASE_URL, path);
        let request = self.client.post(&url).json(body);
        let request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| ApiError {
            message: format!("Request failed: {}", e),
            status: 0,
        })?;
        
        Self::handle_response(response).await
    }
    
    pub async fn put<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> ApiResult<R> {
        let url = format!("{}{}", API_BASE_URL, path);
        let request = self.client.put(&url).json(body);
        let request = self.add_auth_header(request);
        let response = request.send().await.map_err(|e| ApiError {
            message: format!("Request failed: {}", e),
            status: 0,
        })?;
        
        Self::handle_response(response).await
    }
    
    pub async fn delete(&self, path: &str) -> ApiResult<()> {
        let url = format!("{}{}", API_BASE_URL, path);
        let request = self.client.delete(&url);
        let request = self.add_auth_header(request);
        let response = request.send().await.map_err(|e| ApiError {
            message: format!("Request failed: {}", e),
            status: 0,
        })?;
        
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ApiError {
                message: error_msg,
                status: status.as_u16(),
            })
        }
    }
    
    pub async fn patch<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> ApiResult<R> {
        let url = format!("{}{}", API_BASE_URL, path);
        let request = self.client.patch(&url).json(body);
        let request = self.add_auth_header(request);
        let response = request.send().await.map_err(|e| ApiError {
            message: format!("Request failed: {}", e),
            status: 0,
        })?;
        
        Self::handle_response(response).await
    }
}
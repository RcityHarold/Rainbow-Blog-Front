use super::client::{ApiClient, ApiResult, ApiError};
use once_cell::sync::Lazy;
use wasm_bindgen::JsCast;
use web_sys::{File, FormData};

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

#[derive(Debug, serde::Deserialize)]
pub struct UploadResponse {
    pub url: String,
    pub filename: String,
    pub size: i64,
    pub content_type: String,
}

pub struct UploadService;

impl UploadService {
    pub async fn upload_image(file: File) -> ApiResult<UploadResponse> {
        // 创建 FormData
        let form_data = FormData::new().map_err(|_| ApiError {
            message: "无法创建表单数据".to_string(),
            status: 0,
        })?;
        
        form_data.append_with_blob("file", &file).map_err(|_| ApiError {
            message: "无法添加文件到表单".to_string(),
            status: 0,
        })?;
        
        // 使用 API 基础 URL 进行文件上传
        let url = format!("{}/blog/media/upload", crate::api::client::API_BASE_URL);
        
        // 使用 fetch API 进行上传
        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&wasm_bindgen::JsValue::from(form_data));
        
        // 设置认证头
        let headers = web_sys::Headers::new().unwrap();
        if let Some(token) = crate::api::client::ApiClient::get_token() {
            headers.set("Authorization", &format!("Bearer {}", token)).ok();
        }
        opts.set_headers(&headers);
        
        let request = web_sys::Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| ApiError {
                message: "无法创建上传请求".to_string(),
                status: 0,
            })?;
        
        let window = web_sys::window().unwrap();
        let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| ApiError {
                message: "上传失败".to_string(),
                status: 0,
            })?;
        
        let response: web_sys::Response = response.dyn_into().unwrap();
        
        if !response.ok() {
            return Err(ApiError {
                message: "上传失败".to_string(),
                status: response.status(),
            });
        }
        
        let json = wasm_bindgen_futures::JsFuture::from(response.json().unwrap())
            .await
            .map_err(|_| ApiError {
                message: "无法解析响应".to_string(),
                status: 0,
            })?;
        
        let upload_response: UploadResponse = serde_wasm_bindgen::from_value(json)
            .map_err(|_| ApiError {
                message: "无法解析上传结果".to_string(),
                status: 0,
            })?;
        
        Ok(upload_response)
    }
    
    pub async fn upload_avatar(file: File) -> ApiResult<UploadResponse> {
        // 头像上传使用相同的逻辑，但可能有不同的端点
        Self::upload_image(file).await
    }
}
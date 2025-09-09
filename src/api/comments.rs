use serde::{Deserialize, Serialize};
use super::client::{ApiClient, ApiResult};
use crate::models::comment::{Comment, CommentWithAuthor};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

#[derive(Debug, Clone, Serialize)]
pub struct CreateCommentRequest {
    pub article_id: String,
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentWithAuthor>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub has_next: bool,
}

pub struct CommentService;

impl CommentService {
    pub async fn get_article_comments(
        article_id: &str,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<&str>,
    ) -> ApiResult<Vec<CommentWithAuthor>> {
        let url = format!("/blog/comments/article/{}", article_id);
        
        // 后端返回的是嵌套的评论树结构，不需要分页参数
        API_CLIENT.get(&url).await
    }
    
    pub async fn create_comment(request: &CreateCommentRequest) -> ApiResult<Comment> {
        API_CLIENT.post("/blog/comments", request).await
    }
    
    pub async fn update_comment(id: &str, request: &UpdateCommentRequest) -> ApiResult<Comment> {
        API_CLIENT.put(&format!("/blog/comments/{}", id), request).await
    }
    
    pub async fn delete_comment(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/comments/{}", id)).await
    }
    
    pub async fn like_comment(id: &str) -> ApiResult<()> {
        API_CLIENT.post(&format!("/blog/comments/{}/clap", id), &()).await
    }
    
    pub async fn unlike_comment(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/comments/{}/clap", id)).await
    }
    
    pub async fn report_comment(id: &str, reason: &str) -> ApiResult<()> {
        #[derive(Serialize)]
        struct ReportRequest {
            reason: String,
        }
        
        API_CLIENT.post(
            &format!("/blog/comments/{}/report", id),
            &ReportRequest {
                reason: reason.to_string(),
            },
        ).await
    }
}
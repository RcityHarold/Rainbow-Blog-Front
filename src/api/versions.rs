use super::client::{ApiClient, ApiResult};
use crate::models::version::{ArticleVersion, ArticleVersionComparison, CreateVersionRequest, RestoreVersionRequest};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct VersionService;

impl VersionService {
    pub async fn get_article_versions(article_id: &str) -> ApiResult<Vec<ArticleVersion>> {
        API_CLIENT.get(&format!("/blog/articles/{}/versions", article_id)).await
    }
    
    pub async fn get_version(article_id: &str, version_id: &str) -> ApiResult<ArticleVersion> {
        API_CLIENT.get(&format!("/blog/articles/{}/versions/{}", article_id, version_id)).await
    }
    
    pub async fn create_version(request: &CreateVersionRequest) -> ApiResult<ArticleVersion> {
        API_CLIENT.post(&format!("/blog/articles/{}/versions", request.article_id), request).await
    }
    
    pub async fn compare_versions(
        article_id: &str,
        version_a_id: &str,
        version_b_id: &str
    ) -> ApiResult<ArticleVersionComparison> {
        API_CLIENT.get(&format!(
            "/blog/articles/{}/versions/compare?a={}&b={}", 
            article_id, version_a_id, version_b_id
        )).await
    }
    
    pub async fn restore_version(request: &RestoreVersionRequest) -> ApiResult<()> {
        API_CLIENT.post(
            &format!("/blog/articles/{}/versions/{}/restore", request.article_id, request.version_id),
            &()
        ).await
    }
}
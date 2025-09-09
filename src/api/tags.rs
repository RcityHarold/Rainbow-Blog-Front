use serde::{Deserialize, Serialize};
use super::client::{ApiClient, ApiResult};
use crate::models::tag::Tag;
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct TagService;

impl TagService {
    pub async fn get_all_tags() -> ApiResult<Vec<Tag>> {
        API_CLIENT.get("/blog/tags").await
    }
    
    pub async fn get_tag(slug: &str) -> ApiResult<Tag> {
        API_CLIENT.get(&format!("/blog/tags/{}", slug)).await
    }
    
    pub async fn get_popular_tags(limit: Option<i32>) -> ApiResult<Vec<Tag>> {
        let query = if let Some(limit) = limit {
            format!("/blog/tags?sort_by=popular&limit={}", limit)
        } else {
            "/blog/tags?sort_by=popular".to_string()
        };
        API_CLIENT.get(&query).await
    }
    
    pub async fn follow_tag(tag_id: &str) -> ApiResult<()> {
        API_CLIENT.post(&format!("/blog/tags/{}/follow", tag_id), &()).await
    }
    
    pub async fn unfollow_tag(tag_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/tags/{}/follow", tag_id)).await
    }
    
    pub async fn is_following_tag(tag_id: &str) -> ApiResult<bool> {
        #[derive(Deserialize)]
        struct Response {
            is_following: bool,
        }
        
        let response: Response = API_CLIENT.get(&format!("/blog/tags/{}/is-following", tag_id)).await?;
        Ok(response.is_following)
    }
}
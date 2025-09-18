use super::client::{ApiClient, ApiResult};
use crate::models::bookmark::{BookmarkListResponse, BookmarkItem};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct BookmarkService;

impl BookmarkService {
    pub async fn list(page: Option<i32>, limit: Option<i32>) -> ApiResult<Vec<BookmarkItem>> {
        let mut url = String::from("/blog/bookmarks");
        let mut params = vec![];
        if let Some(p) = page { params.push(format!("page={}", p)); }
        if let Some(l) = limit { params.push(format!("limit={}", l)); }
        if !params.is_empty() { url = format!("{}?{}", url, params.join("&")); }

        let resp: BookmarkListResponse = API_CLIENT.get(&url).await?;
        Ok(resp.data)
    }

    pub async fn update_note(id: &str, note: Option<String>) -> ApiResult<()> {
        #[derive(serde::Serialize)]
        struct UpdateReq { note: Option<String> }
        API_CLIENT
            .put(&format!("/blog/bookmarks/{}", id), &UpdateReq { note })
            .await
    }

    pub async fn delete(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/bookmarks/{}", id)).await
    }
}

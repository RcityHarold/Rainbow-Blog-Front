use super::client::{ApiClient, ApiResult};
use crate::models::article::{Article, ArticleListResponse, CreateArticleRequest, UpdateArticleRequest};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct ArticleService;

impl ArticleService {
    pub async fn get_articles(
        page: Option<i32>,
        limit: Option<i32>,
        sort: Option<&str>,
    ) -> ApiResult<ArticleListResponse> {
        let mut url = "/blog/articles".to_string();
        let mut query_params = vec![];
        
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        if let Some(s) = sort {
            query_params.push(format!("sort={}", s));
        }
        
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }
        
        API_CLIENT.get(&url).await
    }
    
    pub async fn get_trending_articles(
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let url = if let Some(l) = limit {
            format!("/blog/articles/trending?limit={}", l)
        } else {
            "/blog/articles/trending".to_string()
        };
        
        API_CLIENT.get(&url).await
    }
    
    pub async fn get_popular_articles(
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let url = if let Some(l) = limit {
            format!("/blog/articles/popular?limit={}", l)
        } else {
            "/blog/articles/popular".to_string()
        };
        
        API_CLIENT.get(&url).await
    }
    
    pub async fn get_article(slug: &str) -> ApiResult<Article> {
        API_CLIENT.get(&format!("/blog/articles/{}", slug)).await
    }
    
    pub async fn create_article(article: &CreateArticleRequest) -> ApiResult<Article> {
        API_CLIENT.post("/blog/articles/create", article).await
    }
    
    pub async fn update_article(id: &str, article: &UpdateArticleRequest) -> ApiResult<Article> {
        API_CLIENT.put(&format!("/blog/articles/{}", id), article).await
    }
    
    pub async fn publish_article(id: &str) -> ApiResult<Article> {
        API_CLIENT.post(&format!("/blog/articles/{}/publish", id), &()).await
    }
    
    pub async fn unpublish_article(id: &str) -> ApiResult<Article> {
        API_CLIENT.post(&format!("/blog/articles/{}/unpublish", id), &()).await
    }
    
    pub async fn delete_article(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/articles/{}", id)).await
    }
    
    pub async fn increment_view_count(id: &str) -> ApiResult<()> {
        API_CLIENT.post(&format!("/blog/articles/{}/view", id), &()).await
    }
    
    pub async fn clap_article(id: &str, count: i32) -> ApiResult<ClapResponse> {
        #[derive(serde::Serialize)]
        struct ClapRequest {
            article_id: String,
            count: i32,
        }
        
        API_CLIENT.post(
            &format!("/blog/articles/{}/clap", id),
            &ClapRequest {
                article_id: id.to_string(),
                count,
            },
        ).await
    }
    
    pub async fn bookmark_article(id: &str, note: Option<String>) -> ApiResult<()> {
        #[derive(serde::Serialize)]
        struct BookmarkRequest {
            article_id: String,
            note: Option<String>,
        }
        
        API_CLIENT.post(
            "/blog/bookmarks",
            &BookmarkRequest {
                article_id: id.to_string(),
                note,
            },
        ).await
    }
    
    pub async fn unbookmark_article(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/bookmarks/article/{}", id)).await
    }
    
    pub async fn get_articles_by_tag(tag_slug: &str, sort_by: &str, page: i32, per_page: Option<i32>) -> ApiResult<ArticleListResponse> {
        let per_page = per_page.unwrap_or(20);
        let url = format!("/blog/articles?tag={}&sort_by={}&page={}&per_page={}", tag_slug, sort_by, page, per_page);
        API_CLIENT.get(&url).await
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ClapResponse {
    pub user_clap_count: i32,
    pub total_claps: i32,
}
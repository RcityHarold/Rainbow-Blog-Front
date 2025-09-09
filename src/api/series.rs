use super::client::{ApiClient, ApiResult};
use crate::models::{
    series::{Series, CreateSeriesRequest, UpdateSeriesRequest, SeriesWithArticles},
    article::ArticleListResponse,
};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct SeriesService;

impl SeriesService {
    pub async fn get_my_series() -> ApiResult<Vec<Series>> {
        API_CLIENT.get("/blog/series/my").await
    }
    
    pub async fn get_series(series_id: &str) -> ApiResult<SeriesWithArticles> {
        API_CLIENT.get(&format!("/blog/series/{}", series_id)).await
    }
    
    pub async fn get_series_by_slug(slug: &str) -> ApiResult<SeriesWithArticles> {
        API_CLIENT.get(&format!("/blog/series/slug/{}", slug)).await
    }
    
    pub async fn create_series(request: &CreateSeriesRequest) -> ApiResult<Series> {
        API_CLIENT.post("/blog/series", request).await
    }
    
    pub async fn update_series(series_id: &str, request: &UpdateSeriesRequest) -> ApiResult<Series> {
        API_CLIENT.put(&format!("/blog/series/{}", series_id), request).await
    }
    
    pub async fn delete_series(series_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/series/{}", series_id)).await
    }
    
    pub async fn add_article_to_series(series_id: &str, article_id: &str, order: i32) -> ApiResult<()> {
        #[derive(serde::Serialize)]
        struct AddArticleRequest {
            article_id: String,
            order: i32,
        }
        
        API_CLIENT.post(
            &format!("/blog/series/{}/articles", series_id),
            &AddArticleRequest {
                article_id: article_id.to_string(),
                order,
            },
        ).await
    }
    
    pub async fn remove_article_from_series(series_id: &str, article_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/series/{}/articles/{}", series_id, article_id)).await
    }
    
    pub async fn reorder_series_articles(series_id: &str, article_orders: Vec<(String, i32)>) -> ApiResult<()> {
        #[derive(serde::Serialize)]
        struct ReorderRequest {
            articles: Vec<ArticleOrder>,
        }
        
        #[derive(serde::Serialize)]
        struct ArticleOrder {
            article_id: String,
            order: i32,
        }
        
        let articles = article_orders
            .into_iter()
            .map(|(article_id, order)| ArticleOrder { article_id, order })
            .collect();
        
        API_CLIENT.post(
            &format!("/blog/series/{}/reorder", series_id),
            &ReorderRequest { articles },
        ).await
    }
}
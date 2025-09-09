use super::client::{ApiClient, ApiResult};
use crate::models::article::Article;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

#[derive(Debug, Serialize)]
pub struct RecommendationParams {
    pub user_id: Option<String>,
    pub limit: Option<i32>,
    pub algorithm: Option<String>,
    pub exclude_read: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RecommendationResponse {
    pub articles: Vec<RecommendedArticle>,
    pub total: i64,
    pub algorithm_used: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecommendedArticle {
    pub article: Article,
    pub score: f64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct TrendingResponse {
    pub articles: Vec<Article>,
    pub period: String,
    pub total: i64,
}

pub struct RecommendationService;

impl RecommendationService {
    // 获取个性化推荐
    pub async fn get_recommendations(params: &RecommendationParams) -> ApiResult<RecommendationResponse> {
        let mut query_params = vec![];
        
        if let Some(user_id) = &params.user_id {
            query_params.push(format!("user_id={}", user_id));
        }
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(algorithm) = &params.algorithm {
            query_params.push(format!("algorithm={}", algorithm));
        }
        if let Some(exclude_read) = params.exclude_read {
            query_params.push(format!("exclude_read={}", exclude_read));
        }
        if let Some(tags) = &params.tags {
            if !tags.is_empty() {
                query_params.push(format!("tags={}", tags.join(",")));
            }
        }
        if let Some(authors) = &params.authors {
            if !authors.is_empty() {
                query_params.push(format!("authors={}", authors.join(",")));
            }
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        API_CLIENT.get(&format!("/blog/recommendations{}", query_string)).await
    }
    
    // 获取热门文章
    pub async fn get_trending(period: &str, limit: Option<i32>) -> ApiResult<TrendingResponse> {
        let query = if let Some(limit) = limit {
            format!("?period={}&limit={}", period, limit)
        } else {
            format!("?period={}", period)
        };
        
        API_CLIENT.get(&format!("/blog/recommendations/trending{}", query)).await
    }
    
    // 获取基于内容的推荐
    pub async fn get_content_based(article_id: &str, limit: Option<i32>) -> ApiResult<Vec<Article>> {
        let query = if let Some(limit) = limit {
            format!("?limit={}", limit)
        } else {
            String::new()
        };
        
        API_CLIENT.get(&format!("/blog/recommendations/content-based/{}{}", article_id, query)).await
    }
    
    // 获取关注用户的文章
    pub async fn get_following_feed(limit: Option<i32>, page: Option<i32>) -> ApiResult<Vec<Article>> {
        let mut query_params = vec![];
        
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        API_CLIENT.get(&format!("/blog/recommendations/following{}", query)).await
    }
}
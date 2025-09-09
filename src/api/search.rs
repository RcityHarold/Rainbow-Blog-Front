use serde::{Deserialize, Serialize};
use super::client::{ApiClient, ApiResult};
use crate::models::{article::Article, user::User, tag::Tag, series::Series};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArticlesRequest {
    pub query: String,
    pub tags: Option<Vec<String>>,
    pub author_id: Option<String>,
    pub publication_id: Option<String>,
    pub sort_by: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchArticlesResponse {
    pub articles: Vec<Article>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAllRequest {
    pub query: String,
    pub types: Option<Vec<String>>,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchAllResponse {
    pub articles: Vec<Article>,
    pub users: Vec<User>,
    pub tags: Vec<Tag>,
    pub series: Vec<Series>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionsRequest {
    pub query: String,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub r#type: String,
    pub relevance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchSuggestionsResponse {
    pub suggestions: Vec<SearchSuggestion>,
}

pub struct SearchService;

impl SearchService {
    pub async fn search_articles(request: SearchArticlesRequest) -> ApiResult<SearchArticlesResponse> {
        API_CLIENT.post("/blog/search/articles", &request).await
    }
    
    pub async fn search_all(request: SearchAllRequest) -> ApiResult<SearchAllResponse> {
        API_CLIENT.post("/blog/search/all", &request).await
    }
    
    pub async fn get_suggestions(request: SearchSuggestionsRequest) -> ApiResult<SearchSuggestionsResponse> {
        API_CLIENT.post("/blog/search/suggestions", &request).await
    }
    
    pub async fn get_trending_searches(limit: Option<i32>) -> ApiResult<Vec<String>> {
        let query = if let Some(limit) = limit {
            format!("/blog/search/trending?limit={}", limit)
        } else {
            "/blog/search/trending".to_string()
        };
        API_CLIENT.get(&query).await
    }
}
use super::client::{ApiClient, ApiResult};
use crate::models::article::Article;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};

// 自定义反序列化函数来处理article ID格式
fn deserialize_article_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    
    match value {
        serde_json::Value::String(s) => {
            // 如果是字符串，检查是否是article:{"String":"uuid"}格式
            if s.starts_with("article:{") && s.ends_with("}") {
                // 尝试解析Thing ID格式
                if let Some(start) = s.find(r#""String":""#) {
                    let start = start + r#""String":""#.len();
                    if let Some(end) = s[start..].find('"') {
                        return Ok(s[start..start + end].to_string());
                    }
                }
            }
            Ok(s)
        },
        _ => Err(serde::de::Error::custom("Expected string for article ID")),
    }
}

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
    pub articles: Vec<TrendingArticle>,
    pub total: i64,
    pub algorithm_used: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrendingArticle {
    #[serde(deserialize_with = "deserialize_article_id")]
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub author: AuthorInfo,
    pub publication: Option<PublicationInfo>,
    pub status: String,
    pub is_paid_content: bool,
    pub is_featured: bool,
    pub reading_time: i32,
    pub view_count: i32,
    pub clap_count: i32,
    pub comment_count: i32,
    pub tags: Vec<TagInfo>,
    pub created_at: String,
    pub published_at: Option<String>,
    pub score: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorInfo {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicationInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
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
        
        // 使用自定义结构来匹配实际的API响应
        #[derive(Debug, Deserialize)]
        struct RawTrendingResponse {
            articles: Vec<TrendingArticle>,
            total: i64,
            algorithm_used: String,
            generated_at: String,
        }
        
        let response: RawTrendingResponse = API_CLIENT.get(&format!("/blog/recommendations/trending{}", query)).await?;
        
        Ok(TrendingResponse {
            articles: response.articles,
            total: response.total,
            algorithm_used: response.algorithm_used,
            generated_at: response.generated_at,
        })
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
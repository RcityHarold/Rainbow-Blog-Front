use super::client::{ApiClient, ApiResult};
use crate::models::article::{Article, ArticleListResponse, CreateArticleRequest, UpdateArticleRequest, Author};
use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

// 后端返回的简化文章数据结构
#[derive(Debug, serde::Deserialize)]
struct RawArticleResponse {
    id: String,
    title: String,
    subtitle: Option<String>,
    slug: String,
    content: String,
    content_html: String,
    excerpt: Option<String>,
    cover_image_url: Option<String>,
    author_id: String,
    status: String,
    is_paid_content: bool,
    is_featured: bool,
    reading_time: i32,
    word_count: i32,
    view_count: i32,
    clap_count: i32,
    comment_count: i32,
    bookmark_count: i32,
    share_count: i32,
    seo_keywords: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published_at: Option<DateTime<Utc>>,
}

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
        let raw_article: RawArticleResponse = API_CLIENT.post("/blog/articles/create", article).await?;
        
        // 转换为前端的 Article 结构
        // 处理 ID 格式，提取实际的 UUID
        let article_id = if raw_article.id.contains(":{\"") {
            // 处理 "article:{\"String\":\"uuid\"}" 格式
            if let Some(start) = raw_article.id.find("\"String\":\"") {
                if let Some(end) = raw_article.id.rfind("\"}") {
                    let uuid_start = start + "\"String\":\"".len();
                    format!("article:{}", &raw_article.id[uuid_start..end])
                } else {
                    raw_article.id.clone()
                }
            } else {
                raw_article.id.clone()
            }
        } else {
            raw_article.id.clone()
        };
        
        Ok(Article {
            id: article_id,
            title: raw_article.title,
            subtitle: raw_article.subtitle,
            slug: raw_article.slug,
            content: raw_article.content,
            content_html: raw_article.content_html,
            excerpt: raw_article.excerpt,
            cover_image_url: raw_article.cover_image_url,
            author: Author {
                id: raw_article.author_id.clone(),
                username: String::new(),  // 暂时为空，可以通过其他 API 获取
                display_name: String::new(),
                avatar_url: None,
                is_verified: false,
            },
            publication: None,
            series: None,
            tags: vec![],  // 标签信息需要另外获取
            status: raw_article.status,
            is_paid_content: raw_article.is_paid_content,
            is_featured: raw_article.is_featured,
            reading_time: raw_article.reading_time,
            word_count: raw_article.word_count,
            view_count: raw_article.view_count,
            clap_count: raw_article.clap_count,
            comment_count: raw_article.comment_count,
            bookmark_count: raw_article.bookmark_count,
            share_count: raw_article.share_count,
            seo_title: None,  // 后端返回的数据中没有这些字段
            seo_description: None,
            seo_keywords: raw_article.seo_keywords,
            created_at: raw_article.created_at,
            updated_at: raw_article.updated_at,
            published_at: raw_article.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        })
    }
    
    pub async fn update_article(id: &str, article: &UpdateArticleRequest) -> ApiResult<Article> {
        // 如果 ID 包含 "article:" 前缀，只取 UUID 部分
        let article_id = if id.starts_with("article:") {
            &id[8..] // 跳过 "article:" 前缀
        } else {
            id
        };
        API_CLIENT.put(&format!("/blog/articles/by-id/{}", article_id), article).await
    }
    
    pub async fn publish_article(id: &str) -> ApiResult<Article> {
        // 如果 ID 包含 "article:" 前缀，只取 UUID 部分
        let article_id = if id.starts_with("article:") {
            &id[8..] // 跳过 "article:" 前缀
        } else {
            id
        };
        
        // 后端返回的是简化的文章数据，需要转换
        let raw_article: RawArticleResponse = API_CLIENT.post(&format!("/blog/articles/by-id/{}/publish", article_id), &()).await?;
        
        // 处理 ID 格式，提取实际的 UUID
        let processed_id = if raw_article.id.contains(":{\"") {
            // 处理 "article:{\"String\":\"uuid\"}" 格式
            if let Some(start) = raw_article.id.find("\"String\":\"") {
                if let Some(end) = raw_article.id.rfind("\"}") {
                    let uuid_start = start + "\"String\":\"".len();
                    format!("article:{}", &raw_article.id[uuid_start..end])
                } else {
                    raw_article.id.clone()
                }
            } else {
                raw_article.id.clone()
            }
        } else {
            raw_article.id.clone()
        };
        
        // 转换为前端的 Article 结构
        Ok(Article {
            id: processed_id,
            title: raw_article.title,
            subtitle: raw_article.subtitle,
            slug: raw_article.slug,
            content: raw_article.content,
            content_html: raw_article.content_html,
            excerpt: raw_article.excerpt,
            cover_image_url: raw_article.cover_image_url,
            author: Author {
                id: raw_article.author_id.clone(),
                username: String::new(),
                display_name: String::new(),
                avatar_url: None,
                is_verified: false,
            },
            publication: None,
            series: None,
            tags: vec![],
            status: raw_article.status,
            is_paid_content: raw_article.is_paid_content,
            is_featured: raw_article.is_featured,
            reading_time: raw_article.reading_time,
            word_count: raw_article.word_count,
            view_count: raw_article.view_count,
            clap_count: raw_article.clap_count,
            comment_count: raw_article.comment_count,
            bookmark_count: raw_article.bookmark_count,
            share_count: raw_article.share_count,
            seo_title: None,
            seo_description: None,
            seo_keywords: raw_article.seo_keywords,
            created_at: raw_article.created_at,
            updated_at: raw_article.updated_at,
            published_at: raw_article.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        })
    }
    
    pub async fn unpublish_article(id: &str) -> ApiResult<Article> {
        // 如果 ID 包含 "article:" 前缀，只取 UUID 部分
        let article_id = if id.starts_with("article:") {
            &id[8..] // 跳过 "article:" 前缀
        } else {
            id
        };
        API_CLIENT.post(&format!("/blog/articles/by-id/{}/unpublish", article_id), &()).await
    }
    
    pub async fn delete_article(id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/articles/{}", id)).await
    }
    
    pub async fn increment_view_count(id: &str) -> ApiResult<()> {
        // 如果 ID 包含 "article:" 前缀，只取 UUID 部分
        let article_id = if id.starts_with("article:") {
            &id[8..] // 跳过 "article:" 前缀
        } else {
            id
        };
        API_CLIENT.post(&format!("/blog/articles/by-id/{}/view", article_id), &()).await
    }
    
    pub async fn clap_article(id: &str, count: i32) -> ApiResult<ClapResponse> {
        #[derive(serde::Serialize)]
        struct ClapRequest {
            article_id: String,
            count: i32,
        }
        
        // 如果 ID 包含 "article:" 前缀，只取 UUID 部分
        let article_id = if id.starts_with("article:") {
            &id[8..] // 跳过 "article:" 前缀
        } else {
            id
        };
        
        API_CLIENT.post(
            &format!("/blog/articles/by-id/{}/clap", article_id),
            &ClapRequest {
                article_id: article_id.to_string(),
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
    pub total_claps: i64,
}
use super::client::{ApiClient, ApiResult};
use crate::models::article::{Article, ArticleListResponse, CreateArticleRequest, UpdateArticleRequest, Author};
use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};
use serde::Deserialize;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

// 后端返回的简化文章数据结构
#[derive(Debug, serde::Deserialize)]
struct RawArticleResponse {
    #[serde(deserialize_with = "deserialize_article_id")]
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

// 列表接口返回的文章数据 (没有content等详细信息)
#[derive(Debug, serde::Deserialize)]
pub struct RawArticleListItem {
    #[serde(deserialize_with = "deserialize_article_id")]
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub author: RawAuthorInfo,
    pub publication: Option<RawPublicationInfo>,
    pub status: String,
    pub is_paid_content: bool,
    pub is_featured: bool,
    pub reading_time: i32,
    pub view_count: i32,
    pub clap_count: i32,
    pub comment_count: i32,
    pub tags: Vec<RawTagInfo>,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawAuthorInfo {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawPublicationInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTagInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
}

// 自定义反序列化函数来处理article ID格式
fn deserialize_article_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
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
                        return Ok(format!("article:{}", &s[start..start + end]));
                    }
                }
            }
            Ok(s)
        },
        _ => Err(serde::de::Error::custom("Expected string for article ID")),
    }
}

// 自定义的文章列表响应，用于处理后端返回的数据
#[derive(Debug, serde::Deserialize)]
pub struct RawArticleListResponse {
    pub articles: Vec<RawArticleListItem>,
    pub pagination: crate::models::article::Pagination,
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
        
        // 获取原始响应并转换
        let raw_response: RawArticleListResponse = API_CLIENT.get(&url).await?;
        
        // 将原始文章转换为前端期望的格式
        let articles = raw_response.articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(), // 列表接口不返回内容
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag {
                id: t.id,
                name: t.name,
                slug: t.slug,
            }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0, // 列表接口不返回字数
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0, // 列表接口不返回收藏数
            share_count: 0, // 列表接口不返回分享数
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at, // 列表接口不返回更新时间
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        
        Ok(ArticleListResponse {
            articles,
            pagination: raw_response.pagination,
        })
    }
    
    pub async fn get_trending_articles(
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let url = if let Some(l) = limit {
            format!("/blog/articles/trending?limit={}", l)
        } else {
            "/blog/articles/trending".to_string()
        };
        
        // 获取原始响应并转换
        let raw_articles: Vec<RawArticleListItem> = API_CLIENT.get(&url).await?;
        
        // 将原始文章转换为前端期望的格式
        let articles: Vec<Article> = raw_articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(),
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag {
                id: t.id,
                name: t.name,
                slug: t.slug,
            }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0,
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0,
            share_count: 0,
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at,
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        
        let article_count = articles.len() as i32;
        
        Ok(ArticleListResponse {
            articles,
            pagination: crate::models::article::Pagination {
                current_page: 1,
                total_pages: 1,
                total_items: article_count,
                items_per_page: article_count,
                has_next: false,
                has_prev: false,
            },
        })
    }
    
    pub async fn get_popular_articles(
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let url = if let Some(l) = limit {
            format!("/blog/articles/popular?limit={}", l)
        } else {
            "/blog/articles/popular".to_string()
        };
        
        // 获取原始响应并转换
        let raw_articles: Vec<RawArticleListItem> = API_CLIENT.get(&url).await?;
        
        // 将原始文章转换为前端期望的格式
        let articles: Vec<Article> = raw_articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(),
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag {
                id: t.id,
                name: t.name,
                slug: t.slug,
            }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0,
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0,
            share_count: 0,
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at,
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        
        let article_count = articles.len() as i32;
        
        Ok(ArticleListResponse {
            articles,
            pagination: crate::models::article::Pagination {
                current_page: 1,
                total_pages: 1,
                total_items: article_count,
                items_per_page: article_count,
                has_next: false,
                has_prev: false,
            },
        })
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
        let url = format!("/blog/articles?tag={}&sort={}&page={}&limit={}", tag_slug, sort_by, page, per_page);
        
        // 获取原始响应并转换
        let raw_response: RawArticleListResponse = API_CLIENT.get(&url).await?;
        
        // 将原始文章转换为前端期望的格式
        let articles = raw_response.articles.into_iter().map(|raw| Article {
            id: raw.id,
            title: raw.title,
            subtitle: raw.subtitle,
            slug: raw.slug,
            content: String::new(),
            content_html: String::new(),
            excerpt: raw.excerpt,
            cover_image_url: raw.cover_image_url,
            author: Author {
                id: raw.author.id,
                username: raw.author.username,
                display_name: raw.author.display_name,
                avatar_url: raw.author.avatar_url,
                is_verified: raw.author.is_verified,
            },
            publication: raw.publication.map(|p| crate::models::article::Publication {
                id: p.id,
                name: p.name,
                slug: p.slug,
                logo_url: p.logo_url,
            }),
            series: None,
            tags: raw.tags.into_iter().map(|t| crate::models::article::Tag {
                id: t.id,
                name: t.name,
                slug: t.slug,
            }).collect(),
            status: raw.status,
            is_paid_content: raw.is_paid_content,
            is_featured: raw.is_featured,
            reading_time: raw.reading_time,
            word_count: 0,
            view_count: raw.view_count,
            clap_count: raw.clap_count,
            comment_count: raw.comment_count,
            bookmark_count: 0,
            share_count: 0,
            seo_title: None,
            seo_description: None,
            seo_keywords: vec![],
            created_at: raw.created_at,
            updated_at: raw.created_at,
            published_at: raw.published_at,
            is_bookmarked: None,
            is_clapped: None,
            user_clap_count: None,
        }).collect();
        
        Ok(ArticleListResponse {
            articles,
            pagination: raw_response.pagination,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ClapResponse {
    pub user_clap_count: i32,
    pub total_claps: i64,
}
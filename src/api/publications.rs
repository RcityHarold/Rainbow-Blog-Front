use super::client::{ApiClient, ApiResult};
use crate::models::publication::*;
use crate::models::article::{Article, ArticleListResponse, Author, Publication as ArticlePublication, Series as ArticleSeries, Tag as ArticleTag};
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct PublicationService;

impl PublicationService {
    // 创建出版物
    pub async fn create_publication(request: &CreatePublicationRequest) -> ApiResult<Publication> {
        API_CLIENT.post("/blog/publications", request).await
    }
    
    // 获取出版物列表
    pub async fn get_publications(
        search: Option<&str>,
        category: Option<&str>,
        sort: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<PublicationListResponse> {
        let mut query_params = vec![];
        
        if let Some(search) = search {
            query_params.push(format!("search={}", urlencoding::encode(search)));
        }
        if let Some(category) = category {
            query_params.push(format!("category={}", category));
        }
        if let Some(sort) = sort {
            query_params.push(format!("sort={}", sort));
        }
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        #[derive(serde::Deserialize)]
        struct BackendPage {
            data: Vec<Publication>,
            total: i64,
            page: i32,
            per_page: i32,
            total_pages: i32,
        }

        let backend: BackendPage = API_CLIENT.get(&format!("/blog/publications{}", query)).await?;
        Ok(PublicationListResponse {
            publications: backend.data,
            total: backend.total,
            page: backend.page,
            total_pages: backend.total_pages,
        })
    }
    
    // 获取出版物详情（兼容 wrapped 与 direct 结构）
    pub async fn get_publication(slug: &str) -> ApiResult<Publication> {
        let v: serde_json::Value = API_CLIENT.get(&format!("/blog/publications/{}", slug)).await?;
        // 如果是 wrapped 模式，ApiClient 已经返回 data 字段内容，因此此处通常为 PublicationResponse
        // 兼容可能返回整个响应体的情况
        let data = if let Some(obj) = v.get("data") { obj.clone() } else { v };
        if let Some(pub_obj) = data.get("publication") {
            let pub_data: Publication = serde_json::from_value(pub_obj.clone()).map_err(|e| super::client::ApiError { message: format!("Failed to parse publication: {}", e), status: 0 })?;
            Ok(pub_data)
        } else {
            // 有些实现可能直接返回 Publication
            let pub_data: Publication = serde_json::from_value(data).map_err(|e| super::client::ApiError { message: format!("Failed to parse publication: {}", e), status: 0 })?;
            Ok(pub_data)
        }
    }
    
    // 更新出版物
    pub async fn update_publication(slug: &str, request: &UpdatePublicationRequest) -> ApiResult<Publication> {
        API_CLIENT.put(&format!("/blog/publications/{}", slug), request).await
    }
    
    // 删除出版物
    pub async fn delete_publication(slug: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/publications/{}", slug)).await
    }
    
    // 添加成员
    pub async fn add_member(publication_id: &str, request: &AddMemberRequest) -> ApiResult<PublicationMember> {
        API_CLIENT.post(&format!("/blog/publications/{}/members", publication_id), request).await
    }
    
    // 获取成员列表
    pub async fn get_members(
        publication_id: &str,
        role: Option<&str>,
        status: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<MemberListResponse> {
        let mut query_params = vec![];
        
        if let Some(role) = role {
            query_params.push(format!("role={}", role));
        }
        if let Some(status) = status {
            query_params.push(format!("status={}", status));
        }
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        #[derive(serde::Deserialize)]
        struct BackendMembersPage {
            data: Vec<PublicationMember>,
            total: i64,
            page: i32,
            per_page: i32,
            total_pages: i32,
        }

        let backend: BackendMembersPage = API_CLIENT.get(&format!("/blog/publications/{}/members{}", publication_id, query)).await?;
        Ok(MemberListResponse {
            members: backend.data,
            total: backend.total,
            page: backend.page,
            total_pages: backend.total_pages,
        })
    }
    
    // 更新成员角色
    pub async fn update_member_role(
        publication_id: &str,
        user_id: &str,
        role: &MemberRole,
    ) -> ApiResult<PublicationMember> {
        API_CLIENT.put(
            &format!("/blog/publications/{}/members/{}", publication_id, user_id),
            &serde_json::json!({ "role": role })
        ).await
    }
    
    // 移除成员
    pub async fn remove_member(publication_id: &str, user_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/publications/{}/members/{}", publication_id, user_id)).await
    }
    
    // 关注出版物
    pub async fn follow_publication(publication_id: &str) -> ApiResult<()> {
        API_CLIENT.post(&format!("/blog/publications/{}/follow", publication_id), &()).await
    }
    
    // 取消关注出版物
    pub async fn unfollow_publication(publication_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/publications/{}/follow", publication_id)).await
    }
    
    // 获取关注的出版物
    pub async fn get_following_publications() -> ApiResult<Vec<Publication>> {
        API_CLIENT.get("/blog/publications/following").await
    }
    
    // 获取出版物文章
    pub async fn get_publication_articles(
        slug: &str,
        status: Option<&str>,
        author: Option<&str>,
        tag: Option<&str>,
        sort: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> ApiResult<ArticleListResponse> {
        let mut query_params = vec![];
        
        if let Some(status) = status {
            query_params.push(format!("status={}", status));
        }
        if let Some(author) = author {
            query_params.push(format!("author={}", author));
        }
        if let Some(tag) = tag {
            query_params.push(format!("tag={}", tag));
        }
        if let Some(sort) = sort {
            query_params.push(format!("sort={}", sort));
        }
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        // 后端返回分页对象：{ data, total, page, per_page, total_pages }
        #[derive(serde::Deserialize)]
        struct BackendPage<T> {
            data: Vec<T>,
            total: i64,
            page: i32,
            per_page: i32,
            total_pages: i32,
        }

        // 兼容不同字段形态的后端条目（ArticleListItem）
        #[derive(serde::Deserialize)]
        struct BackendItem {
            #[serde(default)] id: serde_json::Value,
            title: String,
            #[serde(default)] subtitle: Option<String>,
            slug: String,
            #[serde(default)] excerpt: Option<String>,
            #[serde(default)] cover_image_url: Option<String>,
            #[serde(default)] author_id: serde_json::Value,
            #[serde(default)] reading_time: i32,
            #[serde(default)] view_count: i32,
            #[serde(default)] clap_count: i32,
            #[serde(default)] comment_count: i32,
            #[serde(default)] bookmark_count: i32,
            #[serde(default)] tags: Option<Vec<String>>,
            created_at: chrono::DateTime<chrono::Utc>,
            #[serde(default)] published_at: Option<chrono::DateTime<chrono::Utc>>,
        }

        fn val_to_id(v: &serde_json::Value, table: &str) -> String {
            match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Object(obj) => {
                    let tb = obj.get("tb").and_then(|x| x.as_str()).unwrap_or(table);
                    let id = obj.get("id").and_then(|x| x.as_str())
                        .or_else(|| obj.get("id").and_then(|x| x.get("String")).and_then(|x| x.as_str()))
                        .unwrap_or("unknown");
                    format!("{}:{}", tb, id)
                }
                _ => v.to_string(),
            }
        }

        let backend: BackendPage<BackendItem> = API_CLIENT.get(&format!("/blog/publications/{}/articles{}", slug, query)).await?;

        // 将后端条目映射为前端 Article（填充必要默认值）
        let articles: Vec<Article> = backend.data.into_iter().map(|it| {
            let id = val_to_id(&it.id, "article");
            let author_id = val_to_id(&it.author_id, "user");
            Article {
                id,
                title: it.title,
                subtitle: it.subtitle,
                slug: it.slug,
                content: String::new(),
                content_html: String::new(),
                excerpt: it.excerpt.or_else(|| Some(String::new())),
                cover_image_url: it.cover_image_url,
                author: Author { id: author_id, username: String::new(), display_name: String::new(), avatar_url: None, is_verified: false },
                publication: None::<ArticlePublication>,
                series: None::<ArticleSeries>,
                tags: Vec::<ArticleTag>::new(),
                status: if it.published_at.is_some() { "published".into() } else { "draft".into() },
                is_paid_content: false,
                is_featured: false,
                reading_time: it.reading_time,
                word_count: 0,
                view_count: it.view_count,
                clap_count: it.clap_count,
                comment_count: it.comment_count,
                bookmark_count: it.bookmark_count,
                share_count: 0,
                seo_title: None,
                seo_description: None,
                seo_keywords: Vec::new(),
                created_at: it.created_at,
                updated_at: it.created_at,
                published_at: it.published_at,
                is_bookmarked: Some(false),
                is_clapped: Some(false),
                user_clap_count: Some(0),
            }
        }).collect();

        Ok(ArticleListResponse {
            articles,
            pagination: crate::models::article::Pagination {
                current_page: backend.page,
                total_pages: backend.total_pages,
                total_items: backend.total as i32,
                items_per_page: backend.per_page,
                has_next: backend.page < backend.total_pages,
                has_prev: backend.page > 1,
            }
        })
    }
}

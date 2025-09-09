use super::client::{ApiClient, ApiResult};
use crate::models::publication::*;
use crate::models::article::ArticleListResponse;
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
        
        API_CLIENT.get(&format!("/blog/publications{}", query)).await
    }
    
    // 获取出版物详情
    pub async fn get_publication(slug: &str) -> ApiResult<Publication> {
        API_CLIENT.get(&format!("/blog/publications/{}", slug)).await
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
        
        API_CLIENT.get(&format!("/blog/publications/{}/members{}", publication_id, query)).await
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
        
        API_CLIENT.get(&format!("/blog/publications/{}/articles{}", slug, query)).await
    }
}
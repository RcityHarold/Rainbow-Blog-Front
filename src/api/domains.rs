use super::client::{ApiClient, ApiResult};
use crate::models::domain::*;
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct DomainService;

impl DomainService {
    // 获取出版物的域名列表
    pub async fn get_publication_domains(publication_id: &str) -> ApiResult<Vec<PublicationDomain>> {
        API_CLIENT.get(&format!("/blog/publications/{}/domains", publication_id)).await
    }
    
    // 创建子域名
    pub async fn create_subdomain(
        publication_id: &str, 
        request: &CreateSubdomainRequest
    ) -> ApiResult<CreateSubdomainResponse> {
        API_CLIENT.post(&format!("/blog/publications/{}/domains/subdomain", publication_id), request).await
    }
    
    // 添加自定义域名
    pub async fn add_custom_domain(
        publication_id: &str,
        request: &CreateCustomDomainRequest
    ) -> ApiResult<CreateCustomDomainResponse> {
        API_CLIENT.post(&format!("/blog/publications/{}/domains/custom", publication_id), request).await
    }
    
    // 获取域名状态
    pub async fn get_domain_status(domain_id: &str) -> ApiResult<DomainStatusResponse> {
        API_CLIENT.get(&format!("/blog/domains/{}/status", domain_id)).await
    }
    
    // 设置主域名
    pub async fn set_primary_domain(domain_id: &str) -> ApiResult<PublicationDomain> {
        API_CLIENT.put(&format!("/blog/domains/{}/primary", domain_id), &()).await
    }
    
    // 删除域名
    pub async fn delete_domain(domain_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/domains/{}", domain_id)).await
    }
    
    // 重新验证自定义域名
    pub async fn reverify_domain(domain_id: &str) -> ApiResult<CreateCustomDomainResponse> {
        API_CLIENT.post(&format!("/blog/domains/{}/reverify", domain_id), &()).await
    }
    
    // 获取DNS验证记录
    pub async fn get_verification_records(domain_id: &str) -> ApiResult<Vec<DNSRecord>> {
        API_CLIENT.get(&format!("/blog/domains/{}/verification-records", domain_id)).await
    }
}
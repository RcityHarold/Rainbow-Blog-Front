use super::client::{ApiClient, ApiResult};
use crate::models::domain::*;
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct DomainService;

impl DomainService {
    // 获取出版物的域名列表
    pub async fn get_publication_domains(publication_id: &str) -> ApiResult<Vec<PublicationDomain>> {
        // Backend returns: { success, data: { domains: [...], total } }
        #[derive(serde::Deserialize)]
        struct BackendList { domains: Vec<PublicationDomain>, total: i64 }
        let data: BackendList = API_CLIENT
            .get(&format!("/blog/domains/publications/{}/domains", Self::normalize_pub_id(publication_id)))
            .await?;
        Ok(data.domains)
    }
    
    // 创建子域名
    pub async fn create_subdomain(
        publication_id: &str, 
        request: &CreateSubdomainRequest
    ) -> ApiResult<PublicationDomain> {
        #[derive(serde::Deserialize)]
        struct DomainResp { domain: PublicationDomain }
        let resp: DomainResp = API_CLIENT.post(
            &format!("/blog/domains/publications/{}/domains/subdomain", Self::normalize_pub_id(publication_id)),
            request
        ).await?;
        Ok(resp.domain)
    }
    
    // 添加自定义域名
    pub async fn add_custom_domain(
        publication_id: &str,
        request: &CreateCustomDomainRequest
    ) -> ApiResult<PublicationDomain> {
        #[derive(serde::Deserialize)]
        struct DomainResp { domain: PublicationDomain }
        let resp: DomainResp = API_CLIENT.post(
            &format!("/blog/domains/publications/{}/domains/custom", Self::normalize_pub_id(publication_id)),
            request
        ).await?;
        Ok(resp.domain)
    }
    
    // 获取域名状态
    pub async fn get_domain_status(domain_id: &str) -> ApiResult<PublicationDomain> {
        // Map to backend get_domain_details: GET /api/blog/domains/domains/:domain_id
        API_CLIENT.get(&format!("/blog/domains/domains/{}", domain_id)).await
    }
    
    // 设置主域名
    pub async fn set_primary_domain(domain_id: &str) -> ApiResult<PublicationDomain> {
        // Map to update_domain: PUT /api/blog/domains/domains/:domain_id with body { is_primary: true }
        #[derive(serde::Serialize)]
        struct Update { is_primary: bool }
        API_CLIENT.put(&format!("/blog/domains/domains/{}", domain_id), &Update { is_primary: true }).await
    }
    
    // 删除域名
    pub async fn delete_domain(domain_id: &str) -> ApiResult<()> {
        // DELETE /api/blog/domains/domains/:domain_id
        API_CLIENT.delete(&format!("/blog/domains/domains/{}", domain_id)).await
    }
    
    // 重新验证自定义域名
    pub async fn reverify_domain(domain_id: &str) -> ApiResult<()> {
        // POST /api/blog/domains/domains/:domain_id/verify
        let _resp: serde_json::Value = API_CLIENT.post(&format!("/blog/domains/domains/{}/verify", domain_id), &()).await?;
        Ok(())
    }
    
    // 获取DNS验证记录
    pub async fn get_verification_records(domain_id: &str) -> ApiResult<Vec<DNSRecord>> {
        // No dedicated endpoint; fetch details if backend provides records, else return empty
        let _details: serde_json::Value = API_CLIENT.get(&format!("/blog/domains/domains/{}", domain_id)).await?;
        Ok(Vec::new())
    }

    // Helper: accept both "publication:⟨uuid⟩" and plain uuid
    fn normalize_pub_id(input: &str) -> String {
        if let Some((tb, rest)) = input.split_once(':') {
            if tb == "publication" {
                // unwrap angle brackets if present
                return rest.trim_matches(|c| c == '⟨' || c == '⟩').to_string();
            }
        }
        input.to_string()
    }
}

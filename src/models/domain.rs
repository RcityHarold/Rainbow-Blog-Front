use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicationDomain {
    pub id: String,
    pub publication_id: String,
    pub subdomain: Option<String>,
    pub custom_domain: Option<String>,
    pub status: DomainStatus,
    pub ssl_status: SSLStatus,
    pub is_primary: bool,
    pub verification_token: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub ssl_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DomainStatus {
    Pending,
    Verifying,
    Active,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SSLStatus {
    Pending,
    Active,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubdomainRequest {
    pub subdomain: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomDomainRequest {
    pub domain: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubdomainResponse {
    pub domain_id: String,
    pub subdomain: String,
    pub full_domain: String,
    pub status: String,
    pub ssl_status: String,
    pub is_immediately_available: bool,
    pub estimated_ssl_ready_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomDomainResponse {
    pub domain_id: String,
    pub custom_domain: String,
    pub status: String,
    pub verification_records: Vec<DNSRecord>,
    pub next_steps: Vec<String>,
    pub auto_verification: AutoVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSRecord {
    pub record_type: String,
    pub name: String,
    pub value: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoVerification {
    pub enabled: bool,
    pub check_interval: String,
    pub timeout: String,
    pub next_check_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStatusResponse {
    pub domain_id: String,
    pub status: String,
    pub progress: DomainProgress,
    pub verification_status: VerificationStatus,
    pub ssl_status: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainProgress {
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub remaining_steps: Vec<String>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStatus {
    pub txt_record: RecordVerification,
    pub cname_record: RecordVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordVerification {
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub last_check: Option<DateTime<Utc>>,
    pub next_check: Option<DateTime<Utc>>,
}

impl PublicationDomain {
    pub fn get_full_domain(&self) -> String {
        if let Some(subdomain) = &self.subdomain {
            format!("{}.platform.com", subdomain) // 实际应该从配置获取基础域名
        } else if let Some(custom) = &self.custom_domain {
            custom.clone()
        } else {
            String::new()
        }
    }
}
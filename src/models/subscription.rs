use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionPlan {
    pub id: String,
    pub creator_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64, // 价格，以美分为单位
    pub currency: String,
    pub benefits: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionPlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub currency: Option<String>,
    pub benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionPlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub benefits: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub subscriber_id: String,
    pub plan: SubscriptionPlan,
    pub creator: User,
    pub status: SubscriptionStatus,
    pub started_at: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Expired,
    PastDue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: String,
    pub payment_method_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlansResponse {
    pub plans: Vec<SubscriptionPlan>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionsResponse {
    pub subscriptions: Vec<Subscription>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: String,
    pub card_brand: String,
    pub card_last4: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodRequest {
    pub payment_method_id: String,
    pub set_as_default: bool,
}

impl SubscriptionPlan {
    pub fn format_price(&self) -> String {
        let price_dollars = self.price as f64 / 100.0;
        match self.currency.as_str() {
            "USD" => format!("${:.2}", price_dollars),
            "CNY" => format!("¥{:.2}", price_dollars),
            "EUR" => format!("€{:.2}", price_dollars),
            _ => format!("{:.2} {}", price_dollars, self.currency),
        }
    }
}
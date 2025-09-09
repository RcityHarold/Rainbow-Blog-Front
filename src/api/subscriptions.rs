use super::client::{ApiClient, ApiResult};
use crate::models::subscription::*;
use once_cell::sync::Lazy;

static API_CLIENT: Lazy<ApiClient> = Lazy::new(ApiClient::new);

pub struct SubscriptionService;

impl SubscriptionService {
    // === 订阅计划管理 ===
    
    // 创建订阅计划
    pub async fn create_plan(request: &CreateSubscriptionPlanRequest) -> ApiResult<SubscriptionPlan> {
        API_CLIENT.post("/blog/subscriptions/plans", request).await
    }
    
    // 获取订阅计划详情
    pub async fn get_plan(plan_id: &str) -> ApiResult<SubscriptionPlan> {
        API_CLIENT.get(&format!("/blog/subscriptions/plans/{}", plan_id)).await
    }
    
    // 更新订阅计划
    pub async fn update_plan(plan_id: &str, request: &UpdateSubscriptionPlanRequest) -> ApiResult<SubscriptionPlan> {
        API_CLIENT.put(&format!("/blog/subscriptions/plans/{}", plan_id), request).await
    }
    
    // 停用订阅计划
    pub async fn deactivate_plan(plan_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/subscriptions/plans/{}", plan_id)).await
    }
    
    // 获取创作者的订阅计划列表
    pub async fn get_creator_plans(
        creator_id: &str, 
        page: Option<i32>, 
        limit: Option<i32>,
        is_active: Option<bool>
    ) -> ApiResult<SubscriptionPlansResponse> {
        let mut query_params = vec![];
        
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(is_active) = is_active {
            query_params.push(format!("is_active={}", is_active));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        API_CLIENT.get(&format!("/blog/subscriptions/creator/{}/plans{}", creator_id, query)).await
    }
    
    // === 用户订阅管理 ===
    
    // 创建订阅
    pub async fn create_subscription(request: &CreateSubscriptionRequest) -> ApiResult<Subscription> {
        API_CLIENT.post("/blog/subscriptions", request).await
    }
    
    // 获取订阅详情
    pub async fn get_subscription(subscription_id: &str) -> ApiResult<Subscription> {
        API_CLIENT.get(&format!("/blog/subscriptions/{}", subscription_id)).await
    }
    
    // 取消订阅
    pub async fn cancel_subscription(subscription_id: &str) -> ApiResult<Subscription> {
        API_CLIENT.post(&format!("/blog/subscriptions/{}/cancel", subscription_id), &()).await
    }
    
    // 获取用户订阅列表
    pub async fn get_user_subscriptions(
        user_id: &str,
        page: Option<i32>,
        limit: Option<i32>,
        status: Option<&str>
    ) -> ApiResult<UserSubscriptionsResponse> {
        let mut query_params = vec![];
        
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(status) = status {
            query_params.push(format!("status={}", status));
        }
        
        let query = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        API_CLIENT.get(&format!("/blog/subscriptions/user/{}{}", user_id, query)).await
    }
    
    // === 支付方式管理 ===
    
    // 获取用户支付方式列表
    pub async fn get_payment_methods() -> ApiResult<Vec<PaymentMethod>> {
        API_CLIENT.get("/blog/payment-methods").await
    }
    
    // 添加支付方式
    pub async fn add_payment_method(request: &CreatePaymentMethodRequest) -> ApiResult<PaymentMethod> {
        API_CLIENT.post("/blog/payment-methods", request).await
    }
    
    // 删除支付方式
    pub async fn delete_payment_method(payment_method_id: &str) -> ApiResult<()> {
        API_CLIENT.delete(&format!("/blog/payment-methods/{}", payment_method_id)).await
    }
    
    // 设置默认支付方式
    pub async fn set_default_payment_method(payment_method_id: &str) -> ApiResult<PaymentMethod> {
        API_CLIENT.post(&format!("/blog/payment-methods/{}/default", payment_method_id), &()).await
    }
    
    // === 辅助方法 ===
    
    // 检查用户是否已订阅创作者
    pub async fn check_subscription_status(creator_id: &str) -> ApiResult<Option<Subscription>> {
        match API_CLIENT.get(&format!("/blog/subscriptions/creator/{}/status", creator_id)).await {
            Ok(subscription) => Ok(Some(subscription)),
            Err(_) => Ok(None), // 未订阅
        }
    }
    
    // 获取创作者收益统计
    pub async fn get_earnings_stats(creator_id: &str, period: Option<&str>) -> ApiResult<serde_json::Value> {
        let query = if let Some(period) = period {
            format!("?period={}", period)
        } else {
            String::new()
        };
        
        API_CLIENT.get(&format!("/blog/subscriptions/creator/{}/earnings{}", creator_id, query)).await
    }
}
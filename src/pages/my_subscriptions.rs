use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::subscriptions::SubscriptionService,
    models::subscription::*,
    hooks::use_auth,
    components::ProtectedRoute,
    Route,
};

#[component]
pub fn MySubscriptionsPage() -> Element {
    rsx! {
        ProtectedRoute {
            MySubscriptionsContent {}
        }
    }
}

#[component]
fn MySubscriptionsContent() -> Element {
    let mut subscriptions = use_signal(|| Vec::<Subscription>::new());
    let mut loading = use_signal(|| true);
    let mut filter_status = use_signal(|| "all".to_string());
    let mut error = use_signal(|| None::<String>);
    let auth = use_auth();
    
    // 加载订阅列表
    let load_subscriptions = move || {
        if let Some(user) = &auth.read().user {
            let user_id = user.id.clone();
            let status_str = filter_status();
            let status: Option<String> = if status_str == "all" { 
                None 
            } else { 
                Some(status_str)
            };
            
            spawn(async move {
                loading.set(true);
                
                let status_ref = status.as_ref().map(|s| s.as_str());
                match SubscriptionService::get_user_subscriptions(&user_id, Some(1), Some(50), status_ref).await {
                    Ok(response) => {
                        subscriptions.set(response.subscriptions);
                    }
                    Err(e) => {
                        error.set(Some(format!("加载订阅失败: {}", e.message)));
                    }
                }
                
                loading.set(false);
            });
        }
    };
    
    // 初始加载和状态过滤变化时重新加载
    use_effect(move || {
        load_subscriptions();
    });
    
    use_effect(move || {
        load_subscriptions();
    });
    
    // 取消订阅
    let cancel_subscription = move |subscription_id: String| {
        spawn(async move {
            match SubscriptionService::cancel_subscription(&subscription_id).await {
                Ok(_) => {
                    load_subscriptions();
                }
                Err(e) => {
                    error.set(Some(format!("取消订阅失败: {}", e.message)));
                }
            }
        });
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white dark:bg-gray-900",
            
            // 顶部导航
            nav {
                class: "border-b border-gray-200 dark:border-gray-700",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold text-gray-900 dark:text-white",
                            "Rainbow Blog"
                        }
                        
                        Link {
                            to: Route::Settings {},
                            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                            "← 返回设置"
                        }
                    }
                }
            }
            
            // 页面内容
            div {
                class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // 页面标题和筛选
                div {
                    class: "flex items-center justify-between mb-8",
                    div {
                        h1 {
                            class: "text-3xl font-bold text-gray-900 dark:text-white",
                            "我的订阅"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mt-1",
                            "管理您的所有订阅"
                        }
                    }
                    
                    // 状态筛选
                    select {
                        value: "{filter_status}",
                        onchange: move |e| filter_status.set(e.value()),
                        class: "px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                        
                        option { value: "all", "全部订阅" }
                        option { value: "active", "活跃订阅" }
                        option { value: "canceled", "已取消" }
                        option { value: "expired", "已过期" }
                    }
                }
                
                // 错误提示
                if let Some(error_msg) = error() {
                    div {
                        class: "mb-6 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded-lg",
                        {error_msg}
                    }
                }
                
                // 订阅列表
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if subscriptions().is_empty() {
                    div {
                        class: "text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg",
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400 mb-4",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            }
                        }
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                            if filter_status() == "all" {
                                "您还没有任何订阅"
                            } else {
                                "没有找到符合条件的订阅"
                            }
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mb-4",
                            "订阅优秀创作者，获得独家内容"
                        }
                        Link {
                            to: Route::Home {},
                            class: "inline-block px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                            "发现创作者"
                        }
                    }
                } else {
                    div {
                        class: "space-y-6",
                        for subscription in subscriptions() {
                            SubscriptionCard {
                                subscription,
                                on_cancel: move |id| cancel_subscription(id)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SubscriptionCard(
    subscription: Subscription,
    on_cancel: EventHandler<String>,
) -> Element {
    let status_info = match subscription.status {
        SubscriptionStatus::Active => ("活跃", "text-green-600", "bg-green-100 dark:bg-green-900"),
        SubscriptionStatus::Canceled => ("已取消", "text-gray-600", "bg-gray-100 dark:bg-gray-800"),
        SubscriptionStatus::Expired => ("已过期", "text-red-600", "bg-red-100 dark:bg-red-900"),
        SubscriptionStatus::PastDue => ("逾期", "text-yellow-600", "bg-yellow-100 dark:bg-yellow-900"),
    };
    
    let mut show_cancel_modal = use_signal(|| false);
    
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
            
            div {
                class: "flex items-start justify-between",
                div {
                    class: "flex items-start space-x-4 flex-1",
                    
                    // 创作者头像
                    if let Some(avatar_url) = &subscription.creator.avatar_url {
                        img {
                            src: "{avatar_url}",
                            alt: "{subscription.creator.display_name.as_deref().unwrap_or(&subscription.creator.username)}",
                            class: "w-12 h-12 rounded-full"
                        }
                    } else {
                        div {
                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-full flex items-center justify-center",
                            span {
                                class: "text-lg font-semibold text-gray-600 dark:text-gray-300",
                                {subscription.creator.display_name.as_deref().unwrap_or(&subscription.creator.username).chars().next().unwrap_or('U').to_string()}
                            }
                        }
                    }
                    
                    div {
                        class: "flex-1",
                        
                        // 创作者信息
                        div {
                            class: "flex items-center space-x-2 mb-1",
                            Link {
                                to: Route::Profile { username: subscription.creator.username.clone() },
                                class: "text-lg font-semibold text-gray-900 dark:text-white hover:underline",
                                {subscription.creator.display_name.clone().unwrap_or(subscription.creator.username.clone())}
                            }
                            if subscription.creator.is_verified {
                                svg {
                                    class: "w-5 h-5 text-blue-500",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                        clip_rule: "evenodd"
                                    }
                                }
                            }
                        }
                        
                        // 订阅计划信息
                        div {
                            h3 {
                                class: "font-medium text-gray-900 dark:text-white",
                                {subscription.plan.name.clone()}
                            }
                            p {
                                class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                {subscription.plan.format_price()} "/月"
                            }
                        }
                        
                        // 订阅时间信息
                        div {
                            class: "mt-3 text-sm text-gray-500 space-y-1",
                            div {
                                "开始时间：{subscription.started_at.format(\"%Y-%m-%d\").to_string()}"
                            }
                            if matches!(subscription.status, SubscriptionStatus::Active) {
                                div {
                                    "下次续费：{subscription.current_period_end.format(\"%Y-%m-%d\").to_string()}"
                                }
                            }
                            if let Some(canceled_at) = subscription.canceled_at {
                                div {
                                    "取消时间：{canceled_at.format(\"%Y-%m-%d\").to_string()}"
                                }
                            }
                        }
                    }
                }
                
                // 状态和操作
                div {
                    class: "flex flex-col items-end space-y-3",
                    
                    // 状态标签
                    span {
                        class: format!("px-3 py-1 {} {} text-xs rounded-full", status_info.1, status_info.2),
                        {status_info.0}
                    }
                    
                    // 操作按钮
                    if matches!(subscription.status, SubscriptionStatus::Active) {
                        button {
                            class: "px-4 py-2 text-sm text-red-600 border border-red-300 dark:border-red-700 rounded hover:bg-red-50 dark:hover:bg-red-900/20",
                            onclick: move |_| show_cancel_modal.set(true),
                            "取消订阅"
                        }
                    }
                }
            }
        }
        
        // 取消订阅确认模态框
        if show_cancel_modal() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                onclick: move |_| show_cancel_modal.set(false),
                
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-md mx-4",
                    onclick: move |e| e.stop_propagation(),
                    
                    h3 {
                        class: "text-lg font-semibold mb-4",
                        "确认取消订阅"
                    }
                    
                    p {
                        class: "text-gray-600 dark:text-gray-400 mb-6",
                        {format!("您确定要取消对 {} 的订阅吗？取消后您将失去访问付费内容的权限。", subscription.creator.display_name.as_deref().unwrap_or(&subscription.creator.username))}
                    }
                    
                    div {
                        class: "flex justify-end space-x-3",
                        button {
                            class: "px-4 py-2 text-gray-600 hover:text-gray-800",
                            onclick: move |_| show_cancel_modal.set(false),
                            "保留订阅"
                        }
                        button {
                            class: "px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700",
                            onclick: move |_| {
                                on_cancel.call(subscription.id.clone());
                                show_cancel_modal.set(false);
                            },
                            "确认取消"
                        }
                    }
                }
            }
        }
    }
}
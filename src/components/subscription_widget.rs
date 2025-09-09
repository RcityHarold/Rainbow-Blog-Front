use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::subscriptions::SubscriptionService,
    models::{user::User, subscription::*},
    hooks::use_auth,
    Route,
};

#[component]
pub fn SubscriptionWidget(
    creator: User,
    is_premium_content: Option<bool>,
) -> Element {
    let creator_id = creator.id.clone();
    let mut subscription_status = use_signal(|| None::<Subscription>);
    let mut loading = use_signal(|| false);
    let mut show_plans_modal = use_signal(|| false);
    let mut plans = use_signal(|| Vec::<SubscriptionPlan>::new());
    let auth = use_auth();
    
    let is_premium = is_premium_content.unwrap_or(false);
    let creator_id_for_check = creator_id.clone();
    let creator_id_for_comparison = creator_id.clone();
    let creator_id_for_load = creator_id.clone();
    
    // 检查当前用户的订阅状态
    let check_subscription = {
        let creator_id = creator_id_for_check.clone();
        let auth = auth.clone();
        let mut loading = loading.clone();
        let mut subscription_status = subscription_status.clone();
        move || {
        if let Some(user) = &auth.read().user {
            if user.id == creator_id {
                // 创作者自己的内容，无需订阅
                return;
            }
            
            let creator_id = creator_id.clone();
            let creator_id = creator_id.clone();
            let mut loading = loading.clone();
            let mut subscription_status = subscription_status.clone();
            spawn(async move {
                loading.set(true);
                if let Ok(subscription) = SubscriptionService::check_subscription_status(&creator_id).await {
                    subscription_status.set(subscription);
                }
                loading.set(false);
            });
        }
        }
    };
    
    // 加载创作者的订阅计划
    let load_plans = move || {
        let creator_id = creator_id_for_load.clone();
        let mut plans = plans.clone();
        spawn(async move {
            if let Ok(response) = SubscriptionService::get_creator_plans(&creator_id, Some(1), Some(10), Some(true)).await {
                plans.set(response.plans);
            }
        });
    };
    
    // 初始检查
    use_effect(move || {
        check_subscription();
        load_plans();
    });
    
    // 创建订阅
    let subscribe_to_plan = move |plan_id: String| {
        spawn(async move {
            let request = CreateSubscriptionRequest {
                plan_id,
                payment_method_id: None, // 假设使用默认支付方式
            };
            
            if let Ok(subscription) = SubscriptionService::create_subscription(&request).await {
                subscription_status.set(Some(subscription));
                show_plans_modal.set(false);
            }
        });
    };
    
    // 判断是否需要显示订阅提示
    let needs_subscription = if let Some(user) = &auth.read().user {
        if user.id == creator_id_for_comparison {
            false // 创作者自己
        } else if !is_premium {
            false // 非付费内容
        } else {
            // 付费内容，检查订阅状态
            match subscription_status() {
                Some(subscription) => !matches!(subscription.status, SubscriptionStatus::Active),
                None => true,
            }
        }
    } else {
        is_premium // 未登录且是付费内容
    };
    
    if !needs_subscription {
        return rsx! {};
    }
    
    rsx! {
        // 付费内容锁定提示
        if is_premium && subscription_status().is_none() {
            div {
                class: "bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6 my-8",
                
                div {
                    class: "flex items-start space-x-4",
                    
                    // 锁定图标
                    svg {
                        class: "w-8 h-8 text-blue-600 flex-shrink-0 mt-1",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        path {
                            fill_rule: "evenodd",
                            d: "M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z",
                            clip_rule: "evenodd"
                        }
                    }
                    
                    div {
                        class: "flex-1",
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                            "订阅以继续阅读"
                        }
                        p {
                            class: "text-gray-700 dark:text-gray-300 mb-4",
                            {format!("此内容仅向 {} 的订阅者开放。订阅即可获得独家内容和额外权益。", creator.display_name.as_deref().unwrap_or(&creator.username))}
                        }
                        
                        div {
                            class: "flex items-center space-x-3",
                            
                            if auth.read().user.is_some() {
                                button {
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium",
                                    onclick: move |_| show_plans_modal.set(true),
                                    "查看订阅计划"
                                }
                            } else {
                                Link {
                                    to: Route::Login {},
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium",
                                    "登录订阅"
                                }
                            }
                            
                            Link {
                                to: Route::Profile { username: creator.username.clone() },
                                class: "text-blue-600 hover:underline font-medium",
                                "了解创作者"
                            }
                        }
                    }
                }
            }
        }
        
        // 订阅计划模态框
        if show_plans_modal() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                onclick: move |_| show_plans_modal.set(false),
                
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-2xl mx-4 max-h-[80vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    
                    div {
                        class: "flex items-center justify-between mb-6",
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white",
                            {format!("订阅 {}", creator.display_name.as_deref().unwrap_or(&creator.username))}
                        }
                        button {
                            class: "text-gray-500 hover:text-gray-700",
                            onclick: move |_| show_plans_modal.set(false),
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }
                    }
                    
                    if plans().is_empty() {
                        div {
                            class: "text-center py-8",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "该创作者暂未设置订阅计划"
                            }
                        }
                    } else {
                        div {
                            class: "space-y-4",
                            for plan in plans() {
                                div {
                                    class: "border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:shadow-md transition-shadow",
                                    
                                    div {
                                        class: "flex items-center justify-between mb-3",
                                        h4 {
                                            class: "text-lg font-semibold text-gray-900 dark:text-white",
                                            {plan.name.clone()}
                                        }
                                        div {
                                            class: "text-xl font-bold text-blue-600",
                                            {plan.format_price()}
                                            span {
                                                class: "text-sm font-normal text-gray-500 ml-1",
                                                "/月"
                                            }
                                        }
                                    }
                                    
                                    if let Some(description) = &plan.description {
                                        p {
                                            class: "text-gray-600 dark:text-gray-400 text-sm mb-3",
                                            {description.clone()}
                                        }
                                    }
                                    
                                    if !plan.benefits.is_empty() {
                                        div {
                                            class: "mb-4",
                                            ul {
                                                class: "space-y-1",
                                                for benefit in plan.benefits.iter() {
                                                    li {
                                                        class: "flex items-center text-sm text-gray-600 dark:text-gray-400",
                                                        svg {
                                                            class: "w-4 h-4 text-green-500 mr-2 flex-shrink-0",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                stroke_width: "2",
                                                                d: "M5 13l4 4L19 7"
                                                            }
                                                        }
                                                        {benefit.clone()}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    
                                    button {
                                        class: "w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium",
                                        onclick: move |_| subscribe_to_plan(plan.id.clone()),
                                        "选择此计划"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
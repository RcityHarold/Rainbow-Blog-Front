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
pub fn SubscriptionPlansPage() -> Element {
    rsx! {
        ProtectedRoute {
            SubscriptionPlansContent {}
        }
    }
}

#[component]
fn SubscriptionPlansContent() -> Element {
    let mut plans = use_signal(|| Vec::<SubscriptionPlan>::new());
    let mut loading = use_signal(|| true);
    let mut show_create_modal = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);
    
    // 表单状态
    let mut plan_name = use_signal(|| String::new());
    let mut plan_description = use_signal(|| String::new());
    let mut plan_price = use_signal(|| String::new());
    let mut plan_currency = use_signal(|| "USD".to_string());
    let mut benefits = use_signal(|| vec!["".to_string()]);
    let mut creating = use_signal(|| false);
    
    let auth = use_auth();
    
    // 加载订阅计划
    let load_plans = move || {
        if let Some(user) = &auth.read().user {
            let user_id = user.id.clone();
            spawn(async move {
                loading.set(true);
                
                match SubscriptionService::get_creator_plans(&user_id, Some(1), Some(50), None).await {
                    Ok(response) => {
                        plans.set(response.plans);
                    }
                    Err(e) => {
                        error.set(Some(format!("加载订阅计划失败: {}", e.message)));
                    }
                }
                
                loading.set(false);
            });
        }
    };
    
    // 初始加载
    use_effect(move || {
        load_plans();
    });
    
    // 创建订阅计划
    let mut create_plan = move |_| {
        creating.set(true);
        error.set(None);
        
        // 解析价格
        let price_cents = match plan_price().parse::<f64>() {
            Ok(price) => (price * 100.0) as i64,
            Err(_) => {
                error.set(Some("请输入有效的价格".to_string()));
                creating.set(false);
                return;
            }
        };
        
        // 过滤空的收益项
        let filtered_benefits: Vec<String> = benefits()
            .into_iter()
            .filter(|b| !b.trim().is_empty())
            .collect();
        
        let request = CreateSubscriptionPlanRequest {
            name: plan_name(),
            description: if plan_description().is_empty() { None } else { Some(plan_description()) },
            price: price_cents,
            currency: Some(plan_currency()),
            benefits: filtered_benefits,
        };
        
        spawn(async move {
            match SubscriptionService::create_plan(&request).await {
                Ok(_) => {
                    show_create_modal.set(false);
                    // 重置表单
                    plan_name.set(String::new());
                    plan_description.set(String::new());
                    plan_price.set(String::new());
                    plan_currency.set("USD".to_string());
                    benefits.set(vec!["".to_string()]);
                    
                    success.set(Some("订阅计划创建成功！".to_string()));
                    load_plans();
                    
                    // 3秒后清除成功消息
                    spawn(async move {
                        gloo_timers::future::TimeoutFuture::new(3000).await;
                        success.set(None);
                    });
                }
                Err(e) => {
                    error.set(Some(format!("创建失败: {}", e.message)));
                }
            }
            
            creating.set(false);
        });
    };
    
    // 添加收益项
    let add_benefit = move |_| {
        let mut current_benefits = benefits();
        current_benefits.push("".to_string());
        benefits.set(current_benefits);
    };
    
    // 删除收益项
    let mut remove_benefit = move |index: usize| {
        let mut current_benefits = benefits();
        if current_benefits.len() > 1 {
            current_benefits.remove(index);
            benefits.set(current_benefits);
        }
    };
    
    // 更新收益项
    let mut update_benefit = move |index: usize, value: String| {
        let mut current_benefits = benefits();
        if index < current_benefits.len() {
            current_benefits[index] = value;
            benefits.set(current_benefits);
        }
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
                
                // 页面标题
                div {
                    class: "flex items-center justify-between mb-8",
                    div {
                        h1 {
                            class: "text-3xl font-bold text-gray-900 dark:text-white",
                            "订阅计划管理"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mt-1",
                            "创建和管理您的会员订阅计划"
                        }
                    }
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                        onclick: move |_| show_create_modal.set(true),
                        "创建新计划"
                    }
                }
                
                // 成功/错误提示
                if let Some(success_msg) = success() {
                    div {
                        class: "mb-6 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 text-green-700 dark:text-green-400 px-4 py-3 rounded-lg",
                        {success_msg}
                    }
                }
                
                if let Some(error_msg) = error() {
                    div {
                        class: "mb-6 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded-lg",
                        {error_msg}
                    }
                }
                
                // 订阅计划列表
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if plans().is_empty() {
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
                            "还没有创建订阅计划"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mb-4",
                            "创建订阅计划让读者支持您的创作"
                        }
                        button {
                            class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                            onclick: move |_| show_create_modal.set(true),
                            "创建第一个计划"
                        }
                    }
                } else {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for plan in plans() {
                            SubscriptionPlanCard {
                                plan,
                                on_update: move |_| {
                                    if let Some(user) = &auth.read().user {
                                        let user_id = user.id.clone();
                                        spawn(async move {
                                            loading.set(true);
                                            if let Ok(response) = SubscriptionService::get_creator_plans(&user_id, Some(1), Some(50), None).await {
                                                plans.set(response.plans);
                                            }
                                            loading.set(false);
                                        });
                                    }
                                },
                                on_delete: move |plan_id: String| {
                                    spawn(async move {
                                        if let Ok(_) = SubscriptionService::deactivate_plan(&plan_id).await {
                                            if let Some(user) = &auth.read().user {
                                                let user_id = user.id.clone();
                                                spawn(async move {
                                                    loading.set(true);
                                                    if let Ok(response) = SubscriptionService::get_creator_plans(&user_id, Some(1), Some(50), None).await {
                                                        plans.set(response.plans);
                                                    }
                                                    loading.set(false);
                                                });
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
            
            // 创建计划模态框
            if show_create_modal() {
                div {
                    class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                    onclick: move |_| show_create_modal.set(false),
                    
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto",
                        onclick: move |e| e.stop_propagation(),
                        
                        div {
                            class: "flex items-center justify-between mb-6",
                            h3 {
                                class: "text-lg font-semibold",
                                "创建订阅计划"
                            }
                            button {
                                class: "text-gray-500 hover:text-gray-700",
                                onclick: move |_| {
                                    show_create_modal.set(false);
                                    error.set(None);
                                },
                                svg {
                                    class: "w-5 h-5",
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
                        
                        form {
                            class: "space-y-6",
                            onsubmit: move |e| {
                                e.prevent_default();
                                create_plan(());
                            },
                            
                            // 计划名称
                            div {
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "计划名称"
                                }
                                input {
                                    r#type: "text",
                                    required: true,
                                    placeholder: "例如：高级会员、VIP会员",
                                    value: "{plan_name}",
                                    oninput: move |e| plan_name.set(e.value()),
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                }
                            }
                            
                            // 计划描述
                            div {
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "计划描述"
                                }
                                textarea {
                                    placeholder: "描述此订阅计划的特色和价值...",
                                    value: "{plan_description}",
                                    oninput: move |e| plan_description.set(e.value()),
                                    rows: "3",
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                }
                            }
                            
                            // 价格和货币
                            div {
                                class: "grid grid-cols-2 gap-4",
                                div {
                                    label {
                                        class: "block text-sm font-medium mb-2",
                                        "价格"
                                    }
                                    input {
                                        r#type: "number",
                                        step: "0.01",
                                        min: "0",
                                        required: true,
                                        placeholder: "9.99",
                                        value: "{plan_price}",
                                        oninput: move |e| plan_price.set(e.value()),
                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    }
                                }
                                div {
                                    label {
                                        class: "block text-sm font-medium mb-2",
                                        "货币"
                                    }
                                    select {
                                        value: "{plan_currency}",
                                        onchange: move |e| plan_currency.set(e.value()),
                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        
                                        option { value: "USD", "美元 (USD)" }
                                        option { value: "CNY", "人民币 (CNY)" }
                                        option { value: "EUR", "欧元 (EUR)" }
                                    }
                                }
                            }
                            
                            // 会员权益
                            div {
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "会员权益"
                                }
                                div {
                                    class: "space-y-2",
                                    for (index, benefit) in benefits().iter().enumerate() {
                                        div {
                                            class: "flex items-center space-x-2",
                                            input {
                                                r#type: "text",
                                                placeholder: "例如：访问所有付费文章",
                                                value: "{benefit}",
                                                oninput: move |e| update_benefit(index, e.value()),
                                                class: "flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                            }
                                            if benefits().len() > 1 {
                                                button {
                                                    r#type: "button",
                                                    class: "px-2 py-2 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded",
                                                    onclick: move |_| remove_benefit(index),
                                                    svg {
                                                        class: "w-4 h-4",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        view_box: "0 0 24 24",
                                                        path {
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "mt-2 px-3 py-1 text-sm text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded",
                                    onclick: add_benefit,
                                    "+ 添加权益"
                                }
                            }
                            
                            // 按钮
                            div {
                                class: "flex justify-end space-x-3 pt-4",
                                button {
                                    r#type: "button",
                                    class: "px-4 py-2 text-gray-600 hover:text-gray-800",
                                    onclick: move |_| {
                                        show_create_modal.set(false);
                                        error.set(None);
                                    },
                                    "取消"
                                }
                                button {
                                    r#type: "submit",
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50",
                                    disabled: creating() || plan_name().is_empty() || plan_price().is_empty(),
                                    if creating() {
                                        "创建中..."
                                    } else {
                                        "创建计划"
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

#[component]
fn SubscriptionPlanCard(
    plan: SubscriptionPlan,
    on_update: EventHandler<()>,
    on_delete: EventHandler<String>,
) -> Element {
    let description_text = plan.description.clone();
    let benefits_list = plan.benefits.clone();
    let benefits_count = benefits_list.len();
    
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
            
            div {
                class: "flex items-start justify-between mb-4",
                div {
                    h3 {
                        class: "text-lg font-semibold text-gray-900 dark:text-white",
                        {plan.name.clone()}
                    }
                    p {
                        class: "text-2xl font-bold text-blue-600 mt-1",
                        {plan.format_price()}
                        span {
                            class: "text-sm font-normal text-gray-500 ml-1",
                            "/月"
                        }
                    }
                }
                
                // 状态指示器
                div {
                    span {
                        class: if plan.is_active {
                            "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 text-xs rounded"
                        } else {
                            "px-2 py-1 bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200 text-xs rounded"
                        },
                        if plan.is_active { "活跃" } else { "停用" }
                    }
                }
            }
            
            // 描述
            if let Some(description) = description_text {
                p {
                    class: "text-gray-600 dark:text-gray-400 text-sm mb-4",
                    {description}
                }
            }
            
            // 权益列表
            if !benefits_list.is_empty() {
                div {
                    class: "mb-4",
                    h4 {
                        class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                        "会员权益"
                    }
                    ul {
                        class: "space-y-1",
                        for benefit in benefits_list.iter().take(3).cloned() {
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
                                {benefit}
                            }
                        }
                        if benefits_count > 3 {
                            li {
                                class: "text-sm text-gray-500 italic",
                                "还有 {benefits_count - 3} 项权益..."
                            }
                        }
                    }
                }
            }
            
            // 操作按钮
            div {
                class: "flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-700",
                div {
                    class: "text-xs text-gray-500",
                    "创建于 {plan.created_at.format(\"%Y-%m-%d\").to_string()}"
                }
                
                div {
                    class: "flex items-center space-x-2",
                    button {
                        class: "px-3 py-1 text-sm text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded",
                        onclick: move |_| on_update.call(()),
                        "编辑"
                    }
                    button {
                        class: "px-3 py-1 text-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded",
                        onclick: move |_| on_delete.call(plan.id.clone()),
                        if plan.is_active { "停用" } else { "删除" }
                    }
                }
            }
        }
    }
}
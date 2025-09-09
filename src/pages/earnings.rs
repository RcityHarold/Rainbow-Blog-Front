use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::subscriptions::SubscriptionService,
    hooks::use_auth,
    components::ProtectedRoute,
    Route,
};
use serde_json::Value;

#[component]
pub fn EarningsPage() -> Element {
    rsx! {
        ProtectedRoute {
            EarningsContent {}
        }
    }
}

#[component]
fn EarningsContent() -> Element {
    let mut earnings_data = use_signal(|| None::<Value>);
    let mut loading = use_signal(|| true);
    let mut selected_period = use_signal(|| "month".to_string());
    let auth = use_auth();
    
    // 加载收益数据
    let load_earnings = move || {
        if let Some(user) = &auth.read().user {
            let user_id = user.id.clone();
            let period_str = selected_period();
            let period: Option<String> = if period_str == "all" { 
                None 
            } else { 
                Some(period_str)
            };
            
            spawn(async move {
                loading.set(true);
                let period_ref = period.as_ref().map(|s| s.as_str());
                if let Ok(data) = SubscriptionService::get_earnings_stats(&user_id, period_ref).await {
                    earnings_data.set(Some(data));
                }
                loading.set(false);
            });
        }
    };
    
    // 初始加载和周期变化时重新加载
    use_effect(move || {
        load_earnings();
    });
    
    use_effect(move || {
        load_earnings();
    });
    
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
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // 页面标题和周期选择
                div {
                    class: "flex items-center justify-between mb-8",
                    div {
                        h1 {
                            class: "text-3xl font-bold text-gray-900 dark:text-white",
                            "收益统计"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mt-1",
                            "查看您的订阅收益和统计数据"
                        }
                    }
                    
                    // 时间周期选择
                    select {
                        value: "{selected_period}",
                        onchange: move |e| selected_period.set(e.value()),
                        class: "px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                        
                        option { value: "week", "本周" }
                        option { value: "month", "本月" }
                        option { value: "quarter", "本季度" }
                        option { value: "year", "本年" }
                        option { value: "all", "全部" }
                    }
                }
                
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if let Some(data) = earnings_data() {
                    div {
                        class: "space-y-6",
                        
                        // 核心指标卡片
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                            
                            // 总收益
                            div {
                                class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "flex-shrink-0",
                                        div {
                                            class: "w-8 h-8 bg-green-500 rounded-md flex items-center justify-center",
                                            svg {
                                                class: "w-5 h-5 text-white",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    d: "M4 4a2 2 0 00-2 2v4a2 2 0 002 2V6h10a2 2 0 00-2-2H4zM14 6a2 2 0 012 2v4a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2h6zM4 11a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zM7 9a1 1 0 100 2h6a1 1 0 100-2H7z"
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt {
                                                class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                                "总收益"
                                            }
                                            dd {
                                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                                {data.get("total_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0).to_string()} " 元"
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 活跃订阅数
                            div {
                                class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "flex-shrink-0",
                                        div {
                                            class: "w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center",
                                            svg {
                                                class: "w-5 h-5 text-white",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    d: "M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3z"
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt {
                                                class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                                "活跃订阅"
                                            }
                                            dd {
                                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                                {data.get("active_subscriptions").and_then(|v| v.as_u64()).unwrap_or(0).to_string()}
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 新增订阅数
                            div {
                                class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "flex-shrink-0",
                                        div {
                                            class: "w-8 h-8 bg-purple-500 rounded-md flex items-center justify-center",
                                            svg {
                                                class: "w-5 h-5 text-white",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z",
                                                    clip_rule: "evenodd"
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt {
                                                class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                                "新增订阅"
                                            }
                                            dd {
                                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                                {data.get("new_subscriptions").and_then(|v| v.as_u64()).unwrap_or(0).to_string()}
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 取消订阅数
                            div {
                                class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                                div {
                                    class: "flex items-center",
                                    div {
                                        class: "flex-shrink-0",
                                        div {
                                            class: "w-8 h-8 bg-red-500 rounded-md flex items-center justify-center",
                                            svg {
                                                class: "w-5 h-5 text-white",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                                    clip_rule: "evenodd"
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt {
                                                class: "text-sm font-medium text-gray-500 dark:text-gray-400 truncate",
                                                "取消订阅"
                                            }
                                            dd {
                                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                                {data.get("canceled_subscriptions").and_then(|v| v.as_u64()).unwrap_or(0).to_string()}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // 收益趋势图表占位
                        div {
                            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                            h3 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                                "收益趋势"
                            }
                            div {
                                class: "h-64 flex items-center justify-center bg-gray-50 dark:bg-gray-700 rounded-lg",
                                p {
                                    class: "text-gray-500 dark:text-gray-400",
                                    "图表功能即将推出"
                                }
                            }
                        }
                        
                        // 最近交易
                        div {
                            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
                            h3 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                                "最近交易"
                            }
                            
                            if let Some(transactions) = data.get("recent_transactions").and_then(|v| v.as_array()) {
                                if transactions.is_empty() {
                                    div {
                                        class: "text-center py-8",
                                        p {
                                            class: "text-gray-500 dark:text-gray-400",
                                            "暂无交易记录"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "overflow-hidden",
                                        table {
                                            class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                                            thead {
                                                class: "bg-gray-50 dark:bg-gray-700",
                                                tr {
                                                    th {
                                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                                        "时间"
                                                    }
                                                    th {
                                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                                        "类型"
                                                    }
                                                    th {
                                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                                        "金额"
                                                    }
                                                    th {
                                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                                        "状态"
                                                    }
                                                }
                                            }
                                            tbody {
                                                class: "bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700",
                                                for (index, transaction) in transactions.iter().enumerate() {
                                                    tr {
                                                        key: "{index}",
                                                        td {
                                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white",
                                                            {transaction.get("date").and_then(|v| v.as_str()).unwrap_or("--")}
                                                        }
                                                        td {
                                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white",
                                                            {transaction.get("type").and_then(|v| v.as_str()).unwrap_or("--")}
                                                        }
                                                        td {
                                                            class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white",
                                                            {transaction.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0).to_string()} " 元"
                                                        }
                                                        td {
                                                            class: "px-6 py-4 whitespace-nowrap",
                                                            span {
                                                                class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                                                {transaction.get("status").and_then(|v| v.as_str()).unwrap_or("--")}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                div {
                                    class: "text-center py-8",
                                    p {
                                        class: "text-gray-500 dark:text-gray-400",
                                        "暂无交易记录"
                                    }
                                }
                            }
                        }
                    }
                } else {
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
                                d: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                            }
                        }
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                            "暂无收益数据"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mb-4",
                            "创建订阅计划开始获得收益"
                        }
                        Link {
                            to: Route::SubscriptionPlans {},
                            class: "inline-block px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                            "创建订阅计划"
                        }
                    }
                }
            }
        }
    }
}
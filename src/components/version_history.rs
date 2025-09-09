use dioxus::prelude::*;
use dioxus::events::MouseEvent;
use crate::{
    models::version::ArticleVersion,
    api::versions::VersionService,
};
use chrono::{DateTime, Utc};

#[component]
pub fn VersionHistory(
    article_id: String,
    show: bool,
    on_close: EventHandler<()>,
    on_restore: EventHandler<ArticleVersion>,
) -> Element {
    let mut versions = use_signal(|| Vec::<ArticleVersion>::new());
    let mut loading = use_signal(|| false);
    let mut selected_version = use_signal(|| None::<String>);
    let mut comparing = use_signal(|| false);
    let mut compare_version_a = use_signal(|| None::<String>);
    let mut compare_version_b = use_signal(|| None::<String>);
    let article_id_for_button = article_id.clone();
    
    // 加载版本历史
    use_effect(move || {
        if show && !article_id.is_empty() {
            loading.set(true);
            let article_id = article_id.clone();
            
            spawn(async move {
                if let Ok(version_list) = VersionService::get_article_versions(&article_id).await {
                    versions.set(version_list);
                }
                loading.set(false);
            });
        }
    });
    
    if !show {
        return rsx! {};
    }
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-hidden",
            
            // 背景遮罩
            div {
                class: "absolute inset-0 bg-black bg-opacity-50",
                onclick: move |_| on_close.call(())
            }
            
            // 侧边栏
            div {
                class: "absolute right-0 top-0 h-full w-full max-w-md bg-white dark:bg-gray-800 shadow-xl",
                
                // 头部
                div {
                    class: "flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white",
                        "版本历史"
                    }
                    button {
                        class: "p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| on_close.call(()),
                        svg {
                            class: "w-5 h-5 text-gray-500",
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
                
                // 操作栏
                div {
                    class: "px-6 py-3 border-b border-gray-200 dark:border-gray-700",
                    div {
                        class: "flex items-center justify-between",
                        button {
                            class: if comparing() {
                                "text-sm font-medium text-blue-600 dark:text-blue-400"
                            } else {
                                "text-sm font-medium text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white"
                            },
                            onclick: move |_| {
                                comparing.set(!comparing());
                                compare_version_a.set(None);
                                compare_version_b.set(None);
                            },
                            if comparing() {
                                "取消对比"
                            } else {
                                "对比版本"
                            }
                        }
                        
                        if comparing() && compare_version_a().is_some() && compare_version_b().is_some() {
                            button {
                                class: "px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700",
                                onclick: {
                                    let article_id = article_id_for_button.clone();
                                    move |_| {
                                        // 打开对比视图
                                        if let (Some(a), Some(b)) = (compare_version_a(), compare_version_b()) {
                                            web_sys::window()
                                                .unwrap()
                                                .open_with_url_and_target(
                                                    &format!("/article/{}/versions/compare?a={}&b={}", article_id, a, b),
                                                    "_blank"
                                                )
                                                .ok();
                                        }
                                    }
                                },
                                "查看对比"
                            }
                        }
                    }
                }
                
                // 版本列表
                div {
                    class: "flex-1 overflow-y-auto",
                    
                    if loading() {
                        div {
                            class: "flex justify-center py-8",
                            div {
                                class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                            }
                        }
                    } else if versions().is_empty() {
                        div {
                            class: "text-center py-12",
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "暂无版本历史"
                            }
                        }
                    } else {
                        div {
                            class: "divide-y divide-gray-200 dark:divide-gray-700",
                            
                            for (index, version) in versions().iter().enumerate() {
                                VersionItem {
                                    version: version.clone(),
                                    is_current: index == 0,
                                    is_selected: selected_version() == Some(version.id.clone()),
                                    comparing: comparing(),
                                    is_compare_a: compare_version_a() == Some(version.id.clone()),
                                    is_compare_b: compare_version_b() == Some(version.id.clone()),
                                    on_select: move |v: ArticleVersion| {
                                        if comparing() {
                                            if compare_version_a().is_none() {
                                                compare_version_a.set(Some(v.id.clone()));
                                            } else if compare_version_b().is_none() && compare_version_a() != Some(v.id.clone()) {
                                                compare_version_b.set(Some(v.id.clone()));
                                            } else {
                                                // 重置选择
                                                compare_version_a.set(Some(v.id.clone()));
                                                compare_version_b.set(None);
                                            }
                                        } else {
                                            selected_version.set(Some(v.id.clone()));
                                        }
                                    },
                                    on_restore: move |v| on_restore.call(v)
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
fn VersionItem(
    version: ArticleVersion,
    is_current: bool,
    is_selected: bool,
    comparing: bool,
    is_compare_a: bool,
    is_compare_b: bool,
    on_select: EventHandler<ArticleVersion>,
    on_restore: EventHandler<ArticleVersion>,
) -> Element {
    rsx! {
        div {
            class: {
                if is_selected && !comparing {
                    "p-4 bg-blue-50 dark:bg-blue-900/20 cursor-pointer"
                } else if is_compare_a || is_compare_b {
                    "p-4 bg-yellow-50 dark:bg-yellow-900/20 cursor-pointer"
                } else {
                    "p-4 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer"
                }
            },
            onclick: {
                let version_for_select = version.clone();
                move |_| on_select.call(version_for_select.clone())
            },
            
            div {
                class: "flex items-start justify-between",
                
                div {
                    class: "flex-1",
                    div {
                        class: "flex items-center mb-1",
                        h4 {
                            class: "text-sm font-medium text-gray-900 dark:text-white",
                            {format!("版本 {}", version.version_number)}
                        }
                        if is_current {
                            span {
                                class: "ml-2 px-2 py-1 text-xs bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded",
                                "当前版本"
                            }
                        }
                        if is_compare_a {
                            span {
                                class: "ml-2 px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded",
                                "A"
                            }
                        }
                        if is_compare_b {
                            span {
                                class: "ml-2 px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded",
                                "B"
                            }
                        }
                    }
                    
                    if let Some(summary) = &version.change_summary {
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400 mb-1",
                            {summary.clone()}
                        }
                    }
                    
                    div {
                        class: "text-xs text-gray-500 dark:text-gray-400",
                        {format!("{} · {}", format_time(&version.created_at), version.author_name)}
                    }
                }
                
                if !is_current && !comparing {
                    div {
                        class: "ml-4",
                        button {
                            class: "text-sm text-blue-600 dark:text-blue-400 hover:text-blue-500",
                            onclick: {
                                let version_for_restore = version.clone();
                                move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_restore.call(version_for_restore.clone());
                                }
                            },
                            "恢复"
                        }
                    }
                }
            }
        }
    }
}

fn format_time(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*datetime);
    
    if duration.num_seconds() < 60 {
        "刚刚".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} 分钟前", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} 小时前", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} 天前", duration.num_days())
    } else {
        datetime.format("%Y-%m-%d %H:%M").to_string()
    }
}
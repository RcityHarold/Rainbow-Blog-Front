use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::series::SeriesService,
    models::series::{Series, CreateSeriesRequest, UpdateSeriesRequest},
    hooks::use_auth,
    components::ProtectedRoute,
    Route,
};

#[component]
pub fn SeriesManagePage() -> Element {
    rsx! {
        ProtectedRoute {
            SeriesManageContent {}
        }
    }
}

#[component]
fn SeriesManageContent() -> Element {
    let mut series_list = use_signal(|| Vec::<Series>::new());
    let mut loading = use_signal(|| true);
    let mut show_create_modal = use_signal(|| false);
    let mut edit_series = use_signal(|| None::<Series>);
    
    // 加载系列列表
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            if let Ok(series) = SeriesService::get_my_series().await {
                series_list.set(series);
            }
            loading.set(false);
        });
    });
    
    rsx! {
        div {
            class: "min-h-screen bg-white dark:bg-gray-900",
            
            // 导航栏
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
                            to: Route::Home {},
                            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                            "返回首页"
                        }
                    }
                }
            }
            
            // 主要内容
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // 页面标题和操作
                div {
                    class: "flex justify-between items-center mb-8",
                    h1 {
                        class: "text-3xl font-bold text-gray-900 dark:text-white",
                        "我的系列"
                    }
                    
                    button {
                        class: "px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700",
                        onclick: move |_| show_create_modal.set(true),
                        "创建新系列"
                    }
                }
                
                // 系列列表
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if series_list().is_empty() {
                    div {
                        class: "text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg",
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                            }
                        }
                        h3 {
                            class: "mt-2 text-sm font-medium text-gray-900 dark:text-white",
                            "暂无系列"
                        }
                        p {
                            class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            "创建一个系列来组织您的相关文章"
                        }
                    }
                } else {
                    div {
                        class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                        for s in series_list() {
                            SeriesCard {
                                series: s.clone(),
                                on_edit: move |series| edit_series.set(Some(series)),
                                on_delete: move |series_id: String| {
                                    spawn(async move {
                                        if let Ok(_) = SeriesService::delete_series(&series_id).await {
                                            // 重新加载列表
                                            if let Ok(series) = SeriesService::get_my_series().await {
                                                series_list.set(series);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
            
            // 创建/编辑系列模态框
            if show_create_modal() || edit_series().is_some() {
                SeriesModal {
                    series: edit_series(),
                    on_close: move |_| {
                        show_create_modal.set(false);
                        edit_series.set(None);
                    },
                    on_save: move |_| {
                        // 重新加载列表
                        spawn(async move {
                            if let Ok(series) = SeriesService::get_my_series().await {
                                series_list.set(series);
                            }
                        });
                        show_create_modal.set(false);
                        edit_series.set(None);
                    }
                }
            }
        }
    }
}

#[component]
fn SeriesCard(
    series: Series,
    on_edit: EventHandler<Series>,
    on_delete: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6",
            
            div {
                class: "flex justify-between items-start mb-4",
                h3 {
                    class: "text-lg font-semibold text-gray-900 dark:text-white",
                    Link {
                        to: Route::Home {}, // 这里应该链接到系列详情页
                        class: "hover:underline",
                        {series.title.clone()}
                    }
                }
                
                div {
                    class: "flex space-x-2",
                    Link {
                        to: Route::Home {}, // 链接到系列详情页
                        class: "text-sm text-blue-600 hover:text-blue-500",
                        "管理文章"
                    }
                    button {
                        class: "text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white",
                        onclick: move |_| on_edit.call(series.clone()),
                        "编辑"
                    }
                }
            }
            
            if let Some(description) = &series.description {
                p {
                    class: "text-sm text-gray-600 dark:text-gray-400 mb-4 line-clamp-2",
                    {description.clone()}
                }
            }
            
            div {
                class: "flex items-center justify-between text-sm text-gray-500 dark:text-gray-400",
                span {
                    "{series.article_count} 篇文章"
                }
                
                if series.is_completed {
                    span {
                        class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-xs",
                        "已完成"
                    }
                } else {
                    span {
                        class: "px-2 py-1 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded text-xs",
                        "进行中"
                    }
                }
            }
        }
    }
}

#[component]
fn SeriesModal(
    series: Option<Series>,
    on_close: EventHandler<()>,
    on_save: EventHandler<()>,
) -> Element {
    let series = series.clone();
    let mut title = use_signal(|| series.as_ref().map(|s| s.title.clone()).unwrap_or_default());
    let mut description = use_signal(|| series.as_ref().and_then(|s| s.description.clone()).unwrap_or_default());
    let mut is_completed = use_signal(|| series.as_ref().map(|s| s.is_completed).unwrap_or(false));
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    
    let series_for_save = series.clone();
    let series_for_display = series.clone();
    
    let handle_save = move |_: Event<FormData>| {
        saving.set(true);
        error.set(None);
        
        spawn(async move {
            let result = if let Some(s) = series_for_save {
                // 更新现有系列
                let request = UpdateSeriesRequest {
                    title: Some(title()),
                    description: Some(description()),
                    is_completed: Some(is_completed()),
                };
                SeriesService::update_series(&s.id, &request).await
            } else {
                // 创建新系列
                let request = CreateSeriesRequest {
                    title: title(),
                    description: if description().is_empty() { None } else { Some(description()) },
                };
                SeriesService::create_series(&request).await
            };
            
            match result {
                Ok(_) => {
                    on_save.call(());
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            
            saving.set(false);
        });
    };
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            
            // 背景遮罩
            div {
                class: "fixed inset-0 bg-black bg-opacity-50",
                onclick: move |_| on_close.call(())
            }
            
            // 模态框内容
            div {
                class: "relative min-h-screen flex items-center justify-center p-4",
                div {
                    class: "relative bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6",
                    
                    // 标题
                    h2 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                        if series_for_display.is_some() { "编辑系列" } else { "创建新系列" }
                    }
                    
                    // 表单
                    form {
                        onsubmit: move |e| {
                            e.prevent_default();
                            let title = title.clone();
                            let description = description.clone();
                            let is_completed = is_completed.clone();
                            let mut saving = saving.clone();
                            let mut error = error.clone();
                            let navigator = navigator.clone();
                            
                            saving.set(true);
                            error.set(None);
                            
                            let series = series.clone();
                            spawn(async move {
                                let result = if let Some(s) = series {
                                    // 更新现有系列
                                    let request = UpdateSeriesRequest {
                                        title: Some(title()),
                                        description: Some(description()),
                                        is_completed: Some(is_completed()),
                                    };
                                    SeriesService::update_series(&s.id, &request).await
                                } else {
                                    // 创建新系列
                                    let request = CreateSeriesRequest {
                                        title: title(),
                                        description: Some(description()),
                                    };
                                    SeriesService::create_series(&request).await
                                };
                                
                                match result {
                                    Ok(_) => {
                                        navigator().push(Route::SeriesManage {});
                                    }
                                    Err(e) => {
                                        error.set(Some(e.message));
                                    }
                                }
                                
                                saving.set(false);
                            });
                        },
                        
                        // 系列标题
                        div {
                            class: "mb-4",
                            label {
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "系列标题"
                            }
                            input {
                                r#type: "text",
                                value: "{title}",
                                oninput: move |e| title.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                placeholder: "例如：Rust 入门教程",
                                required: true
                            }
                        }
                        
                        // 系列描述
                        div {
                            class: "mb-4",
                            label {
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "系列描述"
                            }
                            textarea {
                                value: "{description}",
                                oninput: move |e| description.set(e.value()),
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                rows: "3",
                                placeholder: "描述这个系列的内容..."
                            }
                        }
                        
                        // 完成状态
                        if series.is_some() {
                            div {
                                class: "mb-4",
                                label {
                                    class: "flex items-center",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_completed(),
                                        onchange: move |e| is_completed.set(e.checked()),
                                        class: "mr-2"
                                    }
                                    span {
                                        class: "text-sm text-gray-700 dark:text-gray-300",
                                        "标记为已完成"
                                    }
                                }
                            }
                        }
                        
                        // 错误提示
                        if let Some(err) = error() {
                            div {
                                class: "mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 rounded",
                                {err}
                            }
                        }
                        
                        // 操作按钮
                        div {
                            class: "flex justify-end space-x-3",
                            button {
                                r#type: "button",
                                class: "px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white",
                                onclick: move |_| on_close.call(()),
                                "取消"
                            }
                            button {
                                r#type: "submit",
                                class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50",
                                disabled: saving() || title().is_empty(),
                                if saving() { "保存中..." } else { "保存" }
                            }
                        }
                    }
                }
            }
        }
    }
}
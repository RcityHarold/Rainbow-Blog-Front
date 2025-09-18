use dioxus::prelude::*;
use futures_util::StreamExt;
use std::rc::Rc;
use dioxus_router::prelude::*;
use crate::{
    api::{articles::ArticleService, tags::TagService},
    components::{ArticleCard, PersonalizedRecommendations, TrendingArticles},
    models::{article::{Article, ArticleListResponse}, tag::Tag},
    hooks::use_auth,
    Route,
};

#[component]
pub fn HomePage() -> Element {
    let mut articles = use_signal(|| Vec::<Article>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut current_page = use_signal(|| 1);
    let mut has_more = use_signal(|| true);
    let mut selected_sort = use_signal(|| "newest");
    let mut tags = use_signal(|| Vec::<Tag>::new());
    let auth = use_auth();
    
    // 加载文章
    use_effect(move || {
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut articles = articles.clone();
        let mut has_more = has_more.clone();
        let selected_sort = selected_sort();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            // 添加调试日志
            web_sys::console::log_1(&"正在请求文章数据...".into());
            
            match ArticleService::get_articles(Some(1), Some(20), Some(selected_sort)).await {
                Ok(response) => {
                    web_sys::console::log_1(&format!("成功获取文章数据: {} 篇", response.articles.len()).into());
                    articles.set(response.articles);
                    has_more.set(response.pagination.has_next);
                }
                Err(e) => {
                    // API不可用时显示错误，但不影响页面结构显示
                    web_sys::console::error_1(&format!("API请求失败: {}", e.message).into());
                    error.set(Some(format!("API请求失败: {}", e.message)));
                    articles.set(vec![]); // 设置空数组，让页面正常显示
                }
            }
            
            loading.set(false);
        });
        
        // 加载热门标签
        let mut tags = tags.clone();
        spawn(async move {
            web_sys::console::log_1(&"正在请求标签数据...".into());
            if let Ok(popular_tags) = TagService::get_popular_tags(Some(20)).await {
                web_sys::console::log_1(&format!("成功获取标签数据: {} 个", popular_tags.len()).into());
                tags.set(popular_tags);
            } else {
                web_sys::console::error_1(&"标签数据请求失败".into());
            }
            // 如果API不可用，使用默认标签
        });
    });
    
    // 处理排序变化
    let handle_sort_change = use_callback({
        let mut selected_sort = selected_sort.clone();
        let mut current_page = current_page.clone();
        let mut articles = articles.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut has_more = has_more.clone();
        
        move |sort: &str| {
            selected_sort.set(sort);
            current_page.set(1);
            articles.set(vec![]);
            
            let sort_str = sort.to_string();
            let mut loading = loading.clone();
            let mut error = error.clone();
            let mut articles = articles.clone();
            let mut has_more = has_more.clone();
            
            spawn(async move {
                loading.set(true);
                error.set(None);
                
                match ArticleService::get_articles(Some(1), Some(20), Some(&sort_str)).await {
                    Ok(response) => {
                        articles.set(response.articles);
                        has_more.set(response.pagination.has_next);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                        articles.set(vec![]);
                    }
                }
                
                loading.set(false);
            });
        }
    });
    
    // 加载更多
    let load_more = {
        let mut current_page = current_page.clone();
        let mut articles = articles.clone();
        let mut loading = loading.clone();
        let mut has_more = has_more.clone();
        let selected_sort = selected_sort.clone();
        
        move |_| {
            let next_page = current_page() + 1;
            current_page.set(next_page);
            
            let sort_str = selected_sort().to_string();
            let mut loading = loading.clone();
            let mut articles = articles.clone();
            let mut has_more = has_more.clone();
            
            spawn(async move {
                loading.set(true);
                
                match ArticleService::get_articles(Some(next_page), Some(20), Some(&sort_str)).await {
                    Ok(response) => {
                        articles.write().extend(response.articles);
                        has_more.set(response.pagination.has_next);
                    }
                    Err(_) => {
                        // 加载更多失败时，恢复has_more状态
                        has_more.set(false);
                    }
                }
                
                loading.set(false);
            });
        }
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 顶部导航栏 - 网站标题
            nav {
                class: "border-b border-gray-100",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                        
                        // 右侧操作按钮
                        div {
                            class: "flex items-center space-x-4",
                            // 出版物入口
                            Link {
                                to: Route::Publications {},
                                class: "text-sm text-gray-700 hover:text-gray-900",
                                "出版物"
                            }
                            if auth.read().is_authenticated {
                                Link {
                                    to: Route::Write {},
                                    class: "text-sm text-gray-700 hover:text-gray-900",
                                    "写文章"
                                }
                                Link {
                                    to: Route::ProfileById { user_id: auth.read().user.as_ref().map(|u| u.id.clone()).unwrap_or_default() },
                                    class: "text-sm text-gray-700 hover:text-gray-900",
                                    "个人主页"
                                }
                            } else {
                                Link {
                                    to: Route::Login {},
                                    class: "text-sm text-gray-700 hover:text-gray-900",
                                    "登录"
                                }
                                Link {
                                    to: Route::Register {},
                                    class: "text-sm bg-gray-900 text-white px-4 py-2 rounded-full hover:bg-gray-800",
                                    "注册"
                                }
                            }
                        }
                    }
                }
            }
            
            // 排序选项导航栏
            nav {
                class: "border-b border-gray-100 sticky top-16 bg-white z-10",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-14",
                        
                        // 排序选项
                        div {
                            class: "flex items-center space-x-4 sm:space-x-8 overflow-x-auto",
                            button {
                                class: if selected_sort() == "newest" { 
                                    "text-sm font-medium text-gray-900 border-b-2 border-gray-900 pb-4 whitespace-nowrap" 
                                } else { 
                                    "text-sm text-gray-500 hover:text-gray-900 pb-4 whitespace-nowrap" 
                                },
                                onclick: {
                                    let handle_sort_change = handle_sort_change.clone();
                                    move |_| handle_sort_change("newest")
                                },
                                "最新"
                            }
                            button {
                                class: if selected_sort() == "trending" { 
                                    "text-sm font-medium text-gray-900 border-b-2 border-gray-900 pb-4 whitespace-nowrap" 
                                } else { 
                                    "text-sm text-gray-500 hover:text-gray-900 pb-4 whitespace-nowrap" 
                                },
                                onclick: {
                                    let handle_sort_change = handle_sort_change.clone();
                                    move |_| handle_sort_change("trending")
                                },
                                "热门"
                            }
                            button {
                                class: if selected_sort() == "popular" { 
                                    "text-sm font-medium text-gray-900 border-b-2 border-gray-900 pb-4 whitespace-nowrap" 
                                } else { 
                                    "text-sm text-gray-500 hover:text-gray-900 pb-4 whitespace-nowrap" 
                                },
                                onclick: {
                                    let handle_sort_change = handle_sort_change.clone();
                                    move |_| handle_sort_change("popular")
                                },
                                "精选"
                            }
                            if auth.read().is_authenticated {
                                button {
                                    class: if selected_sort() == "recommended" { 
                                        "text-sm font-medium text-gray-900 border-b-2 border-gray-900 pb-4 whitespace-nowrap" 
                                    } else { 
                                        "text-sm text-gray-500 hover:text-gray-900 pb-4 whitespace-nowrap" 
                                    },
                                    onclick: {
                                    let handle_sort_change = handle_sort_change.clone();
                                    move |_| handle_sort_change("recommended")
                                },
                                    "为您推荐"
                                }
                            }
                        }
                    }
                }
            }
            
            // 主要内容区域
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8",
                div {
                    class: "lg:flex lg:gap-8",
                    
                    // 文章列表
                    main {
                        class: "flex-1 lg:max-w-3xl",
                        
                        // 错误信息
                        if let Some(err) = error() {
                            div {
                                class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mt-4",
                                {err}
                            }
                        }
                        
                        // 文章列表或推荐内容
                        if selected_sort() == "recommended" && auth.read().is_authenticated {
                            PersonalizedRecommendations {}
                        } else {
                            div {
                                class: "divide-y divide-gray-100",
                                for article in articles() {
                                    ArticleCard { article }
                                }
                            }
                        }
                        
                        // 加载状态
                        if loading() && current_page() == 1 {
                            div {
                                class: "py-8 text-center",
                                div {
                                    class: "inline-flex items-center",
                                    div {
                                        class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                                    }
                                    span {
                                        class: "ml-2 text-gray-600",
                                        "加载中..."
                                    }
                                }
                            }
                        }
                        
                        // 加载更多按钮
                        if !loading() && has_more() && !articles().is_empty() {
                            div {
                                class: "py-8 text-center",
                                button {
                                    class: "px-6 py-2 bg-gray-900 text-white rounded-full hover:bg-gray-800 transition-colors",
                                    onclick: load_more,
                                    if loading() && current_page() > 1 {
                                        "加载中..."
                                    } else {
                                        "加载更多"
                                    }
                                }
                            }
                        }
                        
                        // 没有更多内容
                        if !has_more() && !articles().is_empty() {
                            div {
                                class: "py-8 text-center text-gray-500",
                                "没有更多文章了"
                            }
                        }
                        
                        // 空状态
                        if !loading() && articles().is_empty() {
                            div {
                                class: "py-16 text-center",
                                svg {
                                    class: "mx-auto h-12 w-12 text-gray-400",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                    }
                                }
                                h3 {
                                    class: "mt-2 text-sm font-medium text-gray-900",
                                    "暂无文章"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-500",
                                    "请稍后再来查看"
                                }
                            }
                        }
                    }
                    
                    // 侧边栏
                    aside {
                        class: "hidden lg:block w-80",
                        div {
                            class: "sticky top-20",
                            
                            // 热门标签
                            div {
                                class: "mb-8",
                                h2 {
                                    class: "text-sm font-bold text-gray-900 uppercase tracking-wide mb-4",
                                    "热门标签"
                                }
                                div {
                                    class: "flex flex-wrap gap-2",
                                    if tags().is_empty() {
                                        for tag in ["Rust", "编程", "技术", "教程", "开源"].iter() {
                                            button {
                                                class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm hover:bg-gray-200 transition-colors",
                                                {tag}
                                            }
                                        }
                                    } else {
                                        {
                                            let tags = tags();
                                            let tags_slice = tags.iter().take(10).cloned().collect::<Vec<_>>();
                                            rsx! {
                                                for tag in tags_slice {
                                                    button {
                                                        class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm hover:bg-gray-200 transition-colors",
                                                        onclick: move |_| {
                                                            web_sys::window()
                                                                .unwrap()
                                                                .location()
                                                                .set_href(&format!("/tag/{}", tag.slug))
                                                                .ok();
                                                        },
                                                        {tag.name.clone()}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Link {
                                    to: Route::Tags {},
                                    class: "text-sm text-green-600 hover:text-green-700 mt-3 inline-block",
                                    "查看所有标签 →"
                                }
                            }
                            
                            // 热门文章
                            div {
                                class: "mb-8",
                                TrendingArticles { period: "week".to_string() }
                            }
                            
                            // 推荐作者
                            div {
                                h2 {
                                    class: "text-sm font-bold text-gray-900 uppercase tracking-wide mb-4",
                                    "推荐作者"
                                }
                                div {
                                    class: "space-y-3",
                                    for i in 0..3 {
                                        div {
                                            class: "flex items-center",
                                            div {
                                                class: "w-10 h-10 bg-gray-200 rounded-full mr-3"
                                            }
                                            div {
                                                class: "flex-1",
                                                div {
                                                    class: "text-sm font-medium text-gray-900",
                                                    "作者 {i + 1}"
                                                }
                                                div {
                                                    class: "text-xs text-gray-500",
                                                    "技术博主"
                                                }
                                            }
                                            button {
                                                class: "text-sm text-green-600 hover:text-green-700",
                                                "关注"
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
    }
}

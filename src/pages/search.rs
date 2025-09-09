use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::search::{SearchService, SearchArticlesRequest, SearchSuggestionsRequest, SearchAllRequest},
    components::ArticleCard,
    models::{article::Article, user::User, tag::Tag},
    Route,
};
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn SearchPage() -> Element {
    let mut query = use_signal(|| String::new());
    let mut search_results = use_signal(|| Vec::<Article>::new());
    let mut user_results = use_signal(|| Vec::<User>::new());
    let mut tag_results = use_signal(|| Vec::<Tag>::new());
    let mut suggestions = use_signal(|| Vec::<String>::new());
    let mut loading = use_signal(|| false);
    let mut show_suggestions = use_signal(|| false);
    let mut active_tab = use_signal(|| "all");
    let mut total_results = use_signal(|| 0);
    let mut current_page = use_signal(|| 1);
    
    // 获取URL参数中的查询
    let route = use_route::<Route>();
    let url_query = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|search| {
            if search.starts_with("?q=") {
                Some(search[3..].to_string())
            } else {
                None
            }
        });
    
    // 初始化搜索
    use_effect(move || {
        if let Some(q) = url_query.clone() {
            let decoded_query = urlencoding::decode(&q).unwrap_or_default().to_string();
            query.set(decoded_query.clone());
            
            spawn(async move {
                if decoded_query.is_empty() {
                    return;
                }
                
                loading.set(true);
                show_suggestions.set(false);
                
                let request = SearchAllRequest {
                    query: decoded_query.clone(),
                    types: None,
                    limit: Some(50),
                };
                
                match SearchService::search_all(request).await {
                    Ok(response) => {
                        let total = response.articles.len() + response.users.len() + response.tags.len();
                        search_results.set(response.articles);
                        user_results.set(response.users);
                        tag_results.set(response.tags);
                        total_results.set(total);
                    }
                    Err(_) => {
                        // 处理错误
                    }
                }
                
                loading.set(false);
            });
        }
    });
    
    // 获取搜索建议
    let get_suggestions = move |search_query: String| {
        spawn(async move {
            if search_query.len() < 2 {
                suggestions.set(vec![]);
                show_suggestions.set(false);
                return;
            }
            
            // 延迟300ms，避免频繁请求
            TimeoutFuture::new(300).await;
            
            let request = SearchSuggestionsRequest {
                query: search_query,
                limit: Some(5),
            };
            
            if let Ok(response) = SearchService::get_suggestions(request).await {
                let suggestion_texts: Vec<String> = response.suggestions
                    .into_iter()
                    .map(|s| s.text)
                    .collect();
                suggestions.set(suggestion_texts);
                show_suggestions.set(true);
            }
        });
    };
    
    // 处理搜索提交
    let handle_search = move |e: Event<FormData>| {
        e.prevent_default();
        let search_query = query();
        
        if !search_query.is_empty() {
            // 更新URL
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&format!("/search?q={}", urlencoding::encode(&search_query)))
                .ok();
        }
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 导航栏
            nav {
                class: "border-b border-gray-100 bg-white sticky top-0 z-50",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                        
                        // 搜索框
                        div {
                            class: "flex-1 max-w-2xl mx-8 relative",
                            form {
                                onsubmit: handle_search,
                                class: "relative",
                                input {
                                    r#type: "text",
                                    value: "{query}",
                                    oninput: move |e| {
                                        query.set(e.value());
                                        get_suggestions(e.value());
                                    },
                                    onfocus: move |_| show_suggestions.set(true),
                                    placeholder: "搜索文章、用户、标签...",
                                    class: "w-full px-4 py-2 border border-gray-300 rounded-full focus:outline-none focus:border-gray-500"
                                }
                                button {
                                    r#type: "submit",
                                    class: "absolute right-2 top-1/2 -translate-y-1/2 p-2",
                                    svg {
                                        class: "w-5 h-5 text-gray-500",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                        }
                                    }
                                }
                            }
                            
                            // 搜索建议下拉
                            if show_suggestions() && !suggestions().is_empty() {
                                div {
                                    class: "absolute top-full left-0 right-0 mt-1 bg-white border border-gray-200 rounded-lg shadow-lg z-50",
                                    onmouseleave: move |_| show_suggestions.set(false),
                                    for suggestion in suggestions() {
                                        button {
                                            class: "w-full px-4 py-2 text-left hover:bg-gray-50",
                                            onclick: move |_| {
                                                let suggestion = suggestion.clone();
                                                let mut query = query.clone();
                                                let mut show_suggestions = show_suggestions.clone();
                                                let mut loading = loading.clone();
                                                let mut search_results = search_results.clone();
                                                let mut user_results = user_results.clone();
                                                let mut tag_results = tag_results.clone();
                                                let mut total_results = total_results.clone();
                                                
                                                query.set(suggestion.clone());
                                                show_suggestions.set(false);
                                                spawn(async move {
                                                    query.set(suggestion.clone());
                                                    
                                                    loading.set(true);
                                                    show_suggestions.set(false);
                                                    
                                                    let request = SearchAllRequest {
                                                        query: suggestion,
                                                        types: None,
                                                        limit: Some(50),
                                                    };
                                                    
                                                    match SearchService::search_all(request).await {
                                                        Ok(response) => {
                                                            let total = response.articles.len() + response.users.len() + response.tags.len();
                                                            search_results.set(response.articles);
                                                            user_results.set(response.users);
                                                            tag_results.set(response.tags);
                                                            total_results.set(total);
                                                        }
                                                        Err(_) => {
                                                            // 处理错误
                                                        }
                                                    }
                                                    
                                                    loading.set(false);
                                                });
                                            },
                                            {suggestion.clone()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 主要内容
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                if query().is_empty() {
                    // 热门搜索
                    div {
                        class: "text-center py-12",
                        h2 {
                            class: "text-2xl font-semibold mb-8",
                            "探索内容"
                        }
                        
                        // 热门标签
                        div {
                            class: "mb-12",
                            h3 {
                                class: "text-lg font-medium mb-4",
                                "热门标签"
                            }
                            div {
                                class: "flex flex-wrap justify-center gap-3",
                                // 这里可以加载热门标签
                            }
                        }
                    }
                } else {
                    // 搜索结果头部
                    div {
                        class: "mb-6",
                        h1 {
                            class: "text-2xl font-semibold",
                            "搜索结果: \"{query}\""
                        }
                        p {
                            class: "text-gray-600 mt-2",
                            "找到 {total_results} 个结果"
                        }
                    }
                    
                    // 标签页导航
                    div {
                        class: "border-b border-gray-200 mb-6",
                        div {
                            class: "flex space-x-8",
                            button {
                                class: if active_tab() == "all" { 
                                    "py-4 px-1 border-b-2 border-gray-900 font-medium text-sm text-gray-900" 
                                } else { 
                                    "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300" 
                                },
                                onclick: move |_| active_tab.set("all"),
                                "全部 ({total_results})"
                            }
                            button {
                                class: if active_tab() == "articles" { 
                                    "py-4 px-1 border-b-2 border-gray-900 font-medium text-sm text-gray-900" 
                                } else { 
                                    "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300" 
                                },
                                onclick: move |_| active_tab.set("articles"),
                                "文章 ({search_results().len()})"
                            }
                            button {
                                class: if active_tab() == "users" { 
                                    "py-4 px-1 border-b-2 border-gray-900 font-medium text-sm text-gray-900" 
                                } else { 
                                    "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300" 
                                },
                                onclick: move |_| active_tab.set("users"),
                                "用户 ({user_results().len()})"
                            }
                            button {
                                class: if active_tab() == "tags" { 
                                    "py-4 px-1 border-b-2 border-gray-900 font-medium text-sm text-gray-900" 
                                } else { 
                                    "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300" 
                                },
                                onclick: move |_| active_tab.set("tags"),
                                "标签 ({tag_results().len()})"
                            }
                        }
                    }
                    
                    // 搜索结果内容
                    if loading() {
                        div {
                            class: "flex justify-center py-12",
                            div {
                                class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                            }
                        }
                    } else {
                        div {
                            // 全部结果
                            if active_tab() == "all" {
                                div {
                                    class: "space-y-8",
                                    
                                    // 文章结果
                                    if !search_results().is_empty() {
                                        div {
                                            h3 {
                                                class: "text-lg font-semibold mb-4",
                                                "文章"
                                            }
                                            div {
                                                class: "space-y-0",
                                                for article in search_results().iter().take(5) {
                                                    ArticleCard { article: article.clone() }
                                                }
                                            }
                                            if search_results().len() > 5 {
                                                button {
                                                    class: "text-gray-600 hover:text-gray-900 mt-4",
                                                    onclick: move |_| active_tab.set("articles"),
                                                    "查看所有 {search_results().len()} 篇文章 →"
                                                }
                                            }
                                        }
                                    }
                                    
                                    // 用户结果
                                    if !user_results().is_empty() {
                                        div {
                                            h3 {
                                                class: "text-lg font-semibold mb-4",
                                                "用户"
                                            }
                                            div {
                                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                                {
                                                    let users = user_results();
                                                    let users_slice = users.iter().take(4).cloned().collect::<Vec<_>>();
                                                    rsx! {
                                                        for user in users_slice {
                                                            div {
                                                                class: "flex items-center p-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow cursor-pointer",
                                                                onclick: move |_| {
                                                                    web_sys::window()
                                                                        .unwrap()
                                                                        .location()
                                                                        .set_href(&format!("/@{}", user.username))
                                                                        .ok();
                                                                },
                                                                
                                                                {
                                                                    if let Some(avatar) = &user.avatar_url {
                                                                        rsx! {
                                                                            img {
                                                                                src: "{avatar}",
                                                                                alt: "{user.username}",
                                                                                class: "w-12 h-12 rounded-full mr-4"
                                                                            }
                                                                        }
                                                                    } else {
                                                                        rsx! {
                                                                            div {
                                                                                class: "w-12 h-12 rounded-full mr-4 bg-gray-200 dark:bg-gray-600"
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                
                                                                div {
                                                                    h4 {
                                                                        class: "font-medium",
                                                                        {user.display_name.clone().unwrap_or(user.username.clone())}
                                                                    }
                                                                    {
                                                                        if let Some(bio) = &user.bio {
                                                                            rsx! {
                                                                                p {
                                                                                    class: "text-sm text-gray-600 line-clamp-2",
                                                                                    {bio.clone()}
                                                                                }
                                                                            }
                                                                        } else {
                                                                            rsx! {}
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
                                    
                                    // 标签结果
                                    if !tag_results().is_empty() {
                                        div {
                                            h3 {
                                                class: "text-lg font-semibold mb-4",
                                                "标签"
                                            }
                                            div {
                                                class: "flex flex-wrap gap-3",
                                                for tag in tag_results().iter().take(10) {
                                                    button {
                                                        class: "px-4 py-2 bg-gray-100 text-gray-700 rounded-full hover:bg-gray-200",
                                                        onclick: move |_| {
                                                            // 导航到标签页面
                                                        },
                                                        "{tag.name} ({tag.article_count})"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 文章标签页
                            if active_tab() == "articles" {
                                if search_results().is_empty() {
                                    div {
                                        class: "text-center py-12",
                                        p {
                                            class: "text-gray-500",
                                            "没有找到相关文章"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "space-y-0",
                                        for article in search_results() {
                                            ArticleCard { article: article.clone() }
                                        }
                                    }
                                }
                            }
                            
                            // 用户标签页
                            if active_tab() == "users" {
                                if user_results().is_empty() {
                                    div {
                                        class: "text-center py-12",
                                        p {
                                            class: "text-gray-500",
                                            "没有找到相关用户"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                        for user in user_results() {
                                            div {
                                                class: "flex items-center p-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow cursor-pointer",
                                                onclick: move |_| {
                                                    web_sys::window()
                                                        .unwrap()
                                                        .location()
                                                        .set_href(&format!("/@{}", user.username))
                                                        .ok();
                                                },
                                                if let Some(avatar) = &user.avatar_url {
                                                    img {
                                                        src: "{avatar}",
                                                        alt: "{user.username}",
                                                        class: "w-16 h-16 rounded-full mr-4"
                                                    }
                                                }
                                                div {
                                                    class: "flex-1",
                                                    h4 {
                                                        class: "font-medium text-lg",
                                                        {user.display_name.clone().unwrap_or(user.username.clone())}
                                                    }
                                                    p {
                                                        class: "text-sm text-gray-500",
                                                        "@{user.username}"
                                                    }
                                                    if let Some(bio) = &user.bio {
                                                        p {
                                                            class: "text-sm text-gray-600 mt-1 line-clamp-2",
                                                            {bio.clone()}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 标签标签页
                            if active_tab() == "tags" {
                                if tag_results().is_empty() {
                                    div {
                                        class: "text-center py-12",
                                        p {
                                            class: "text-gray-500",
                                            "没有找到相关标签"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                                        for tag in tag_results() {
                                            div {
                                                class: "p-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow cursor-pointer",
                                                onclick: move |_| {
                                                    // 导航到标签页面
                                                },
                                                h4 {
                                                    class: "font-medium text-lg mb-1",
                                                    {tag.name.clone()}
                                                }
                                                p {
                                                    class: "text-sm text-gray-600",
                                                    "{tag.article_count} 篇文章"
                                                }
                                                if let Some(desc) = &tag.description {
                                                    p {
                                                        class: "text-sm text-gray-500 mt-2 line-clamp-2",
                                                        {desc.clone()}
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
    }
}
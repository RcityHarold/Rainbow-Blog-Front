use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::{series::SeriesService, articles::ArticleService},
    models::{
        series::{SeriesWithArticles, SeriesArticle},
        article::Article,
    },
    hooks::use_auth,
    Route,
};

#[component]
pub fn SeriesDetailPage(slug: String) -> Element {
    let mut series_data = use_signal(|| None::<SeriesWithArticles>);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut show_add_article = use_signal(|| false);
    let mut available_articles = use_signal(|| Vec::<Article>::new());
    let mut is_owner = use_signal(|| false);
    
    let auth = use_auth();
    let slug_for_effect = slug.clone();
    let slug_for_add = slug.clone();
    let slug_for_remove = slug.clone();
    
    // 加载系列详情
    use_effect(move || {
        let slug = slug_for_effect.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            match SeriesService::get_series_by_slug(&slug).await {
                Ok(data) => {
                    // 检查是否为系列所有者
                    if let Some(user) = &auth.read().user {
                        is_owner.set(data.series.author_id == user.id);
                    }
                    series_data.set(Some(data));
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            
            loading.set(false);
        });
    });
    
    // 加载可添加的文章
    let load_available_articles = move || {
        if let Some(data) = series_data() {
            spawn(async move {
                // 获取用户的所有文章
                if let Ok(response) = ArticleService::get_articles(Some(1), Some(100), None).await {
                    // 过滤掉已在系列中的文章
                    let series_article_ids: Vec<String> = data.articles
                        .iter()
                        .map(|sa| sa.article.id.clone())
                        .collect();
                    
                    let available: Vec<Article> = response.articles
                        .into_iter()
                        .filter(|a| !series_article_ids.contains(&a.id))
                        .collect();
                    
                    available_articles.set(available);
                }
            });
        }
    };
    
    // 添加文章到系列
    let add_article_to_series = move |article_id: String| {
        if let Some(data) = series_data() {
            let series_id = data.series.id.clone();
            let next_order = data.articles.len() as i32 + 1;
            let slug = slug_for_add.clone();
            
            spawn(async move {
                if let Ok(_) = SeriesService::add_article_to_series(&series_id, &article_id, next_order).await {
                    // 重新加载系列数据
                    if let Ok(updated_data) = SeriesService::get_series_by_slug(&slug).await {
                        series_data.set(Some(updated_data));
                    }
                    show_add_article.set(false);
                }
            });
        }
    };
    
    // 从系列中移除文章
    let remove_article = move |article_id: String| {
        if let Some(data) = series_data() {
            let series_id = data.series.id.clone();
            let slug = slug_for_remove.clone();
            
            spawn(async move {
                if let Ok(_) = SeriesService::remove_article_from_series(&series_id, &article_id).await {
                    // 重新加载系列数据
                    if let Ok(updated_data) = SeriesService::get_series_by_slug(&slug).await {
                        series_data.set(Some(updated_data));
                    }
                }
            });
        }
    };
    
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
                        
                        if is_owner() {
                            Link {
                                to: Route::Home {}, // 应该链接到系列管理页
                                class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                "返回系列管理"
                            }
                        }
                    }
                }
            }
            
            // 主要内容
            if loading() {
                div {
                    class: "flex justify-center py-12",
                    div {
                        class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                    }
                }
            } else if let Some(err) = error() {
                div {
                    class: "max-w-7xl mx-auto px-4 py-8",
                    div {
                        class: "bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded",
                        {err}
                    }
                }
            } else if let Some(data) = series_data() {
                div {
                    // 系列头部信息
                    div {
                        class: "bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
                        div {
                            class: "max-w-7xl mx-auto px-4 py-8",
                            div {
                                class: "flex justify-between items-start",
                                div {
                                    h1 {
                                        class: "text-3xl font-bold text-gray-900 dark:text-white mb-2",
                                        {data.series.title.clone()}
                                    }
                                    
                                    if let Some(description) = &data.series.description {
                                        p {
                                            class: "text-lg text-gray-600 dark:text-gray-400 mb-4",
                                            {description.clone()}
                                        }
                                    }
                                    
                                    div {
                                        class: "flex items-center space-x-4 text-sm text-gray-500 dark:text-gray-400",
                                        span {
                                            "{data.series.article_count} 篇文章"
                                        }
                                        
                                        if data.series.is_completed {
                                            span {
                                                class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded",
                                                "已完成"
                                            }
                                        } else {
                                            span {
                                                class: "px-2 py-1 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded",
                                                "进行中"
                                            }
                                        }
                                    }
                                }
                                
                                // 操作按钮
                                if is_owner() {
                                    button {
                                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                                        onclick: move |_| {
                                            load_available_articles();
                                            show_add_article.set(true);
                                        },
                                        "添加文章"
                                    }
                                }
                            }
                        }
                    }
                    
                    // 文章列表
                    div {
                        class: "max-w-7xl mx-auto px-4 py-8",
                        
                        if data.articles.is_empty() {
                            div {
                                class: "text-center py-12",
                                p {
                                    class: "text-gray-500 dark:text-gray-400",
                                    "该系列暂无文章"
                                }
                            }
                        } else {
                            div {
                                class: "space-y-6",
                                for (index, series_article) in data.articles.iter().enumerate() {
                                    SeriesArticleItem {
                                        series_article: series_article.clone(),
                                        index: index + 1,
                                        is_owner: is_owner(),
                                        on_remove: {
                                            let article_id = series_article.article.id.clone();
                                            let series_data = series_data.clone();
                                            let slug = slug.clone();
                                            move |_| {
                                                if let Some(data) = series_data() {
                                                    let series_id = data.series.id.clone();
                                                    let article_id = article_id.clone();
                                                    let slug = slug.clone();
                                                    let mut series_data = series_data.clone();
                                                    
                                                    spawn(async move {
                                                        if let Ok(_) = SeriesService::remove_article_from_series(&series_id, &article_id).await {
                                                            // 重新加载系列数据
                                                            if let Ok(updated_data) = SeriesService::get_series_by_slug(&slug).await {
                                                                series_data.set(Some(updated_data));
                                                            }
                                                        }
                                                    });
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
            
            // 添加文章模态框
            if show_add_article() {
                AddArticleModal {
                    articles: available_articles(),
                    on_close: move |_| show_add_article.set(false),
                    on_add: {
                        let series_data = series_data.clone();
                        let slug = slug.clone();
                        let show_add_article = show_add_article.clone();
                        move |article_id: String| {
                            if let Some(data) = series_data() {
                                let series_id = data.series.id.clone();
                                let next_order = data.articles.len() as i32 + 1;
                                let slug = slug.clone();
                                let mut series_data = series_data.clone();
                                let mut show_add_article = show_add_article.clone();
                                
                                spawn(async move {
                                    if let Ok(_) = SeriesService::add_article_to_series(&series_id, &article_id, next_order).await {
                                        // 重新加载系列数据
                                        if let Ok(updated_data) = SeriesService::get_series_by_slug(&slug).await {
                                            series_data.set(Some(updated_data));
                                        }
                                        show_add_article.set(false);
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SeriesArticleItem(
    series_article: SeriesArticle,
    index: usize,
    is_owner: bool,
    on_remove: EventHandler<String>,
) -> Element {
    let article = series_article.article.clone();
    
    rsx! {
        div {
            class: "flex items-start space-x-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow-sm",
            
            // 序号
            div {
                class: "flex-shrink-0 w-10 h-10 bg-gray-100 dark:bg-gray-700 rounded-full flex items-center justify-center",
                span {
                    class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                    "{index}"
                }
            }
            
            // 文章信息
            div {
                class: "flex-1",
                h3 {
                    class: "text-lg font-medium text-gray-900 dark:text-white mb-1",
                    Link {
                        to: Route::Article { slug: article.slug.clone() },
                        class: "hover:underline",
                        {article.title.clone()}
                    }
                }
                
                if let Some(subtitle) = &article.subtitle {
                    p {
                        class: "text-gray-600 dark:text-gray-400 mb-2",
                        {subtitle.clone()}
                    }
                }
                
                div {
                    class: "flex items-center text-sm text-gray-500 dark:text-gray-400",
                    span {
                        "{article.reading_time} 分钟阅读"
                    }
                    span {
                        class: "mx-2",
                        "·"
                    }
                    span {
                        {article.published_at.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or("草稿".to_string())}
                    }
                }
            }
            
            // 操作按钮
            if is_owner {
                div {
                    class: "flex-shrink-0",
                    button {
                        class: "text-red-600 hover:text-red-500 text-sm",
                        onclick: move |_| on_remove.call(article.id.clone()),
                        "移除"
                    }
                }
            }
        }
    }
}

#[component]
fn AddArticleModal(
    articles: Vec<Article>,
    on_close: EventHandler<()>,
    on_add: EventHandler<String>,
) -> Element {
    let mut search_query = use_signal(|| String::new());
    
    let filtered_articles = use_memo(move || {
        let query = search_query().to_lowercase();
        if query.is_empty() {
            articles.clone()
        } else {
            articles.iter()
                .filter(|a| a.title.to_lowercase().contains(&query))
                .cloned()
                .collect::<Vec<_>>()
        }
    });
    
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
                    class: "relative bg-white dark:bg-gray-800 rounded-lg max-w-2xl w-full max-h-[80vh] flex flex-col",
                    
                    // 标题
                    div {
                        class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                        h2 {
                            class: "text-xl font-semibold text-gray-900 dark:text-white",
                            "添加文章到系列"
                        }
                    }
                    
                    // 搜索框
                    div {
                        class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                        input {
                            r#type: "search",
                            placeholder: "搜索文章...",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                        }
                    }
                    
                    // 文章列表
                    div {
                        class: "flex-1 overflow-y-auto px-6 py-4",
                        
                        if filtered_articles().is_empty() {
                            div {
                                class: "text-center py-8",
                                p {
                                    class: "text-gray-500 dark:text-gray-400",
                                    "没有可添加的文章"
                                }
                            }
                        } else {
                            div {
                                class: "space-y-3",
                                for article in filtered_articles() {
                                    button {
                                        class: "w-full text-left p-3 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700",
                                        onclick: move |_| on_add.call(article.id.clone()),
                                        
                                        h4 {
                                            class: "font-medium text-gray-900 dark:text-white mb-1",
                                            {article.title.clone()}
                                        }
                                        
                                        if !article.excerpt.is_empty() {
                                            p {
                                                class: "text-sm text-gray-600 dark:text-gray-400 line-clamp-2",
                                                {article.excerpt.clone()}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 底部按钮
                    div {
                        class: "px-6 py-4 border-t border-gray-200 dark:border-gray-700",
                        button {
                            class: "w-full px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white",
                            onclick: move |_| on_close.call(()),
                            "取消"
                        }
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::recommendations::{RecommendationService, RecommendationParams, RecommendedArticle, TrendingArticle},
    components::ArticleCard,
    hooks::use_auth,
    Route,
};

#[component]
pub fn PersonalizedRecommendations() -> Element {
    let mut recommendations = use_signal(|| Vec::<RecommendedArticle>::new());
    let mut loading = use_signal(|| true);
    let mut algorithm = use_signal(|| "hybrid".to_string());
    let auth = use_auth();
    
    // 加载推荐
    let load_recommendations = move || {
        spawn(async move {
            loading.set(true);
            
            let params = RecommendationParams {
                user_id: auth.read().user.as_ref().map(|u| u.id.clone()),
                limit: Some(10),
                algorithm: Some(algorithm()),
                exclude_read: Some(true),
                tags: None,
                authors: None,
            };
            
            match RecommendationService::get_recommendations(&params).await {
                Ok(response) => {
                    recommendations.set(response.articles);
                }
                Err(_) => {
                    // 错误处理
                }
            }
            
            loading.set(false);
        });
    };
    
    // 初始加载
    use_effect(move || {
        load_recommendations();
    });
    
    // 算法变化时重新加载
    use_effect(move || {
        load_recommendations();
    });
    
    rsx! {
        div {
            class: "py-8",
            
            // 标题和算法选择
            div {
                class: "flex items-center justify-between mb-6",
                h2 {
                    class: "text-2xl font-bold text-gray-900 dark:text-white",
                    "为您推荐"
                }
                
                // 算法选择器
                select {
                    class: "px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                    value: "{algorithm}",
                    onchange: move |e| algorithm.set(e.value()),
                    
                    option { value: "hybrid", "智能推荐" }
                    option { value: "content_based", "基于内容" }
                    option { value: "collaborative_filtering", "协同过滤" }
                    option { value: "trending", "热门文章" }
                    option { value: "following", "关注的人" }
                }
            }
            
            if loading() {
                div {
                    class: "flex justify-center py-8",
                    div {
                        class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                    }
                }
            } else if recommendations.read().is_empty() {
                div {
                    class: "text-center py-12",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "暂无推荐内容"
                    }
                    if !auth.read().is_authenticated {
                        Link {
                            to: Route::Login {},
                            class: "mt-4 inline-block text-blue-600 hover:text-blue-700",
                            "登录以获得个性化推荐"
                        }
                    }
                }
            } else {
                div {
                    class: "grid gap-6",
                    for rec in recommendations.read().iter().cloned() {
                        div {
                            class: "relative",
                            
                            // 推荐原因
                            if !rec.reason.is_empty() {
                                div {
                                    class: "mb-2 text-sm text-gray-600 dark:text-gray-400 flex items-center",
                                    svg {
                                        class: "w-4 h-4 mr-1",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
                                        }
                                    }
                                    {rec.reason}
                                }
                            }
                            
                            ArticleCard {
                                article: rec.article.clone()
                            }
                            
                            // 推荐分数指示器（调试用，可以隐藏）
                            if false {
                                div {
                                    class: "absolute top-2 right-2 bg-blue-600 text-white text-xs px-2 py-1 rounded",
                                    "Score: {rec.score:.1}"
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
pub fn TrendingArticles(period: String) -> Element {
    let period = period.clone();
    let mut articles = use_signal(|| Vec::<TrendingArticle>::new());
    let mut loading = use_signal(|| true);
    
    use_effect({
        let period = period.clone();
        let mut articles = articles.clone();
        let mut loading = loading.clone();
        move || {
            let period_clone = period.clone();
            let mut articles = articles.clone();
            let mut loading = loading.clone();
            spawn(async move {
                loading.set(true);
                
                match RecommendationService::get_trending(&period_clone, Some(5)).await {
                    Ok(response) => {
                        articles.set(response.articles);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to load trending articles: {:?}", e).into());
                    }
                }
                
                loading.set(false);
            });
        }
    });
    
    rsx! {
        div {
            class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-6",
            
            h3 {
                class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                match period.as_str() {
                    "today" => "今日热门",
                    "week" => "本周热门",
                    "month" => "本月热门",
                    _ => "热门文章"
                }
            }
            
            if loading() {
                div {
                    class: "animate-pulse space-y-3",
                    for _ in 0..3 {
                        div {
                            class: "h-4 bg-gray-200 dark:bg-gray-700 rounded"
                        }
                    }
                }
            } else {
                div {
                    class: "space-y-3",
                    for (i, article) in articles().iter().enumerate() {
                        Link {
                            to: Route::Article { slug: article.slug.clone() },
                            class: "block group",
                            div {
                                class: "flex items-start",
                                span {
                                    class: "text-2xl font-serif text-gray-300 dark:text-gray-600 mr-3",
                                    "{i + 1}"
                                }
                                div {
                                    h4 {
                                        class: "font-medium text-gray-900 dark:text-white group-hover:underline",
                                        {article.title.clone()}
                                    }
                                    div {
                                        class: "flex items-center mt-1 text-xs text-gray-500",
                                        span {
                                            {article.author.display_name.clone()}
                                        }
                                        span { class: "mx-1", "·" }
                                        span {
                                            "{article.reading_time} 分钟"
                                        }
                                        if article.clap_count > 0 {
                                            span { class: "mx-1", "·" }
                                            span {
                                                "{article.clap_count} 👏"
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

#[component]
pub fn RelatedArticles(article_id: String) -> Element {
    let mut articles = use_signal(|| Vec::<crate::models::article::Article>::new());
    let mut loading = use_signal(|| true);
    
    use_effect(move || {
        let id = article_id.clone();
        spawn(async move {
            loading.set(true);
            
            match RecommendationService::get_content_based(&id, Some(4)).await {
                Ok(related) => {
                    articles.set(related);
                }
                Err(_) => {
                    // 错误处理
                }
            }
            
            loading.set(false);
        });
    });
    
    if articles().is_empty() {
        return rsx! {};
    }
    
    rsx! {
        div {
            class: "mt-12 pt-12 border-t border-gray-200 dark:border-gray-700",
            
            h3 {
                class: "text-xl font-semibold text-gray-900 dark:text-white mb-6",
                "相关文章"
            }
            
            if loading() {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    for _ in 0..4 {
                        div {
                            class: "animate-pulse",
                            div {
                                class: "h-40 bg-gray-200 dark:bg-gray-700 rounded-lg mb-3"
                            }
                            div {
                                class: "h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    for article in articles() {
                        Link {
                            to: Route::Article { slug: article.slug.clone() },
                            class: "group",
                            
                            if let Some(cover_url) = &article.cover_image_url {
                                img {
                                    src: "{cover_url}",
                                    alt: "{article.title}",
                                    class: "w-full h-40 object-cover rounded-lg mb-3"
                                }
                            } else {
                                div {
                                    class: "w-full h-40 bg-gray-200 dark:bg-gray-700 rounded-lg mb-3"
                                }
                            }
                            
                            h4 {
                                class: "font-medium text-gray-900 dark:text-white group-hover:underline",
                                {article.title.clone()}
                            }
                            
                            p {
                                class: "text-sm text-gray-600 dark:text-gray-400 mt-1",
                                {article.excerpt.clone()}
                            }
                            
                            div {
                                class: "flex items-center mt-2 text-xs text-gray-500",
                                span {
                                    {article.author.display_name.clone()}
                                }
                                span { class: "mx-1", "·" }
                                span {
                                    "{article.reading_time} 分钟"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
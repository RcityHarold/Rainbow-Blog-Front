use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::articles::{ArticleService, ClapResponse},
    models::article::Article,
    hooks::use_auth,
    components::{CommentSection, ShareModal, HighlightSystem, RelatedArticles},
    Route,
};

#[component]
pub fn ArticlePage(slug: String) -> Element {
    let mut article = use_signal(|| None::<Article>);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut is_clapping = use_signal(|| false);
    let mut is_bookmarking = use_signal(|| false);
    let mut show_share = use_signal(|| false);
    
    let auth = use_auth();
    let navigator = use_navigator();
    
    // 加载文章详情
    use_effect(move || {
        let slug = slug.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut article = article.clone();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            // 获取文章详情
            match ArticleService::get_article(&slug).await {
                Ok(mut article_data) => {
                    // 增加浏览次数
                    let _ = ArticleService::increment_view_count(&article_data.id).await;
                    article_data.view_count += 1;
                    article.set(Some(article_data));
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            
            loading.set(false);
        });
    });
    
    // 处理点赞
    let handle_clap = move |_| {
        if !auth.read().is_authenticated {
            navigator.push(Route::Login {});
            return;
        }
        
        if let Some(ref art) = article() {
            let article_id = art.id.clone();
            spawn(async move {
                is_clapping.set(true);
                
                match ArticleService::clap_article(&article_id, 1).await {
                    Ok(ClapResponse { user_clap_count, total_claps }) => {
                        article.write().as_mut().unwrap().user_clap_count = Some(user_clap_count);
                        article.write().as_mut().unwrap().clap_count = total_claps as i32;
                        article.write().as_mut().unwrap().is_clapped = Some(true);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to clap article: {:?}", e).into());
                    }
                }
                
                is_clapping.set(false);
            });
        }
    };
    
    // 处理收藏
    let handle_bookmark = move |_| {
        if !auth.read().is_authenticated {
            navigator.push(Route::Login {});
            return;
        }
        
        if let Some(ref art) = article() {
            let article_id = art.id.clone();
            let is_bookmarked = art.is_bookmarked.unwrap_or(false);
            
            spawn(async move {
                is_bookmarking.set(true);
                
                let result = if is_bookmarked {
                    ArticleService::unbookmark_article(&article_id).await
                } else {
                    ArticleService::bookmark_article(&article_id, None).await
                };
                
                if result.is_ok() {
                    article.write().as_mut().unwrap().is_bookmarked = Some(!is_bookmarked);
                    if !is_bookmarked {
                        article.write().as_mut().unwrap().bookmark_count += 1;
                    } else {
                        article.write().as_mut().unwrap().bookmark_count -= 1;
                    }
                }
                
                is_bookmarking.set(false);
            });
        }
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 导航栏
            nav {
                class: "border-b border-gray-100 sticky top-0 bg-white z-10",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                        
                        div {
                            class: "flex items-center space-x-4",
                            if auth.read().is_authenticated {
                                button {
                                    class: "px-4 py-2 text-sm font-medium text-white bg-gray-900 rounded-full hover:bg-gray-800",
                                    "写文章"
                                }
                            } else {
                                Link {
                                    to: Route::Login {},
                                    class: "text-sm font-medium text-gray-900 hover:text-gray-700",
                                    "登录"
                                }
                            }
                        }
                    }
                }
            }
            
            // 主要内容
            if loading() {
                div {
                    class: "flex items-center justify-center h-64",
                    div {
                        class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                    }
                }
            } else if let Some(err) = error() {
                div {
                    class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8",
                    div {
                        class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                        {err}
                    }
                }
            } else if let Some(art) = article() {
                article {
                    class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8",
                    
                    // 标题部分
                    header {
                        class: "mb-8",
                        h1 {
                            class: "text-2xl sm:text-3xl md:text-4xl font-bold text-gray-900 mb-4 font-serif",
                            {art.title.clone()}
                        }
                        
                        if let Some(subtitle) = &art.subtitle {
                            h2 {
                                class: "text-xl text-gray-600 mb-6",
                                {subtitle.clone()}
                            }
                        }
                        
                        // 作者信息
                        div {
                            class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4",
                            div {
                                class: "flex items-center",
                                if let Some(avatar_url) = &art.author.avatar_url {
                                    img {
                                        src: "{avatar_url}",
                                        alt: "{art.author.display_name}",
                                        class: "w-12 h-12 rounded-full mr-4"
                                    }
                                } else {
                                    div {
                                        class: "w-12 h-12 rounded-full bg-gray-200 mr-4"
                                    }
                                }
                                
                                div {
                                    Link {
                                        to: Route::Home {},
                                        class: "text-base font-medium text-gray-900 hover:underline",
                                        {art.author.display_name.clone()}
                                    }
                                    div {
                                        class: "flex items-center text-sm text-gray-500",
                                        span {
                                            {art.published_at.map(|d| d.format("%Y年%m月%d日").to_string()).unwrap_or_default()}
                                        }
                                        span { class: "mx-2", "·" }
                                        span { "{art.reading_time} 分钟阅读" }
                                    }
                                }
                            }
                            
                            // 关注按钮
                            if auth.read().is_authenticated && auth.read().user.as_ref().map(|u| u.id != art.author.id).unwrap_or(true) {
                                button {
                                    class: "px-4 py-1 text-sm border border-green-600 text-green-600 rounded-full hover:bg-green-50",
                                    "关注"
                                }
                            }
                        }
                    }
                    
                    // 封面图
                    if let Some(cover_url) = &art.cover_image_url {
                        div {
                            class: "mb-8",
                            img {
                                src: "{cover_url}",
                                alt: "{art.title}",
                                class: "w-full rounded-lg"
                            }
                        }
                    }
                    
                    // 文章内容（带高亮系统）
                    if auth.read().is_authenticated {
                        HighlightSystem {
                            article_id: art.id.clone(),
                            article_html: art.content_html.clone()
                        }
                    } else {
                        div {
                            class: "prose prose-lg max-w-none mb-12",
                            dangerous_inner_html: "{art.content_html}"
                        }
                    }
                    
                    // 标签
                    if !art.tags.is_empty() {
                        div {
                            class: "flex flex-wrap gap-2 mb-8",
                            for tag in art.tags.iter() {
                                Link {
                                    to: Route::Home {},
                                    class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm hover:bg-gray-200",
                                    {tag.name.clone()}
                                }
                            }
                        }
                    }
                    
                    // 底部互动栏
                    div {
                        class: "border-t border-gray-100 pt-8",
                        div {
                            class: "flex items-center justify-between",
                            
                            // 左侧互动按钮
                            div {
                                class: "flex items-center space-x-4",
                                
                                // 点赞按钮
                                button {
                                    class: "flex items-center space-x-2 px-4 py-2 rounded-full hover:bg-gray-100 transition-colors",
                                    onclick: handle_clap,
                                    disabled: is_clapping(),
                                    
                                    svg {
                                        class: if art.is_clapped.unwrap_or(false) { "w-6 h-6 text-gray-900" } else { "w-6 h-6 text-gray-500" },
                                        fill: if art.is_clapped.unwrap_or(false) { "currentColor" } else { "none" },
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"
                                        }
                                    }
                                    
                                    span {
                                        class: "text-sm font-medium",
                                        {art.clap_count.to_string()}
                                    }
                                    
                                    if art.user_clap_count.unwrap_or(0) > 0 {
                                        span {
                                            class: "text-xs text-gray-500",
                                            "(+{art.user_clap_count.unwrap_or(0)})"
                                        }
                                    }
                                }
                                
                                // 评论按钮
                                button {
                                    class: "flex items-center space-x-2 px-4 py-2 rounded-full hover:bg-gray-100 transition-colors",
                                    svg {
                                        class: "w-6 h-6 text-gray-500",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                                        }
                                    }
                                    span {
                                        class: "text-sm font-medium",
                                        {art.comment_count.to_string()}
                                    }
                                }
                            }
                            
                            // 右侧操作按钮
                            div {
                                class: "flex items-center space-x-2",
                                
                                // 收藏按钮
                                button {
                                    class: "p-2 rounded-full hover:bg-gray-100 transition-colors",
                                    onclick: handle_bookmark,
                                    disabled: is_bookmarking(),
                                    
                                    svg {
                                        class: if art.is_bookmarked.unwrap_or(false) { "w-6 h-6 text-gray-900 fill-current" } else { "w-6 h-6 text-gray-500" },
                                        fill: if art.is_bookmarked.unwrap_or(false) { "currentColor" } else { "none" },
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"
                                        }
                                    }
                                }
                                
                                // 分享按钮
                                button {
                                    class: "p-2 rounded-full hover:bg-gray-100 transition-colors",
                                    onclick: move |_| show_share.set(true),
                                    svg {
                                        class: "w-6 h-6 text-gray-500",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"
                                        }
                                    }
                                }
                                
                                // 更多操作
                                button {
                                    class: "p-2 rounded-full hover:bg-gray-100 transition-colors",
                                    svg {
                                        class: "w-6 h-6 text-gray-500",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 相关文章推荐
                    RelatedArticles { article_id: art.id.clone() }
                    
                    // 评论区
                    CommentSection { article_id: art.id.clone() }
                    
                    // 作者信息卡片
                    div {
                        class: "mt-12 p-6 bg-gray-50 rounded-lg",
                        div {
                            class: "flex items-start",
                            if let Some(avatar_url) = &art.author.avatar_url {
                                img {
                                    src: "{avatar_url}",
                                    alt: "{art.author.display_name}",
                                    class: "w-16 h-16 rounded-full mr-4"
                                }
                            } else {
                                div {
                                    class: "w-16 h-16 rounded-full bg-gray-200 mr-4"
                                }
                            }
                            
                            div {
                                class: "flex-1",
                                h3 {
                                    class: "text-lg font-medium text-gray-900 mb-1",
                                    {art.author.display_name.clone()}
                                }
                                p {
                                    class: "text-gray-600 text-sm mb-3",
                                    "技术博主，专注于分享编程知识和经验"
                                }
                                if auth.read().is_authenticated && auth.read().user.as_ref().map(|u| u.id != art.author.id).unwrap_or(true) {
                                    button {
                                        class: "px-4 py-2 bg-green-600 text-white rounded-full text-sm font-medium hover:bg-green-700",
                                        "关注作者"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // 分享模态框
        if let Some(ref art) = article() {
            ShareModal {
                show: show_share(),
                on_close: move |_| show_share.set(false),
                article_url: {
                    let window = web_sys::window().unwrap();
                    let location = window.location();
                    let origin = location.origin().unwrap_or_default();
                    format!("{}/article/{}", origin, art.slug)
                },
                article_title: art.title.clone()
            }
        }
    }
}
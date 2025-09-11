use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::{users::UserService, articles::ArticleService},
    components::ArticleCard,
    models::{user::UserProfile, article::Article},
    hooks::use_auth,
    Route,
};

#[component]
pub fn ProfilePage(username: String) -> Element {
    let mut profile = use_signal(|| None::<UserProfile>);
    let mut articles = use_signal(|| Vec::<Article>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut active_tab = use_signal(|| "articles");
    let mut is_following = use_signal(|| false);
    let mut is_loading_follow = use_signal(|| false);
    
    let auth = use_auth();
    let navigator = use_navigator();
    
    // 加载用户资料和文章
    use_effect(move || {
        let username = username.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut profile = profile.clone();
        let mut articles = articles.clone();
        let auth = auth.clone();
        let mut is_following = is_following.clone();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            // 获取用户资料
            match UserService::get_user_profile(&username).await {
                Ok(response) => {
                    let user_profile = response.profile;
                    let user_id = user_profile.user_id.clone();
                    profile.set(Some(user_profile));
                    
                    // 检查是否关注
                    if auth.read().is_authenticated {
                        if let Ok(following) = UserService::is_following(&user_id).await {
                            is_following.set(following);
                        }
                    }
                }
                Err(e) => {
                    error.set(Some(e.message));
                    loading.set(false);
                    return;
                }
            }
            
            // 获取用户文章
            match UserService::get_user_articles(&username, Some(1), Some(20)).await {
                Ok(response) => {
                    articles.set(response.articles);
                }
                Err(_) => {
                    // 忽略文章加载错误
                }
            }
            
            loading.set(false);
        });
    });
    
    // 处理关注/取消关注
    let handle_follow = move |_| {
        if !auth.read().is_authenticated {
            navigator.push(Route::Login {});
            return;
        }
        
        if let Some(ref user_profile) = profile() {
            let user_id = user_profile.user_id.clone();
            let following = is_following();
            
            spawn(async move {
                is_loading_follow.set(true);
                
                let result = if following {
                    UserService::unfollow_user(&user_id).await
                } else {
                    UserService::follow_user(&user_id).await
                };
                
                if result.is_ok() {
                    is_following.set(!following);
                    if let Some(mut p) = profile() {
                        if following {
                            p.follower_count -= 1;
                        } else {
                            p.follower_count += 1;
                        }
                        profile.set(Some(p));
                    }
                }
                
                is_loading_follow.set(false);
            });
        }
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            
            // 主要内容
            if loading() {
                div {
                    class: "flex items-center justify-center min-h-screen",
                    div {
                        class: "flex flex-col items-center",
                        div {
                            class: "animate-spin rounded-full h-12 w-12 border-2 border-gray-200 border-t-green-600 mb-4"
                        }
                        p { class: "text-gray-500 text-sm", "加载中..." }
                    }
                }
            } else if let Some(err) = error() {
                div {
                    class: "min-h-screen flex items-center justify-center",
                    div {
                        class: "max-w-md mx-auto text-center px-6 py-12 bg-white rounded-lg shadow-sm",
                        div {
                            class: "w-16 h-16 mx-auto mb-6 bg-red-100 rounded-full flex items-center justify-center",
                            svg {
                                class: "w-8 h-8 text-red-600",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                }
                            }
                        }
                        h2 { 
                            class: "text-2xl font-bold text-gray-900 mb-3",
                            "页面加载失败" 
                        }
                        p { 
                            class: "text-gray-600 mb-6 leading-relaxed",
                            {err}
                        }
                        Link {
                            to: Route::Home {},
                            class: "inline-flex items-center px-6 py-3 bg-green-600 text-white rounded-full font-medium hover:bg-green-700 transition-colors",
                            "返回首页"
                        }
                    }
                }
            } else if let Some(user_profile) = profile() {
                div {
                    // Hero Section - Medium 风格的头部区域
                    div {
                        class: "bg-white border-b",
                        div {
                            class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8",
                            div {
                                class: "py-12 md:py-20",
                                div {
                                    class: "max-w-3xl",
                                    
                                    // 头像和基本信息
                                    div {
                                        class: "flex items-start space-x-6 mb-8",
                                        
                                        // 头像
                                        div {
                                            class: "flex-shrink-0",
                                            if let Some(avatar_url) = &user_profile.avatar_url {
                                                img {
                                                    src: "{avatar_url}",
                                                    alt: "{user_profile.username}",
                                                    class: "w-20 h-20 md:w-24 md:h-24 rounded-full border-2 border-gray-100"
                                                }
                                            } else {
                                                div {
                                                    class: "w-20 h-20 md:w-24 md:h-24 rounded-full bg-gradient-to-br from-green-400 to-blue-500 flex items-center justify-center text-white text-2xl font-bold",
                                                    {user_profile.display_name.clone().unwrap_or(user_profile.username.clone()).chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                }
                                            }
                                        }
                                        
                                        // 用户信息
                                        div {
                                            class: "flex-1 min-w-0",
                                            h1 {
                                                class: "text-3xl md:text-4xl font-bold text-gray-900 mb-3 leading-tight",
                                                {user_profile.display_name.clone().unwrap_or(user_profile.username.clone())}
                                            }
                                            
                                            if let Some(bio) = &user_profile.bio {
                                                p {
                                                    class: "text-lg text-gray-700 mb-4 leading-relaxed",
                                                    {bio.clone()}
                                                }
                                            }
                                            
                                            // 社交链接和位置
                                            div {
                                                class: "flex flex-wrap items-center gap-4 text-sm text-gray-600 mb-6",
                                                
                                                if let Some(location) = &user_profile.location {
                                                    div {
                                                        class: "flex items-center gap-1",
                                                        svg {
                                                            class: "w-4 h-4",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                stroke_width: "2",
                                                                d: "M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
                                                            }
                                                        }
                                                        span { {location.clone()} }
                                                    }
                                                }
                                                
                                                if let Some(website) = &user_profile.website {
                                                    a {
                                                        href: "{website}",
                                                        target: "_blank",
                                                        class: "flex items-center gap-1 text-green-600 hover:text-green-700 transition-colors",
                                                        svg {
                                                            class: "w-4 h-4",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                stroke_width: "2",
                                                                d: "M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                                                            }
                                                        }
                                                        "网站"
                                                    }
                                                }
                                                
                                                if let Some(twitter) = &user_profile.twitter_username {
                                                    a {
                                                        href: "https://twitter.com/{twitter}",
                                                        target: "_blank",
                                                        class: "flex items-center gap-1 text-blue-500 hover:text-blue-600 transition-colors",
                                                        svg {
                                                            class: "w-4 h-4",
                                                            fill: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                d: "M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"
                                                            }
                                                        }
                                                        "@{twitter}"
                                                    }
                                                }
                                                
                                                if let Some(github) = &user_profile.github_username {
                                                    a {
                                                        href: "https://github.com/{github}",
                                                        target: "_blank",
                                                        class: "flex items-center gap-1 text-gray-700 hover:text-gray-900 transition-colors",
                                                        svg {
                                                            class: "w-4 h-4",
                                                            fill: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                fill_rule: "evenodd",
                                                                d: "M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z",
                                                                clip_rule: "evenodd"
                                                            }
                                                        }
                                                        "@{github}"
                                                    }
                                                }
                                                
                                                span {
                                                    class: "text-gray-400",
                                                    "加入于 {user_profile.created_at.format(\"%Y年%m月\")}"
                                                }
                                            }
                                            
                                            // 统计信息
                                            div {
                                                class: "flex items-center gap-6 text-sm",
                                                div {
                                                    class: "flex items-center gap-2",
                                                    span { 
                                                        class: "text-2xl font-bold text-gray-900", 
                                                        {user_profile.article_count.to_string()} 
                                                    }
                                                    span { 
                                                        class: "text-gray-600 font-medium", 
                                                        "文章" 
                                                    }
                                                }
                                                div {
                                                    class: "flex items-center gap-2",
                                                    span { 
                                                        class: "text-2xl font-bold text-gray-900", 
                                                        {user_profile.follower_count.to_string()} 
                                                    }
                                                    span { 
                                                        class: "text-gray-600 font-medium", 
                                                        "关注者" 
                                                    }
                                                }
                                                div {
                                                    class: "flex items-center gap-2",
                                                    span { 
                                                        class: "text-2xl font-bold text-gray-900", 
                                                        {user_profile.total_claps_received.to_string()} 
                                                    }
                                                    span { 
                                                        class: "text-gray-600 font-medium", 
                                                        "点赞" 
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    
                                    // 操作按钮区域
                                    div {
                                        class: "flex items-center gap-4 pt-6",
                                        if auth.read().is_authenticated {
                                            if auth.read().user.as_ref().map(|u| u.id != user_profile.user_id).unwrap_or(true) {
                                                button {
                                                    class: if is_following() {
                                                        "flex items-center gap-2 px-6 py-3 border-2 border-gray-300 text-gray-700 rounded-full font-medium hover:bg-gray-50 transition-all duration-200"
                                                    } else {
                                                        "flex items-center gap-2 px-6 py-3 bg-green-600 text-white rounded-full font-medium hover:bg-green-700 transition-all duration-200 shadow-sm"
                                                    },
                                                    onclick: handle_follow,
                                                    disabled: is_loading_follow(),
                                                    if is_loading_follow() {
                                                        svg {
                                                            class: "animate-spin w-4 h-4",
                                                            fill: "none",
                                                            view_box: "0 0 24 24",
                                                            circle {
                                                                class: "opacity-25",
                                                                cx: "12",
                                                                cy: "12",
                                                                r: "10",
                                                                stroke: "currentColor",
                                                                stroke_width: "4"
                                                            }
                                                            path {
                                                                class: "opacity-75",
                                                                fill: "currentColor",
                                                                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                                            }
                                                        }
                                                        "处理中"
                                                    } else if is_following() {
                                                        svg {
                                                            class: "w-4 h-4",
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
                                                        "已关注"
                                                    } else {
                                                        svg {
                                                            class: "w-4 h-4",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            view_box: "0 0 24 24",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                stroke_width: "2",
                                                                d: "M12 4v16m8-8H4"
                                                            }
                                                        }
                                                        "关注"
                                                    }
                                                }
                                            } else {
                                                Link {
                                                    to: Route::Home {},
                                                    class: "flex items-center gap-2 px-6 py-3 border-2 border-gray-300 text-gray-700 rounded-full font-medium hover:bg-gray-50 transition-all duration-200",
                                                    svg {
                                                        class: "w-4 h-4",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        view_box: "0 0 24 24",
                                                        path {
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                                        }
                                                    }
                                                    "编辑资料"
                                                }
                                            }
                                        }
                                        
                                        // 分享按钮
                                        button {
                                            class: "flex items-center gap-2 px-6 py-3 bg-gray-100 text-gray-700 rounded-full font-medium hover:bg-gray-200 transition-all duration-200",
                                            onclick: move |_| {
                                                // 复制链接到剪贴板
                                            },
                                            svg {
                                                class: "w-4 h-4",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"
                                                }
                                            }
                                            "分享"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 导航标签
                    div {
                        class: "bg-white border-b sticky top-0 z-10",
                        div {
                            class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8",
                            nav {
                                class: "flex space-x-8",
                                button {
                                    class: if active_tab() == "articles" { 
                                        "py-4 px-1 border-b-2 border-green-600 font-medium text-sm text-green-600" 
                                    } else { 
                                        "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300 transition-all duration-200" 
                                    },
                                    onclick: move |_| active_tab.set("articles"),
                                    {
                                        if let Some(p) = profile() {
                                            format!("文章 ({})", p.article_count)
                                        } else {
                                            "文章".to_string()
                                        }
                                    }
                                }
                                button {
                                    class: if active_tab() == "about" { 
                                        "py-4 px-1 border-b-2 border-green-600 font-medium text-sm text-green-600" 
                                    } else { 
                                        "py-4 px-1 border-b-2 border-transparent font-medium text-sm text-gray-500 hover:text-gray-700 hover:border-gray-300 transition-all duration-200" 
                                    },
                                    onclick: move |_| active_tab.set("about"),
                                    "关于"
                                }
                            }
                        }
                    }
                    
                    // 内容区域
                    div {
                        class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                        
                        if active_tab() == "articles" {
                            if articles().is_empty() {
                                // 空状态
                                div {
                                    class: "text-center py-20",
                                    div {
                                        class: "w-24 h-24 mx-auto mb-6 bg-gray-100 rounded-full flex items-center justify-center",
                                        svg {
                                            class: "w-10 h-10 text-gray-400",
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
                                    }
                                    h3 { 
                                        class: "text-xl font-semibold text-gray-900 mb-3",
                                        "还没有发布文章"
                                    }
                                    p { 
                                        class: "text-gray-600 max-w-md mx-auto leading-relaxed",
                                        {user_profile.display_name.clone().unwrap_or(user_profile.username.clone())}
                                        " 还没有发布任何文章，期待 TA 的第一篇作品吧！"
                                    }
                                    
                                    if auth.read().user.as_ref().map(|u| u.id == user_profile.user_id).unwrap_or(false) {
                                        div {
                                            class: "mt-8",
                                            Link {
                                                to: Route::Write {},
                                                class: "inline-flex items-center gap-2 px-6 py-3 bg-green-600 text-white rounded-full font-medium hover:bg-green-700 transition-all duration-200",
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M12 4v16m8-8H4"
                                                    }
                                                }
                                                "写第一篇文章"
                                            }
                                        }
                                    }
                                }
                            } else {
                                // 文章列表
                                div {
                                    class: "max-w-3xl",
                                    div {
                                        class: "divide-y divide-gray-100",
                                        for article in articles() {
                                            ArticleCard { article }
                                        }
                                    }
                                }
                            }
                        } else {
                            // 关于页面
                            div {
                                class: "max-w-3xl",
                                div {
                                    class: "bg-white rounded-xl p-8 shadow-sm border",
                                    h2 { 
                                        class: "text-2xl font-bold text-gray-900 mb-6",
                                        "关于 {user_profile.display_name.clone().unwrap_or(user_profile.username.clone())}" 
                                    }
                                    
                                    if let Some(bio) = &user_profile.bio {
                                        div {
                                            class: "prose prose-lg max-w-none mb-8",
                                            p { 
                                                class: "text-gray-700 leading-relaxed",
                                                {bio.clone()} 
                                            }
                                        }
                                    }
                                    
                                    // 详细统计
                                    div {
                                        class: "grid grid-cols-2 md:grid-cols-4 gap-6 pt-8 border-t",
                                        div {
                                            class: "text-center",
                                            div {
                                                class: "text-3xl font-bold text-green-600 mb-1",
                                                {user_profile.article_count.to_string()}
                                            }
                                            div {
                                                class: "text-sm text-gray-600 font-medium",
                                                "发布文章"
                                            }
                                        }
                                        div {
                                            class: "text-center",
                                            div {
                                                class: "text-3xl font-bold text-blue-600 mb-1",
                                                {user_profile.follower_count.to_string()}
                                            }
                                            div {
                                                class: "text-sm text-gray-600 font-medium",
                                                "关注者"
                                            }
                                        }
                                        div {
                                            class: "text-center",
                                            div {
                                                class: "text-3xl font-bold text-purple-600 mb-1",
                                                {user_profile.following_count.to_string()}
                                            }
                                            div {
                                                class: "text-sm text-gray-600 font-medium",
                                                "正在关注"
                                            }
                                        }
                                        div {
                                            class: "text-center",
                                            div {
                                                class: "text-3xl font-bold text-orange-600 mb-1",
                                                {user_profile.total_claps_received.to_string()}
                                            }
                                            div {
                                                class: "text-sm text-gray-600 font-medium",
                                                "获得点赞"
                                            }
                                        }
                                    }
                                    
                                    // 加入时间
                                    div {
                                        class: "mt-8 pt-6 border-t",
                                        p {
                                            class: "text-gray-600 flex items-center gap-2",
                                            svg {
                                                class: "w-4 h-4",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                                                }
                                            }
                                            "加入于 {user_profile.created_at.format(\"%Y年%m月%d日\")}"
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
pub fn ProfileByIdPage(user_id: String) -> Element {
    let mut profile = use_signal(|| None::<UserProfile>);
    let mut articles = use_signal(|| Vec::<Article>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut active_tab = use_signal(|| "articles");
    let mut is_following = use_signal(|| false);
    let mut is_loading_follow = use_signal(|| false);
    
    let auth = use_auth();
    let navigator = use_navigator();
    
    // 加载用户资料和文章
    use_effect(move || {
        let user_id = user_id.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut profile = profile.clone();
        let mut articles = articles.clone();
        let auth = auth.clone();
        let mut is_following = is_following.clone();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            // 通过用户ID获取用户资料 - 需要新的API端点
            match UserService::get_user_profile_by_id(&user_id).await {
                Ok(response) => {
                    let user_profile = response.profile;
                    let profile_user_id = user_profile.user_id.clone();
                    profile.set(Some(user_profile));
                    
                    // 检查是否关注
                    if auth.read().is_authenticated {
                        if let Ok(following) = UserService::is_following(&profile_user_id).await {
                            is_following.set(following);
                        }
                    }
                    
                    // 获取用户文章 - 也需要基于ID的API
                    if let Ok(articles_response) = UserService::get_user_articles_by_id(&user_id, Some(1), Some(20)).await {
                        articles.set(articles_response.articles);
                    }
                }
                Err(e) => {
                    error.set(Some(format!("用户不存在或加载失败: {}", e.message)));
                }
            }
            
            loading.set(false);
        });
    });
    
    // 显示加载状态或错误
    if loading() {
        return rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                div {
                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                }
            }
        };
    }
    
    if let Some(err) = error() {
        return rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                div {
                    class: "text-center",
                    h1 { 
                        class: "text-2xl font-bold text-gray-900 mb-4",
                        "用户未找到" 
                    }
                    p { 
                        class: "text-gray-600 mb-8",
                        {err}
                    }
                    Link {
                        to: Route::Home {},
                        class: "text-blue-600 hover:text-blue-800",
                        "返回首页"
                    }
                }
            }
        };
    }
    
    // 如果没有个人资料，显示设置用户名的界面
    if profile().is_none() {
        return rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                div {
                    class: "max-w-md mx-auto text-center",
                    h1 {
                        class: "text-2xl font-bold mb-4",
                        "完善个人资料"
                    }
                    p {
                        class: "text-gray-600 mb-6",
                        "请先设置您的用户名和个人信息"
                    }
                    Link {
                        to: Route::Settings {},
                        class: "px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                        "去设置"
                    }
                }
            }
        };
    }
    
    // 处理关注/取消关注
    let handle_follow = move |_| {
        if !auth.read().is_authenticated {
            navigator.push(Route::Login {});
            return;
        }
        
        if let Some(ref user_profile) = profile() {
            let profile_user_id = user_profile.user_id.clone();
            let following = is_following();
            
            spawn(async move {
                is_loading_follow.set(true);
                
                let result = if following {
                    UserService::unfollow_user(&profile_user_id).await
                } else {
                    UserService::follow_user(&profile_user_id).await
                };
                
                if result.is_ok() {
                    is_following.set(!following);
                    if let Some(mut p) = profile() {
                        if following {
                            p.follower_count -= 1;
                        } else {
                            p.follower_count += 1;
                        }
                        profile.set(Some(p));
                    }
                }
                
                is_loading_follow.set(false);
            });
        }
    };

    // 显示个人资料（使用Medium风格设计）
    let user_profile = profile().unwrap();
    let auth_state = auth.read();
    let is_own_profile = auth_state.is_authenticated && 
                        auth_state.user.as_ref().map(|u| u.id == user_profile.user_id).unwrap_or(false);
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // Hero Section with Medium-style design
            div {
                class: "bg-gradient-to-br from-gray-50 to-gray-100 border-b border-gray-200",
                div {
                    class: "max-w-4xl mx-auto px-6 py-12",
                    div {
                        class: "flex flex-col md:flex-row items-start gap-8",
                        
                        // Avatar with gradient fallback
                        div {
                            class: "flex-shrink-0",
                            if let Some(ref avatar_url) = user_profile.avatar_url {
                                img {
                                    src: "{avatar_url}",
                                    alt: "{user_profile.display_name.as_ref().unwrap_or(&user_profile.username)} 的头像",
                                    class: "w-24 h-24 md:w-32 md:h-32 rounded-full object-cover border-4 border-white shadow-lg"
                                }
                            } else {
                                // Gradient avatar fallback with initials
                                div {
                                    class: "w-24 h-24 md:w-32 md:h-32 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 border-4 border-white shadow-lg flex items-center justify-center",
                                    span {
                                        class: "text-white font-bold text-xl md:text-3xl",
                                        {user_profile.display_name.as_ref().unwrap_or(&user_profile.username).chars().take(1).collect::<String>().to_uppercase()}
                                    }
                                }
                            }
                        }
                        
                        // User info
                        div {
                            class: "flex-1",
                            h1 {
                                class: "text-3xl md:text-4xl font-bold text-gray-900 mb-2",
                                {user_profile.display_name.as_ref().unwrap_or(&user_profile.username).clone()}
                            }
                            
                            p {
                                class: "text-lg text-gray-600 mb-1",
                                "@{user_profile.username}"
                            }
                            
                            if let Some(ref bio) = user_profile.bio {
                                p {
                                    class: "text-gray-700 text-lg leading-relaxed mb-4 max-w-2xl",
                                    {bio.clone()}
                                }
                            }
                            
                            // Enhanced stats with colors
                            div {
                                class: "flex flex-wrap gap-6 mb-6",
                                div {
                                    class: "flex items-center gap-1",
                                    span { class: "text-2xl font-bold text-green-600", {user_profile.article_count.to_string()} }
                                    span { class: "text-gray-600", "篇文章" }
                                }
                                div {
                                    class: "flex items-center gap-1",
                                    span { class: "text-2xl font-bold text-blue-600", {user_profile.follower_count.to_string()} }
                                    span { class: "text-gray-600", "关注者" }
                                }
                                div {
                                    class: "flex items-center gap-1",
                                    span { class: "text-2xl font-bold text-purple-600", {user_profile.following_count.to_string()} }
                                    span { class: "text-gray-600", "正在关注" }
                                }
                                div {
                                    class: "flex items-center gap-1",
                                    span { class: "text-2xl font-bold text-orange-600", {user_profile.total_claps_received.to_string()} }
                                    span { class: "text-gray-600", "获得拍手" }
                                }
                            }
                            
                            // Follow/Edit button
                            div {
                                class: "flex gap-3",
                                if is_own_profile {
                                    Link {
                                        to: Route::Settings {},
                                        class: "px-6 py-2 border border-gray-300 text-gray-700 rounded-full font-medium hover:bg-gray-50 transition-colors",
                                        "编辑个人资料"
                                    }
                                } else if auth_state.is_authenticated {
                                    button {
                                        onclick: handle_follow,
                                        disabled: is_loading_follow(),
                                        class: if is_following() {
                                            "px-6 py-2 border border-gray-300 text-gray-700 rounded-full font-medium hover:bg-gray-50 transition-colors"
                                        } else {
                                            "px-6 py-2 bg-green-600 text-white rounded-full font-medium hover:bg-green-700 transition-colors"
                                        },
                                        if is_loading_follow() {
                                            "处理中..."
                                        } else if is_following() {
                                            "已关注"
                                        } else {
                                            "关注"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Navigation tabs with sticky behavior
            div {
                class: "sticky top-0 bg-white border-b border-gray-200 z-10",
                div {
                    class: "max-w-4xl mx-auto px-6",
                    nav {
                        class: "flex space-x-8",
                        button {
                            onclick: move |_| active_tab.set("articles"),
                            class: if active_tab() == "articles" {
                                "py-4 px-1 border-b-2 border-gray-900 font-medium text-gray-900"
                            } else {
                                "py-4 px-1 border-b-2 border-transparent font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300"
                            },
                            "文章"
                        }
                        button {
                            onclick: move |_| active_tab.set("about"),
                            class: if active_tab() == "about" {
                                "py-4 px-1 border-b-2 border-gray-900 font-medium text-gray-900"
                            } else {
                                "py-4 px-1 border-b-2 border-transparent font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300"
                            },
                            "关于"
                        }
                    }
                }
            }
            
            // Content area
            div {
                class: "max-w-4xl mx-auto px-6 py-8",
                
                // Articles tab
                if active_tab() == "articles" {
                    div {
                        class: "space-y-8",
                        if articles().is_empty() {
                            // Professional empty state
                            div {
                                class: "text-center py-16",
                                div {
                                    class: "w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center",
                                    svg {
                                        class: "w-8 h-8 text-gray-400",
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
                                }
                                h3 {
                                    class: "text-lg font-medium text-gray-900 mb-2",
                                    if is_own_profile { "还没有发表文章" } else { "暂无文章" }
                                }
                                p {
                                    class: "text-gray-500 max-w-md mx-auto",
                                    if is_own_profile { 
                                        "开始写作，与世界分享你的想法和见解。"
                                    } else {
                                        "这位作者还没有发表任何文章。"
                                    }
                                }
                                if is_own_profile {
                                    div {
                                        class: "mt-6",
                                        Link {
                                            to: Route::Write {},
                                            class: "inline-flex items-center px-6 py-3 bg-gray-900 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors",
                                            "开始写作"
                                        }
                                    }
                                }
                            }
                        } else {
                            for article in articles() {
                                ArticleCard { article }
                            }
                        }
                    }
                }
                
                // About tab
                if active_tab() == "about" {
                    div {
                        class: "max-w-2xl",
                        div {
                            class: "bg-gray-50 rounded-lg p-6 mb-8",
                            h3 {
                                class: "text-lg font-semibold text-gray-900 mb-4",
                                "关于 {user_profile.display_name.as_ref().unwrap_or(&user_profile.username)}"
                            }
                            if let Some(ref bio) = user_profile.bio {
                                p {
                                    class: "text-gray-700 leading-relaxed",
                                    {bio.clone()}
                                }
                            } else {
                                p {
                                    class: "text-gray-500 italic",
                                    "这位作者还没有添加简介。"
                                }
                            }
                        }
                        
                        // Additional info
                        if user_profile.location.is_some() || user_profile.website.is_some() {
                            div {
                                class: "space-y-3",
                                h4 {
                                    class: "font-medium text-gray-900 mb-3",
                                    "详细信息"
                                }
                                if let Some(ref location) = user_profile.location {
                                    div {
                                        class: "flex items-center gap-2 text-gray-600",
                                        svg {
                                            class: "w-4 h-4",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z",
                                                clip_rule: "evenodd"
                                            }
                                        }
                                        span { {location.clone()} }
                                    }
                                }
                                if let Some(ref website) = user_profile.website {
                                    div {
                                        class: "flex items-center gap-2 text-gray-600",
                                        svg {
                                            class: "w-4 h-4",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M4.083 9h1.946c.089-1.546.383-2.97.837-4.118A6.004 6.004 0 004.083 9zM10 2a8 8 0 100 16 8 8 0 000-16zm0 2c-.076 0-.232.032-.465.262-.238.234-.497.623-.737 1.182-.389.907-.673 2.142-.766 3.556h3.936c-.093-1.414-.377-2.649-.766-3.556-.24-.56-.5-.948-.737-1.182C10.232 4.032 10.076 4 10 4zm3.971 5c-.089-1.546-.383-2.97-.837-4.118A6.004 6.004 0 0115.917 9h-1.946zm-2.003 2H8.032c.093 1.414.377 2.649.766 3.556.24.56.5.948.737 1.182.233.23.389.262.465.262.076 0 .232-.032.465-.262.238-.234.498-.623.737-1.182.389-.907.673-2.142.766-3.556zm1.166 4.118c.454-1.147.748-2.572.837-4.118h1.946a6.004 6.004 0 01-2.783 4.118zm-6.268 0C6.412 13.97 6.118 12.546 6.03 11H4.083a6.004 6.004 0 002.783 4.118z",
                                                clip_rule: "evenodd"
                                            }
                                        }
                                        a {
                                            href: "{website}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: "text-blue-600 hover:text-blue-800 hover:underline",
                                            {website.clone()}
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
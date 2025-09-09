use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::users::UserService,
    models::user::{UpdateProfileRequest, UserProfile},
    hooks::use_auth,
    components::{ProtectedRoute, ImageDropZone},
    Route,
};
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        ProtectedRoute {
            SettingsContent {}
        }
    }
}

#[component]
fn SettingsContent() -> Element {
    let auth = use_auth();
    let mut active_tab = use_signal(|| "profile");
    let mut profile = use_signal(|| None::<UserProfile>);
    let mut loading = use_signal(|| true);
    
    // 表单字段
    let mut display_name = use_signal(|| String::new());
    let mut bio = use_signal(|| String::new());
    let mut website = use_signal(|| String::new());
    let mut location = use_signal(|| String::new());
    let mut twitter_username = use_signal(|| String::new());
    let mut github_username = use_signal(|| String::new());
    let mut linkedin_url = use_signal(|| String::new());
    let mut avatar_url = use_signal(|| String::new());
    
    let mut saving = use_signal(|| false);
    let mut save_message = use_signal(|| None::<String>);
    
    // 加载用户资料
    use_effect(move || {
        if let Some(user) = &auth.read().user {
            let username = user.username.clone();
            spawn(async move {
                loading.set(true);
                
                if let Ok(response) = UserService::get_user_profile(&username).await {
                    let p = response.profile;
                    
                    // 设置表单初始值
                    display_name.set(p.display_name.clone().unwrap_or_default());
                    bio.set(p.bio.clone().unwrap_or_default());
                    website.set(p.website.clone().unwrap_or_default());
                    location.set(p.location.clone().unwrap_or_default());
                    twitter_username.set(p.twitter_username.clone().unwrap_or_default());
                    github_username.set(p.github_username.clone().unwrap_or_default());
                    linkedin_url.set(p.linkedin_url.clone().unwrap_or_default());
                    avatar_url.set(p.avatar_url.clone().unwrap_or_default());
                    
                    profile.set(Some(p));
                }
                
                loading.set(false);
            });
        }
    });
    
    // 保存个人资料
    let mut save_profile = move |_| {
        saving.set(true);
        save_message.set(None);
        
        let request = UpdateProfileRequest {
            display_name: Some(display_name()),
            bio: Some(bio()),
            website: Some(website()),
            location: Some(location()),
            twitter_username: Some(twitter_username()),
            github_username: Some(github_username()),
            linkedin_url: Some(linkedin_url()),
            avatar_url: if avatar_url().is_empty() { None } else { Some(avatar_url()) },
            cover_image_url: None,
            facebook_url: None,
        };
        
        spawn(async move {
            match UserService::update_profile(&request).await {
                Ok(_) => {
                    save_message.set(Some("个人资料已保存".to_string()));
                    
                    // 3秒后清除消息
                    spawn(async move {
                        TimeoutFuture::new(3000).await;
                        save_message.set(None);
                    });
                }
                Err(e) => {
                    save_message.set(Some(format!("保存失败: {}", e.message)));
                }
            }
            saving.set(false);
        });
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
                        
                        Link {
                            to: Route::Home {},
                            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                            "返回首页"
                        }
                    }
                }
            }
            
            // 设置内容
            div {
                class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                h1 {
                    class: "text-3xl font-bold text-gray-900 dark:text-white mb-8",
                    "设置"
                }
                
                div {
                    class: "flex flex-col md:flex-row gap-8",
                    
                    // 侧边栏
                    aside {
                        class: "w-full md:w-64",
                        nav {
                            class: "space-y-1",
                            button {
                                class: if active_tab() == "profile" {
                                    "w-full text-left px-4 py-2 text-sm font-medium text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-800 rounded-md"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md"
                                },
                                onclick: move |_| active_tab.set("profile"),
                                "个人资料"
                            }
                            button {
                                class: if active_tab() == "account" {
                                    "w-full text-left px-4 py-2 text-sm font-medium text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-800 rounded-md"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md"
                                },
                                onclick: move |_| active_tab.set("account"),
                                "账户设置"
                            }
                            button {
                                class: if active_tab() == "notifications" {
                                    "w-full text-left px-4 py-2 text-sm font-medium text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-800 rounded-md"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md"
                                },
                                onclick: move |_| active_tab.set("notifications"),
                                "通知设置"
                            }
                            button {
                                class: if active_tab() == "privacy" {
                                    "w-full text-left px-4 py-2 text-sm font-medium text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-800 rounded-md"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md"
                                },
                                onclick: move |_| active_tab.set("privacy"),
                                "隐私设置"
                            }
                            
                            // 订阅管理相关链接
                            div {
                                class: "pt-4 mt-4 border-t border-gray-200 dark:border-gray-700",
                                p {
                                    class: "px-4 py-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                    "订阅管理"
                                }
                                Link {
                                    to: Route::MySubscriptions {},
                                    class: "flex items-center w-full px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md",
                                    svg {
                                        class: "w-4 h-4 mr-3",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
                                        }
                                    }
                                    "我的订阅"
                                }
                                Link {
                                    to: Route::SubscriptionPlans {},
                                    class: "flex items-center w-full px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md",
                                    svg {
                                        class: "w-4 h-4 mr-3",
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
                                    "订阅计划"
                                }
                                Link {
                                    to: Route::Earnings {},
                                    class: "flex items-center w-full px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md",
                                    svg {
                                        class: "w-4 h-4 mr-3",
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
                                    "收益统计"
                                }
                            }
                        }
                    }
                    
                    // 主要内容区
                    main {
                        class: "flex-1",
                        
                        if loading() {
                            div {
                                class: "flex justify-center py-8",
                                div {
                                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                                }
                            }
                        } else {
                            // 个人资料设置
                            if active_tab() == "profile" {
                                div {
                                    class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                                    
                                    h2 {
                                        class: "text-lg font-medium text-gray-900 dark:text-white mb-6",
                                        "个人资料"
                                    }
                                    
                                    form {
                                        class: "space-y-6",
                                        onsubmit: move |e| {
                                            e.prevent_default();
                                            save_profile(());
                                        },
                                        
                                        // 头像上传
                                        div {
                                            label {
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "头像"
                                            }
                                            ImageDropZone {
                                                on_upload: move |url| avatar_url.set(url),
                                                on_error: move |err| save_message.set(Some(err)),
                                                current_image: if avatar_url().is_empty() { None } else { Some(avatar_url()) },
                                                placeholder_text: "点击上传头像图片".to_string()
                                            }
                                        }
                                        
                                        // 显示名称
                                        div {
                                            label {
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "显示名称"
                                            }
                                            input {
                                                r#type: "text",
                                                value: "{display_name}",
                                                oninput: move |e| display_name.set(e.value()),
                                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                placeholder: "您的显示名称"
                                            }
                                        }
                                        
                                        // 个人简介
                                        div {
                                            label {
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "个人简介"
                                            }
                                            textarea {
                                                value: "{bio}",
                                                oninput: move |e| bio.set(e.value()),
                                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                rows: "4",
                                                placeholder: "介绍一下您自己..."
                                            }
                                        }
                                        
                                        // 网站
                                        div {
                                            label {
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "个人网站"
                                            }
                                            input {
                                                r#type: "url",
                                                value: "{website}",
                                                oninput: move |e| website.set(e.value()),
                                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                placeholder: "https://example.com"
                                            }
                                        }
                                        
                                        // 位置
                                        div {
                                            label {
                                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                "位置"
                                            }
                                            input {
                                                r#type: "text",
                                                value: "{location}",
                                                oninput: move |e| location.set(e.value()),
                                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                placeholder: "城市，国家"
                                            }
                                        }
                                        
                                        // 社交媒体
                                        div {
                                            h3 {
                                                class: "text-sm font-medium text-gray-900 dark:text-white mb-4",
                                                "社交媒体"
                                            }
                                            
                                            div {
                                                class: "space-y-4",
                                                
                                                // Twitter
                                                div {
                                                    label {
                                                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                        "Twitter 用户名"
                                                    }
                                                    div {
                                                        class: "flex",
                                                        span {
                                                            class: "inline-flex items-center px-3 text-sm text-gray-500 bg-gray-50 dark:bg-gray-700 dark:text-gray-400 border border-r-0 border-gray-300 dark:border-gray-600 rounded-l-md",
                                                            "@"
                                                        }
                                                        input {
                                                            r#type: "text",
                                                            value: "{twitter_username}",
                                                            oninput: move |e| twitter_username.set(e.value()),
                                                            class: "flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-r-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                            placeholder: "username"
                                                        }
                                                    }
                                                }
                                                
                                                // GitHub
                                                div {
                                                    label {
                                                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                        "GitHub 用户名"
                                                    }
                                                    input {
                                                        r#type: "text",
                                                        value: "{github_username}",
                                                        oninput: move |e| github_username.set(e.value()),
                                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                        placeholder: "username"
                                                    }
                                                }
                                                
                                                // LinkedIn
                                                div {
                                                    label {
                                                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                                        "LinkedIn URL"
                                                    }
                                                    input {
                                                        r#type: "url",
                                                        value: "{linkedin_url}",
                                                        oninput: move |e| linkedin_url.set(e.value()),
                                                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                                        placeholder: "https://linkedin.com/in/username"
                                                    }
                                                }
                                            }
                                        }
                                        
                                        // 保存按钮
                                        div {
                                            class: "flex items-center justify-between",
                                            
                                            if let Some(message) = save_message() {
                                                p {
                                                    class: if message.contains("失败") {
                                                        "text-sm text-red-600 dark:text-red-400"
                                                    } else {
                                                        "text-sm text-green-600 dark:text-green-400"
                                                    },
                                                    {message.clone()}
                                                }
                                            }
                                            
                                            button {
                                                r#type: "submit",
                                                class: "px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed",
                                                disabled: saving(),
                                                if saving() {
                                                    "保存中..."
                                                } else {
                                                    "保存更改"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 账户设置
                            if active_tab() == "account" {
                                div {
                                    class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                                    
                                    h2 {
                                        class: "text-lg font-medium text-gray-900 dark:text-white mb-6",
                                        "账户设置"
                                    }
                                    
                                    div {
                                        class: "space-y-6",
                                        
                                        // 邮箱
                                        div {
                                            h3 {
                                                class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                                                "邮箱地址"
                                            }
                                            p {
                                                class: "text-sm text-gray-600 dark:text-gray-400",
                                                if let Some(user) = &auth.read().user {
                                                    {user.email.clone()}
                                                }
                                            }
                                            button {
                                                class: "mt-2 text-sm text-blue-600 hover:text-blue-500",
                                                "更改邮箱"
                                            }
                                        }
                                        
                                        // 密码
                                        div {
                                            h3 {
                                                class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                                                "密码"
                                            }
                                            p {
                                                class: "text-sm text-gray-600 dark:text-gray-400",
                                                "上次更改时间：从未"
                                            }
                                            button {
                                                class: "mt-2 text-sm text-blue-600 hover:text-blue-500",
                                                "更改密码"
                                            }
                                        }
                                        
                                        // 删除账户
                                        div {
                                            class: "pt-6 border-t border-gray-200 dark:border-gray-700",
                                            h3 {
                                                class: "text-sm font-medium text-red-600 dark:text-red-400 mb-2",
                                                "危险区域"
                                            }
                                            p {
                                                class: "text-sm text-gray-600 dark:text-gray-400 mb-4",
                                                "删除账户后，您的所有数据将被永久删除，无法恢复。"
                                            }
                                            button {
                                                class: "px-4 py-2 border border-red-300 dark:border-red-700 text-red-600 dark:text-red-400 rounded-md hover:bg-red-50 dark:hover:bg-red-900/20",
                                                "删除账户"
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 通知设置
                            if active_tab() == "notifications" {
                                div {
                                    class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                                    
                                    h2 {
                                        class: "text-lg font-medium text-gray-900 dark:text-white mb-6",
                                        "通知设置"
                                    }
                                    
                                    div {
                                        class: "space-y-4",
                                        
                                        NotificationToggle {
                                            title: "新关注者",
                                            description: "当有人关注您时收到通知",
                                            checked: true
                                        }
                                        
                                        NotificationToggle {
                                            title: "评论回复",
                                            description: "当有人回复您的评论时收到通知",
                                            checked: true
                                        }
                                        
                                        NotificationToggle {
                                            title: "文章互动",
                                            description: "当有人点赞或评论您的文章时收到通知",
                                            checked: false
                                        }
                                        
                                        NotificationToggle {
                                            title: "每周摘要",
                                            description: "接收每周的统计数据和热门内容",
                                            checked: true
                                        }
                                    }
                                }
                            }
                            
                            // 隐私设置
                            if active_tab() == "privacy" {
                                div {
                                    class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                                    
                                    h2 {
                                        class: "text-lg font-medium text-gray-900 dark:text-white mb-6",
                                        "隐私设置"
                                    }
                                    
                                    div {
                                        class: "space-y-4",
                                        
                                        PrivacyToggle {
                                            title: "公开个人资料",
                                            description: "其他用户可以查看您的个人资料页面",
                                            checked: true
                                        }
                                        
                                        PrivacyToggle {
                                            title: "显示邮箱地址",
                                            description: "在您的个人资料页显示邮箱地址",
                                            checked: false
                                        }
                                        
                                        PrivacyToggle {
                                            title: "允许被搜索引擎索引",
                                            description: "您的个人资料和文章可以被搜索引擎收录",
                                            checked: true
                                        }
                                        
                                        PrivacyToggle {
                                            title: "显示阅读历史",
                                            description: "其他用户可以看到您的阅读历史",
                                            checked: false
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
fn NotificationToggle(title: String, description: String, checked: bool) -> Element {
    let mut is_checked = use_signal(|| checked);
    
    rsx! {
        div {
            class: "flex items-center justify-between py-4",
            div {
                h3 {
                    class: "text-sm font-medium text-gray-900 dark:text-white",
                    {title}
                }
                p {
                    class: "text-sm text-gray-600 dark:text-gray-400",
                    {description}
                }
            }
            
            label {
                class: "relative inline-flex items-center cursor-pointer",
                input {
                    r#type: "checkbox",
                    class: "sr-only peer",
                    checked: is_checked(),
                    onchange: move |_| is_checked.set(!is_checked())
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
                }
            }
        }
    }
}

#[component]
fn PrivacyToggle(title: String, description: String, checked: bool) -> Element {
    let mut is_checked = use_signal(|| checked);
    
    rsx! {
        div {
            class: "flex items-center justify-between py-4",
            div {
                h3 {
                    class: "text-sm font-medium text-gray-900 dark:text-white",
                    {title}
                }
                p {
                    class: "text-sm text-gray-600 dark:text-gray-400",
                    {description}
                }
            }
            
            label {
                class: "relative inline-flex items-center cursor-pointer",
                input {
                    r#type: "checkbox",
                    class: "sr-only peer",
                    checked: is_checked(),
                    onchange: move |_| is_checked.set(!is_checked())
                }
                div {
                    class: "w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
                }
            }
        }
    }
}
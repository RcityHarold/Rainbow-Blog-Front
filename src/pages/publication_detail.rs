use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::{publications::PublicationService, articles::ArticleService, domains::DomainService},
    models::{publication::Publication, article::Article, domain::PublicationDomain},
    components::ArticleCard,
    hooks::use_auth,
    Route,
};

#[component]
pub fn PublicationDetailPage(slug: String) -> Element {
    let mut publication = use_signal(|| None::<Publication>);
    let mut articles = use_signal(|| Vec::<Article>::new());
    let mut domains = use_signal(|| Vec::<PublicationDomain>::new());
    let mut loading = use_signal(|| true);
    let mut is_following = use_signal(|| false);
    let mut active_tab = use_signal(|| "articles");
    let mut members = use_signal(|| Vec::<crate::models::publication::PublicationMember>::new());
    let mut members_loading = use_signal(|| false);
    let mut add_user_id = use_signal(|| String::new());
    let mut add_role = use_signal(|| String::from("writer"));
    let mut members_error = use_signal(|| None::<String>);
    let auth = use_auth();
    
    // 加载出版物详情
    use_effect(move || {
        let slug = slug.clone();
        spawn(async move {
            loading.set(true);
            
            // 获取出版物详情
            match PublicationService::get_publication(&slug).await {
                Ok(pub_data) => {
                    publication.set(Some(pub_data));
                }
                Err(_) => {
                    // 错误处理
                }
            }
            
            // 获取出版物文章
            match PublicationService::get_publication_articles(&slug, Some("published"), None, None, Some("newest"), Some(1), Some(20)).await {
                Ok(response) => {
                    articles.set(response.articles);
                }
                Err(_) => {
                    // 错误处理
                }
            }
            
            loading.set(false);
        });
    });
    
    // 关注/取消关注
    let toggle_follow = move |_| {
        if let Some(pub_data) = publication() {
            spawn(async move {
                let result = if is_following() {
                    PublicationService::unfollow_publication(&pub_data.id).await
                } else {
                    PublicationService::follow_publication(&pub_data.id).await
                };
                
                if result.is_ok() {
                    is_following.set(!is_following());
                }
            });
        }
    };

    // 加载成员列表（当切换到成员Tab时）
    use_effect(move || {
        if active_tab() == "members" {
            if let Some(pub_data) = publication() {
                let pub_id = pub_data.id.clone();
                spawn(async move {
                    members_loading.set(true);
                    members_error.set(None);
                    match PublicationService::get_members(&pub_id, None, None, Some(1), Some(50)).await {
                        Ok(resp) => members.set(resp.members),
                        Err(e) => members_error.set(Some(format!("加载成员失败: {}", e.message))),
                    }
                    members_loading.set(false);
                });
            }
        }
    });

    // 添加成员（简单版，权限由后端校验）
    let handle_add_member = move |e: Event<FormData>| {
        e.prevent_default();
        if let Some(pub_data) = publication() {
            let pub_id = pub_data.id.clone();
            let uid = add_user_id();
            let role = add_role();
            spawn(async move {
                members_loading.set(true);
                members_error.set(None);
                let req = crate::models::publication::AddMemberRequest {
                    user_id: uid.clone(),
                    role: match role.as_str() {
                        "owner" => crate::models::publication::MemberRole::Owner,
                        "editor" => crate::models::publication::MemberRole::Editor,
                        "writer" => crate::models::publication::MemberRole::Writer,
                        _ => crate::models::publication::MemberRole::Contributor,
                    },
                    message: None,
                };
                match PublicationService::add_member(&pub_id, &req).await {
                    Ok(new_member) => {
                        members.write().push(new_member);
                        add_user_id.set(String::new());
                    }
                    Err(e) => members_error.set(Some(format!("添加失败: {}", e.message))),
                }
                members_loading.set(false);
            });
        }
    };
    
    if loading() {
        return rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                div {
                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                }
            }
        };
    }
    
    if let Some(pub_data) = publication() {
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
                                to: Route::Publications {},
                                class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                "← 返回出版物列表"
                            }
                        }
                    }
                }
                
                // 头部横幅
                div {
                    class: "relative",
                    if let Some(header_url) = &pub_data.header_image_url {
                        img {
                            src: "{header_url}",
                            alt: "{pub_data.name}",
                            class: "w-full h-64 object-cover"
                        }
                    } else {
                        div {
                            class: "w-full h-64 bg-gradient-to-r from-blue-500 to-purple-600"
                        }
                    }
                    
                    // 出版物信息覆盖层
                    div {
                        class: "absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-8",
                        div {
                            class: "max-w-7xl mx-auto",
                            div {
                                class: "flex items-end space-x-6",
                                // Logo
                                if let Some(logo_url) = &pub_data.logo_url {
                                    img {
                                        src: "{logo_url}",
                                        alt: "{pub_data.name}",
                                        class: "w-24 h-24 rounded-lg border-4 border-white"
                                    }
                                } else {
                                    div {
                                        class: "w-24 h-24 bg-white rounded-lg border-4 border-white flex items-center justify-center",
                                        span {
                                            class: "text-3xl font-bold text-gray-800",
                                            {pub_data.name.chars().next().unwrap_or('P').to_string()}
                                        }
                                    }
                                }
                                
                                // 信息
                                div {
                                    class: "flex-1 text-white",
                                    div {
                                        class: "flex items-center space-x-3",
                                        h1 {
                                            class: "text-3xl font-bold",
                                            {pub_data.name.clone()}
                                        }
                                        if pub_data.is_verified {
                                            svg {
                                                class: "w-6 h-6 text-blue-400",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                                    clip_rule: "evenodd"
                                                }
                                            }
                                        }
                                    }
                                    if let Some(tagline) = &pub_data.tagline {
                                        p {
                                            class: "text-lg mt-1",
                                            {tagline.clone()}
                                        }
                                    }
                                }
                                
                                // 关注按钮
                                if auth.read().is_authenticated {
                                    button {
                                        class: if is_following() {
                                            "px-6 py-2 bg-white text-gray-900 rounded-full font-medium hover:bg-gray-100"
                                        } else {
                                            "px-6 py-2 bg-blue-600 text-white rounded-full font-medium hover:bg-blue-700"
                                        },
                                        onclick: toggle_follow,
                                        if is_following() { "已关注" } else { "关注" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 统计信息栏
                div {
                    class: "bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
                    div {
                        class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4",
                        div {
                            class: "flex items-center space-x-8 text-sm",
                            div {
                                class: "flex items-center space-x-2",
                                span { class: "font-semibold text-gray-900 dark:text-white", "{pub_data.member_count}" }
                                span { class: "text-gray-600 dark:text-gray-400", "成员" }
                            }
                            div {
                                class: "flex items-center space-x-2",
                                span { class: "font-semibold text-gray-900 dark:text-white", "{pub_data.article_count}" }
                                span { class: "text-gray-600 dark:text-gray-400", "文章" }
                            }
                            div {
                                class: "flex items-center space-x-2",
                                span { class: "font-semibold text-gray-900 dark:text-white", "{pub_data.follower_count}" }
                                span { class: "text-gray-600 dark:text-gray-400", "关注者" }
                            }
                        }
                    }
                }
                
                // 标签页导航
                div {
                    class: "border-b border-gray-200 dark:border-gray-700",
                    div {
                        class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                        nav {
                            class: "flex space-x-8",
                            button {
                                class: if active_tab() == "articles" {
                                    "py-4 px-1 border-b-2 border-gray-900 dark:border-white text-sm font-medium text-gray-900 dark:text-white"
                                } else {
                                    "py-4 px-1 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                                },
                                onclick: move |_| active_tab.set("articles"),
                                "文章"
                            }
                            button {
                                class: if active_tab() == "about" {
                                    "py-4 px-1 border-b-2 border-gray-900 dark:border-white text-sm font-medium text-gray-900 dark:text-white"
                                } else {
                                    "py-4 px-1 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                                },
                                onclick: move |_| active_tab.set("about"),
                                "关于"
                            }
                            button {
                                class: if active_tab() == "members" {
                                    "py-4 px-1 border-b-2 border-gray-900 dark:border-white text-sm font-medium text-gray-900 dark:text-white"
                                } else {
                                    "py-4 px-1 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                                },
                                onclick: move |_| active_tab.set("members"),
                                "成员"
                            }
                            if auth.read().is_authenticated {
                                button {
                                    class: if active_tab() == "domains" {
                                        "py-4 px-1 border-b-2 border-gray-900 dark:border-white text-sm font-medium text-gray-900 dark:text-white"
                                    } else {
                                        "py-4 px-1 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                                    },
                                    onclick: move |_| active_tab.set("domains"),
                                    "域名管理"
                                }
                            }
                        }
                    }
                }
                
                // 内容区域
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                    
                    // 文章列表
                    if active_tab() == "articles" {
                        if articles().is_empty() {
                            div {
                                class: "text-center py-12",
                                p {
                                    class: "text-gray-500 dark:text-gray-400",
                                    "暂无文章"
                                }
                            }
                        } else {
                            div {
                                class: "grid gap-6",
                                for article in articles() {
                                    ArticleCard {
                                        article
                                    }
                                }
                            }
                        }
                    }
                    
                    // 关于页面
                    if active_tab() == "about" {
                        div {
                            class: "prose prose-lg dark:prose-invert max-w-none",
                            if let Some(description) = &pub_data.description {
                                p { {description.clone()} }
                            } else {
                                p {
                                    class: "text-gray-500 dark:text-gray-400",
                                    "暂无介绍"
                                }
                            }
                            
                            // 社交链接
                            if let Some(social) = &pub_data.social_links {
                                div {
                                    class: "mt-8",
                                    h3 {
                                        class: "text-lg font-semibold mb-4",
                                        "关注我们"
                                    }
                                    div {
                                        class: "flex space-x-4",
                                        if let Some(twitter) = &social.twitter {
                                            a {
                                                href: "{twitter}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                class: "text-gray-600 hover:text-gray-900 dark:hover:text-white",
                                                "Twitter"
                                            }
                                        }
                                        if let Some(website) = &social.website {
                                            a {
                                                href: "{website}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                class: "text-gray-600 hover:text-gray-900 dark:hover:text-white",
                                                "网站"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 成员列表
                    if active_tab() == "members" {
                        div {
                            class: "space-y-6",
                            if let Some(err) = members_error() { 
                                div { class: "p-3 rounded bg-red-50 text-red-700 border border-red-200", {err} }
                            }
                            // 添加成员表单（权限由后端校验）
                            form { onsubmit: handle_add_member,
                                class: "flex flex-col md:flex-row gap-3 items-start md:items-end",
                                div { class: "flex-1",
                                    label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "用户ID" }
                                    input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                        value: add_user_id(),
                                        oninput: move |e| add_user_id.set(e.value())
                                    }
                                }
                                div { class: "",
                                    label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "角色" }
                                    select { class: "border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                        value: add_role(),
                                        oninput: move |e| add_role.set(e.value()),
                                        option { value: "owner", "Owner" }
                                        option { value: "editor", "Editor" }
                                        option { value: "writer", "Writer" }
                                        option { value: "contributor", "Contributor" }
                                    }
                                }
                                button { r#type: "submit",
                                    class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50",
                                    disabled: members_loading(),
                                    if members_loading() { "处理中..." } else { "添加成员" }
                                }
                            }

                            // 成员列表展示
                            if members().is_empty() {
                                div { class: "text-gray-500 dark:text-gray-400", "暂无成员" }
                            } else {
                                div { class: "divide-y divide-gray-200 dark:divide-gray-700 rounded border border-gray-200 dark:border-gray-700",
                                    for m in members() {
                                        div { class: "p-4 flex items-center justify-between",
                                            div { class: "",
                                                div { class: "font-medium text-gray-900 dark:text-white", { m.user.display_name.clone().unwrap_or(m.user.username.clone()) } }
                                                div { class: "text-sm text-gray-500 dark:text-gray-400", {m.user.id.clone()} }
                                            }
                                            div { class: "text-sm text-gray-600 dark:text-gray-300",
                                                { format!("角色: {:?}", m.role) }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 域名管理
                    if active_tab() == "domains" {
                        div {
                            class: "text-center py-12",
                            p {
                                class: "text-gray-500 dark:text-gray-400 mb-4",
                                "管理出版物的自定义域名"
                            }
                            Link {
                                to: Route::DomainManagement { publication_id: pub_data.id.clone() },
                                class: "inline-block px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                                "打开域名管理"
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "min-h-screen flex items-center justify-center",
                p {
                    class: "text-gray-500",
                    "出版物不存在"
                }
            }
        }
    }
}

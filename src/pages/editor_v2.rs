use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::{articles::ArticleService, versions::VersionService, client::API_BASE_URL},
    models::{
        article::{Article, CreateArticleRequest, UpdateArticleRequest},
        version::{ArticleVersion, CreateVersionRequest},
    },
    components::{VersionHistory, ImageDropZone},
    hooks::use_auth,
    Route,
};
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn EditorV2Page(slug: Option<String>) -> Element {
    let mut article = use_signal(|| None::<Article>);
    let mut title = use_signal(|| String::new());
    let mut subtitle = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut excerpt = use_signal(|| String::new());
    let mut cover_image_url = use_signal(|| String::new());
    let mut tags_input = use_signal(|| String::new());
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut preview_mode = use_signal(|| false);
    let mut show_versions = use_signal(|| false);
    let mut auto_save_enabled = use_signal(|| true);
    let mut last_saved = use_signal(|| None::<String>);
    
    let auth = use_auth();
    let navigator = use_navigator();
    
    // 加载现有文章
    use_effect(move || {
        if let Some(slug) = &slug {
            let slug = slug.clone();
            spawn(async move {
                match ArticleService::get_article(&slug).await {
                    Ok(art) => {
                        title.set(art.title.clone());
                        subtitle.set(art.subtitle.clone().unwrap_or_default());
                        content.set(art.content.clone());
                        excerpt.set(art.excerpt.clone().unwrap_or_default());
                        cover_image_url.set(art.cover_image_url.clone().unwrap_or_default());
                        
                        // 设置标签
                        let tags: Vec<String> = art.tags.iter().map(|t| t.name.clone()).collect();
                        tags_input.set(tags.join(", "));
                        
                        article.set(Some(art));
                    }
                    Err(e) => {
                        error.set(Some(format!("无法加载文章: {}", e.message)));
                    }
                }
            });
        }
    });
    
    // 保存草稿
    let save_draft = move || {
        spawn(async move {
            is_saving.set(true);
            error.set(None);
            
            let tags: Vec<String> = tags_input()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if let Some(art) = article() {
                // 更新现有文章
                // 处理封面图URL，如果是相对路径则转换为完整URL
                let processed_cover_image_url = if cover_image_url().is_empty() {
                    None
                } else {
                    let url = cover_image_url();
                    if url.starts_with("/api/") {
                        // API相对路径，需要添加后端域名
                        let api_origin = API_BASE_URL.trim_end_matches("/api");
                        Some(format!("{}{}", api_origin, url))
                    } else if url.starts_with("http://") || url.starts_with("https://") {
                        // 已经是完整URL
                        Some(url)
                    } else {
                        // 其他格式，尝试作为完整URL
                        Some(url)
                    }
                };
                
                let request = UpdateArticleRequest {
                    title: Some(title()),
                    subtitle: if subtitle().is_empty() { None } else { Some(subtitle()) },
                    content: Some(content()),
                    excerpt: if excerpt().is_empty() { None } else { Some(excerpt()) },
                    cover_image_url: processed_cover_image_url,
                    tags: Some(tags),
                    is_paid_content: None,
                    series_id: None,
                    publication_id: None,
                    series_order: None,
                };
                
                match ArticleService::update_article(&art.id, &request).await {
                    Ok(_) => {
                        last_saved.set(Some("已保存".to_string()));
                        
                        // 创建版本
                        let version_request = CreateVersionRequest {
                            article_id: art.id.clone(),
                            change_summary: Some("自动保存".to_string()),
                        };
                        let _ = VersionService::create_version(&version_request).await;
                        
                        spawn(async move {
                            TimeoutFuture::new(3000).await;
                            last_saved.set(None);
                        });
                    }
                    Err(e) => {
                        error.set(Some(format!("保存失败: {}", e.message)));
                    }
                }
            } else {
                // 创建新文章
                // 处理封面图URL，如果是相对路径则转换为完整URL
                let processed_cover_image_url = if cover_image_url().is_empty() {
                    None
                } else {
                    let url = cover_image_url();
                    if url.starts_with("/api/") {
                        // API相对路径，需要添加后端域名
                        let api_origin = API_BASE_URL.trim_end_matches("/api");
                        Some(format!("{}{}", api_origin, url))
                    } else if url.starts_with("http://") || url.starts_with("https://") {
                        // 已经是完整URL
                        Some(url)
                    } else {
                        // 其他格式，尝试作为完整URL
                        Some(url)
                    }
                };
                
                let request = CreateArticleRequest {
                    title: title(),
                    subtitle: if subtitle().is_empty() { None } else { Some(subtitle()) },
                    content: content(),
                    excerpt: if excerpt().is_empty() { None } else { Some(excerpt()) },
                    cover_image_url: processed_cover_image_url,
                    publication_id: None,
                    series_id: None,
                    series_order: None,
                    is_paid_content: false,
                    tags,
                    save_as_draft: true,  // 这个参数已经不再使用，保留是为了兼容性
                    seo_title: None,
                    seo_description: None,
                    seo_keywords: None,
                };
                
                match ArticleService::create_article(&request).await {
                    Ok(created_article) => {
                        article.set(Some(created_article.clone()));
                        last_saved.set(Some("草稿已保存".to_string()));
                        
                        spawn(async move {
                            TimeoutFuture::new(3000).await;
                            last_saved.set(None);
                        });
                    }
                    Err(e) => {
                        error.set(Some(format!("创建草稿失败: {}", e.message)));
                    }
                }
            }
            
            is_saving.set(false);
        });
    };
    
    // 自动保存
    use_effect(move || {
        if auto_save_enabled() && article().is_some() {
            let handle = gloo_timers::callback::Interval::new(30_000, move || {
                if !title().is_empty() && !content().is_empty() {
                    save_draft();
                }
            });
            
            // 定时器会在组件卸载时自动清理
        }
    });
    
    // 发布文章
    let publish = move || {
        spawn(async move {
            is_saving.set(true);
            error.set(None);
            
            let tags: Vec<String> = tags_input()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if let Some(art) = article() {
                // 更新现有文章并发布
                // 处理封面图URL，如果是相对路径则转换为完整URL
                let processed_cover_image_url = if cover_image_url().is_empty() {
                    None
                } else {
                    let url = cover_image_url();
                    if url.starts_with("/api/") {
                        // API相对路径，需要添加后端域名
                        let api_origin = API_BASE_URL.trim_end_matches("/api");
                        Some(format!("{}{}", api_origin, url))
                    } else if url.starts_with("http://") || url.starts_with("https://") {
                        // 已经是完整URL
                        Some(url)
                    } else {
                        // 其他格式，尝试作为完整URL
                        Some(url)
                    }
                };
                
                let update_request = UpdateArticleRequest {
                    title: Some(title()),
                    subtitle: if subtitle().is_empty() { None } else { Some(subtitle()) },
                    content: Some(content()),
                    excerpt: if excerpt().is_empty() { None } else { Some(excerpt()) },
                    cover_image_url: processed_cover_image_url,
                    tags: Some(tags),
                    is_paid_content: None,
                    series_id: None,
                    publication_id: None,
                    series_order: None,
                };
                
                match ArticleService::update_article(&art.id, &update_request).await {
                    Ok(_) => {
                        // 发布文章
                        match ArticleService::publish_article(&art.id).await {
                            Ok(published) => {
                                // 创建版本
                                let version_request = CreateVersionRequest {
                                    article_id: art.id.clone(),
                                    change_summary: Some("发布文章".to_string()),
                                };
                                let _ = VersionService::create_version(&version_request).await;
                                
                                // 导航到文章页面
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href(&format!("/article/{}", published.slug))
                                    .ok();
                            }
                            Err(e) => {
                                error.set(Some(format!("发布失败: {}", e.message)));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("更新失败: {}", e.message)));
                    }
                }
            } else {
                // 创建新文章并直接发布
                // 处理封面图URL，如果是相对路径则转换为完整URL
                let processed_cover_image_url = if cover_image_url().is_empty() {
                    None
                } else {
                    let url = cover_image_url();
                    if url.starts_with("/api/") {
                        // API相对路径，需要添加后端域名
                        let api_origin = API_BASE_URL.trim_end_matches("/api");
                        Some(format!("{}{}", api_origin, url))
                    } else if url.starts_with("http://") || url.starts_with("https://") {
                        // 已经是完整URL
                        Some(url)
                    } else {
                        // 其他格式，尝试作为完整URL
                        Some(url)
                    }
                };
                
                let request = CreateArticleRequest {
                    title: title(),
                    subtitle: if subtitle().is_empty() { None } else { Some(subtitle()) },
                    content: content(),
                    excerpt: if excerpt().is_empty() { None } else { Some(excerpt()) },
                    cover_image_url: processed_cover_image_url,
                    publication_id: None,
                    series_id: None,
                    series_order: None,
                    is_paid_content: false,
                    tags,
                    save_as_draft: true,  // create 接口总是创建草稿
                    seo_title: None,
                    seo_description: None,
                    seo_keywords: None,
                };
                
                match ArticleService::create_article(&request).await {
                    Ok(created_article) => {
                        // 创建成功后，立即发布文章
                        web_sys::console::log_1(&format!("Created article with ID: {}", created_article.id).into());
                        
                        match ArticleService::publish_article(&created_article.id).await {
                            Ok(published) => {
                                // 导航到文章页面
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href(&format!("/article/{}", published.slug))
                                    .ok();
                            }
                            Err(e) => {
                                error.set(Some(format!("发布失败: {}", e.message)));
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("创建文章失败: {}", e.message)));
                    }
                }
            }
            
            is_saving.set(false);
        });
    };
    
    // 恢复版本
    let handle_restore_version = move |version: ArticleVersion| {
        title.set(version.title);
        subtitle.set(version.subtitle.unwrap_or_default());
        content.set(version.content);
        excerpt.set(version.excerpt);
        cover_image_url.set(version.cover_image_url.unwrap_or_default());
        tags_input.set(version.tags.join(", "));
        show_versions.set(false);
        
        // 保存恢复的内容
        save_draft();
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white dark:bg-gray-900",
            
            // 顶部工具栏
            header {
                class: "border-b border-gray-200 dark:border-gray-700 sticky top-0 bg-white dark:bg-gray-900 z-40",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        
                        // 左侧Logo
                        Link {
                            to: Route::Home {},
                            class: "text-xl font-serif font-bold text-gray-900 dark:text-white",
                            "Rainbow Blog"
                        }
                        
                        // 右侧操作按钮
                        div {
                            class: "flex items-center space-x-4",
                            
                            // 自动保存状态
                            if let Some(saved_text) = last_saved() {
                                span {
                                    class: "text-sm text-gray-500",
                                    {saved_text}
                                }
                            }
                            
                            // 版本历史按钮
                            if article().is_some() {
                                button {
                                    class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                    onclick: move |_| show_versions.set(true),
                                    svg {
                                        class: "w-5 h-5",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                        }
                                    }
                                }
                            }
                            
                            // 预览切换
                            button {
                                class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                onclick: move |_| preview_mode.set(!preview_mode()),
                                if preview_mode() { "编辑" } else { "预览" }
                            }
                            
                            // 保存草稿按钮
                            button {
                                class: "px-4 py-2 text-sm text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 disabled:opacity-50",
                                disabled: is_saving() || title().is_empty() || content().is_empty(),
                                onclick: move |_| save_draft(),
                                if is_saving() { "保存中..." } else { "保存草稿" }
                            }
                            
                            // 发布按钮
                            button {
                                class: "px-4 py-2 text-sm text-white bg-green-600 rounded-full hover:bg-green-700 disabled:opacity-50",
                                disabled: is_saving() || title().is_empty() || content().is_empty(),
                                onclick: move |_| publish(),
                                "发布"
                            }
                        }
                    }
                }
            }
            
            // 主编辑区
            div {
                class: "max-w-4xl mx-auto px-4 py-8",
                
                // 错误提示
                if let Some(err) = error() {
                    div {
                        class: "mb-6 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded",
                        {err}
                    }
                }
                
                if !preview_mode() {
                    // 编辑模式
                    div {
                        // 标题输入
                        input {
                            r#type: "text",
                            placeholder: "输入标题...",
                            value: "{title}",
                            oninput: move |evt| title.set(evt.value()),
                            class: "w-full text-4xl font-serif font-bold placeholder-gray-400 dark:placeholder-gray-600 bg-transparent text-gray-900 dark:text-white border-0 outline-none mb-4",
                            maxlength: "150"
                        }
                        
                        // 副标题输入
                        input {
                            r#type: "text",
                            placeholder: "输入副标题（可选）...",
                            value: "{subtitle}",
                            oninput: move |evt| subtitle.set(evt.value()),
                            class: "w-full text-xl text-gray-600 dark:text-gray-400 placeholder-gray-400 dark:placeholder-gray-600 bg-transparent border-0 outline-none mb-6",
                            maxlength: "200"
                        }
                        
                        // 封面图片上传
                        div {
                            class: "mb-6",
                            h3 {
                                class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "封面图片"
                            }
                            ImageDropZone {
                                on_upload: move |url| cover_image_url.set(url),
                                on_error: move |err| error.set(Some(err)),
                                current_image: if cover_image_url().is_empty() { None } else { Some(cover_image_url()) },
                                placeholder_text: "点击或拖拽图片到此处上传封面图片".to_string()
                            }
                        }
                        
                        // 文章内容
                        textarea {
                            placeholder: "开始写作...",
                            value: "{content}",
                            oninput: move |evt| content.set(evt.value()),
                            class: "w-full min-h-[400px] text-lg leading-relaxed placeholder-gray-400 dark:placeholder-gray-600 bg-transparent text-gray-900 dark:text-white border-0 outline-none resize-none mb-6",
                            style: "font-family: 'Georgia', serif;"
                        }
                        
                        // 摘要
                        textarea {
                            placeholder: "文章摘要（用于预览）...",
                            value: "{excerpt}",
                            oninput: move |evt| excerpt.set(evt.value()),
                            class: "w-full h-20 text-sm text-gray-600 dark:text-gray-400 placeholder-gray-400 dark:placeholder-gray-600 bg-transparent border border-gray-300 dark:border-gray-700 rounded-lg p-3 outline-none resize-none mb-6"
                        }
                        
                        // 标签输入
                        input {
                            r#type: "text",
                            placeholder: "标签（用逗号分隔）...",
                            value: "{tags_input}",
                            oninput: move |evt| tags_input.set(evt.value()),
                            class: "w-full text-sm text-gray-600 dark:text-gray-400 placeholder-gray-400 dark:placeholder-gray-600 bg-transparent border border-gray-300 dark:border-gray-700 rounded-lg p-3 outline-none"
                        }
                    }
                } else {
                    // 预览模式
                    article {
                        class: "prose prose-lg dark:prose-invert max-w-none",
                        
                        h1 {
                            class: "text-4xl font-serif font-bold mb-4",
                            {title()}
                        }
                        
                        if !subtitle().is_empty() {
                            h2 {
                                class: "text-xl text-gray-600 dark:text-gray-400 mb-6",
                                {subtitle()}
                            }
                        }
                        
                        if !cover_image_url().is_empty() {
                            img {
                                src: "{cover_image_url}",
                                alt: "{title}",
                                class: "w-full rounded-lg mb-8"
                            }
                        }
                        
                        div {
                            class: "whitespace-pre-wrap",
                            {content()}
                        }
                        
                        if !tags_input().is_empty() {
                            div {
                                class: "mt-8 flex flex-wrap gap-2",
                                for tag in tags_input().split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                                    span {
                                        class: "px-3 py-1 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-full text-sm",
                                        {tag}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 版本历史侧边栏
            if let Some(art) = article() {
                VersionHistory {
                    article_id: art.id.clone(),
                    show: show_versions(),
                    on_close: move |_| show_versions.set(false),
                    on_restore: handle_restore_version
                }
            }
        }
    }
}
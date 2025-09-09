use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::articles::ArticleService,
    models::article::CreateArticleRequest,
    hooks::use_auth,
    Route,
};

#[component]
pub fn EditorPage() -> Element {
    let mut title = use_signal(|| String::new());
    let mut subtitle = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut excerpt = use_signal(|| String::new());
    let mut cover_image_url = use_signal(|| String::new());
    let mut tags_input = use_signal(|| String::new());
    let mut is_paid_content = use_signal(|| false);
    let mut save_as_draft = use_signal(|| true);
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut preview_mode = use_signal(|| false);
    
    let auth = use_auth();
    let navigator = use_navigator();
    
    // 检查认证状态
    use_effect(move || {
        if !auth.read().is_authenticated {
            navigator.push(Route::Login {});
        }
    });
    
    // 自动保存草稿
    use_effect(move || {
        // 每30秒自动保存一次草稿
        let handle = gloo_timers::callback::Interval::new(30_000, move || {
            if !title().is_empty() && !content().is_empty() && save_as_draft() {
                // 这里可以实现自动保存逻辑
                log::info!("Auto-saving draft...");
            }
        });
        
        // 定时器会在组件卸载时自动清理
    });
    
    // 处理发布
    let handle_publish = move |is_draft: bool| {
        spawn(async move {
            is_saving.set(true);
            error.set(None);
            save_as_draft.set(is_draft);
            
            let tags: Vec<String> = tags_input()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            let request = CreateArticleRequest {
                title: title(),
                subtitle: if subtitle().is_empty() { None } else { Some(subtitle()) },
                content: content(),
                excerpt: if excerpt().is_empty() { None } else { Some(excerpt()) },
                cover_image_url: if cover_image_url().is_empty() { None } else { Some(cover_image_url()) },
                publication_id: None,
                series_id: None,
                series_order: None,
                is_paid_content: is_paid_content(),
                tags,
                save_as_draft: is_draft,
            };
            
            match ArticleService::create_article(&request).await {
                Ok(article) => {
                    if !is_draft {
                        // 发布文章
                        match ArticleService::publish_article(&article.id).await {
                            Ok(_) => {
                                navigator.push(format!("/article/{}", article.slug));
                            }
                            Err(e) => {
                                error.set(Some(format!("发布失败: {}", e.message)));
                            }
                        }
                    } else {
                        // 保存为草稿，跳转到草稿列表或继续编辑
                        navigator.push(Route::Home {});
                    }
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            
            is_saving.set(false);
        });
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 顶部工具栏
            header {
                class: "border-b border-gray-100 sticky top-0 bg-white z-10",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        
                        // 左侧Logo
                        Link {
                            to: Route::Home {},
                            class: "text-xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                        
                        // 右侧操作按钮
                        div {
                            class: "flex items-center space-x-4",
                            
                            // 预览切换
                            button {
                                class: "text-sm text-gray-600 hover:text-gray-900",
                                onclick: move |_| preview_mode.set(!preview_mode()),
                                if preview_mode() { "编辑" } else { "预览" }
                            }
                            
                            // 保存草稿按钮
                            button {
                                class: "px-4 py-2 text-sm text-gray-700 bg-gray-100 rounded-full hover:bg-gray-200 disabled:opacity-50",
                                disabled: is_saving() || title().is_empty() || content().is_empty(),
                                onclick: move |_| handle_publish(true),
                                if is_saving() && save_as_draft() { "保存中..." } else { "保存草稿" }
                            }
                            
                            // 发布按钮
                            button {
                                class: "px-4 py-2 text-sm text-white bg-green-600 rounded-full hover:bg-green-700 disabled:opacity-50",
                                disabled: is_saving() || title().is_empty() || content().is_empty(),
                                onclick: move |_| handle_publish(false),
                                if is_saving() && !save_as_draft() { "发布中..." } else { "发布" }
                            }
                            
                            // 用户头像
                            if let Some(user) = &auth.read().user {
                                if let Some(avatar_url) = &user.avatar_url {
                                    img {
                                        src: "{avatar_url}",
                                        alt: "{user.username}",
                                        class: "w-8 h-8 rounded-full"
                                    }
                                } else {
                                    div {
                                        class: "w-8 h-8 rounded-full bg-gray-300"
                                    }
                                }
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
                        class: "mb-6 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
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
                            class: "w-full text-4xl font-serif font-bold placeholder-gray-400 border-0 outline-none mb-4",
                            maxlength: "150"
                        }
                        
                        // 副标题输入
                        input {
                            r#type: "text",
                            placeholder: "输入副标题（可选）...",
                            value: "{subtitle}",
                            oninput: move |evt| subtitle.set(evt.value()),
                            class: "w-full text-xl text-gray-600 placeholder-gray-400 border-0 outline-none mb-6",
                            maxlength: "200"
                        }
                        
                        // 封面图URL
                        div {
                            class: "mb-6",
                            input {
                                r#type: "url",
                                placeholder: "封面图URL（可选）...",
                                value: "{cover_image_url}",
                                oninput: move |evt| cover_image_url.set(evt.value()),
                                class: "w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                            }
                            if !cover_image_url().is_empty() {
                                div {
                                    class: "mt-2",
                                    img {
                                        src: "{cover_image_url}",
                                        alt: "封面图预览",
                                        class: "max-h-48 rounded-lg",
                                        onerror: move |_| cover_image_url.set(String::new())
                                    }
                                }
                            }
                        }
                        
                        // 内容编辑器
                        div {
                            class: "mb-6",
                            textarea {
                                placeholder: "开始写作...\n\n支持 Markdown 语法：\n# 标题\n**粗体** *斜体*\n- 列表项\n[链接](url)\n![图片](url)\n`代码`\n```代码块```",
                                value: "{content}",
                                oninput: move |evt| content.set(evt.value()),
                                class: "w-full min-h-[500px] px-0 py-2 text-lg leading-relaxed placeholder-gray-400 border-0 outline-none resize-none",
                                maxlength: "50000"
                            }
                        }
                        
                        // 高级选项（折叠面板）
                        details {
                            class: "mb-6 border-t border-gray-200 pt-6",
                            summary {
                                class: "cursor-pointer text-sm font-medium text-gray-700 mb-4",
                                "高级选项"
                            }
                            
                            // 摘要输入
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-2",
                                    "文章摘要"
                                }
                                textarea {
                                    placeholder: "输入文章摘要（可选，将显示在文章列表中）...",
                                    value: "{excerpt}",
                                    oninput: move |evt| excerpt.set(evt.value()),
                                    class: "w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent",
                                    rows: "3",
                                    maxlength: "300"
                                }
                            }
                            
                            // 标签输入
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-2",
                                    "标签"
                                }
                                input {
                                    r#type: "text",
                                    placeholder: "输入标签，用逗号分隔...",
                                    value: "{tags_input}",
                                    oninput: move |evt| tags_input.set(evt.value()),
                                    class: "w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent"
                                }
                            }
                            
                            // 付费内容选项
                            div {
                                class: "mb-4",
                                label {
                                    class: "flex items-center",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_paid_content(),
                                        oninput: move |evt| is_paid_content.set(evt.checked()),
                                        class: "mr-2"
                                    }
                                    span {
                                        class: "text-sm text-gray-700",
                                        "设为付费内容"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // 预览模式
                    div {
                        class: "prose prose-lg max-w-none",
                        
                        // 预览标题
                        h1 {
                            class: "text-4xl font-serif font-bold mb-4",
                            {title()}
                        }
                        
                        // 预览副标题
                        if !subtitle().is_empty() {
                            h2 {
                                class: "text-xl text-gray-600 mb-6",
                                {subtitle()}
                            }
                        }
                        
                        // 预览封面图
                        if !cover_image_url().is_empty() {
                            img {
                                src: "{cover_image_url}",
                                alt: "封面图",
                                class: "w-full rounded-lg mb-8"
                            }
                        }
                        
                        // 预览内容（这里应该渲染Markdown，但为了简化先显示原始内容）
                        div {
                            class: "whitespace-pre-wrap",
                            {content()}
                        }
                    }
                }
            }
        }
    }
}
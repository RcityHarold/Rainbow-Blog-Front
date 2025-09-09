use dioxus::prelude::*;
use crate::{
    models::comment::{Comment, CommentWithAuthor},
    api::comments::{CommentService, CreateCommentRequest},
    hooks::use_auth,
};
use chrono::{DateTime, Utc};
use gloo_timers::future::TimeoutFuture;
use web_sys;

#[component]
pub fn CommentSection(article_id: String) -> Element {
    let mut comments = use_signal(|| Vec::<CommentWithAuthor>::new());
    let mut loading = use_signal(|| true);
    let mut sort_by = use_signal(|| "newest");
    let mut show_reply_form = use_signal(|| None::<String>);
    
    let auth = use_auth();
    let article_id_for_effect = article_id.clone();
    
    // 初始加载和处理排序变化
    use_effect(move || {
        let article_id = article_id_for_effect.clone();
        spawn(async move {
            loading.set(true);
            
            match CommentService::get_article_comments(&article_id, None, None, None).await {
                Ok(comment_list) => {
                    comments.set(comment_list);
                }
                Err(_) => {
                    // 处理错误
                }
            }
            
            loading.set(false);
        });
    });
    
    // 计算总评论数（包括所有层级）
    fn count_all_comments(comments: &[CommentWithAuthor]) -> usize {
        comments.iter().map(|c| {
            1 + count_all_comments(&c.replies)
        }).sum()
    }
    
    let total_comments = count_all_comments(&comments());
    
    rsx! {
        div {
            class: "mt-12 border-t border-gray-200 dark:border-gray-700 pt-8",
            
            // 评论头部
            div {
                class: "flex items-center justify-between mb-6",
                h3 {
                    class: "text-lg font-semibold text-gray-900 dark:text-white",
                    "评论 ({total_comments})"
                }
                
                // 排序选项
                div {
                    class: "flex items-center space-x-4",
                    button {
                        class: if sort_by() == "newest" { 
                            "font-medium text-gray-900 dark:text-white" 
                        } else { 
                            "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300" 
                        },
                        onclick: move |_| sort_by.set("newest"),
                        "最新"
                    }
                    button {
                        class: if sort_by() == "popular" { 
                            "font-medium text-gray-900 dark:text-white" 
                        } else { 
                            "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300" 
                        },
                        onclick: move |_| sort_by.set("popular"),
                        "热门"
                    }
                }
            }
            
            // 评论输入框（顶级评论）
            if auth.read().is_authenticated {
                CommentForm {
                    article_id: article_id.clone(),
                    parent_id: None,
                    on_success: {
                        let article_id = article_id.clone();
                        move |_| {
                            let article_id = article_id.clone();
                            spawn(async move {
                                loading.set(true);
                                if let Ok(comment_list) = CommentService::get_article_comments(&article_id, None, None, None).await {
                                    comments.set(comment_list);
                                }
                                loading.set(false);
                            });
                        }
                    }
                }
            } else {
                div {
                    class: "text-center py-8 bg-gray-50 dark:bg-gray-800 rounded-lg",
                    p {
                        class: "text-gray-600 dark:text-gray-400",
                        "请 "
                        button {
                            class: "text-blue-600 hover:underline",
                            onclick: move |_| {
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href("/login")
                                    .ok();
                            },
                            "登录"
                        }
                        " 后发表评论"
                    }
                }
            }
            
            // 评论列表
            if loading() {
                div {
                    class: "flex justify-center py-8",
                    div {
                        class: "animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900 dark:border-white"
                    }
                }
            } else if comments().is_empty() {
                div {
                    class: "text-center py-12",
                    p {
                        class: "text-gray-500 dark:text-gray-400",
                        "暂无评论，来发表第一条评论吧！"
                    }
                }
            } else {
                div {
                    class: "space-y-6 mt-6",
                    for comment in comments() {
                        CommentThread {
                            comment_with_replies: comment.clone(),
                            article_id: article_id.clone(),
                            show_reply_form: show_reply_form.clone(),
                            on_reply_success: {
                                let article_id = article_id.clone();
                                move || {
                                    let article_id = article_id.clone();
                                    spawn(async move {
                                        loading.set(true);
                                        if let Ok(comment_list) = CommentService::get_article_comments(&article_id, None, None, None).await {
                                            comments.set(comment_list);
                                        }
                                        loading.set(false);
                                    });
                                }
                            },
                            depth: 0,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentThread(
    comment_with_replies: CommentWithAuthor,
    article_id: String,
    show_reply_form: Signal<Option<String>>,
    on_reply_success: EventHandler<()>,
    depth: usize,
) -> Element {
    let comment = comment_with_replies.comment.clone();
    let replies = comment_with_replies.replies.clone();
    
    // 限制最大缩进深度，超过后不再缩进但仍可继续回复
    let max_indent_depth = 5;
    let indent_class = if depth < max_indent_depth {
        format!("ml-{}", depth * 8)
    } else {
        format!("ml-{}", max_indent_depth * 8)
    };
    
    rsx! {
        div {
            class: if depth > 0 { 
                format!("{} border-l-2 border-gray-100 dark:border-gray-700 pl-4", indent_class) 
            } else { 
                "".to_string() 
            },
            
            // 评论内容
            CommentItem {
                comment: comment.clone(),
                comment_with_author: comment_with_replies.clone(),
                article_id: article_id.clone(),
                show_reply_form: show_reply_form.clone(),
                on_reply_success: on_reply_success.clone(),
                depth: depth,
            }
            
            // 嵌套的回复
            if !replies.is_empty() {
                div {
                    class: "mt-4 space-y-4",
                    for reply in replies {
                        CommentThread {
                            comment_with_replies: reply,
                            article_id: article_id.clone(),
                            show_reply_form: show_reply_form.clone(),
                            on_reply_success: on_reply_success.clone(),
                            depth: depth + 1,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    comment: Comment,
    comment_with_author: CommentWithAuthor,
    article_id: String,
    show_reply_form: Signal<Option<String>>,
    on_reply_success: EventHandler<()>,
    depth: usize,
) -> Element {
    let mut is_liked = use_signal(|| comment_with_author.user_has_clapped);
    let mut like_count = use_signal(|| comment.clap_count);
    
    let comment_id = comment.id.clone();
    let comment_id_for_like = comment_id.clone();
    let comment_id_for_reply = comment_id.clone();
    let comment_id_for_form_check = comment_id.clone();
    let comment_id_for_form = comment_id.clone();
    let auth = use_auth();
    
    // 处理点赞
    let handle_like = move |_| {
        if !auth.read().is_authenticated {
            web_sys::window()
                .unwrap()
                .location()
                .set_href("/login")
                .ok();
            return;
        }
        
        let comment_id = comment_id_for_like.clone();
        let liked = is_liked();
        
        spawn(async move {
            let result = if liked {
                CommentService::unlike_comment(&comment_id).await
            } else {
                CommentService::like_comment(&comment_id).await
            };
            
            if result.is_ok() {
                is_liked.set(!liked);
                if liked {
                    like_count.set(like_count() - 1);
                } else {
                    like_count.set(like_count() + 1);
                }
            }
        });
    };
    
    rsx! {
        div {
            class: "flex space-x-3",
            
            // 用户头像
            if let Some(avatar_url) = &comment_with_author.author_avatar {
                img {
                    src: "{avatar_url}",
                    alt: "{comment_with_author.author_username}",
                    class: "w-10 h-10 rounded-full flex-shrink-0"
                }
            } else {
                div {
                    class: "w-10 h-10 rounded-full bg-gray-200 dark:bg-gray-600 flex-shrink-0"
                }
            }
            
            // 评论内容
            div {
                class: "flex-1 min-w-0",
                
                // 作者信息
                div {
                    class: "flex flex-wrap items-center mb-1 gap-x-2",
                    button {
                        class: "font-medium text-gray-900 dark:text-white hover:underline",
                        onclick: move |_| {
                            web_sys::window()
                                .unwrap()
                                .location()
                                .set_href(&format!("/@{}", comment_with_author.author_username))
                                .ok();
                        },
                        {comment_with_author.author_name.clone()}
                    }
                    
                    // 深度指示器（可选）
                    if depth > 0 {
                        span {
                            class: "text-xs text-gray-400 dark:text-gray-500",
                            "L{depth}"
                        }
                    }
                    
                    span {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        {format_relative_time(&comment.created_at)}
                    }
                    
                    if comment.is_edited {
                        span {
                            class: "text-sm text-gray-500 dark:text-gray-400",
                            "（已编辑）"
                        }
                    }
                }
                
                // 评论内容
                p {
                    class: "text-gray-800 dark:text-gray-200 whitespace-pre-wrap break-words",
                    dangerous_inner_html: "{markdown_to_html(&comment.content)}"
                }
                
                // 操作按钮
                div {
                    class: "flex items-center space-x-4 mt-2",
                    
                    // 点赞按钮
                    button {
                        class: "flex items-center space-x-1 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300",
                        onclick: handle_like,
                        svg {
                            class: if is_liked() { 
                                "w-4 h-4 text-red-500 fill-current" 
                            } else { 
                                "w-4 h-4" 
                            },
                            fill: if is_liked() { "currentColor" } else { "none" },
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                            }
                        }
                        span { {like_count().to_string()} }
                    }
                    
                    // 回复按钮 - 所有评论都可以被回复
                    button {
                        class: "text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300",
                        onclick: {
                            let comment_id = comment_id_for_reply.clone();
                            move |_| {
                                if auth.read().is_authenticated {
                                    show_reply_form.set(Some(comment_id.clone()));
                                } else {
                                    web_sys::window()
                                        .unwrap()
                                        .location()
                                        .set_href("/login")
                                        .ok();
                                }
                            }
                        },
                        "回复"
                    }
                    
                    // 回复数量显示
                    if !comment_with_author.replies.is_empty() {
                        span {
                            class: "text-sm text-gray-500 dark:text-gray-400",
                            "{comment_with_author.replies.len()} 条回复"
                        }
                    }
                }
                
                // 回复表单
                if show_reply_form() == Some(comment_id_for_form_check.clone()) {
                    div {
                        class: "mt-4",
                        CommentForm {
                            article_id: article_id.clone(),
                            parent_id: Some(comment_id_for_form.clone()),
                            on_success: move |_| {
                                show_reply_form.set(None);
                                on_reply_success.call(());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentForm(
    article_id: String,
    parent_id: Option<String>,
    on_success: EventHandler<Comment>,
) -> Element {
    let mut content = use_signal(|| String::new());
    let mut submitting = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    
    let parent_id_clone = parent_id.clone();
    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        
        let comment_content = content();
        if comment_content.trim().is_empty() {
            return;
        }
        
        submitting.set(true);
        error.set(None);
        
        let request = CreateCommentRequest {
            article_id: article_id.clone(),
            content: comment_content,
            parent_id: parent_id_clone.clone(),
        };
        
        spawn(async move {
            match CommentService::create_comment(&request).await {
                Ok(comment) => {
                    content.set(String::new());
                    on_success.call(comment);
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            submitting.set(false);
        });
    };
    
    rsx! {
        form {
            onsubmit: handle_submit,
            class: "space-y-4",
            
            textarea {
                value: "{content}",
                oninput: move |e| content.set(e.value()),
                placeholder: if parent_id.is_some() { 
                    "写下你的回复..." 
                } else { 
                    "写下你的评论..." 
                },
                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-800 dark:text-white",
                rows: "3",
                disabled: submitting()
            }
            
            if let Some(err) = error() {
                p {
                    class: "text-sm text-red-600 dark:text-red-400",
                    {err}
                }
            }
            
            div {
                class: "flex items-center justify-between",
                p {
                    class: "text-sm text-gray-500 dark:text-gray-400",
                    "支持 Markdown 格式"
                }
                
                div {
                    class: "flex items-center space-x-2",
                    if parent_id.is_some() {
                        button {
                            r#type: "button",
                            class: "text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300",
                            onclick: move |_| on_success.call(Comment::default()),
                            "取消"
                        }
                    }
                    button {
                        r#type: "submit",
                        class: "px-4 py-2 bg-green-600 text-white rounded-full text-sm hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: submitting() || content().trim().is_empty(),
                        if submitting() {
                            "发布中..."
                        } else {
                            "发布"
                        }
                    }
                }
            }
        }
    }
}

fn format_relative_time(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*datetime);
    
    if duration.num_seconds() < 60 {
        "刚刚".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} 分钟前", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} 小时前", duration.num_hours())
    } else if duration.num_days() < 30 {
        format!("{} 天前", duration.num_days())
    } else if duration.num_days() < 365 {
        format!("{} 个月前", duration.num_days() / 30)
    } else {
        format!("{} 年前", duration.num_days() / 365)
    }
}

fn markdown_to_html(markdown: &str) -> String {
    // 简单的 Markdown 处理，实际应该使用更完整的 Markdown 解析器
    markdown
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\n", "<br>")
}
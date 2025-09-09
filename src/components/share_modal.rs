use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn ShareModal(
    show: bool,
    on_close: EventHandler<()>,
    article_url: String,
    article_title: String,
) -> Element {
    let mut copy_message = use_signal(|| false);
    let article_url_clone = article_url.clone();
    let article_title_clone = article_title.clone();
    
    // 复制链接到剪贴板
    let copy_to_clipboard = move |_| {
        if let Some(window) = window() {
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let url = article_url_clone.clone();
            spawn(async move {
                // 使用 JavaScript 的 navigator.clipboard.writeText
                let _ = wasm_bindgen_futures::JsFuture::from(
                    clipboard.write_text(&url)
                ).await;
                
                copy_message.set(true);
                
                // 3秒后隐藏提示
                gloo_timers::future::TimeoutFuture::new(2000).await;
                copy_message.set(false);
            });
        }
    };
    
    // 分享到不同平台的URL
    let twitter_url = format!(
        "https://twitter.com/intent/tweet?text={}&url={}",
        urlencoding::encode(&article_title_clone),
        urlencoding::encode(&article_url)
    );
    
    let facebook_url = format!(
        "https://www.facebook.com/sharer/sharer.php?u={}",
        urlencoding::encode(&article_url)
    );
    
    let linkedin_url = format!(
        "https://www.linkedin.com/sharing/share-offsite/?url={}",
        urlencoding::encode(&article_url)
    );
    
    let email_url = format!(
        "mailto:?subject={}&body={}",
        urlencoding::encode(&article_title),
        urlencoding::encode(&format!("我想和你分享这篇文章：{}", article_url))
    );
    
    if !show {
        return rsx! {};
    }
    
    rsx! {
        // 背景遮罩
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            aria_labelledby: "modal-title",
            role: "dialog",
            aria_modal: "true",
            
            div {
                class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                
                // 背景遮罩
                div {
                    class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                    aria_hidden: "true",
                    onclick: move |_| on_close.call(())
                }
                
                // 居中定位
                span {
                    class: "hidden sm:inline-block sm:align-middle sm:h-screen",
                    aria_hidden: "true",
                    "​"
                }
                
                // Modal内容
                div {
                    class: "inline-block align-bottom bg-white dark:bg-gray-800 rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    
                    // 标题栏
                    div {
                        class: "bg-gray-50 dark:bg-gray-900 px-4 py-3 sm:px-6",
                        div {
                            class: "flex items-center justify-between",
                            h3 {
                                class: "text-lg font-medium text-gray-900 dark:text-white",
                                id: "modal-title",
                                "分享文章"
                            }
                            button {
                                class: "text-gray-400 hover:text-gray-500",
                                onclick: move |_| on_close.call(()),
                                svg {
                                    class: "h-5 w-5",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12"
                                    }
                                }
                            }
                        }
                    }
                    
                    // 内容区
                    div {
                        class: "bg-white dark:bg-gray-800 px-4 pt-5 pb-4 sm:p-6",
                        
                        // 社交媒体分享按钮
                        div {
                            class: "space-y-4",
                            
                            h4 {
                                class: "text-sm font-medium text-gray-900 dark:text-white mb-3",
                                "分享到社交媒体"
                            }
                            
                            div {
                                class: "grid grid-cols-2 gap-3",
                                
                                // Twitter
                                a {
                                    href: twitter_url,
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "flex items-center justify-center px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600",
                                    svg {
                                        class: "h-5 w-5 mr-2",
                                        fill: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            d: "M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z"
                                        }
                                    }
                                    "Twitter"
                                }
                                
                                // Facebook
                                a {
                                    href: facebook_url,
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "flex items-center justify-center px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600",
                                    svg {
                                        class: "h-5 w-5 mr-2",
                                        fill: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            d: "M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"
                                        }
                                    }
                                    "Facebook"
                                }
                                
                                // LinkedIn
                                a {
                                    href: linkedin_url,
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "flex items-center justify-center px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600",
                                    svg {
                                        class: "h-5 w-5 mr-2",
                                        fill: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"
                                        }
                                    }
                                    "LinkedIn"
                                }
                                
                                // Email
                                a {
                                    href: email_url,
                                    class: "flex items-center justify-center px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600",
                                    svg {
                                        class: "h-5 w-5 mr-2",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                        }
                                    }
                                    "邮件"
                                }
                            }
                            
                            // 复制链接
                            div {
                                class: "mt-6",
                                h4 {
                                    class: "text-sm font-medium text-gray-900 dark:text-white mb-3",
                                    "或复制链接"
                                }
                                
                                div {
                                    class: "flex",
                                    input {
                                        r#type: "text",
                                        value: "{article_url}",
                                        readonly: true,
                                        class: "flex-1 block w-full rounded-l-md border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white sm:text-sm",
                                        onfocus: move |_| {
                                            // 选中文本的功能需要通过 JavaScript 实现
                                        }
                                    }
                                    button {
                                        class: "inline-flex items-center px-4 py-2 border border-l-0 border-gray-300 dark:border-gray-600 rounded-r-md bg-gray-50 dark:bg-gray-700 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600",
                                        onclick: copy_to_clipboard,
                                        if copy_message() {
                                            "已复制!"
                                        } else {
                                            "复制"
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
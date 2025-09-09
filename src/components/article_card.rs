use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::models::article::Article;

#[component]
pub fn ArticleCard(article: Article) -> Element {
    let reading_time_text = format!("{} min read", article.reading_time);
    let published_date = article.published_at
        .map(|d| d.format("%b %d").to_string())
        .unwrap_or_default();
    
    rsx! {
        article {
            class: "py-6 sm:py-8 border-b border-gray-100 hover:bg-gray-50 transition-colors cursor-pointer",
            onclick: move |_| {
                // 导航到文章详情页
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(&format!("/article/{}", article.slug))
                    .ok();
            },
            
            div {
                class: "flex flex-col sm:flex-row sm:items-start sm:justify-between",
                
                // 左侧内容区域
                div {
                    class: "flex-1 sm:pr-8 order-2 sm:order-1",
                    
                    // 作者信息
                    div {
                        class: "flex items-center mb-2",
                        if let Some(avatar_url) = &article.author.avatar_url {
                            img {
                                src: "{avatar_url}",
                                alt: "{article.author.display_name}",
                                class: "w-6 h-6 rounded-full mr-2"
                            }
                        }
                        a {
                            href: "/@{article.author.username}",
                            onclick: move |e| {
                                e.stop_propagation();
                            },
                            class: "text-sm text-gray-700 hover:text-gray-900",
                            {article.author.display_name}
                        }
                        if article.author.is_verified {
                            svg {
                                class: "w-4 h-4 text-green-500 ml-1",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }
                        if let Some(publication) = &article.publication {
                            span {
                                class: "text-sm text-gray-700 ml-1",
                                " in "
                                span {
                                    class: "font-medium",
                                    {publication.name.clone()}
                                }
                            }
                        }
                    }
                    
                    // 标题
                    h2 {
                        class: "text-xl font-bold text-gray-900 mb-2 line-clamp-2",
                        {article.title.clone()}
                    }
                    
                    // 副标题或摘要
                    if let Some(subtitle) = &article.subtitle {
                        h3 {
                            class: "text-base text-gray-600 mb-3 line-clamp-2",
                            {subtitle.clone()}
                        }
                    } else {
                        p {
                            class: "text-base text-gray-600 mb-3 line-clamp-2",
                            {article.excerpt.clone()}
                        }
                    }
                    
                    // 底部元数据
                    div {
                        class: "flex items-center text-sm text-gray-500",
                        span { {published_date} }
                        span { class: "mx-2", "·" }
                        span { {reading_time_text} }
                        
                        // 标签
                        if !article.tags.is_empty() {
                            span { class: "mx-2", "·" }
                            div {
                                class: "flex items-center space-x-2",
                                for tag in article.tags.iter().take(2) {
                                    span {
                                        class: "bg-gray-100 px-2 py-1 rounded text-xs",
                                        {tag.name.clone()}
                                    }
                                }
                            }
                        }
                        
                        // 互动数据
                        div {
                            class: "ml-auto flex items-center space-x-4",
                            
                            // 点赞数
                            if article.clap_count > 0 {
                                div {
                                    class: "flex items-center space-x-1",
                                    svg {
                                        class: "w-4 h-4",
                                        fill: "currentColor",
                                        view_box: "0 0 20 20",
                                        path {
                                            d: "M10 12a2 2 0 100-4 2 2 0 000 4z"
                                        }
                                        path {
                                            fill_rule: "evenodd",
                                            d: "M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z",
                                            clip_rule: "evenodd"
                                        }
                                    }
                                    span { {article.clap_count.to_string()} }
                                }
                            }
                            
                            // 评论数
                            if article.comment_count > 0 {
                                div {
                                    class: "flex items-center space-x-1",
                                    svg {
                                        class: "w-4 h-4",
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
                                    span { {article.comment_count.to_string()} }
                                }
                            }
                            
                            // 书签按钮
                            button {
                                class: "p-1",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    // 处理书签点击
                                },
                                svg {
                                    class: if article.is_bookmarked { "w-5 h-5 text-gray-900 fill-current" } else { "w-5 h-5 text-gray-500" },
                                    fill: if article.is_bookmarked { "currentColor" } else { "none" },
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
                        }
                    }
                }
                
                // 右侧封面图
                if let Some(cover_url) = &article.cover_image_url {
                    div {
                        class: "flex-shrink-0 mb-4 sm:mb-0 order-1 sm:order-2",
                        img {
                            src: "{cover_url}",
                            alt: "{article.title}",
                            class: "w-full sm:w-32 h-48 sm:h-32 object-cover rounded"
                        }
                    }
                }
            }
        }
    }
}
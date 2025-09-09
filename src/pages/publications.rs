use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::publications::PublicationService,
    models::publication::Publication,
    hooks::use_auth,
    Route,
};

#[component]
pub fn PublicationsPage() -> Element {
    let mut publications = use_signal(|| Vec::<Publication>::new());
    let mut loading = use_signal(|| true);
    let mut search = use_signal(|| String::new());
    let mut selected_category = use_signal(|| "all".to_string());
    let mut selected_sort = use_signal(|| "popular".to_string());
    let auth = use_auth();
    
    // 加载出版物
    let load_publications = move || {
        spawn(async move {
            loading.set(true);
            
            let category_str = selected_category();
            let category = if category_str == "all" { 
                None 
            } else { 
                Some(category_str.as_str()) 
            };
            
            let search_str = search();
            let search_query = if search_str.is_empty() { 
                None 
            } else { 
                Some(search_str.as_str()) 
            };
            
            match PublicationService::get_publications(
                search_query,
                category,
                Some(&selected_sort()),
                Some(1),
                Some(20),
            ).await {
                Ok(response) => {
                    publications.set(response.publications);
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
        load_publications();
    });
    
    // 搜索和过滤变化时重新加载
    use_effect(move || {
        load_publications();
    });
    
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
                        
                        div {
                            class: "flex items-center space-x-4",
                            Link {
                                to: Route::Home {},
                                class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                                "返回首页"
                            }
                            
                            if auth.read().is_authenticated {
                                Link {
                                    to: Route::CreatePublication {},
                                    class: "px-4 py-2 bg-green-600 text-white rounded-full text-sm hover:bg-green-700",
                                    "创建出版物"
                                }
                            }
                        }
                    }
                }
            }
            
            // 页面标题和搜索
            div {
                class: "bg-gray-50 dark:bg-gray-800 py-12",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                        "探索出版物"
                    }
                    p {
                        class: "text-lg text-gray-600 dark:text-gray-400 mb-8",
                        "发现优质内容创作者和他们的出版物"
                    }
                    
                    // 搜索框
                    div {
                        class: "max-w-2xl",
                        input {
                            r#type: "search",
                            placeholder: "搜索出版物...",
                            value: "{search}",
                            oninput: move |e| search.set(e.value()),
                            class: "w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        }
                    }
                }
            }
            
            // 过滤和排序
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6",
                div {
                    class: "flex flex-wrap items-center gap-4",
                    
                    // 分类过滤
                    div {
                        class: "flex items-center space-x-2",
                        label {
                            class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                            "分类："
                        }
                        select {
                            value: "{selected_category}",
                            onchange: move |e| selected_category.set(e.value()),
                            class: "px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                            
                            option { value: "all", "全部" }
                            option { value: "technology", "技术" }
                            option { value: "business", "商业" }
                            option { value: "culture", "文化" }
                            option { value: "education", "教育" }
                            option { value: "health", "健康" }
                            option { value: "lifestyle", "生活方式" }
                        }
                    }
                    
                    // 排序
                    div {
                        class: "flex items-center space-x-2",
                        label {
                            class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                            "排序："
                        }
                        select {
                            value: "{selected_sort}",
                            onchange: move |e| selected_sort.set(e.value()),
                            class: "px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                            
                            option { value: "popular", "最受欢迎" }
                            option { value: "newest", "最新创建" }
                            option { value: "alphabetical", "字母顺序" }
                        }
                    }
                }
            }
            
            // 出版物列表
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12",
                
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if publications().is_empty() {
                    div {
                        class: "text-center py-12",
                        p {
                            class: "text-gray-500 dark:text-gray-400",
                            "暂无出版物"
                        }
                    }
                } else {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for publication in publications() {
                            PublicationCard { publication }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PublicationCard(publication: Publication) -> Element {
    let tagline_text = publication.tagline.clone();
    
    rsx! {
        Link {
            to: Route::PublicationDetail { slug: publication.slug.clone() },
            class: "block bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg transition-shadow",
            
            // 头部图片
            if let Some(header_url) = &publication.header_image_url {
                img {
                    src: "{header_url}",
                    alt: "{publication.name}",
                    class: "w-full h-32 object-cover rounded-t-lg"
                }
            } else {
                div {
                    class: "w-full h-32 bg-gradient-to-r from-blue-500 to-purple-600 rounded-t-lg"
                }
            }
            
            div {
                class: "p-6",
                
                // Logo和名称
                div {
                    class: "flex items-start space-x-4",
                    if let Some(logo_url) = &publication.logo_url {
                        img {
                            src: "{logo_url}",
                            alt: "{publication.name}",
                            class: "w-16 h-16 rounded-lg"
                        }
                    } else {
                        div {
                            class: "w-16 h-16 bg-gray-200 dark:bg-gray-700 rounded-lg flex items-center justify-center",
                            span {
                                class: "text-2xl font-bold text-gray-500",
                                {publication.name.chars().next().unwrap_or('P').to_string()}
                            }
                        }
                    }
                    
                    div {
                        class: "flex-1",
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white",
                            {publication.name.clone()}
                        }
                        if publication.is_verified {
                            div {
                                class: "flex items-center mt-1",
                                svg {
                                    class: "w-4 h-4 text-blue-500 mr-1",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                        clip_rule: "evenodd"
                                    }
                                }
                                span {
                                    class: "text-sm text-gray-500",
                                    "已认证"
                                }
                            }
                        }
                    }
                }
                
                // 标语
                if let Some(tagline) = tagline_text {
                    p {
                        class: "mt-3 text-sm text-gray-600 dark:text-gray-400",
                        {tagline}
                    }
                }
                
                // 统计信息
                div {
                    class: "mt-4 flex items-center text-sm text-gray-500 dark:text-gray-400 space-x-4",
                    div {
                        class: "flex items-center",
                        svg {
                            class: "w-4 h-4 mr-1",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                            }
                        }
                        "{publication.member_count} 成员"
                    }
                    div {
                        class: "flex items-center",
                        svg {
                            class: "w-4 h-4 mr-1",
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
                        "{publication.article_count} 文章"
                    }
                    div {
                        class: "flex items-center",
                        svg {
                            class: "w-4 h-4 mr-1",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
                            }
                        }
                        "{publication.follower_count} 关注者"
                    }
                }
                
                // 分类标签
                if !publication.categories.is_empty() {
                    div {
                        class: "mt-3 flex flex-wrap gap-2",
                        for category in publication.categories.iter().take(3) {
                            span {
                                class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-xs rounded",
                                {category.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::{tags::TagService, articles::ArticleService},
    models::{tag::Tag, article::Article},
    components::ArticleCard,
    hooks::use_auth,
    Route,
};

#[component]
pub fn TagsPage() -> Element {
    let mut tags = use_signal(|| Vec::<Tag>::new());
    let mut loading = use_signal(|| true);
    
    // 加载所有标签
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            
            if let Ok(tags_list) = TagService::get_all_tags().await {
                tags.set(tags_list);
            }
            
            loading.set(false);
        });
    });
    
    // 仅显示后端返回的标签（不做前端自定义分组筛选）
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 导航栏
            nav {
                class: "border-b border-gray-100",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                    }
                }
            }
            
            // 主要内容
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // 页面标题
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-3xl font-bold text-gray-900 mb-4",
                        "探索主题"
                    }
                    p {
                        class: "text-lg text-gray-600",
                        "发现您感兴趣的内容"
                    }
                }
                
                // 取消前端硬编码的一级标签，只展示后端返回数据
                
                // 标签网格
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                        }
                    }
                } else {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for tag in tags() {
                            TagCard { tag }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TagCard(tag: Tag) -> Element {
    rsx! {
        div {
            class: "border border-gray-200 rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer",
            onclick: move |_| {
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(&format!("/tag/{}", tag.slug))
                    .ok();
            },
            
            // 标签名称
            h3 {
                class: "text-xl font-semibold mb-2",
                {tag.name.clone()}
            }
            
            // 标签描述
            if let Some(description) = &tag.description {
                p {
                    class: "text-gray-600 mb-4 line-clamp-2",
                    {description.clone()}
                }
            }
            
            // 统计信息
            div {
                class: "flex items-center justify-between text-sm text-gray-500",
                span {
                    "{tag.article_count} 篇文章"
                }
                if tag.follower_count > 0 {
                    span {
                        "{tag.follower_count} 关注者"
                    }
                }
            }
            
            // 标签类别
            if let Some(category) = &tag.category {
                div {
                    class: "mt-4",
                    span {
                        class: "inline-block px-2 py-1 bg-gray-100 text-gray-700 rounded text-xs",
                        {category.clone()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn TagDetailPage(slug: String) -> Element {
    let mut tag = use_signal(|| None::<Tag>);
    let mut articles = use_signal(|| Vec::<Article>::new());
    let mut loading = use_signal(|| true);
    let mut is_following = use_signal(|| false);
    let mut sort_by = use_signal(|| "trending");
    let mut page = use_signal(|| 1);
    
    let auth = use_auth();
    
    // 加载标签详情和文章
    use_effect({
        let slug = slug.clone();
        move || {
            let slug = slug.clone();
            let mut loading = loading.clone();
            let mut tag = tag.clone();
            let auth = auth.clone();
            let mut is_following = is_following.clone();
            let mut articles = articles.clone();
            
            spawn(async move {
                loading.set(true);
                
                // 获取标签详情
                if let Ok(tag_data) = TagService::get_tag(&slug).await {
                    tag.set(Some(tag_data.clone()));
                    
                    // 检查是否关注
                    if auth.read().is_authenticated {
                        if let Ok(following) = TagService::is_following_tag(&tag_data.id).await {
                            is_following.set(following);
                        }
                    }
                }
                
                // 获取标签文章
                if let Ok(response) = ArticleService::get_articles_by_tag(&slug, sort_by(), page(), Some(20)).await {
                    articles.set(response.articles);
                }
                
                loading.set(false);
            });
        }
    });
    
    // 处理关注/取消关注
    let handle_follow = move |_| {
        if !auth.read().is_authenticated {
            web_sys::window()
                .unwrap()
                .location()
                .set_href("/login")
                .ok();
            return;
        }
        
        if let Some(tag_data) = tag() {
            let tag_id = tag_data.id.clone();
            let following = is_following();
            
            spawn(async move {
                let result = if following {
                    TagService::unfollow_tag(&tag_id).await
                } else {
                    TagService::follow_tag(&tag_id).await
                };
                
                if result.is_ok() {
                    is_following.set(!following);
                }
            });
        }
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white",
            
            // 导航栏
            nav {
                class: "border-b border-gray-100",
                div {
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div {
                        class: "flex items-center justify-between h-16",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-serif font-bold",
                            "Rainbow Blog"
                        }
                    }
                }
            }
            
            if loading() {
                div {
                    class: "flex justify-center py-12",
                    div {
                        class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                    }
                }
            } else if let Some(tag_data) = tag() {
                div {
                    // 标签头部信息
                    div {
                        class: "bg-gray-50 border-b border-gray-200",
                        div {
                            class: "max-w-4xl mx-auto px-4 py-8",
                            div {
                                class: "flex items-start justify-between",
                                div {
                                    h1 {
                                        class: "text-3xl font-bold text-gray-900 mb-2",
                                        {tag_data.name.clone()}
                                    }
                                    if let Some(description) = &tag_data.description {
                                        p {
                                            class: "text-lg text-gray-600 mb-4",
                                            {description.clone()}
                                        }
                                    }
                                    div {
                                        class: "flex items-center space-x-4 text-sm text-gray-500",
                                        span {
                                            "{tag_data.article_count} 篇文章"
                                        }
                                        if tag_data.follower_count > 0 {
                                            span {
                                                "{tag_data.follower_count} 关注者"
                                            }
                                        }
                                    }
                                }
                                
                                // 关注按钮
                                if auth.read().is_authenticated {
                                    button {
                                        class: if is_following() {
                                            "px-4 py-2 border border-gray-300 text-gray-700 rounded-full hover:bg-gray-50"
                                        } else {
                                            "px-4 py-2 bg-green-600 text-white rounded-full hover:bg-green-700"
                                        },
                                        onclick: handle_follow,
                                        if is_following() {
                                            "已关注"
                                        } else {
                                            "关注"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // 排序选项
                    div {
                        class: "max-w-4xl mx-auto px-4 py-4",
                        div {
                            class: "flex space-x-4 border-b border-gray-200",
                            button {
                                class: if sort_by() == "trending" {
                                    "pb-4 px-1 border-b-2 border-gray-900 font-medium text-sm"
                                } else {
                                    "pb-4 px-1 text-sm text-gray-500 hover:text-gray-700"
                                },
                                onclick: move |_| {
                                    sort_by.set("trending");
                                    page.set(1);
                                },
                                "热门"
                            }
                            button {
                                class: if sort_by() == "latest" {
                                    "pb-4 px-1 border-b-2 border-gray-900 font-medium text-sm"
                                } else {
                                    "pb-4 px-1 text-sm text-gray-500 hover:text-gray-700"
                                },
                                onclick: move |_| {
                                    sort_by.set("latest");
                                    page.set(1);
                                },
                                "最新"
                            }
                            button {
                                class: if sort_by() == "popular" {
                                    "pb-4 px-1 border-b-2 border-gray-900 font-medium text-sm"
                                } else {
                                    "pb-4 px-1 text-sm text-gray-500 hover:text-gray-700"
                                },
                                onclick: move |_| {
                                    sort_by.set("popular");
                                    page.set(1);
                                },
                                "最受欢迎"
                            }
                        }
                    }
                    
                    // 文章列表
                    div {
                        class: "max-w-4xl mx-auto px-4",
                        if articles().is_empty() {
                            div {
                                class: "text-center py-12",
                                p {
                                    class: "text-gray-500",
                                    "暂无文章"
                                }
                            }
                        } else {
                            div {
                                class: "space-y-0",
                                for article in articles() {
                                    ArticleCard { article }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

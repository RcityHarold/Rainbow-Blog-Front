use dioxus::prelude::*;
use dioxus::prelude::Key;
use dioxus_router::prelude::*;
use crate::{
    api::publications::PublicationService,
    models::publication::{CreatePublicationRequest, SocialLinks},
    Route,
};

#[component]
pub fn CreatePublicationPage() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut tagline = use_signal(|| String::new());
    let mut logo_url = use_signal(|| String::new());
    let mut header_image_url = use_signal(|| String::new());
    let mut categories = use_signal(|| String::new());
    let mut website = use_signal(|| String::new());
    let mut twitter = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let navigator = use_navigator();

    let mut handle_create = move |_| {
        if name().trim().is_empty() {
            error.set(Some("名称不能为空".into()));
            return;
        }
        let req = CreatePublicationRequest {
            name: name().trim().to_string(),
            description: if description().trim().is_empty() { None } else { Some(description()) },
            tagline: if tagline().trim().is_empty() { None } else { Some(tagline()) },
            logo_url: if logo_url().trim().is_empty() { None } else { Some(logo_url()) },
            header_image_url: if header_image_url().trim().is_empty() { None } else { Some(header_image_url()) },
            categories: categories()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            social_links: {
                let w = website();
                let t = twitter();
                if w.trim().is_empty() && t.trim().is_empty() { None } else {
                    Some(SocialLinks {
                        website: if w.trim().is_empty() { None } else { Some(w) },
                        twitter: if t.trim().is_empty() { None } else { Some(t) },
                        facebook: None,
                        instagram: None,
                        linkedin: None,
                    })
                }
            },
        };

        spawn(async move {
            loading.set(true);
            error.set(None);
            match PublicationService::create_publication(&req).await {
                Ok(pub_data) => {
                    let _ = navigator.push(Route::PublicationDetail { slug: pub_data.slug });
                }
                Err(e) => {
                    error.set(Some(format!("创建失败: {}", e.message)));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        // 捕获整页 Enter，防止任何默认提交/导航
        div { class: "min-h-screen bg-white dark:bg-gray-900",
            onkeydown: {
                let mut handle_create = handle_create.clone();
                move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| {
                    if e.key() == Key::Enter {
                        e.prevent_default();
                        // 如果需要也可以触发创建：handle_create(());
                    }
                }
            },
            div { class: "max-w-3xl mx-auto px-6 py-10",
                h1 { class: "text-2xl font-bold mb-6 text-gray-900 dark:text-white", "创建出版物" }

                if let Some(err) = error() {
                    div { class: "mb-4 p-3 rounded bg-red-50 text-red-700 border border-red-200", {err} }
                }

                div {
                    div { class: "space-y-4",
                        // 名称
                        div {
                            label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "名称" }
                            input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                value: name(),
                                oninput: move |e| name.set(e.value()),
                                onkeydown: {
                                    let mut handle_create = handle_create.clone();
                                    move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| {
                                        if e.key() == Key::Enter { e.prevent_default(); handle_create(()); }
                                    }
                                }
                            }
                        }
                        // 宣传语
                        div {
                            label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "宣传语" }
                            input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                value: tagline(),
                                oninput: move |e| tagline.set(e.value()),
                                onkeydown: {
                                    let mut handle_create = handle_create.clone();
                                    move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| {
                                        if e.key() == Key::Enter { e.prevent_default(); handle_create(()); }
                                    }
                                }
                            }
                        }
                        // 简介
                        div {
                            label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "简介" }
                            textarea { class: "w-full border rounded px-3 py-2 h-24 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                value: description(),
                                oninput: move |e| description.set(e.value()),
                                onkeydown: {
                                    let mut handle_create = handle_create.clone();
                                    move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| {
                                        if e.key() == Key::Enter && (e.modifiers().meta() || e.modifiers().ctrl()) {
                                            e.prevent_default(); handle_create(());
                                        }
                                    }
                                }
                            }
                        }
                        // Logo 与 Header
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div { 
                                label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "Logo URL" }
                                input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    value: logo_url(),
                                    oninput: move |e| logo_url.set(e.value()),
                                    onkeydown: {
                                        let mut handle_create = handle_create.clone();
                                        move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| { if e.key() == Key::Enter { e.prevent_default(); handle_create(()); } }
                                    }
                                }
                            }
                            div { 
                                label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "头图 URL" }
                                input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    value: header_image_url(),
                                    oninput: move |e| header_image_url.set(e.value()),
                                    onkeydown: {
                                        let mut handle_create = handle_create.clone();
                                        move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| { if e.key() == Key::Enter { e.prevent_default(); handle_create(()); } }
                                    }
                                }
                            }
                        }
                        // 分类
                        div {
                            label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "分类（逗号分隔）" }
                            input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                value: categories(),
                                oninput: move |e| categories.set(e.value()),
                                onkeydown: {
                                    let mut handle_create = handle_create.clone();
                                    move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| { if e.key() == Key::Enter { e.prevent_default(); handle_create(()); } }
                                }
                            }
                        }
                        // 链接
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div { 
                                label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "官网链接" }
                                input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    value: website(),
                                    oninput: move |e| website.set(e.value()),
                                    onkeydown: {
                                        let mut handle_create = handle_create.clone();
                                        move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| { if e.key() == Key::Enter { e.prevent_default(); handle_create(()); } }
                                    }
                                }
                            }
                            div { 
                                label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "Twitter" }
                                input { class: "w-full border rounded px-3 py-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    value: twitter(),
                                    oninput: move |e| twitter.set(e.value()),
                                    onkeydown: {
                                        let mut handle_create = handle_create.clone();
                                        move |e: dioxus::prelude::Event<dioxus::prelude::KeyboardData>| { if e.key() == Key::Enter { e.prevent_default(); handle_create(()); } }
                                    }
                                }
                            }
                        }
                        // 提交
                        div {
                            button { r#type: "button",
                                class: "px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50",
                                disabled: loading(),
                                onclick: move |e| { e.prevent_default(); e.stop_propagation(); handle_create(()); },
                                if loading() { "创建中..." } else { "创建" }
                            }
                            Link { to: Route::Publications {}, class: "ml-4 text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white", "取消" }
                        }
                    }
                }
            }
        }
    }
}

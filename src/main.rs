#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod api;
mod components;
mod hooks;
mod models;
mod pages;

use components::*;
use hooks::*;
use pages::*;
use pages::create_publication::CreatePublicationPage;

fn main() {
    // 初始化控制台错误处理
    console_error_panic_hook::set_once();
    
    // 启动应用
    launch(App);
}

fn App() -> Element {
    use_provide_auth();
    use_provide_theme();
    
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/login")]
    Login {},
    
    #[route("/register")]
    Register {},
    
    #[route("/article/:slug")]
    Article { slug: String },
    
    #[route("/write")]
    Write {},
    
    #[route("/edit/:slug")]
    Edit { slug: String },
    
    #[route("/@:username")]
    Profile { username: String },
    
    #[route("/profile/:user_id")]
    ProfileById { user_id: String },
    
    #[route("/search")]
    Search {},
    
    #[route("/tags")]
    Tags {},
    
    #[route("/tag/:slug")]
    TagDetail { slug: String },
    
    #[route("/settings")]
    Settings {},
    
    #[route("/series")]
    SeriesManage {},
    
    #[route("/series/:slug")]
    SeriesDetail { slug: String },
    
    #[route("/publications")]
    Publications {},
    
    #[route("/publications/create")]
    CreatePublication {},
    
    #[route("/publications/:slug")]
    PublicationDetail { slug: String },
    
    #[route("/domain-management/:publication_id")]
    DomainManagement { publication_id: String },
    
    #[route("/subscription-plans")]
    SubscriptionPlans {},
    
    #[route("/my-subscriptions")]
    MySubscriptions {},
    
    #[route("/earnings")]
    Earnings {},
    
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Home() -> Element {
    rsx! {
        HomePage {}
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        PublicRoute {
            LoginPage {}
        }
    }
}

#[component]
fn Register() -> Element {
    rsx! {
        PublicRoute {
            RegisterPage {}
        }
    }
}

#[component]
fn Article(slug: String) -> Element {
    rsx! {
        ArticlePage { slug }
    }
}

#[component]
fn Write() -> Element {
    rsx! {
        ProtectedRoute {
            EditorV2Page { slug: None }
        }
    }
}

#[component]
fn Edit(slug: String) -> Element {
    rsx! {
        ProtectedRoute {
            EditorV2Page { slug: Some(slug) }
        }
    }
}

#[component]
fn Profile(username: String) -> Element {
    rsx! {
        ProfilePage { username }
    }
}

#[component]
fn ProfileById(user_id: String) -> Element {
    rsx! {
        ProfileByIdPage { user_id }
    }
}

#[component]
fn Search() -> Element {
    rsx! {
        SearchPage {}
    }
}

#[component]
fn Tags() -> Element {
    rsx! {
        TagsPage {}
    }
}

#[component]
fn TagDetail(slug: String) -> Element {
    rsx! {
        TagDetailPage { slug }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        SettingsPage {}
    }
}

#[component]
fn SeriesManage() -> Element {
    rsx! {
        SeriesManagePage {}
    }
}

#[component]
fn SeriesDetail(slug: String) -> Element {
    rsx! {
        SeriesDetailPage { slug }
    }
}

#[component]
fn Publications() -> Element {
    rsx! {
        PublicationsPage {}
    }
}

#[component]
fn CreatePublication() -> Element {
    rsx! {
        ProtectedRoute { 
            CreatePublicationPage {}
        }
    }
}

#[component]
fn PublicationDetail(slug: String) -> Element {
    rsx! {
        PublicationDetailPage { slug }
    }
}

#[component]
fn DomainManagement(publication_id: String) -> Element {
    rsx! {
        DomainManagementPage { publication_id }
    }
}

#[component]
fn SubscriptionPlans() -> Element {
    rsx! {
        SubscriptionPlansPage {}
    }
}

#[component]
fn MySubscriptions() -> Element {
    rsx! {
        MySubscriptionsPage {}
    }
}

#[component]
fn Earnings() -> Element {
    rsx! {
        EarningsPage {}
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-white flex items-center justify-center",
            div {
                class: "text-center",
                h1 { 
                    class: "text-6xl font-serif font-bold text-gray-900 mb-4",
                    "404" 
                }
                p { 
                    class: "text-xl text-gray-600 mb-8",
                    "页面未找到" 
                }
                p {
                    class: "text-gray-500 mb-8",
                    {format!("请求的页面不存在: /{}", route.join("/"))}
                }
                Link {
                    to: Route::Home {},
                    class: "text-gray-900 font-medium underline",
                    "返回首页"
                }
            }
        }
    }
}

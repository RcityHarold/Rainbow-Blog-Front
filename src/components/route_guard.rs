use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::hooks::use_auth;

#[component]
pub fn ProtectedRoute(children: Element) -> Element {
    let auth = use_auth();
    let navigator = use_navigator();
    
    use_effect(move || {
        // 只在加载完成后且未认证时才重定向
        if !auth.read().loading && !auth.read().is_authenticated {
            navigator.push("/login");
        }
    });
    
    if auth.read().loading {
        // 认证状态还在加载中，显示加载动画
        rsx! {
            div {
                class: "flex items-center justify-center min-h-screen",
                div {
                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                }
            }
        }
    } else if auth.read().is_authenticated {
        // 已认证，显示子组件
        rsx! { {children} }
    } else {
        // 未认证，显示重定向提示（实际上会被 use_effect 重定向）
        rsx! {
            div {
                class: "flex items-center justify-center min-h-screen",
                div {
                    class: "text-gray-500",
                    "重定向中..."
                }
            }
        }
    }
}

#[component]
pub fn PublicRoute(children: Element) -> Element {
    let auth = use_auth();
    let navigator = use_navigator();
    
    use_effect(move || {
        // 只在加载完成后且已认证时才重定向到首页
        if !auth.read().loading && auth.read().is_authenticated {
            navigator.push("/");
        }
    });
    
    if auth.read().loading {
        // 认证状态还在加载中，显示加载动画
        rsx! {
            div {
                class: "flex items-center justify-center min-h-screen",
                div {
                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
                }
            }
        }
    } else if !auth.read().is_authenticated {
        // 未认证，显示子组件（登录/注册页面）
        rsx! { {children} }
    } else {
        // 已认证，显示重定向提示（实际上会被 use_effect 重定向到首页）
        rsx! {
            div {
                class: "flex items-center justify-center min-h-screen",
                div {
                    class: "text-gray-500",
                    "重定向到首页..."
                }
            }
        }
    }
}
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::auth::AuthService,
    hooks::use_auth,
    Route,
};

#[component]
pub fn RegisterPage() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut full_name = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    
    let mut auth = use_auth();
    let navigator = use_navigator();
    
    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let full_name_opt = if full_name().is_empty() {
                None
            } else {
                Some(full_name())
            };
            
            match AuthService::register(username(), email(), password(), full_name_opt).await {
                Ok(response) => {
                    auth.write().user = Some(response.user);
                    auth.write().is_authenticated = true;
                    navigator.push("/");
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            
            loading.set(false);
        });
    };
    
    rsx! {
        div {
            class: "min-h-screen bg-white flex flex-col justify-center",
            div {
                class: "max-w-md w-full mx-auto px-4",
                div {
                    class: "text-center mb-12",
                    h1 {
                        class: "text-6xl font-serif mb-4",
                        "Rainbow Blog"
                    }
                    p {
                        class: "text-lg text-gray-600",
                        "创建你的账号，开始分享故事"
                    }
                }
                
                form {
                    onsubmit: handle_submit,
                    class: "space-y-6",
                    
                    if let Some(err) = error() {
                        div {
                            class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                            {err}
                        }
                    }
                    
                    div {
                        input {
                            r#type: "text",
                            placeholder: "用户名",
                            value: "{username}",
                            oninput: move |evt| username.set(evt.value()),
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent",
                            required: true,
                        }
                    }
                    
                    div {
                        input {
                            r#type: "text",
                            placeholder: "全名（可选）",
                            value: "{full_name}",
                            oninput: move |evt| full_name.set(evt.value()),
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent",
                        }
                    }
                    
                    div {
                        input {
                            r#type: "email",
                            placeholder: "邮箱",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent",
                            required: true,
                        }
                    }
                    
                    div {
                        input {
                            r#type: "password",
                            placeholder: "密码（至少8位）",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value()),
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent",
                            required: true,
                            minlength: "8",
                        }
                    }
                    
                    button {
                        r#type: "submit",
                        disabled: loading(),
                        class: "w-full bg-gray-900 text-white py-3 rounded-lg font-medium hover:bg-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                        if loading() { "注册中..." } else { "注册" }
                    }
                    
                    p {
                        class: "text-sm text-gray-500 text-center",
                        "点击注册即表示您同意我们的服务条款和隐私政策"
                    }
                }
                
                div {
                    class: "mt-8 text-center",
                    p {
                        class: "text-gray-600",
                        "已有账号？"
                        Link {
                            to: Route::Login {},
                            class: "text-gray-900 font-medium underline ml-1",
                            "立即登录"
                        }
                    }
                }
                
                div {
                    class: "mt-8 text-center",
                    div {
                        class: "relative",
                        div {
                            class: "absolute inset-0 flex items-center",
                            div {
                                class: "w-full border-t border-gray-300"
                            }
                        }
                        div {
                            class: "relative flex justify-center text-sm",
                            span {
                                class: "px-4 bg-white text-gray-500",
                                "或"
                            }
                        }
                    }
                    
                    div {
                        class: "mt-6",
                        button {
                            onclick: move |_| {
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href("/api/auth/login/google")
                                    .ok();
                            },
                            class: "w-full flex items-center justify-center px-4 py-3 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50",
                            svg {
                                class: "w-5 h-5 mr-2",
                                view_box: "0 0 24 24",
                                path {
                                    fill: "#4285F4",
                                    d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                }
                                path {
                                    fill: "#34A853",
                                    d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                }
                                path {
                                    fill: "#FBBC05",
                                    d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                }
                                path {
                                    fill: "#EA4335",
                                    d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                }
                            }
                            "使用 Google 账号注册"
                        }
                    }
                }
            }
        }
    }
}
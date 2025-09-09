use dioxus::prelude::*;
use crate::{
    models::auth::User,
    api::{auth::AuthService, client::ApiClient},
};

#[derive(Clone, Debug, PartialEq)]
pub struct AuthState {
    pub user: Option<User>,
    pub is_authenticated: bool,
    pub loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
            loading: false,
        }
    }
}

pub fn use_auth() -> Signal<AuthState> {
    use_context::<Signal<AuthState>>()
}

pub fn use_provide_auth() -> Signal<AuthState> {
    let auth_state = use_context_provider(|| Signal::new(AuthState {
        loading: true,
        ..Default::default()
    }));
    
    // 在组件挂载时检查是否有保存的 token
    use_effect(move || {
        let mut auth_state = auth_state.clone();
        spawn(async move {
            // 检查是否有保存的 token
            if let Some(_token) = ApiClient::get_token() {
                // 如果有 token，尝试获取当前用户信息
                match AuthService::get_current_user().await {
                    Ok(user) => {
                        auth_state.write().user = Some(user);
                        auth_state.write().is_authenticated = true;
                    }
                    Err(_) => {
                        // 如果获取用户信息失败，清除无效的 token
                        ApiClient::clear_token();
                        auth_state.write().user = None;
                        auth_state.write().is_authenticated = false;
                    }
                }
            } else {
                // 没有 token，用户未登录
                auth_state.write().user = None;
                auth_state.write().is_authenticated = false;
            }
            
            // 加载完成
            auth_state.write().loading = false;
        });
    });
    
    auth_state
}
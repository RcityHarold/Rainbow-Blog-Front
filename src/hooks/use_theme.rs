use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn class(&self) -> &'static str {
        match self {
            Theme::Light => "",
            Theme::Dark => "dark",
        }
    }
    
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    theme: Signal<Theme>,
}

impl ThemeState {
    pub fn current(&self) -> Theme {
        *self.theme.read()
    }
    
    pub fn toggle(&mut self) {
        let current = *self.theme.read();
        let new_theme = current.toggle();
        self.theme.set(new_theme.clone());
        
        // 保存到本地存储
        let _ = LocalStorage::set("theme", match new_theme {
            Theme::Light => "light",
            Theme::Dark => "dark",
        });
        
        // 更新HTML元素的class
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.document_element() {
                    let class_list = element.class_list();
                    match new_theme {
                        Theme::Dark => {
                            let _ = class_list.add_1("dark");
                        }
                        Theme::Light => {
                            let _ = class_list.remove_1("dark");
                        }
                    }
                }
            }
        }
    }
}

pub fn use_provide_theme() -> ThemeState {
    let mut theme = use_signal(|| {
        // 从本地存储读取主题
        let stored_theme = LocalStorage::get::<String>("theme").unwrap_or_else(|_| "light".to_string());
        match stored_theme.as_str() {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    });
    
    // 初始化时设置HTML class
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.document_element() {
                    let class_list = element.class_list();
                    if theme() == Theme::Dark {
                        let _ = class_list.add_1("dark");
                    }
                }
            }
        }
    });
    
    let state = ThemeState { theme };
    use_context_provider(|| state);
    state
}

pub fn use_theme() -> ThemeState {
    use_context()
}
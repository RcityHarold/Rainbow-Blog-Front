use dioxus::prelude::*;
use web_sys::{File, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::api::upload::UploadService;
use js_sys::Math;

#[component]
pub fn ImageUpload(
    on_upload: EventHandler<String>,
    on_error: Option<EventHandler<String>>,
    accept: Option<String>,
    max_size_mb: Option<f64>,
    button_text: Option<String>,
    button_class: Option<String>,
) -> Element {
    let mut uploading = use_signal(|| false);
    let mut progress = use_signal(|| 0.0);
    let input_id = use_memo(move || format!("image-upload-{}", (Math::random() * 1000000.0) as u32));
    
    let accept_types = accept.unwrap_or_else(|| "image/jpeg,image/png,image/gif,image/webp".to_string());
    let accept_types_clone = accept_types.clone();
    let max_size = max_size_mb.unwrap_or(10.0) * 1024.0 * 1024.0; // 默认10MB
    let text = button_text.unwrap_or_else(|| "上传图片".to_string());
    let class = button_class.unwrap_or_else(|| "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50".to_string());
    
    let handle_file_change = move |_| {
        let input_id = input_id.clone();
        let accept_types = accept_types.clone();
        let max_size_value = max_size;
        let max_size_mb_value = max_size_mb.unwrap_or(10.0);
        let on_upload = on_upload.clone();
        let on_error = on_error.clone();
        
        spawn(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            
            if let Some(input) = document.get_element_by_id(&input_id()).and_then(|e| e.dyn_into::<HtmlInputElement>().ok()) {
                if let Some(files) = input.files() {
                    if files.length() > 0 {
                        if let Some(file) = files.get(0) {
                            // 检查文件大小
                            let size = file.size() as f64;
                            if size > max_size_value {
                                if let Some(ref on_error) = on_error {
                                    on_error.call(format!("文件大小不能超过 {} MB", max_size_mb_value));
                                }
                                return;
                            }
                            
                            // 检查文件类型
                            let file_type = file.type_();
                            if !accept_types.split(',').any(|t| file_type.contains(t.trim())) {
                                if let Some(ref on_error) = on_error {
                                    on_error.call("不支持的文件类型".to_string());
                                }
                                return;
                            }
                            
                            uploading.set(true);
                            progress.set(0.0);
                            
                            match UploadService::upload_image(file).await {
                                Ok(response) => {
                                    on_upload.call(response.url);
                                    progress.set(100.0);
                                }
                                Err(e) => {
                                    if let Some(ref on_error) = on_error {
                                        on_error.call(e.message);
                                    }
                                }
                            }
                            
                            uploading.set(false);
                            // 清空输入
                            input.set_value("");
                        }
                    }
                }
            }
        });
    };
    
    rsx! {
        div {
            class: "relative inline-block",
            
            // 隐藏的文件输入
            input {
                id: "{input_id}",
                r#type: "file",
                accept: "{accept_types_clone}",
                class: "hidden",
                onchange: handle_file_change
            }
            
            // 上传按钮
            label {
                r#for: "{input_id}",
                class: if uploading() {
                    format!("{} cursor-not-allowed", class)
                } else {
                    format!("{} cursor-pointer", class)
                },
                
                if uploading() {
                    div {
                        class: "flex items-center",
                        div {
                            class: "animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"
                        }
                        "上传中..."
                    }
                } else {
                    {text}
                }
            }
            
            // 进度条
            if uploading() && progress() > 0.0 {
                div {
                    class: "absolute left-0 right-0 bottom-0 h-1 bg-gray-200 rounded-b",
                    div {
                        class: "h-full bg-blue-600 transition-all duration-300",
                        style: "width: {progress}%"
                    }
                }
            }
        }
    }
}

// 图片上传区域组件（支持拖拽）
#[component]
pub fn ImageDropZone(
    on_upload: EventHandler<String>,
    on_error: Option<EventHandler<String>>,
    current_image: Option<String>,
    placeholder_text: Option<String>,
) -> Element {
    let mut dragging = use_signal(|| false);
    let mut uploading = use_signal(|| false);
    let placeholder = placeholder_text.unwrap_or_else(|| "点击或拖拽图片到此处上传".to_string());
    
    let handle_drop = move |_: DragEvent| {
        // 拖拽功能暂时禁用，因为Dioxus的DragData API与web_sys::File不兼容
        dragging.set(false);
    };
    
    let handle_drag_over = move |_: DragEvent| {
        dragging.set(true);
    };
    
    let handle_drag_leave = move |_: DragEvent| {
        dragging.set(false);
    };
    
    rsx! {
        div {
            class: "relative",
            
            div {
                class: if dragging() {
                    "border-2 border-dashed border-blue-500 bg-blue-50 dark:bg-blue-900/20 rounded-lg p-8 text-center transition-all"
                } else {
                    "border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-8 text-center transition-all hover:border-gray-400"
                },
                onclick: move |_| {
                    // 点击区域触发文件选择
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(input) = document.query_selector(&format!("#dropzone-file-input")).ok().flatten() {
                                if let Ok(input) = input.dyn_into::<HtmlInputElement>() {
                                    input.click();
                                }
                            }
                        }
                    }
                },
                
                if let Some(image_url) = current_image {
                    div {
                        class: "relative",
                        img {
                            src: "{image_url}",
                            alt: "上传的图片",
                            class: "max-w-full h-auto rounded-lg mb-4 mx-auto"
                        }
                        
                        div {
                            class: "mt-4",
                            ImageUpload {
                                on_upload: on_upload.clone(),
                                on_error: on_error.clone(),
                                button_text: "更换图片".to_string(),
                                button_class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700".to_string()
                            }
                        }
                    }
                } else {
                    div {
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400 mb-4",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                            }
                        }
                        
                        p {
                            class: "text-gray-600 dark:text-gray-400 mb-4",
                            {placeholder}
                        }
                        
                        ImageUpload {
                            on_upload: on_upload.clone(),
                            on_error: on_error.clone()
                        }
                        
                        // 隐藏的文件输入用于点击上传
                        input {
                            id: "dropzone-file-input",
                            r#type: "file",
                            accept: "image/jpeg,image/png,image/gif,image/webp",
                            class: "hidden",
                            onchange: move |_| {
                                spawn(async move {
                                    let window = web_sys::window().unwrap();
                                    let document = window.document().unwrap();
                                    
                                    if let Some(input) = document.get_element_by_id("dropzone-file-input").and_then(|e| e.dyn_into::<HtmlInputElement>().ok()) {
                                        if let Some(files) = input.files() {
                                            if files.length() > 0 {
                                                if let Some(file) = files.get(0) {
                                                    uploading.set(true);
                                                    
                                                    match UploadService::upload_image(file).await {
                                                        Ok(response) => {
                                                            on_upload.call(response.url);
                                                        }
                                                        Err(e) => {
                                                            if let Some(ref on_error) = on_error {
                                                                on_error.call(e.message);
                                                            }
                                                        }
                                                    }
                                                    
                                                    uploading.set(false);
                                                    input.set_value("");
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
            
            if uploading() {
                div {
                    class: "absolute inset-0 bg-white bg-opacity-75 dark:bg-gray-900 dark:bg-opacity-75 rounded-lg flex items-center justify-center",
                    div {
                        class: "text-center",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white mx-auto mb-2"
                        }
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400",
                            "上传中..."
                        }
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use web_sys::{window, Selection, Range, HtmlElement};
use wasm_bindgen::JsCast;
use crate::models::highlight::{Highlight, CreateHighlightRequest};
use gloo_storage::{LocalStorage, Storage};
use std::rc::Rc;
use std::cell::RefCell;

// 高亮颜色选项
const HIGHLIGHT_COLORS: &[(&str, &str)] = &[
    ("yellow", "#FEF3C7"),
    ("green", "#D1FAE5"), 
    ("blue", "#DBEAFE"),
    ("purple", "#E9D5FF"),
    ("red", "#FEE2E2"),
];

#[component]
pub fn HighlightSystem(
    article_id: String,
    article_html: String,
) -> Element {
    let article_id = article_id.clone();
    let mut highlights = use_signal(|| Vec::<Highlight>::new());
    let mut show_toolbar = use_signal(|| false);
    let mut selected_text = use_signal(|| String::new());
    let mut toolbar_position = use_signal(|| (0.0, 0.0));
    let mut show_note_modal = use_signal(|| false);
    let mut current_highlight_color = use_signal(|| "yellow".to_string());
    let mut note_text = use_signal(|| String::new());
    let mut selection_info = use_signal(|| None::<SelectionInfo>);

    // 加载保存的高亮
    use_effect({
        let article_id = article_id.clone();
        let mut highlights = highlights.clone();
        move || {
            let key = format!("highlights_{}", article_id);
            if let Ok(saved_highlights) = LocalStorage::get::<Vec<Highlight>>(&key) {
                highlights.set(saved_highlights);
            }
        }
    });

    // 监听文本选择事件
    let handle_selection = move |_| {
        if let Some(window) = window() {
            if let Ok(selection) = window.get_selection() {
                if let Some(selection) = selection {
                    if !selection.is_collapsed() {
                        if let Some(text) = selection.to_string().as_string() {
                            if !text.trim().is_empty() {
                                selected_text.set(text);
                                
                                // 获取选择范围的位置
                                if let Some(range) = selection.get_range_at(0).ok() {
                                    let rect = range.get_bounding_client_rect();
                                    let x = rect.left() + rect.width() / 2.0;
                                    let y = rect.top() + window.scroll_y().unwrap_or(0.0);
                                    toolbar_position.set((x, y));
                                    show_toolbar.set(true);
                                    
                                    // 保存选择信息
                                    selection_info.set(Some(SelectionInfo {
                                        range,
                                        selection,
                                    }));
                                }
                            }
                        }
                    } else {
                        show_toolbar.set(false);
                    }
                }
            }
        }
    };

    // 添加高亮
    let add_highlight = Rc::new({
        let article_id = article_id.clone();
        let selection_info = selection_info.clone();
        let selected_text = selected_text.clone();
        let highlights = highlights.clone();
        let show_toolbar = show_toolbar.clone();
        let show_note_modal = show_note_modal.clone();
        let current_highlight_color = current_highlight_color.clone();
        let note_text = note_text.clone();
        
        move |color: String, with_note: bool| {
            let selection_info = selection_info.clone();
            let selected_text = selected_text.clone();
            let mut highlights = highlights.clone();
            let mut show_toolbar = show_toolbar.clone();
            let mut show_note_modal = show_note_modal.clone();
            let mut note_text = note_text.clone();
            
            if selection_info.read().is_some() {
                let content = selected_text();
            
            // 创建新的高亮
            let new_highlight = Highlight {
                id: format!("hl_{}", chrono::Utc::now().timestamp_millis()),
                user_id: "current_user".to_string(), // 实际应该从auth获取
                article_id: article_id.clone(),
                content: content.clone(),
                note: if with_note { Some(note_text()) } else { None },
                start_offset: 0, // 实际应该计算偏移量
                end_offset: content.len(),
                start_container_path: "".to_string(), // 实际应该计算路径
                end_container_path: "".to_string(),
                color: color.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            // 添加到列表
            let mut current_highlights = highlights();
            current_highlights.push(new_highlight);
            highlights.set(current_highlights.clone());
            
            // 保存到LocalStorage
            let key = format!("highlights_{}", article_id);
            let _ = LocalStorage::set(&key, &current_highlights);
            
            // 应用高亮样式
            if let Some(info) = &*selection_info.read() {
                apply_highlight_style(&info.range, &color);
                
                // 清除选择
                info.selection.remove_all_ranges().ok();
            }
            show_toolbar.set(false);
            show_note_modal.set(false);
            note_text.set(String::new());
        }
        }
    });

    // 显示高亮笔记
    let show_highlight_note = move |highlight: &Highlight| {
        // TODO: 显示笔记的逻辑
    };

    rsx! {
        div {
            class: "relative",
            onmouseup: handle_selection,
            
            // 文章内容（带高亮）
            div {
                class: "highlight-container",
                dangerous_inner_html: "{article_html}"
            }
            
            // 高亮工具栏
            if show_toolbar() {
                div {
                    class: "fixed z-50 bg-gray-800 text-white rounded-lg shadow-lg p-2 flex items-center space-x-2",
                    style: "left: {toolbar_position().0}px; top: {toolbar_position().1 - 50.0}px; transform: translateX(-50%);",
                    
                    // 颜色选择器
                    for (name, color) in HIGHLIGHT_COLORS {
                        button {
                            class: "w-6 h-6 rounded-full border-2 border-white",
                            style: "background-color: {color}",
                            onclick: {
                                let add_highlight = add_highlight.clone();
                                let color_name = name.to_string();
                                move |_| {
                                    current_highlight_color.set(color_name.clone());
                                    (add_highlight.as_ref())(color_name.clone(), false);
                                }
                            }
                        }
                    }
                    
                    // 分隔线
                    div {
                        class: "w-px h-6 bg-gray-600 mx-1"
                    }
                    
                    // 添加笔记按钮
                    button {
                        class: "px-2 py-1 text-sm hover:bg-gray-700 rounded",
                        onclick: move |_| {
                            show_note_modal.set(true);
                            show_toolbar.set(false);
                        },
                        svg {
                            class: "w-4 h-4 inline mr-1",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                            }
                        }
                        "笔记"
                    }
                }
            }
            
            // 笔记模态框
            if show_note_modal() {
                div {
                    class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                    onclick: move |_| show_note_modal.set(false),
                    
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-96 max-w-full mx-4",
                        onclick: move |e| e.stop_propagation(),
                        
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "添加笔记"
                        }
                        
                        // 显示选中的文本
                        div {
                            class: "mb-4 p-3 bg-gray-100 dark:bg-gray-700 rounded",
                            p {
                                class: "text-sm italic",
                                "\"{selected_text}\""
                            }
                        }
                        
                        // 颜色选择
                        div {
                            class: "mb-4",
                            label {
                                class: "block text-sm font-medium mb-2",
                                "选择高亮颜色"
                            }
                            div {
                                class: "flex space-x-2",
                                for (name, color) in HIGHLIGHT_COLORS {
                                    button {
                                        class: if current_highlight_color() == *name {
                                            "w-8 h-8 rounded-full border-2 border-gray-800 ring-2 ring-offset-2 ring-gray-800"
                                        } else {
                                            "w-8 h-8 rounded-full border-2 border-gray-300"
                                        },
                                        style: "background-color: {color}",
                                        onclick: move |_| current_highlight_color.set(name.to_string())
                                    }
                                }
                            }
                        }
                        
                        // 笔记输入
                        textarea {
                            class: "w-full p-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "添加您的笔记...",
                            rows: "4",
                            value: "{note_text}",
                            oninput: move |e| note_text.set(e.value())
                        }
                        
                        // 按钮
                        div {
                            class: "mt-4 flex justify-end space-x-3",
                            button {
                                class: "px-4 py-2 text-gray-600 hover:text-gray-800",
                                onclick: move |_| {
                                    show_note_modal.set(false);
                                    note_text.set(String::new());
                                },
                                "取消"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                                onclick: {
                                    let add_highlight = add_highlight.clone();
                                    move |_| {
                                        (add_highlight.as_ref())(current_highlight_color(), true);
                                    }
                                },
                                "保存"
                            }
                        }
                    }
                }
            }
            
            // 高亮列表侧边栏
            HighlightsSidebar {
                highlights: highlights(),
                on_delete: move |id| {
                    let mut current = highlights();
                    current.retain(|h| h.id != id);
                    highlights.set(current.clone());
                    
                    // 更新LocalStorage
                    let key = format!("highlights_{}", article_id);
                    let _ = LocalStorage::set(&key, &current);
                }
            }
        }
    }
}

// 选择信息结构
struct SelectionInfo {
    range: Range,
    selection: Selection,
}

// 应用高亮样式
fn apply_highlight_style(range: &Range, color: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Ok(span) = document.create_element("span") {
                let color_value = match color {
                    "yellow" => "#FEF3C7",
                    "green" => "#D1FAE5",
                    "blue" => "#DBEAFE",
                    "purple" => "#E9D5FF",
                    "red" => "#FEE2E2",
                    _ => "#FEF3C7",
                };
                
                span.set_class_name("highlight-span");
                span.set_attribute("style", &format!("background-color: {}; padding: 2px 0; cursor: pointer;", color_value)).ok();
                
                // 将选中的内容包裹在 span 中
                range.surround_contents(&span).ok();
            }
        }
    }
}

#[component]
fn HighlightsSidebar(
    highlights: Vec<Highlight>,
    on_delete: EventHandler<String>,
) -> Element {
    let mut show_sidebar = use_signal(|| false);
    
    rsx! {
        div {
            // 切换按钮
            button {
                class: "fixed right-4 top-32 z-40 p-2 bg-white dark:bg-gray-800 rounded-l-lg shadow-lg",
                onclick: move |_| show_sidebar.set(!show_sidebar()),
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
                    }
                }
                if !highlights.is_empty() {
                    span {
                        class: "absolute -top-2 -right-2 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center",
                        "{highlights.len()}"
                    }
                }
            }
            
            // 侧边栏
            div {
                class: if show_sidebar() {
                    "fixed right-0 top-0 h-full w-80 bg-white dark:bg-gray-800 shadow-lg transform translate-x-0 transition-transform z-50"
                } else {
                    "fixed right-0 top-0 h-full w-80 bg-white dark:bg-gray-800 shadow-lg transform translate-x-full transition-transform z-50"
                },
                
                // 头部
                div {
                    class: "p-4 border-b border-gray-200 dark:border-gray-700",
                    div {
                        class: "flex items-center justify-between",
                        h3 {
                            class: "text-lg font-semibold",
                            "我的高亮和笔记"
                        }
                        button {
                            class: "p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded",
                            onclick: move |_| show_sidebar.set(false),
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }
                    }
                }
                
                // 高亮列表
                div {
                    class: "overflow-y-auto h-full pb-20",
                    if highlights.is_empty() {
                        div {
                            class: "p-8 text-center text-gray-500",
                            p { "还没有高亮或笔记" }
                            p { class: "text-sm mt-2", "选择文本开始添加" }
                        }
                    } else {
                        div {
                            class: "p-4 space-y-3",
                            for highlight in highlights {
                                HighlightCard {
                                    highlight: highlight.clone(),
                                    on_delete: move |id| on_delete.call(id)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HighlightCard(
    highlight: Highlight,
    on_delete: EventHandler<String>,
) -> Element {
    let color_value = match highlight.color.as_str() {
        "yellow" => "#FEF3C7",
        "green" => "#D1FAE5",
        "blue" => "#DBEAFE",
        "purple" => "#E9D5FF",
        "red" => "#FEE2E2",
        _ => "#FEF3C7",
    };
    
    let note_text = highlight.note.clone();
    
    rsx! {
        div {
            class: "border border-gray-200 dark:border-gray-700 rounded-lg p-3 hover:shadow-md transition-shadow",
            
            // 高亮文本
            div {
                class: "mb-2",
                style: "background-color: {color_value}; padding: 4px 8px; border-radius: 4px;",
                p {
                    class: "text-sm",
                    "\"{highlight.content}\""
                }
            }
            
            // 笔记
            if let Some(note) = note_text {
                div {
                    class: "mb-2",
                    p {
                        class: "text-sm text-gray-600 dark:text-gray-400",
                        {note}
                    }
                }
            }
            
            // 底部操作
            div {
                class: "flex items-center justify-between text-xs text-gray-500",
                span {
                    {highlight.created_at.format("%Y-%m-%d %H:%M").to_string()}
                }
                button {
                    class: "text-red-500 hover:text-red-700",
                    onclick: move |_| on_delete.call(highlight.id.clone()),
                    "删除"
                }
            }
        }
    }
}
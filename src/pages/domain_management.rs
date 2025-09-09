use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    api::domains::DomainService,
    models::domain::*,
    hooks::use_auth,
    Route,
};

#[component]
pub fn DomainManagementPage(publication_id: String) -> Element {
    let publication_id = publication_id.clone();
    let mut domains = use_signal(|| Vec::<PublicationDomain>::new());
    let mut loading = use_signal(|| true);
    let mut show_add_modal = use_signal(|| false);
    let mut domain_type = use_signal(|| "subdomain".to_string());
    let mut error = use_signal(|| None::<String>);
    
    // 表单状态
    let mut subdomain_input = use_signal(|| String::new());
    let mut custom_domain_input = use_signal(|| String::new());
    let mut is_primary = use_signal(|| false);
    let mut creating = use_signal(|| false);
    
    let auth = use_auth();
    
    // 加载域名列表
    let load_domains = {
        let loading = loading.clone();
        let domains = domains.clone();
        let error = error.clone();
        let publication_id = publication_id.clone();
        
        move || {
            let mut loading = loading.clone();
            let mut domains = domains.clone();
            let mut error = error.clone();
            let publication_id = publication_id.clone();
            
            spawn(async move {
                loading.set(true);
                
                match DomainService::get_publication_domains(&publication_id).await {
                    Ok(domain_list) => {
                        domains.set(domain_list);
                    }
                    Err(e) => {
                        error.set(Some(format!("加载域名失败: {}", e.message)));
                    }
                }
                
                loading.set(false);
            });
        }
    };
    
    // 初始加载
    use_effect({
        let load_domains = load_domains.clone();
        move || {
            load_domains();
        }
    });
    
    // 创建域名
    let create_domain = {
        let publication_id = publication_id.clone();
        move |_| {
        let mut creating = creating.clone();
        let mut error = error.clone();
        let domain_type = domain_type.clone();
        let subdomain_input = subdomain_input.clone();
        let is_primary = is_primary.clone();
        let custom_domain_input = custom_domain_input.clone();
        let mut show_add_modal = show_add_modal.clone();
        let mut subdomain_input = subdomain_input.clone();
        let mut custom_domain_input = custom_domain_input.clone();
        let mut is_primary = is_primary.clone();
        let load_domains = load_domains.clone();
        
        creating.set(true);
        error.set(None);
        
        let pub_id = publication_id.clone();
        
        spawn(async move {
            let result = if domain_type() == "subdomain" {
                let request = CreateSubdomainRequest {
                    subdomain: subdomain_input(),
                    is_primary: is_primary(),
                };
                DomainService::create_subdomain(&pub_id, &request).await.map(|_| ())
            } else {
                let request = CreateCustomDomainRequest {
                    domain: custom_domain_input(),
                    is_primary: is_primary(),
                };
                DomainService::add_custom_domain(&pub_id, &request).await.map(|_| ())
            };
            
            match result {
                Ok(_) => {
                    show_add_modal.set(false);
                    subdomain_input.set(String::new());
                    custom_domain_input.set(String::new());
                    is_primary.set(false);
                    load_domains();
                }
                Err(e) => {
                    error.set(Some(format!("创建域名失败: {}", e.message)));
                }
            }
            
            creating.set(false);
        });
        }
    };
    
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
                        
                        Link {
                            to: Route::Publications {},
                            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white",
                            "← 返回出版物管理"
                        }
                    }
                }
            }
            
            // 页面内容
            div {
                class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // 页面标题
                div {
                    class: "flex items-center justify-between mb-8",
                    h1 {
                        class: "text-3xl font-bold text-gray-900 dark:text-white",
                        "域名管理"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                        onclick: move |_| show_add_modal.set(true),
                        "添加域名"
                    }
                }
                
                // 错误提示
                if let Some(err) = error() {
                    div {
                        class: "mb-6 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 rounded-lg",
                        {err}
                    }
                }
                
                // 域名列表
                if loading() {
                    div {
                        class: "flex justify-center py-12",
                        div {
                            class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"
                        }
                    }
                } else if domains().is_empty() {
                    div {
                        class: "text-center py-12 bg-gray-50 dark:bg-gray-800 rounded-lg",
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400 mb-4",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0 9c0 1.657-4.03 3-9 3s-9-1.343-9-3m9-9c0-1.657-4.03-3-9-3s-9 1.343-9 3m0 9v-9"
                            }
                        }
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                            "还没有配置域名"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 mb-4",
                            "为您的出版物配置自定义域名或子域名"
                        }
                        button {
                            class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                            onclick: move |_| show_add_modal.set(true),
                            "添加第一个域名"
                        }
                    }
                } else {
                    div {
                        class: "space-y-4",
                        for domain in domains() {
                            DomainCard {
                                domain,
                                on_delete: {
                                    let pub_id = publication_id.clone();
                                    let mut domains = domains.clone();
                                    move |domain_id: String| {
                                        let pub_id = pub_id.clone();
                                        let mut domains = domains.clone();
                                        spawn(async move {
                                            if let Ok(_) = DomainService::delete_domain(&domain_id).await {
                                                // 重新加载域名列表
                                                if let Ok(domain_list) = DomainService::get_publication_domains(&pub_id).await {
                                                    domains.set(domain_list);
                                                }
                                            }
                                        });
                                    }
                                },
                                on_set_primary: {
                                    let pub_id = publication_id.clone();
                                    let mut domains = domains.clone();
                                    move |domain_id: String| {
                                        let pub_id = pub_id.clone();
                                        let mut domains = domains.clone();
                                        spawn(async move {
                                        if let Ok(_) = DomainService::set_primary_domain(&domain_id).await {
                                            // 重新加载域名列表
                                            if let Ok(domain_list) = DomainService::get_publication_domains(&pub_id).await {
                                                domains.set(domain_list);
                                            }
                                        }
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 添加域名模态框
            if show_add_modal() {
                div {
                    class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                    onclick: move |_| show_add_modal.set(false),
                    
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-md mx-4",
                        onclick: move |e| e.stop_propagation(),
                        
                        h3 {
                            class: "text-lg font-semibold mb-6",
                            "添加域名"
                        }
                        
                        // 域名类型选择
                        div {
                            class: "mb-6",
                            label {
                                class: "block text-sm font-medium mb-2",
                                "域名类型"
                            }
                            div {
                                class: "flex space-x-4",
                                label {
                                    class: "flex items-center",
                                    input {
                                        r#type: "radio",
                                        name: "domain_type",
                                        value: "subdomain",
                                        checked: domain_type() == "subdomain",
                                        onchange: move |_| domain_type.set("subdomain".to_string()),
                                        class: "mr-2"
                                    }
                                    "子域名"
                                }
                                label {
                                    class: "flex items-center",
                                    input {
                                        r#type: "radio",
                                        name: "domain_type",
                                        value: "custom",
                                        checked: domain_type() == "custom",
                                        onchange: move |_| domain_type.set("custom".to_string()),
                                        class: "mr-2"
                                    }
                                    "自定义域名"
                                }
                            }
                        }
                        
                        // 域名输入
                        if domain_type() == "subdomain" {
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "子域名"
                                }
                                div {
                                    class: "flex items-center",
                                    input {
                                        r#type: "text",
                                        placeholder: "myblog",
                                        value: "{subdomain_input}",
                                        oninput: move |e| subdomain_input.set(e.value()),
                                        class: "flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-l-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    }
                                    span {
                                        class: "px-3 py-2 bg-gray-100 dark:bg-gray-700 border border-l-0 border-gray-300 dark:border-gray-600 rounded-r-lg text-sm",
                                        ".platform.com"
                                    }
                                }
                                p {
                                    class: "text-xs text-gray-500 mt-1",
                                    "3-63个字符，仅限字母数字和连字符"
                                }
                            }
                        } else {
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium mb-2",
                                    "自定义域名"
                                }
                                input {
                                    r#type: "text",
                                    placeholder: "blog.example.com",
                                    value: "{custom_domain_input}",
                                    oninput: move |e| custom_domain_input.set(e.value()),
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                }
                                p {
                                    class: "text-xs text-gray-500 mt-1",
                                    "需要DNS验证，请确保您拥有该域名"
                                }
                            }
                        }
                        
                        // 设为主域名选项
                        div {
                            class: "mb-6",
                            label {
                                class: "flex items-center",
                                input {
                                    r#type: "checkbox",
                                    checked: is_primary(),
                                    onchange: move |_| is_primary.set(!is_primary()),
                                    class: "mr-2"
                                }
                                "设为主域名"
                            }
                            p {
                                class: "text-xs text-gray-500 mt-1",
                                "主域名将作为默认访问地址"
                            }
                        }
                        
                        // 按钮
                        div {
                            class: "flex justify-end space-x-3",
                            button {
                                class: "px-4 py-2 text-gray-600 hover:text-gray-800",
                                onclick: move |_| {
                                    show_add_modal.set(false);
                                    error.set(None);
                                },
                                "取消"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50",
                                disabled: creating() || (domain_type() == "subdomain" && subdomain_input().is_empty()) || (domain_type() == "custom" && custom_domain_input().is_empty()),
                                onclick: create_domain,
                                if creating() {
                                    "创建中..."
                                } else {
                                    "创建域名"
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
fn DomainCard(
    domain: PublicationDomain,
    on_delete: EventHandler<String>,
    on_set_primary: EventHandler<String>,
) -> Element {
    let mut show_dns_modal = use_signal(|| false);
    let mut verification_records = use_signal(|| Vec::<DNSRecord>::new());
    let mut loading_records = use_signal(|| false);
    
    // 提取需要的值
    let domain_status = domain.status.clone();
    let domain_id_for_records = domain.id.clone();
    
    
    let status_color = match domain.status {
        DomainStatus::Active => "text-green-600",
        DomainStatus::Pending | DomainStatus::Verifying => "text-yellow-600",
        DomainStatus::Failed => "text-red-600",
    };
    
    let ssl_status_color = match domain.ssl_status {
        SSLStatus::Active => "text-green-600",
        SSLStatus::Pending => "text-yellow-600",
        SSLStatus::Failed | SSLStatus::Expired => "text-red-600",
    };
    
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-6",
            
            div {
                class: "flex items-start justify-between",
                div {
                    class: "flex-1",
                    
                    // 域名
                    div {
                        class: "flex items-center space-x-2 mb-2",
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white",
                            {domain.get_full_domain()}
                        }
                        if domain.is_primary {
                            span {
                                class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-xs rounded",
                                "主域名"
                            }
                        }
                    }
                    
                    // 状态信息
                    div {
                        class: "space-y-1 text-sm",
                        div {
                            class: "flex items-center space-x-2",
                            span { class: "text-gray-500", "状态：" }
                            span { 
                                class: status_color,
                                match domain.status {
                                    DomainStatus::Active => "活跃",
                                    DomainStatus::Pending => "待验证",
                                    DomainStatus::Verifying => "验证中",
                                    DomainStatus::Failed => "验证失败",
                                }
                            }
                        }
                        div {
                            class: "flex items-center space-x-2",
                            span { class: "text-gray-500", "SSL：" }
                            span {
                                class: ssl_status_color,
                                match domain.ssl_status {
                                    SSLStatus::Active => "已配置",
                                    SSLStatus::Pending => "配置中",
                                    SSLStatus::Failed => "配置失败",
                                    SSLStatus::Expired => "已过期",
                                }
                            }
                        }
                    }
                    
                    // 验证提示
                    if matches!(domain.status, DomainStatus::Pending | DomainStatus::Verifying) && domain.custom_domain.is_some() {
                        div {
                            class: "mt-3 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg",
                            p {
                                class: "text-sm text-yellow-800 dark:text-yellow-200",
                                "需要配置DNS记录来验证域名所有权"
                            }
                            button {
                                class: "mt-2 text-sm text-yellow-600 hover:text-yellow-800 underline",
                                onclick: move |_| {
                                    show_dns_modal.set(true);
                                    if matches!(domain_status, DomainStatus::Pending | DomainStatus::Verifying) {
                                        loading_records.set(true);
                                        let domain_id = domain_id_for_records.clone();
                                        spawn(async move {
                                            if let Ok(records) = DomainService::get_verification_records(&domain_id).await {
                                                verification_records.set(records);
                                            }
                                            loading_records.set(false);
                                        });
                                    }
                                },
                                "查看DNS配置说明"
                            }
                        }
                    }
                }
                
                // 操作菜单
                div {
                    class: "flex items-center space-x-2",
                    if !domain.is_primary && matches!(domain.status, DomainStatus::Active) {
                        button {
                            class: "px-3 py-1 text-sm text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded",
                            onclick: {
                                let domain_id = domain.id.clone();
                                move |_| on_set_primary.call(domain_id.clone())
                            },
                            "设为主域名"
                        }
                    }
                    button {
                        class: "px-3 py-1 text-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded",
                        onclick: {
                            let domain_id = domain.id.clone();
                            move |_| on_delete.call(domain_id.clone())
                        },
                        "删除"
                    }
                }
            }
        }
        
        // DNS配置模态框
        if show_dns_modal() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
                onclick: move |_| show_dns_modal.set(false),
                
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    
                    div {
                        class: "flex items-center justify-between mb-6",
                        h3 {
                            class: "text-lg font-semibold",
                            "DNS配置说明"
                        }
                        button {
                            class: "text-gray-500 hover:text-gray-700",
                            onclick: move |_| show_dns_modal.set(false),
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
                    
                    p {
                        class: "text-gray-600 dark:text-gray-400 mb-6",
                        "请在您的域名DNS管理后台添加以下记录来验证域名所有权："
                    }
                    
                    if loading_records() {
                        div {
                            class: "flex justify-center py-4",
                            div {
                                class: "animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"
                            }
                        }
                    } else {
                        div {
                            class: "space-y-4",
                            for record in verification_records() {
                                div {
                                    class: "border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                    div {
                                        class: "grid grid-cols-4 gap-4 text-sm",
                                        div {
                                            span { class: "font-medium", "类型" }
                                            p { class: "font-mono text-blue-600", {record.record_type.clone()} }
                                        }
                                        div {
                                            class: "col-span-2",
                                            span { class: "font-medium", "名称" }
                                            p { class: "font-mono text-blue-600 break-all", {record.name.clone()} }
                                        }
                                        div {
                                            span { class: "font-medium", "值" }
                                            p { class: "font-mono text-blue-600 break-all", {record.value.clone()} }
                                        }
                                    }
                                    p {
                                        class: "text-xs text-gray-500 mt-2",
                                        "用途：{record.purpose}"
                                    }
                                }
                            }
                        }
                    }
                    
                    div {
                        class: "mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg",
                        h4 {
                            class: "text-sm font-medium mb-2",
                            "配置完成后"
                        }
                        ul {
                            class: "text-sm text-gray-600 dark:text-gray-400 space-y-1",
                            li { "• 系统会自动检测DNS配置（每5分钟检查一次）" }
                            li { "• 验证成功后会自动激活域名" }
                            li { "• SSL证书会在域名激活后自动配置" }
                            li { "• 整个过程通常需要5-30分钟" }
                        }
                    }
                }
            }
        }
    }
}
# Rainbow Blog Frontend

基于 Dioxus (Rust) 框架的现代化博客前端，采用 Medium 风格设计，提供优雅的阅读和写作体验。

## 技术栈

- **框架**: Dioxus 0.6.x - Rust 原生的响应式 UI 框架
- **目标**: WebAssembly (WASM) - 高性能 Web 应用
- **样式**: Tailwind CSS - 实用优先的 CSS 框架
- **路由**: Dioxus Router - 单页面应用路由管理
- **HTTP客户端**: reqwest - 异步 HTTP 请求库
- **状态管理**: Dioxus Signals - 响应式状态管理
- **存储**: gloo-storage - 本地存储封装

## 项目架构

### 微服务架构
项目采用微服务架构，前端与多个后端服务通信：

- **Rainbow-Auth** (端口 8080): 用户认证服务
- **Rainbow-Blog** (端口 3001): 博客核心服务
- **前端** (端口 3000): Dioxus Web 应用

### 目录结构

```
src/
├── api/                    # API 客户端模块
│   ├── client.rs          # HTTP 客户端封装
│   ├── auth.rs            # 认证相关 API
│   ├── users.rs           # 用户管理 API
│   ├── articles.rs        # 文章相关 API
│   ├── comments.rs        # 评论系统 API
│   ├── search.rs          # 搜索功能 API
│   └── ...
├── components/             # 可复用组件
│   ├── article_card.rs    # 文章卡片组件
│   ├── comment_v2.rs      # 评论系统组件
│   ├── header.rs          # 页面头部组件
│   ├── image_upload.rs    # 图片上传组件
│   └── ...
├── hooks/                  # 自定义 Hooks
│   ├── use_auth.rs        # 认证状态管理
│   └── use_theme.rs       # 主题切换管理
├── models/                 # 数据模型
│   ├── auth.rs            # 认证相关模型
│   ├── user.rs            # 用户数据模型
│   ├── article.rs         # 文章数据模型
│   └── ...
├── pages/                  # 页面组件
│   ├── home.rs            # 首页
│   ├── profile.rs         # 个人中心页面
│   ├── article.rs         # 文章详情页
│   ├── editor_v2.rs       # 文章编辑器
│   └── ...
├── main.rs                 # 应用入口
└── lib.rs                  # 库文件
```

## 核心功能

### 🎨 Medium 风格设计
- 优雅的排版和间距设计
- 响应式布局，支持多种屏幕尺寸
- 渐变头像、彩色统计数据等视觉亮点
- 专业的空状态和加载状态

### 👤 用户系统
- JWT Token 认证
- 用户注册/登录
- 个人资料管理
- 关注/取消关注功能
- Medium 风格的个人中心页面

### 📝 文章系统
- 富文本编辑器 (Editor v2)
- 文章发布/草稿保存
- 标签系统
- 文章搜索
- 阅读时间估算

### 💬 交互功能
- 评论系统 (支持嵌套回复)
- 文章点赞 (Clap) 功能
- 高亮系统 (文本标注)
- 图片上传和展示

### 🔍 搜索和发现
- 全文搜索功能
- 标签浏览
- 文章推荐系统
- 热门用户展示

## 开发环境设置

### 环境要求
- Rust 1.70+ 
- Dioxus CLI (`cargo install dioxus-cli`)
- 现代浏览器 (支持 WebAssembly)

### 安装依赖
```bash
# 克隆项目
git clone https://github.com/your-org/rainbow-blog-front
cd rainbow-blog-front

# 安装 Rust 依赖
cargo build
```

### 开发模式运行
```bash
# 启动开发服务器 (热重载)
dx serve --port 3000 --hot-reload

# 或者使用标准端口
dx serve
```

### 生产构建
```bash
# 构建 WebAssembly 版本
dx build --platform web --release

# 构建后的文件位于 dist/ 目录
```

## API 配置

### 环境变量
创建 `.env` 文件配置后端服务地址：

```env
# 认证服务地址
RAINBOW_AUTH_URL=http://localhost:8080

# 博客服务地址
RAINBOW_BLOG_URL=http://localhost:3001
```

### API 端点配置
在 `src/api/client.rs` 中配置 API 基础 URL：

```rust
const AUTH_BASE_URL: &str = "http://localhost:8080";
const BLOG_BASE_URL: &str = "http://localhost:3001";
```

## 主要特性

### 🚀 高性能
- WebAssembly 带来接近原生的执行速度
- 虚拟 DOM 优化和批量更新
- 代码分割和懒加载支持

### 📱 响应式设计
- 移动端优先的设计理念
- 灵活的网格系统
- Touch 友好的交互设计

### 🔒 安全性
- JWT Token 自动管理
- XSS 防护
- CSRF 保护
- 安全的文件上传

### 🎯 用户体验
- 流畅的页面切换动画
- 智能的加载状态
- 优雅的错误处理
- 键盘快捷键支持

## 路由系统

应用使用 Dioxus Router 进行单页面路由管理：

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/@:username")]
    Profile { username: String },
    
    #[route("/profile/:user_id")]
    ProfileById { user_id: String },
    
    #[route("/article/:slug")]
    Article { slug: String },
    
    #[route("/write")]
    Write {},
    
    // ... 更多路由
}
```

## 状态管理

使用 Dioxus Signals 进行响应式状态管理：

```rust
// 全局认证状态
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub is_authenticated: Signal<bool>,
    pub user: Signal<Option<User>>,
    pub token: Signal<Option<String>>,
}

// 主题状态
#[derive(Clone, Copy)]  
pub struct ThemeContext {
    pub theme: Signal<Theme>,
}
```

## 组件开发指南

### 创建新组件
```rust
#[component]
pub fn MyComponent(title: String, content: Option<String>) -> Element {
    rsx! {
        div {
            class: "my-component",
            h2 { {title} }
            if let Some(content) = content {
                p { {content} }
            }
        }
    }
}
```

### 使用 Hooks
```rust
pub fn MyPage() -> Element {
    let auth = use_auth(); // 获取认证状态
    let theme = use_theme(); // 获取主题状态
    
    // 组件逻辑...
}
```

## 样式开发

项目使用 Tailwind CSS，遵循以下约定：

### 颜色方案
- 主色调: `gray-900` (深色文字)
- 次要色: `gray-600` (次要文字)
- 强调色: `blue-600`, `green-600`, `purple-600`
- 背景色: `white`, `gray-50`, `gray-100`

### 常用样式类
```css
/* 容器 */
.container: "max-w-4xl mx-auto px-6"
.card: "bg-white rounded-lg shadow-sm border border-gray-200"

/* 按钮 */
.btn-primary: "px-6 py-2 bg-gray-900 text-white rounded-lg font-medium hover:bg-gray-800"
.btn-secondary: "px-6 py-2 border border-gray-300 text-gray-700 rounded-lg font-medium hover:bg-gray-50"

/* 文字 */
.heading: "text-3xl font-bold text-gray-900"
.subheading: "text-xl font-semibold text-gray-800"
.body: "text-gray-700 leading-relaxed"
```

## 性能优化

### 构建优化
```toml
# Dioxus.toml
[application]
name = "rainbow-blog-front"
default_platform = "web"

[web.watcher]
reload_html = true
watch_path = ["src", "assets"]

[web.resource]
style = ["./assets/tailwind.css"]
```

### 代码分割
大型组件可以使用懒加载：

```rust
// 懒加载大型编辑器组件
let editor = use_memo(|| {
    // 只在需要时加载编辑器
    if show_editor() {
        Some(rsx! { EditorV2Page { slug: None } })
    } else {
        None
    }
});
```

## 测试

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test auth

# 运行集成测试
cargo test --test integration
```

### 测试覆盖率
```bash
# 安装 cargo-tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

## 部署

### Docker 部署
```dockerfile
FROM nginx:alpine
COPY dist/ /usr/share/nginx/html/
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
```

### Vercel 部署
```json
{
  "build": {
    "env": {
      "RUST_VERSION": "1.75.0"
    }
  },
  "buildCommand": "dx build --platform web --release",
  "outputDirectory": "dist"
}
```

## 贡献指南

### 代码规范
- 使用 `rustfmt` 格式化代码
- 遵循 Rust 命名约定
- 组件名使用 PascalCase
- 文件名使用 snake_case

### 提交规范
```
feat: 添加新功能
fix: 修复问题
docs: 更新文档
style: 代码格式调整
refactor: 代码重构
test: 添加测试
chore: 构建相关
```

### Pull Request 流程
1. Fork 项目并创建功能分支
2. 完成开发并添加测试
3. 确保所有测试通过
4. 提交 PR 并描述更改内容

## 常见问题

### 编译错误
**Q: 出现 "cannot find crate" 错误？**
A: 确保运行了 `cargo build` 安装依赖

**Q: WebAssembly 相关错误？**
A: 检查是否安装了正确的 target: `rustup target add wasm32-unknown-unknown`

### 运行时错误
**Q: API 请求失败？**
A: 检查后端服务是否启动，确认 API 地址配置正确

**Q: 路由不工作？**
A: 确保使用了正确的路由格式，检查 `Route` 枚举定义

### 样式问题
**Q: Tailwind 样式不生效？**
A: 确保 CSS 文件正确引入，检查类名是否正确

## 许可证

MIT License

## 联系方式

- 项目地址: https://github.com/your-org/rainbow-blog-front
- 问题反馈: https://github.com/your-org/rainbow-blog-front/issues
- 邮箱: contact@rainbowblog.com

---

**Rainbow Blog Frontend** - 用 Rust 构建的下一代博客前端 ✨
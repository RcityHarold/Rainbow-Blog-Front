pub mod route_guard;
pub mod article_card;
pub mod comment_v2;
pub mod version_history;
pub mod image_upload;
pub mod share_modal;
pub mod highlight_system;
pub mod recommendations;
pub mod subscription_widget;

pub use route_guard::*;
pub use article_card::*;
// 使用新的评论组件
pub use comment_v2::CommentSection;
pub use version_history::*;
pub use image_upload::*;
pub use share_modal::*;
pub use highlight_system::*;
pub use recommendations::*;
pub use subscription_widget::*;
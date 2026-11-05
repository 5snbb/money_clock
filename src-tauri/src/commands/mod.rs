//! IPC command layer.
//!
//! Commands are the only backend entry points exposed to the webview. They
//! validate arguments and delegate to services/storage; they must not contain
//! business logic.

mod app;
mod platform;
mod storage;

pub use app::get_app_info;
pub use platform::get_platform_info;
pub use storage::{
    storage_delete_document, storage_list_documents, storage_read_document, storage_write_document,
};

//! Service layer.
//!
//! Services orchestrate domain logic and system capabilities. Tauri commands
//! call into this layer; services never depend on the frontend.

mod app;
mod platform;

pub use app::{AppInfo, AppService};
pub use platform::{PlatformInfo, PlatformService};

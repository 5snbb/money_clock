use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};

use crate::error::AppError;
use crate::platform::{self, PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

/// Platform information combined from the active adapter and Tauri's path
/// resolver.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub kind: PlatformKind,
    pub display_name: String,
    pub os: String,
    pub arch: String,
    pub family: String,
    pub app_data_dir: Option<String>,
    pub config_dir: Option<String>,
    pub capabilities: PlatformCapabilities,
    pub window_defaults: WindowDefaults,
}

/// Service that exposes the current platform adapter.
pub struct PlatformService {
    adapter: &'static dyn PlatformAdapter,
}

impl PlatformService {
    pub fn new() -> Self {
        Self {
            adapter: platform::current(),
        }
    }

    pub fn describe<R: Runtime>(&self, app: &AppHandle<R>) -> Result<PlatformInfo, AppError> {
        let paths = app.path();

        Ok(PlatformInfo {
            kind: self.adapter.kind(),
            display_name: self.adapter.display_name().to_owned(),
            os: std::env::consts::OS.to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            family: std::env::consts::FAMILY.to_owned(),
            app_data_dir: paths
                .app_data_dir()
                .ok()
                .map(|path| path.display().to_string()),
            config_dir: paths
                .app_config_dir()
                .ok()
                .map(|path| path.display().to_string()),
            capabilities: self.adapter.capabilities(),
            window_defaults: self.adapter.window_defaults(),
        })
    }
}

impl Default for PlatformService {
    fn default() -> Self {
        Self::new()
    }
}

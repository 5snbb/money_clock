use serde::Serialize;
use tauri::{AppHandle, Runtime};

/// Application metadata exposed to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub tauri_version: String,
    pub os: String,
    pub arch: String,
    pub build_profile: String,
    pub debug: bool,
}

/// Read-only service that summarizes application metadata.
#[derive(Debug, Default)]
pub struct AppService;

impl AppService {
    pub fn info<R: Runtime>(&self, app: &AppHandle<R>) -> AppInfo {
        let package = app.package_info();
        let name = app
            .config()
            .product_name
            .clone()
            .unwrap_or_else(|| package.name.clone());

        AppInfo {
            name,
            version: package.version.to_string(),
            tauri_version: tauri::VERSION.to_owned(),
            os: std::env::consts::OS.to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            build_profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .to_owned(),
            debug: cfg!(debug_assertions),
        }
    }
}

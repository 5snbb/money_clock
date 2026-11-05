use tauri::{AppHandle, Runtime, State};

use crate::services::AppInfo;
use crate::state::AppState;

/// Returns application metadata resolved from the Tauri configuration and the
/// compiled Rust package.
#[tauri::command]
pub fn get_app_info<R: Runtime>(app: AppHandle<R>, state: State<'_, AppState>) -> AppInfo {
    state.app.info(&app)
}

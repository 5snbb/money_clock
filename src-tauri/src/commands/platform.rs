use tauri::{AppHandle, Runtime, State};

use crate::error::AppError;
use crate::services::PlatformInfo;
use crate::state::AppState;

/// Returns OS details, adapter capabilities and backend-resolved directories.
#[tauri::command]
pub fn get_platform_info<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<PlatformInfo, AppError> {
    state.platform.describe(&app)
}

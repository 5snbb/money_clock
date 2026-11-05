mod commands;
mod error;
mod platform;
mod services;
mod state;
mod storage;

use tauri::Manager;

use crate::commands::{
    get_app_info, get_platform_info, storage_delete_document, storage_list_documents,
    storage_read_document, storage_write_document,
};
use crate::state::AppState;

/// Builds the application shell, registers shared state and starts the Tauri
/// event loop.
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = AppState::init(app.handle())?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_platform_info,
            storage_list_documents,
            storage_read_document,
            storage_write_document,
            storage_delete_document,
        ])
        .run(tauri::generate_context!())
        .expect("error while running money_clock");
}

use serde_json::Value;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

/// Lists the JSON documents persisted by the storage layer.
#[tauri::command]
pub fn storage_list_documents(state: State<'_, AppState>) -> Result<Vec<String>, AppError> {
    state.storage.list()
}

/// Reads a JSON document; returns `null` when the document does not exist.
#[tauri::command]
pub fn storage_read_document(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<Value>, AppError> {
    state.storage.read::<Value>(&name)
}

/// Writes a JSON document atomically.
#[tauri::command]
pub fn storage_write_document(
    state: State<'_, AppState>,
    name: String,
    value: Value,
) -> Result<(), AppError> {
    state.storage.write(&name, &value)
}

/// Deletes a JSON document; returns whether a file was removed.
#[tauri::command]
pub fn storage_delete_document(state: State<'_, AppState>, name: String) -> Result<bool, AppError> {
    state.storage.remove(&name)
}

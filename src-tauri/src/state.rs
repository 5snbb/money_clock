use std::sync::Arc;

use tauri::{AppHandle, Manager, Runtime};

use crate::error::AppError;
use crate::services::{AppService, PlatformService};
use crate::storage::JsonDocumentStore;

/// Shared application state managed by Tauri and injected into commands.
pub struct AppState {
    pub app: AppService,
    pub platform: PlatformService,
    pub storage: Arc<JsonDocumentStore>,
}

impl AppState {
    /// Resolves backend-owned directories and prepares the storage root.
    pub fn init<R: Runtime>(app: &AppHandle<R>) -> Result<Self, AppError> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|error| AppError::PathResolution(error.to_string()))?;

        let storage = JsonDocumentStore::new(app_data_dir.join("storage"));
        storage.ensure_ready()?;

        Ok(Self {
            app: AppService,
            platform: PlatformService::new(),
            storage: Arc::new(storage),
        })
    }
}

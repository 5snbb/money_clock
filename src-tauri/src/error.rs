use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

/// Unified error type for every Tauri command.
///
/// Commands return `Result<_, AppError>`; the serialized payload (`code` and
/// `message`) is normalized on the frontend by `services/ipc.ts`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("文件操作失败：{0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 序列化失败：{0}")]
    Serialization(#[from] serde_json::Error),

    #[error("存储文档名称不合法：{0}")]
    InvalidDocumentName(String),

    #[error("无法解析应用目录：{0}")]
    PathResolution(String),
}

impl AppError {
    /// Stable machine-readable code exposed to the frontend.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Io(_) => "io_error",
            Self::Serialization(_) => "serialization_error",
            Self::InvalidDocumentName(_) => "invalid_document_name",
            Self::PathResolution(_) => "path_resolution_error",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

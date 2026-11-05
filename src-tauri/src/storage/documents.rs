use std::fs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AppError;

const DOCUMENT_NAME_MAX_LEN: usize = 64;

/// Minimal JSON document store rooted in a directory owned by the backend.
///
/// Documents are addressed by a validated name (letters, digits, `_`, `-`) and
/// written atomically through a temporary file. The frontend never supplies or
/// receives filesystem paths.
#[derive(Debug, Clone)]
pub struct JsonDocumentStore {
    root: PathBuf,
}

impl JsonDocumentStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Creates the storage directory if it does not exist yet.
    pub fn ensure_ready(&self) -> Result<(), AppError> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    /// Reads a document, returning `None` when it does not exist yet.
    pub fn read<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>, AppError> {
        let path = self.document_path(name)?;

        match fs::read_to_string(&path) {
            Ok(contents) => Ok(Some(serde_json::from_str(&contents)?)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    /// Writes a document atomically: serialize, write a sibling temp file, then
    /// rename it over the target.
    pub fn write<T: Serialize + ?Sized>(&self, name: &str, value: &T) -> Result<(), AppError> {
        let path = self.document_path(name)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let serialized = serde_json::to_vec_pretty(value)?;
        let temp_path = path.with_extension("json.tmp");

        fs::write(&temp_path, serialized)?;
        fs::rename(&temp_path, &path)?;

        Ok(())
    }

    /// Deletes a document, returning `false` when it was already absent.
    pub fn remove(&self, name: &str) -> Result<bool, AppError> {
        let path = self.document_path(name)?;

        match fs::remove_file(path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    /// Lists available document names in a stable order.
    pub fn list(&self) -> Result<Vec<String>, AppError> {
        let mut names = Vec::new();

        if !self.root.exists() {
            return Ok(names);
        }

        for entry in fs::read_dir(&self.root)? {
            let path = entry?.path();

            if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
                continue;
            }

            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                names.push(stem.to_owned());
            }
        }

        names.sort();
        Ok(names)
    }

    fn document_path(&self, name: &str) -> Result<PathBuf, AppError> {
        if !is_valid_document_name(name) {
            return Err(AppError::InvalidDocumentName(name.to_owned()));
        }

        Ok(self.root.join(format!("{name}.json")))
    }
}

/// Document names are restricted to a safe charset to rule out path traversal
/// and platform-specific filename surprises.
fn is_valid_document_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= DOCUMENT_NAME_MAX_LEN
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_store() -> JsonDocumentStore {
        let root = std::env::temp_dir().join(format!(
            "money_clock-storage-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock should be after the UNIX epoch")
                .as_nanos()
        ));

        JsonDocumentStore::new(root)
    }

    #[test]
    fn rejects_unsafe_document_names() {
        for name in ["", "../escape", "nested/doc", "with space", "dot.name"] {
            assert!(
                !is_valid_document_name(name),
                "expected `{name}` to be rejected"
            );
        }
    }

    #[test]
    fn writes_reads_lists_and_removes_documents() {
        let store = test_store();
        store
            .ensure_ready()
            .expect("storage root should be created");

        store
            .write("settings", &json!({ "theme": "dark" }))
            .expect("document should be written");

        let document: Option<serde_json::Value> =
            store.read("settings").expect("document should be readable");
        assert_eq!(document, Some(json!({ "theme": "dark" })));
        assert_eq!(
            store.list().expect("documents should be listed"),
            vec!["settings".to_owned()]
        );

        assert!(store
            .remove("settings")
            .expect("document should be removed"));
        assert_eq!(
            store
                .read::<serde_json::Value>("settings")
                .expect("missing document should be empty"),
            None
        );

        std::fs::remove_dir_all(&store.root).expect("temporary storage should be cleaned up");
    }
}

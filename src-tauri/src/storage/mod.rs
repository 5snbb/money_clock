//! Persistence primitives.
//!
//! The storage layer owns path resolution, validation and file I/O. Callers
//! only exchange document names and JSON values.

mod documents;

pub use documents::JsonDocumentStore;

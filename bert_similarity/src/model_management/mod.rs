// Model management module - model registry, loading, and persistence

pub mod loader;
pub mod persistence;
pub mod registry;

// Re-export for backward compatibility
pub use loader::HuggingFaceModelLoader;
pub use persistence::{ModelMetadata, ModelPersistence, ModelVersion};
pub use registry::ModelRegistry;

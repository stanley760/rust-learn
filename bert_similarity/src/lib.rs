// Core modules - new architecture
pub mod core;
pub mod inference;
pub mod model_management;

// Legacy modules - maintained for backward compatibility
pub mod api;
pub mod model;
pub mod training;
pub mod utils;

// Re-exports for backward compatibility
// The new architecture exports through these paths to maintain API compatibility
pub use core::{BertModel, TokenizerWrapper, get_device};
pub use inference::{ModelEngine, SimilarityCalculator, OppositionDetector};
pub use model_management::{HuggingFaceModelLoader, ModelRegistry, ModelPersistence, ModelMetadata, ModelVersion};

// Legacy re-exports (will be deprecated in future)
pub use model::{BertModel as LegacyBertModel, ModelEngine as LegacyModelEngine};

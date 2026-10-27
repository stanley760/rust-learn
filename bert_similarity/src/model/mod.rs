// Model module - handles BERT model loading, inference, and embeddings
//
// This module now re-exports from the new core architecture for backward compatibility.

pub mod persistence;
pub mod tokenizer;

// Re-export from new architecture
pub use crate::core::{get_device, BertModel, TokenizerWrapper};
pub use crate::inference::{ModelEngine, OppositionDetector, SimilarityCalculator};
pub use crate::model_management::{
    HuggingFaceModelLoader, ModelMetadata, ModelPersistence, ModelVersion,
};

// Note: The sub-modules (persistence.rs, tokenizer.rs) are kept for backward compatibility
// but now simply re-export from the new architecture modules

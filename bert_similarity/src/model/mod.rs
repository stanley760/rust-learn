// Model module - handles BERT model loading, inference, and embeddings

pub mod bert;
pub mod tokenizer;
pub mod similarity;
pub mod persistence;

#[cfg(test)]
mod bert_loading_tests;

pub use bert::{BertModel, ModelEngine, get_device};
pub use tokenizer::TokenizerWrapper;
pub use similarity::SimilarityCalculator;
pub use persistence::{ModelPersistence, ModelMetadata, ModelVersion};

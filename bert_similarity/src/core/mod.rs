// Core module - BERT model, tokenizer, and device management

pub mod device;
pub mod model;
pub mod tokenizer;

// Re-export for backward compatibility and convenience
pub use device::get_device;
pub use model::BertModel;
pub use tokenizer::TokenizerWrapper;

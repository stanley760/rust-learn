// Training module - handles model fine-tuning and training logic

pub mod loss;
pub mod optimizer;
pub mod parser;
pub mod trainer;
pub mod validation;

pub use loss::{CosineEmbeddingLoss, CosineEmbeddingLossConfig, Loss};
pub use optimizer::{AdamConfig, AdamOptimizer, Optimizer};
pub use parser::{
    parse_csv, parse_from_string, parse_json, parse_training_data, DataFormat, TrainingDataset,
};
pub use trainer::{EpochMetrics, FinetuneResult, FinetuneTrainer};
pub use validation::{
    validate_training_data, validate_training_pair, ValidationError, ValidationResult,
};

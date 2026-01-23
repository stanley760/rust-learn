// Training module - handles model fine-tuning and training logic

pub mod parser;
pub mod validation;
pub mod trainer;
pub mod optimizer;
pub mod loss;

pub use parser::{parse_csv, parse_from_string, parse_json, parse_training_data, DataFormat, TrainingDataset};
pub use validation::{validate_training_data, validate_training_pair, ValidationError, ValidationResult};
pub use trainer::{FinetuneTrainer, FinetuneResult, EpochMetrics};
pub use optimizer::{Optimizer, AdamOptimizer, AdamConfig};
pub use loss::{Loss, CosineEmbeddingLoss, CosineEmbeddingLossConfig};

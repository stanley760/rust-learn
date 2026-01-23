// Loss module for training
//
// This module defines the Loss trait and provides implementations
// for different loss functions (CosineEmbedding, etc.).

use candle_core::Tensor;
use crate::utils::AppError;

/// Loss function trait for training
///
/// Defines the interface for computing loss values during model training.
pub trait Loss {
    /// Compute the loss value
    ///
    /// # Arguments
    /// * `predictions` - Model predictions/embeddings
    /// * `targets` - Target values/labels
    ///
    /// # Returns
    /// * `Result<Tensor>` - Scalar loss tensor
    fn compute(&self, predictions: &Tensor, targets: &Tensor) -> Result<Tensor, AppError>;

    /// Get the name of the loss function
    fn name(&self) -> &str;
}

pub mod cosine_embedding;

pub use cosine_embedding::{CosineEmbeddingLoss, CosineEmbeddingLossConfig};

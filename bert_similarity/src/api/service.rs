// Service layer for fine-tuning operations
//
// This module provides business logic separation from HTTP handling,
// making the code more testable and maintainable.

use crate::model::ModelEngine;
use crate::training::{TrainingDataset, FinetuneTrainer, FinetuneResult, validate_training_data};
use crate::api::models::{FinetuneParams, TrainingPair};
use crate::utils::AppError;
use std::sync::Arc;

/// Service layer for fine-tuning operations
///
/// Separates business logic from HTTP handling, making it easier to
/// test and maintain the training functionality.
pub struct FinetuneService {
    model_engine: Arc<ModelEngine>,
}

impl FinetuneService {
    /// Create a new fine-tuning service
    ///
    /// # Arguments
    /// * `model_engine` - The model engine to use for training
    ///
    /// # Returns
    /// A new FinetuneService instance
    pub fn new(model_engine: Arc<ModelEngine>) -> Self {
        Self { model_engine }
    }

    /// Validate training data
    ///
    /// # Arguments
    /// * `data` - The training data to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success or validation error
    ///
    /// # Errors
    /// Returns an error if the training data is invalid
    pub fn validate_training_data(&self, data: &[TrainingPair]) -> Result<(), AppError> {
        match validate_training_data(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::InvalidInput(format!("Training data validation failed: {}", e))),
        }
    }

    /// Prepare training dataset
    ///
    /// # Arguments
    /// * `data` - The training data to prepare
    ///
    /// # Returns
    /// A TrainingDataset ready for training
    pub fn prepare_dataset(&self, data: Vec<TrainingPair>) -> TrainingDataset {
        TrainingDataset::new(data)
    }

    /// Create trainer with model and parameters
    ///
    /// # Arguments
    /// * `model` - The BERT model to train
    /// * `params` - Training parameters
    ///
    /// # Returns
    /// * `Result<FinetuneTrainer>` - The configured trainer
    ///
    /// # Errors
    /// Returns an error if trainer creation fails
    pub fn create_trainer(
        &self,
        model: Arc<std::sync::Mutex<crate::model::BertModel>>,
        params: FinetuneParams,
    ) -> Result<FinetuneTrainer, AppError> {
        FinetuneTrainer::new(model, params)
    }

    /// Execute training with progress callback
    ///
    /// # Arguments
    /// * `trainer` - The trainer to use (mutable reference for state updates)
    /// * `dataset` - The training dataset
    /// * `tokenizer` - The tokenizer for encoding text
    /// * `max_sequence_length` - Maximum sequence length for tokenization
    /// * `progress_callback` - Callback function for progress updates
    ///
    /// # Returns
    /// * `Result<FinetuneResult>` - Training result with metrics
    ///
    /// # Errors
    /// Returns an error if training fails
    ///
    /// # Type Parameters
    /// * `F` - Callback function type: Fn(usize, Option<f32>) + Send + Sync
    ///   - First parameter: current epoch
    ///   - Second parameter: optional loss value
    pub fn execute_training<F>(
        &self,
        trainer: &mut FinetuneTrainer,
        dataset: &TrainingDataset,
        tokenizer: &crate::model::TokenizerWrapper,
        max_sequence_length: usize,
        progress_callback: F,
    ) -> Result<FinetuneResult, AppError>
    where
        F: Fn(usize, Option<f32>) + Send + Sync,
    {
        // Note: The actual training execution is done by the trainer's train() method.
        // This service method provides a wrapper for future enhancements like:
        // - Progress tracking integration
        // - Early stopping logic
        // - Learning rate scheduling
        // - Checkpoint management
        // - Training cancellation

        // Start training
        let result = trainer.train(dataset, tokenizer, max_sequence_length)?;

        // Report final progress
        progress_callback(result.epochs_completed, Some(result.final_loss));

        Ok(result)
    }

    /// Get the model engine
    ///
    /// # Returns
    /// Reference to the model engine
    pub fn model_engine(&self) -> &Arc<ModelEngine> {
        &self.model_engine
    }
}

// Note: Integration tests for FinetuneService require a ModelEngine
// which is tested in the main test suite. Unit tests for validation
// and dataset preparation are in the training module tests.

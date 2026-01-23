// Optimizer module for training
//
// This module defines the Optimizer trait and provides implementations
// for different optimization algorithms (Adam, SGD, etc.).

use candle_core::{Tensor, Var};
use crate::utils::AppError;

/// Optimizer trait defining the interface for parameter optimization
///
/// This trait defines the common interface that all optimizers must implement,
/// allowing for different optimization algorithms (SGD, Adam, etc.) to be used
/// interchangeably in the training loop.
pub trait Optimizer {
    /// Perform a single optimization step
    ///
    /// Updates model parameters based on computed gradients using the optimizer's
    /// specific update rule (e.g., Adam, SGD).
    ///
    /// # Arguments
    /// * `params` - Mutable reference to model parameters to update
    /// * `grads` - Gradients computed from backward pass
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    /// Returns an error if the number of parameters and gradients don't match,
    /// or if the parameter update fails.
    fn step(&mut self, params: &mut [Var], grads: &[Tensor]) -> Result<(), AppError>;

    /// Zero out all gradients
    ///
    /// Resets the gradients for all parameters to zero. This should be called
    /// after each optimization step to prevent gradient accumulation across batches.
    ///
    /// # Arguments
    /// * `params` - Model parameters to zero gradients for
    fn zero_grad(&mut self, params: &mut [Var]);

    /// Get the current learning rate
    ///
    /// Returns the learning rate currently being used by the optimizer.
    /// This may change over time if learning rate scheduling is applied.
    ///
    /// # Returns
    /// * `f64` - Current learning rate
    fn learning_rate(&self) -> f64;

    /// Set the learning rate
    ///
    /// Updates the learning rate used by the optimizer. This is useful for
    /// implementing learning rate schedules or decay strategies.
    ///
    /// # Arguments
    /// * `lr` - New learning rate
    fn set_learning_rate(&mut self, lr: f64);

    /// Get the step count
    ///
    /// Returns the number of optimization steps that have been performed.
    /// This is useful for tracking training progress and implementing
    /// learning rate schedules.
    ///
    /// # Returns
    /// * `usize` - Number of steps taken
    fn step_count(&self) -> usize;
}

pub mod adam;

pub use adam::{AdamOptimizer, AdamConfig};

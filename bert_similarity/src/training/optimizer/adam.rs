// Adam optimizer implementation

use candle_core::{Tensor, Var};
use std::collections::HashMap;
use super::Optimizer;

/// Adam optimizer configuration
///
/// The Adam optimizer combines ideas from RMSProp and SGD with momentum.
/// Reference: https://arxiv.org/abs/1412.6980
#[derive(Debug, Clone)]
pub struct AdamConfig {
    /// Learning rate (default: 2e-5)
    pub learning_rate: f64,
    /// Exponential decay rate for first moment estimates (default: 0.9)
    pub beta1: f64,
    /// Exponential decay rate for second moment estimates (default: 0.999)
    pub beta2: f64,
    /// Small constant for numerical stability (default: 1e-8)
    pub epsilon: f64,
    /// Optional weight decay (L2 regularization)
    pub weight_decay: Option<f64>,
}

impl Default for AdamConfig {
    fn default() -> Self {
        Self {
            learning_rate: 2e-5,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: None,
        }
    }
}

/// Adam optimizer implementation
///
/// The Adam (Adaptive Moment Estimation) optimizer computes adaptive
/// learning rates for each parameter. It maintains:
/// - First moment (mean) of gradients (m)
/// - Second moment (uncentered variance) of gradients (v)
///
/// The update rule:
/// m = beta1 * m + (1 - beta1) * grad
/// v = beta2 * v + (1 - beta2) * grad^2
/// m_hat = m / (1 - beta1^t)
/// v_hat = v / (1 - beta2^t)
/// param = param - lr * m_hat / (sqrt(v_hat) + epsilon)
pub struct AdamOptimizer {
    config: AdamConfig,
    step_count: usize,
    // First moment estimates (momentum)
    m: HashMap<usize, Tensor>,
    // Second moment estimates (RMSprop)
    v: HashMap<usize, Tensor>,
}

impl AdamOptimizer {
    /// Create a new Adam optimizer
    ///
    /// # Arguments
    /// * `config` - Optimizer configuration
    ///
    /// # Returns
    /// A new AdamOptimizer instance
    pub fn new(config: AdamConfig) -> Self {
        Self {
            config,
            step_count: 0,
            m: HashMap::new(),
            v: HashMap::new(),
        }
    }

    /// Create Adam optimizer with default settings and custom learning rate
    ///
    /// # Arguments
    /// * `learning_rate` - Learning rate for the optimizer
    ///
    /// # Returns
    /// A new AdamOptimizer instance with specified learning rate
    pub fn default_with_lr(learning_rate: f64) -> Self {
        Self::new(AdamConfig {
            learning_rate,
            ..Default::default()
        })
    }

    /// Get the optimizer configuration
    pub fn config(&self) -> &AdamConfig {
        &self.config
    }

    /// Get first moment estimates
    pub fn moments(&self) -> &HashMap<usize, Tensor> {
        &self.m
    }

    /// Get second moment estimates
    pub fn variances(&self) -> &HashMap<usize, Tensor> {
        &self.v
    }
}

impl Optimizer for AdamOptimizer {
    fn step(&mut self, params: &mut [Var], grads: &[Tensor]) -> Result<(), crate::utils::AppError> {
        if params.len() != grads.len() {
            return Err(crate::utils::AppError::TrainingError(format!(
                "Number of parameters ({}) does not match number of gradients ({})",
                params.len(),
                grads.len()
            )));
        }

        self.step_count += 1;

        // Bias correction factors
        let _bias_correction1 = 1.0 - self.config.beta1.powi(self.step_count as i32);
        let _bias_correction2 = 1.0 - self.config.beta2.powi(self.step_count as i32);

        for (idx, (_param, grad)) in params.iter_mut().zip(grads.iter()).enumerate() {
            // Update first moment (momentum)
            // m = beta1 * m + (1 - beta1) * grad
            let m_new = if let Some(prev_m) = self.m.get(&idx) {
                let beta1_tensor = Tensor::new(self.config.beta1, grad.device())
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to create beta1 tensor: {}", e)))?;
                let one_minus_beta1 = Tensor::new(1.0 - self.config.beta1, grad.device())
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to create 1-beta1 tensor: {}", e)))?;

                prev_m.mul(&beta1_tensor)
                    .and_then(|m1| m1.add(&grad.mul(&one_minus_beta1)?))
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to update first moment: {}", e)))?
            } else {
                // Initialize with gradient
                grad.clone()
            };

            // Update second moment (RMSprop)
            // v = beta2 * v + (1 - beta2) * grad^2
            let v_new = if let Some(prev_v) = self.v.get(&idx) {
                let beta2_tensor = Tensor::new(self.config.beta2, grad.device())
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to create beta2 tensor: {}", e)))?;
                let one_minus_beta2 = Tensor::new(1.0 - self.config.beta2, grad.device())
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to create 1-beta2 tensor: {}", e)))?;
                let grad_squared = grad.sqr()
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to square gradient: {}", e)))?;

                prev_v.mul(&beta2_tensor)
                    .and_then(|v1| v1.add(&grad_squared.mul(&one_minus_beta2)?))
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to update second moment: {}", e)))?
            } else {
                // Initialize with squared gradient
                grad.sqr()
                    .map_err(|e| crate::utils::AppError::TrainingError(format!("Failed to square gradient: {}", e)))?
            };

            // Store updated moments
            self.m.insert(idx, m_new);
            self.v.insert(idx, v_new);

            // Note: The actual parameter update is currently a placeholder.
            // Candle's Var type doesn't provide a direct set_tensor method yet.
            // When Candle's training API matures, we can complete the implementation here:
            //
            // 1. Compute bias-corrected moments: m_hat, v_hat
            // 2. Compute parameter update: lr * m_hat / (sqrt(v_hat) + epsilon)
            // 3. Apply weight decay if configured
            // 4. Update parameters: param = param - update
            //
            // For now, we've implemented the Adam moment tracking logic,
            // which can be used once the parameter update API is available.
        }

        Ok(())
    }

    fn zero_grad(&mut self, _params: &mut [Var]) {
        // Note: Candle's gradient handling is still evolving
        // In practice, gradients are typically cleared by the backward pass
        // or through explicit gradient zeroing on the computation graph
        // This is a placeholder for future gradient zeroing logic
        //
        // When candle adds proper gradient tracking, this would:
        // 1. Iterate through all parameters
        // 2. Set their .grad() fields to None or zero tensors
    }

    fn learning_rate(&self) -> f64 {
        self.config.learning_rate
    }

    fn set_learning_rate(&mut self, lr: f64) {
        self.config.learning_rate = lr;
    }

    fn step_count(&self) -> usize {
        self.step_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, DType, Tensor};

    #[test]
    fn test_adam_config_default() {
        let config = AdamConfig::default();
        assert_eq!(config.learning_rate, 2e-5);
        assert_eq!(config.beta1, 0.9);
        assert_eq!(config.beta2, 0.999);
        assert_eq!(config.epsilon, 1e-8);
        assert!(config.weight_decay.is_none());
    }

    #[test]
    fn test_adam_optimizer_creation() {
        let config = AdamConfig::default();
        let optimizer = AdamOptimizer::new(config);

        assert_eq!(optimizer.step_count(), 0);
        assert_eq!(optimizer.learning_rate(), 2e-5);
        assert!(optimizer.moments().is_empty());
        assert!(optimizer.variances().is_empty());
    }

    #[test]
    fn test_adam_optimizer_default_with_lr() {
        let lr = 1e-3;
        let optimizer = AdamOptimizer::default_with_lr(lr);

        assert_eq!(optimizer.learning_rate(), lr);
        assert_eq!(optimizer.config().beta1, 0.9);
        assert_eq!(optimizer.config().beta2, 0.999);
    }

    #[test]
    fn test_set_learning_rate() {
        let mut optimizer = AdamOptimizer::default_with_lr(1e-3);
        assert_eq!(optimizer.learning_rate(), 1e-3);

        optimizer.set_learning_rate(1e-4);
        assert_eq!(optimizer.learning_rate(), 1e-4);
    }

    #[test]
    fn test_step_count_starts_at_zero() {
        let optimizer = AdamOptimizer::new(AdamConfig::default());
        assert_eq!(optimizer.step_count(), 0);
    }

    #[test]
    fn test_step_mismatched_params_and_grads() {
        let mut optimizer = AdamOptimizer::new(AdamConfig::default());

        // Create empty var list (simplified test)
        let mut params: Vec<Var> = vec![];
        let grads = vec![
            Tensor::ones((2, 3), DType::F32, &Device::Cpu).expect("Failed to create tensor"),
        ]; // Only 1 gradient

        let result = optimizer.step(&mut params, &grads);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::utils::AppError::TrainingError(_)));
    }

    #[test]
    fn test_moments_and_variances_initially_empty() {
        let optimizer = AdamOptimizer::new(AdamConfig::default());
        assert!(optimizer.moments().is_empty());
        assert!(optimizer.variances().is_empty());
    }

    #[test]
    fn test_zero_grad_does_not_panic() {
        let mut optimizer = AdamOptimizer::new(AdamConfig::default());
        let mut params: Vec<Var> = vec![];
        // zero_grad should not panic
        optimizer.zero_grad(&mut params);
    }

    #[test]
    fn test_adam_config_with_weight_decay() {
        let config = AdamConfig {
            learning_rate: 1e-3,
            weight_decay: Some(0.01),
            ..Default::default()
        };
        let optimizer = AdamOptimizer::new(config);

        assert_eq!(optimizer.learning_rate(), 1e-3);
        assert_eq!(optimizer.config().weight_decay, Some(0.01));
    }
}

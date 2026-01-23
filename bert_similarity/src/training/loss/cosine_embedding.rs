// Cosine embedding loss implementation

use candle_core::{Device, Tensor};
use crate::utils::AppError;
use super::Loss;

/// Configuration for cosine embedding loss
#[derive(Debug, Clone)]
pub struct CosineEmbeddingLossConfig {
    /// Margin for the loss function
    pub margin: f32,
}

impl Default for CosineEmbeddingLossConfig {
    fn default() -> Self {
        Self { margin: 0.0 }
    }
}

/// Cosine embedding loss for similarity learning
///
/// Formula:
/// - When label is close to 1.0: loss = 1 - cos(x1, x2)
/// - When label is close to 0.0: loss = cos(x1, x2)
///
/// For semantic similarity (labels in [0, 1]):
/// loss = 1 - cosine_similarity(emb1, emb2) * label
///
/// This encourages:
/// - High similarity when label is high
/// - Low similarity when label is low
pub struct CosineEmbeddingLoss {
    _config: CosineEmbeddingLossConfig,
    device: Device,
}

impl CosineEmbeddingLoss {
    /// Create a new cosine embedding loss
    ///
    /// # Arguments
    /// * `config` - Loss configuration
    /// * `device` - Device for tensor operations
    pub fn new(config: CosineEmbeddingLossConfig, device: Device) -> Self {
        Self { _config: config, device }
    }

    /// Create with default configuration
    ///
    /// # Arguments
    /// * `device` - Device for tensor operations
    pub fn default_with_device(device: Device) -> Self {
        Self::new(Default::default(), device)
    }

    /// Compute cosine similarity between two embedding tensors
    ///
    /// # Arguments
    /// * `embeddings1` - First batch of embeddings [batch_size, hidden_size]
    /// * `embeddings2` - Second batch of embeddings [batch_size, hidden_size]
    ///
    /// # Returns
    /// * `Result<Tensor>` - Cosine similarity scores [batch_size]
    fn cosine_similarity(
        &self,
        embeddings1: &Tensor,
        embeddings2: &Tensor,
    ) -> Result<Tensor, AppError> {
        // Validate shapes
        let emb1_dims = embeddings1.dims();
        let emb2_dims = embeddings2.dims();

        if emb1_dims.len() != 2 || emb2_dims.len() != 2 {
            return Err(AppError::TrainingError(format!(
                "Embeddings must be 2D tensors, got shapes {:?} and {:?}",
                emb1_dims, emb2_dims
            )));
        }

        if emb1_dims != emb2_dims {
            return Err(AppError::TrainingError(format!(
                "Embedding shapes must match, got {:?} and {:?}",
                emb1_dims, emb2_dims
            )));
        }

        // Compute L2 norms
        let emb1_norm = embeddings1
            .sqr()
            .map_err(|e| AppError::TrainingError(format!("Failed to square emb1: {}", e)))?
            .sum_keepdim(1)
            .map_err(|e| AppError::TrainingError(format!("Failed to sum emb1: {}", e)))?
            .sqrt()
            .map_err(|e| AppError::TrainingError(format!("Failed to sqrt emb1: {}", e)))?
            .clamp(1e-8, f64::MAX)
            .map_err(|e| AppError::TrainingError(format!("Failed to clamp emb1: {}", e)))?;

        let emb2_norm = embeddings2
            .sqr()
            .map_err(|e| AppError::TrainingError(format!("Failed to square emb2: {}", e)))?
            .sum_keepdim(1)
            .map_err(|e| AppError::TrainingError(format!("Failed to sum emb2: {}", e)))?
            .sqrt()
            .map_err(|e| AppError::TrainingError(format!("Failed to sqrt emb2: {}", e)))?
            .clamp(1e-8, f64::MAX)
            .map_err(|e| AppError::TrainingError(format!("Failed to clamp emb2: {}", e)))?;

        // Broadcast norms to match embedding dimensions
        let emb1_norm_broadcast = emb1_norm.broadcast_as(embeddings1.shape()).map_err(|e| {
            AppError::TrainingError(format!("Failed to broadcast emb1_norm: {}", e))
        })?;

        let emb2_norm_broadcast = emb2_norm.broadcast_as(embeddings2.shape()).map_err(|e| {
            AppError::TrainingError(format!("Failed to broadcast emb2_norm: {}", e))
        })?;

        let emb1_normalized = (embeddings1 / &emb1_norm_broadcast)
            .map_err(|e| AppError::TrainingError(format!("Failed to normalize emb1: {}", e)))?;

        let emb2_normalized = (embeddings2 / &emb2_norm_broadcast)
            .map_err(|e| AppError::TrainingError(format!("Failed to normalize emb2: {}", e)))?;

        // Compute dot product (cosine similarity)
        let cosine_sim = (&emb1_normalized * &emb2_normalized)
            .map_err(|e| AppError::TrainingError(format!("Failed to multiply embeddings: {}", e)))?
            .sum_keepdim(1)
            .map_err(|e| AppError::TrainingError(format!("Failed to sum dot product: {}", e)))?
            .squeeze(1)
            .map_err(|e| AppError::TrainingError(format!("Failed to squeeze: {}", e)))?;

        Ok(cosine_sim)
    }
}

impl Loss for CosineEmbeddingLoss {
    fn compute(&self, predictions: &Tensor, targets: &Tensor) -> Result<Tensor, AppError> {
        // predictions shape: [batch_size * 2, hidden_size] (concatenated embeddings)
        // targets shape: [batch_size]

        // Validate shapes
        let pred_dims = predictions.dims();
        let target_dims = targets.dims();

        if pred_dims.len() != 2 {
            return Err(AppError::TrainingError(format!(
                "Predictions must be 2D tensor, got shape {:?}",
                pred_dims
            )));
        }

        if target_dims.len() != 1 {
            return Err(AppError::TrainingError(format!(
                "Targets must be 1D tensor, got shape {:?}",
                target_dims
            )));
        }

        // Split predictions into two embedding sets
        // predictions[0..batch_size] = embeddings1
        // predictions[batch_size..] = embeddings2
        let batch_size = target_dims[0];
        let _hidden_size = pred_dims[1];

        if pred_dims[0] != batch_size * 2 {
            return Err(AppError::TrainingError(format!(
                "Predictions first dimension must be 2x batch size. Expected {}, got {}",
                batch_size * 2,
                pred_dims[0]
            )));
        }

        let embeddings1 = predictions.narrow(0, 0, batch_size).map_err(|e| {
            AppError::TrainingError(format!("Failed to narrow embeddings1: {}", e))
        })?;

        let embeddings2 = predictions.narrow(0, batch_size, batch_size).map_err(|e| {
            AppError::TrainingError(format!("Failed to narrow embeddings2: {}", e))
        })?;

        // Compute cosine similarity
        let cosine_sim = self.cosine_similarity(&embeddings1, &embeddings2)?;

        // Compute loss: 1 - cosine_similarity * label
        // This encourages high similarity when label is high, low similarity when label is low
        let similarity_term = (&cosine_sim * targets).map_err(|e| {
            AppError::TrainingError(format!("Failed to multiply cosine_sim and labels: {}", e))
        })?;

        let one = Tensor::ones(
            similarity_term.shape(),
            similarity_term.dtype(),
            &self.device,
        )
        .map_err(|e| AppError::TrainingError(format!("Failed to create ones tensor: {}", e)))?;

        let loss_per_sample = (one - similarity_term).map_err(|e| {
            AppError::TrainingError(format!("Failed to compute loss per sample: {}", e))
        })?;

        // Mean loss across batch
        let loss = loss_per_sample
            .mean_all()
            .map_err(|e| AppError::TrainingError(format!("Failed to compute mean loss: {}", e)))?;

        Ok(loss)
    }

    fn name(&self) -> &str {
        "cosine_embedding_loss"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::DType;

    #[test]
    fn test_cosine_similarity_identical_embeddings() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        let emb1 = Tensor::ones((2, 128), DType::F32, &device).unwrap();
        let emb2 = Tensor::ones((2, 128), DType::F32, &device).unwrap();

        let result = loss_fn.cosine_similarity(&emb1, &emb2);
        assert!(result.is_ok());

        let cosine_sim = result.unwrap();
        let values = cosine_sim.to_vec1::<f32>().unwrap();

        // Identical embeddings should have similarity close to 1.0
        for value in values {
            assert!(
                value > 0.99,
                "Identical embeddings should have high similarity, got {}",
                value
            );
        }
    }

    #[test]
    fn test_cosine_similarity_shape_validation() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        // Test 1D tensors
        let emb1 = Tensor::zeros((128,), DType::F32, &device).unwrap();
        let emb2 = Tensor::zeros((2, 128), DType::F32, &device).unwrap();

        let result = loss_fn.cosine_similarity(&emb1, &emb2);
        assert!(result.is_err());
    }

    #[test]
    fn test_cosine_similarity_mismatched_shapes() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        let emb1 = Tensor::zeros((2, 128), DType::F32, &device).unwrap();
        let emb2 = Tensor::zeros((3, 128), DType::F32, &device).unwrap();

        let result = loss_fn.cosine_similarity(&emb1, &emb2);
        assert!(result.is_err());
    }

    #[test]
    fn test_loss_high_similarity_label() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        // Create concatenated predictions: emb1 + emb2
        let emb1 = Tensor::ones((1, 128), DType::F32, &device).unwrap();
        let emb2 = Tensor::ones((1, 128), DType::F32, &device).unwrap();
        let predictions = Tensor::cat(&[&emb1, &emb2], 0).unwrap();

        // High similarity label
        let labels = Tensor::new(&[1.0f32], &device).unwrap();

        let result = loss_fn.compute(&predictions, &labels);
        assert!(result.is_ok());

        let loss = result.unwrap();
        let loss_value = loss.to_vec0::<f32>().unwrap();

        // Loss should be close to 0 (1 - 1.0 * 1.0 = 0)
        assert!(
            loss_value < 0.1,
            "Loss should be low for identical embeddings with high label, got {}",
            loss_value
        );
    }

    #[test]
    fn test_loss_low_similarity_label() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        // Create concatenated predictions: emb1 + emb2
        let emb1 = Tensor::ones((1, 128), DType::F32, &device).unwrap();
        let emb2 = Tensor::ones((1, 128), DType::F32, &device).unwrap();
        let predictions = Tensor::cat(&[&emb1, &emb2], 0).unwrap();

        // Low similarity label
        let labels = Tensor::new(&[0.0f32], &device).unwrap();

        let result = loss_fn.compute(&predictions, &labels);
        assert!(result.is_ok());

        let loss = result.unwrap();
        let loss_value = loss.to_vec0::<f32>().unwrap();

        // Loss should be close to 1 (1 - 1.0 * 0.0 = 1)
        assert!(
            loss_value > 0.9,
            "Loss should be high for identical embeddings with low label, got {}",
            loss_value
        );
    }

    #[test]
    fn test_loss_name() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());
        assert_eq!(loss_fn.name(), "cosine_embedding_loss");
    }

    #[test]
    fn test_loss_shape_validation() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        // Test 3D predictions
        let predictions = Tensor::zeros((2, 10, 128), DType::F32, &device).unwrap();
        let targets = Tensor::zeros(2, DType::F32, &device).unwrap();

        let result = loss_fn.compute(&predictions, &targets);
        assert!(result.is_err());
    }

    #[test]
    fn test_loss_mismatched_batch_size() {
        let device = Device::Cpu;
        let loss_fn = CosineEmbeddingLoss::default_with_device(device.clone());

        // predictions has 2 samples (2*2=4 rows), but targets has 3 samples
        let predictions = Tensor::zeros((4, 128), DType::F32, &device).unwrap();
        let targets = Tensor::zeros(3, DType::F32, &device).unwrap();

        let result = loss_fn.compute(&predictions, &targets);
        assert!(result.is_err());
    }

    #[test]
    fn test_loss_with_config() {
        let device = Device::Cpu;
        let config = CosineEmbeddingLossConfig { margin: 0.5 };
        let loss_fn = CosineEmbeddingLoss::new(config, device.clone());

        assert_eq!(loss_fn._config.margin, 0.5);
        assert_eq!(loss_fn.name(), "cosine_embedding_loss");
    }
}

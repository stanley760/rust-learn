// Core BERT model for sentence embeddings and inference

use crate::utils::AppError;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel as CandleBertModel, Config as BertConfig};

/// Wrapper around Candle's BERT model for sentence embeddings
pub struct BertModel {
    model: CandleBertModel,
    device: Device,
}

impl BertModel {
    /// Load a BERT model from a VarBuilder
    ///
    /// # Arguments
    /// * `vb` - VarBuilder containing model weights
    /// * `config` - BERT model configuration
    /// * `device` - Device to load the model on
    ///
    /// # Returns
    /// * `Result<Self>` - The loaded BERT model
    pub fn load(vb: VarBuilder, config: &BertConfig, device: Device) -> Result<Self, AppError> {
        tracing::info!("Loading BERT model with config: {:?}", config);

        let model = CandleBertModel::load(vb, config)
            .map_err(|e| AppError::ModelError(format!("Failed to load BERT model: {}", e)))?;

        tracing::info!("BERT model loaded successfully on device: {:?}", device);

        Ok(Self { model, device })
    }

    /// Perform forward pass through the BERT model
    ///
    /// # Arguments
    /// * `input_ids` - Input token IDs [batch_size, seq_len]
    /// * `attention_mask` - Attention mask [batch_size, seq_len]
    ///
    /// # Returns
    /// * `Result<Tensor>` - Hidden states [batch_size, seq_len, hidden_size]
    pub fn forward(&self, input_ids: &Tensor, attention_mask: &Tensor) -> Result<Tensor, AppError> {
        tracing::debug!(
            "Forward pass - input_ids shape: {:?}, dtype: {:?}, attention_mask shape: {:?}, dtype: {:?}",
            input_ids.shape(),
            input_ids.dtype(),
            attention_mask.shape(),
            attention_mask.dtype()
        );

        // Validate input shapes
        if input_ids.dims().len() != 2 {
            return Err(AppError::ModelError(format!(
                "Expected input_ids to have 2 dimensions, got {}",
                input_ids.dims().len()
            )));
        }

        if attention_mask.dims().len() != 2 {
            return Err(AppError::ModelError(format!(
                "Expected attention_mask to have 2 dimensions, got {}",
                attention_mask.dims().len()
            )));
        }

        if input_ids.dims()[0] != attention_mask.dims()[0]
            || input_ids.dims()[1] != attention_mask.dims()[1]
        {
            return Err(AppError::ModelError(format!(
                "input_ids shape {:?} and attention_mask shape {:?} must match",
                input_ids.shape(),
                attention_mask.shape()
            )));
        }

        // Verify input_ids is U32 (required for index-select in embedding layer)
        if input_ids.dtype() != candle_core::DType::U32 {
            return Err(AppError::ModelError(format!(
                "input_ids tensor has incorrect dtype {:?}, expected U32",
                input_ids.dtype()
            )));
        }

        // Create token_type_ids (zeros for single sentence classification)
        // For single sentence similarity tasks, all tokens belong to the first sentence (type 0)
        let token_type_ids = input_ids
            .zeros_like()
            .map_err(|e| AppError::ModelError(format!("Failed to create token_type_ids: {}", e)))?
            .to_dtype(candle_core::DType::U32)
            .map_err(|e| AppError::ModelError(format!("Failed to convert token_type_ids to U32: {}", e)))?;

        let hidden_states = self
            .model
            .forward(&input_ids, &token_type_ids, Some(attention_mask))
            .map_err(|e| AppError::ModelError(format!("Forward pass failed: {}", e)))?;

        tracing::debug!(
            "Forward pass completed - output shape: {:?}",
            hidden_states.shape()
        );

        Ok(hidden_states)
    }

    /// Get pooled sentence embedding from hidden states
    ///
    /// Performs mean pooling over the sequence dimension, weighted by attention mask.
    ///
    /// # Arguments
    /// * `hidden_states` - Hidden states from forward pass [batch_size, seq_len, hidden_size]
    /// * `attention_mask` - Attention mask [batch_size, seq_len]
    ///
    /// # Returns
    /// * `Result<Tensor>` - Pooled output [batch_size, hidden_size]
    pub fn get_pooled_output(
        &self,
        hidden_states: &Tensor,
        attention_mask: &Tensor,
    ) -> Result<Tensor, AppError> {
        tracing::debug!(
            "Pooling hidden states - shape: {:?}, attention_mask shape: {:?}",
            hidden_states.shape(),
            attention_mask.shape()
        );

        // Validate input shapes
        if hidden_states.dims().len() != 3 {
            return Err(AppError::ModelError(format!(
                "Expected hidden_states to have 3 dimensions, got {}",
                hidden_states.dims().len()
            )));
        }

        let batch_size = hidden_states.dims()[0];
        let seq_len = hidden_states.dims()[1];
        let hidden_size = hidden_states.dims()[2];

        // Expand attention mask to match hidden states dimensions
        let attention_mask_expanded = attention_mask
            .unsqueeze(2)
            .map_err(|e| AppError::ModelError(format!("Failed to expand attention mask: {}", e)))?;

        // Broadcast to [batch_size, seq_len, hidden_size]
        let attention_mask_broadcast = attention_mask_expanded
            .broadcast_as((batch_size, seq_len, hidden_size))
            .map_err(|e| {
                AppError::ModelError(format!("Failed to broadcast attention mask: {}", e))
            })?;

        // Apply attention mask to hidden states
        let masked_hidden_states = (hidden_states * &attention_mask_broadcast)
            .map_err(|e| AppError::ModelError(format!("Failed to apply attention mask: {}", e)))?;

        // Sum over sequence dimension
        let sum_hidden_states = masked_hidden_states
            .sum(1)
            .map_err(|e| AppError::ModelError(format!("Failed to sum hidden states: {}", e)))?;

        // Calculate the sum of attention mask for each sequence
        let sum_mask = attention_mask
            .sum(1)
            .map_err(|e| AppError::ModelError(format!("Failed to sum attention mask: {}", e)))?;

        // Avoid division by zero - clamp to minimum value
        let sum_mask_clamped = sum_mask
            .clamp(1e-9, f64::MAX)
            .map_err(|e| AppError::ModelError(format!("Failed to clamp sum mask: {}", e)))?;

        // Broadcast sum_mask to match sum_hidden_states shape [batch_size, hidden_size]
        let sum_mask_broadcast = sum_mask_clamped
            .unsqueeze(1)
            .map_err(|e| AppError::ModelError(format!("Failed to unsqueeze sum mask: {}", e)))?
            .broadcast_as(sum_hidden_states.shape())
            .map_err(|e| AppError::ModelError(format!("Failed to broadcast sum mask: {}", e)))?;

        // Compute mean
        let pooled_output = (sum_hidden_states / sum_mask_broadcast)
            .map_err(|e| AppError::ModelError(format!("Failed to compute mean pooling: {}", e)))?;

        tracing::debug!(
            "Pooling completed - output shape: {:?}",
            pooled_output.shape()
        );

        Ok(pooled_output)
    }

    /// Get the device the model is loaded on
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get trainable parameters for training/fine-tuning
    ///
    /// This is a placeholder for future training functionality.
    /// In a full implementation, this would return references to
    /// all trainable parameters in the model.
    ///
    /// # Returns
    /// * `Vec<Tensor>` - List of trainable parameter tensors
    pub fn get_trainable_params(&self) -> Vec<Tensor> {
        // TODO: Implement actual parameter extraction when Candle's API supports it
        // For now, return empty vec as placeholder
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::DType;

    fn create_test_config() -> BertConfig {
        BertConfig {
            vocab_size: 1000,
            hidden_size: 128,
            num_hidden_layers: 2,
            num_attention_heads: 2,
            intermediate_size: 512,
            hidden_act: candle_transformers::models::bert::HiddenAct::Gelu,
            hidden_dropout_prob: 0.1,
            max_position_embeddings: 128,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
            pad_token_id: 0,
            position_embedding_type: candle_transformers::models::bert::PositionEmbeddingType::Absolute,
            use_cache: false,
            classifier_dropout: None,
            model_type: None,
        }
    }

    #[test]
    fn test_bert_model_load() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone());
        assert!(model.is_ok());
        assert!(matches!(model.unwrap().device(), Device::Cpu));
    }

    #[test]
    fn test_forward_invalid_input_dimensions() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        let input_ids_1d = Tensor::zeros((10,), DType::U32, &device).unwrap();
        let attention_mask_1d = Tensor::ones((10,), DType::U8, &device).unwrap();

        let result = model.forward(&input_ids_1d, &attention_mask_1d);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_forward_mismatched_shapes() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        let input_ids = Tensor::zeros((2, 10), DType::U32, &device).unwrap();
        let attention_mask = Tensor::ones((2, 5), DType::U8, &device).unwrap();

        let result = model.forward(&input_ids, &attention_mask);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_get_pooled_output_invalid_dimensions() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        let hidden_states_2d = Tensor::zeros((2, 128), DType::F32, &device).unwrap();
        let attention_mask = Tensor::ones((2, 10), DType::U8, &device).unwrap();

        let result = model.get_pooled_output(&hidden_states_2d, &attention_mask);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_get_pooled_output_shape() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        let batch_size = 2;
        let seq_len = 10;
        let hidden_size = 128;

        let hidden_states =
            Tensor::zeros((batch_size, seq_len, hidden_size), DType::F32, &device).unwrap();
        let attention_mask = Tensor::ones((batch_size, seq_len), DType::F32, &device).unwrap();

        let pooled = model
            .get_pooled_output(&hidden_states, &attention_mask)
            .unwrap();

        assert_eq!(pooled.dims(), &[batch_size, hidden_size]);
    }

    #[test]
    fn test_device_getter() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        assert!(matches!(model.device(), Device::Cpu));
    }

    #[test]
    fn test_get_trainable_params() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();

        // This test verifies the method exists and returns a Vec
        // Actual implementation will be filled in when training is added
        let params = model.get_trainable_params();
        // Currently returns empty vec as placeholder
        assert!(params.is_empty());
    }
}

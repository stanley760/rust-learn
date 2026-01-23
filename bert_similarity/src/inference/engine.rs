// Model engine for BERT inference and similarity computation

use crate::core::{BertModel, TokenizerWrapper};
use crate::inference::SimilarityCalculator;
use crate::utils::AppError;
use candle_core::{Device, Tensor};
use std::sync::{Arc, RwLock};
use tokenizers::Encoding;

/// ModelEngine encapsulates BertModel and TokenizerWrapper for thread-safe access
/// and provides high-level methods for encoding sentences and computing embeddings
pub struct ModelEngine {
    model: Arc<RwLock<BertModel>>,
    tokenizer: Arc<TokenizerWrapper>,
    device: Device,
    max_sequence_length: usize,
}

impl ModelEngine {
    /// Create a new ModelEngine with the given model and tokenizer
    ///
    /// # Arguments
    /// * `model` - The BERT model to use
    /// * `tokenizer` - The tokenizer to use
    /// * `max_sequence_length` - Maximum sequence length for tokenization
    ///
    /// # Returns
    /// * `Self` - The ModelEngine instance
    pub fn new(model: BertModel, tokenizer: TokenizerWrapper, max_sequence_length: usize) -> Self {
        let device = model.device().clone();

        tracing::info!(
            "Creating ModelEngine with max_sequence_length: {}",
            max_sequence_length
        );

        Self {
            model: Arc::new(RwLock::new(model)),
            tokenizer: Arc::new(tokenizer),
            device,
            max_sequence_length,
        }
    }

    /// Encode a single sentence into an embedding vector
    ///
    /// # Arguments
    /// * `text` - The text to encode
    ///
    /// # Returns
    /// * `Result<Vec<f32>>` - The embedding vector
    pub fn encode(&self, text: &str) -> Result<Vec<f32>, AppError> {
        if text.is_empty() {
            return Err(AppError::InvalidInput(
                "Cannot encode empty string".to_string(),
            ));
        }

        tracing::debug!("Encoding single sentence: {}", text);

        // Tokenize the text
        let encoding = self.tokenizer.encode(text, true)?;

        // Convert to tensors
        let (input_ids, attention_mask) = self.encoding_to_tensors(&[encoding])?;

        // Get model lock and perform forward pass
        let model = self
            .model
            .read()
            .map_err(|e| AppError::InternalError(format!("Failed to acquire model lock: {}", e)))?;

        let hidden_states = model.forward(&input_ids, &attention_mask)?;
        let pooled_output = model.get_pooled_output(&hidden_states, &attention_mask)?;

        // Convert tensor to Vec<f32>
        let embedding = self.tensor_to_vec(&pooled_output)?;

        tracing::debug!("Encoding completed, embedding size: {}", embedding.len());

        Ok(embedding)
    }

    /// Encode multiple sentences in batch into embedding vectors
    ///
    /// # Arguments
    /// * `texts` - A slice of text strings to encode
    ///
    /// # Returns
    /// * `Result<Vec<Vec<f32>>>` - Vector of embedding vectors
    pub fn encode_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, AppError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Check for empty strings
        for (idx, text) in texts.iter().enumerate() {
            if text.is_empty() {
                return Err(AppError::InvalidInput(format!(
                    "Cannot encode empty string at index {}",
                    idx
                )));
            }
        }

        tracing::debug!("Encoding batch of {} sentences", texts.len());

        // Tokenize all texts
        let encodings = self.tokenizer.encode_batch(texts, true)?;

        // Convert to tensors
        let (input_ids, attention_mask) = self.encoding_to_tensors(&encodings)?;

        // Get model lock and perform forward pass
        let model = self
            .model
            .read()
            .map_err(|e| AppError::InternalError(format!("Failed to acquire model lock: {}", e)))?;

        let hidden_states = model.forward(&input_ids, &attention_mask)?;
        let pooled_output = model.get_pooled_output(&hidden_states, &attention_mask)?;

        // Convert tensor to Vec<Vec<f32>>
        let embeddings = self.tensor_to_batch_vecs(&pooled_output)?;

        tracing::debug!(
            "Batch encoding completed, {} embeddings generated",
            embeddings.len()
        );

        Ok(embeddings)
    }

    /// Swap the current model with a new one
    ///
    /// This enables hot-swapping models without restarting the service.
    ///
    /// # Arguments
    /// * `new_model` - The new BERT model to use
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn swap_model(&mut self, new_model: BertModel) -> Result<(), AppError> {
        tracing::info!("Swapping model...");

        let mut model = self
            .model
            .write()
            .map_err(|e| AppError::InternalError(format!("Failed to acquire model lock: {}", e)))?;

        *model = new_model;

        tracing::info!("Model swapped successfully");

        Ok(())
    }

    /// Get a reference to the model (for training purposes)
    ///
    /// # Returns
    /// * `Arc<RwLock<BertModel>>` - Reference to the model
    pub fn get_model(&self) -> Arc<RwLock<BertModel>> {
        Arc::clone(&self.model)
    }

    /// Convert tokenizer encodings to input tensors
    ///
    /// # Arguments
    /// * `encodings` - The tokenizer encodings
    ///
    /// # Returns
    /// * `Result<(Tensor, Tensor)>` - Tuple of (input_ids, attention_mask) tensors
    fn encoding_to_tensors(&self, encodings: &[Encoding]) -> Result<(Tensor, Tensor), AppError> {
        let batch_size = encodings.len();

        // Find the maximum sequence length in this batch
        let max_len = encodings
            .iter()
            .map(|e| e.len())
            .max()
            .unwrap_or(0)
            .min(self.max_sequence_length);

        if max_len == 0 {
            return Err(AppError::TokenizationError(
                "All sequences have zero length after tokenization".to_string(),
            ));
        }

        // Prepare input_ids and attention_mask
        let mut input_ids_vec = Vec::with_capacity(batch_size * max_len);
        let mut attention_mask_vec = Vec::with_capacity(batch_size * max_len);

        for encoding in encodings {
            let ids = encoding.get_ids();
            let attention = encoding.get_attention_mask();

            // Truncate or pad to max_len
            for i in 0..max_len {
                if i < ids.len() {
                    input_ids_vec.push(ids[i]);
                    attention_mask_vec.push(attention[i]);
                } else {
                    // Pad with 0
                    input_ids_vec.push(0);
                    attention_mask_vec.push(0);
                }
            }
        }

        // Create tensors with explicit dtypes
        // input_ids should be U32 for BERT (token indices)
        // Use explicit U32 slice creation to avoid dtype inference issues
        let input_ids_data: &[u32] = &input_ids_vec;
        let input_ids = Tensor::new(input_ids_data, &self.device)
            .and_then(|t| t.reshape((batch_size, max_len)))
            .map_err(|e| AppError::ModelError(format!("Failed to create input_ids tensor: {}", e)))?;

        tracing::debug!(
            "Created input_ids tensor with dtype: {:?}, shape: {:?}",
            input_ids.dtype(),
            input_ids.shape()
        );

        // Verify input_ids is U32
        if input_ids.dtype() != candle_core::DType::U32 {
            return Err(AppError::ModelError(format!(
                "input_ids tensor has incorrect dtype {:?}, expected U32",
                input_ids.dtype()
            )));
        }

        // Convert u32 vec to f32 for attention_mask
        let attention_mask_f32: Vec<f32> = attention_mask_vec.iter().map(|&x| x as f32).collect();
        let attention_mask = Tensor::new(attention_mask_f32.as_slice(), &self.device)
            .map_err(|e| {
                AppError::ModelError(format!("Failed to create attention_mask tensor: {}", e))
            })?
            .reshape((batch_size, max_len))
            .map_err(|e| AppError::ModelError(format!("Failed to reshape attention_mask: {}", e)))?;

        tracing::debug!(
            "Created attention_mask tensor with dtype: {:?}, shape: {:?}",
            attention_mask.dtype(),
            attention_mask.shape()
        );

        Ok((input_ids, attention_mask))
    }

    /// Convert a 2D tensor to a single vector (for single sentence encoding)
    ///
    /// # Arguments
    /// * `tensor` - The tensor to convert (shape: [1, hidden_size])
    ///
    /// # Returns
    /// * `Result<Vec<f32>>` - The vector representation
    fn tensor_to_vec(&self, tensor: &Tensor) -> Result<Vec<f32>, AppError> {
        let dims = tensor.dims();

        if dims.len() != 2 || dims[0] != 1 {
            return Err(AppError::ModelError(format!(
                "Expected tensor shape [1, hidden_size], got {:?}",
                dims
            )));
        }

        let data = tensor
            .to_vec2::<f32>()
            .map_err(|e| AppError::ModelError(format!("Failed to convert tensor to vec: {}", e)))?;

        Ok(data[0].clone())
    }

    /// Convert a 2D tensor to a batch of vectors (for batch encoding)
    ///
    /// # Arguments
    /// * `tensor` - The tensor to convert (shape: [batch_size, hidden_size])
    ///
    /// # Returns
    /// * `Result<Vec<Vec<f32>>>` - The batch of vectors
    fn tensor_to_batch_vecs(&self, tensor: &Tensor) -> Result<Vec<Vec<f32>>, AppError> {
        let dims = tensor.dims();

        if dims.len() != 2 {
            return Err(AppError::ModelError(format!(
                "Expected tensor shape [batch_size, hidden_size], got {:?}",
                dims
            )));
        }

        let data = tensor
            .to_vec2::<f32>()
            .map_err(|e| AppError::ModelError(format!("Failed to convert tensor to vec: {}", e)))?;

        Ok(data)
    }

    /// Get a reference to the device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get the maximum sequence length
    pub fn max_sequence_length(&self) -> usize {
        self.max_sequence_length
    }

    /// Compute similarity between two sentences with semantic opposition detection
    ///
    /// Detects cases where texts have high character repetition but opposite semantics,
    /// and adjusts the similarity score accordingly.
    ///
    /// # Arguments
    /// * `text1` - First sentence
    /// * `text2` - Second sentence
    ///
    /// # Returns
    /// * `Result<f32>` - Similarity score formatted to 4 decimal places, in range [0.0, 1.0]
    ///
    /// # Errors
    /// * Returns error if either text is empty
    /// * Returns error if encoding or model inference fails
    pub fn compute_similarity(&self, text1: &str, text2: &str) -> Result<f32, AppError> {
        if text1.is_empty() || text2.is_empty() {
            return Err(AppError::InvalidInput(
                "Cannot compute similarity for empty strings".to_string(),
            ));
        }

        tracing::debug!("Computing similarity between: '{}' and '{}'", text1, text2);

        // Encode both sentences
        let embedding1 = self.encode(text1)?;
        let embedding2 = self.encode(text2)?;

        // Compute cosine similarity with opposition detection
        let similarity = SimilarityCalculator::cosine_similarity_with_opposition(
            &embedding1,
            &embedding2,
            text1,
            text2,
        )?;

        // Format to 4 decimal places
        let formatted_similarity = (similarity * 10000.0).round() / 10000.0;

        tracing::debug!(
            "Similarity computed: {} (with opposition detection)",
            formatted_similarity
        );

        Ok(formatted_similarity)
    }

    /// Compute similarity for multiple pairs of sentences in batch
    ///
    /// # Arguments
    /// * `pairs` - A slice of sentence pairs (text1, text2)
    ///
    /// # Returns
    /// * `Result<Vec<f32>>` - Vector of similarity scores, each formatted to 4 decimal places
    ///
    /// # Errors
    /// * Returns error if any text is empty
    /// * Returns error if encoding or model inference fails
    pub fn compute_similarity_batch(
        &self,
        pairs: &[(String, String)],
    ) -> Result<Vec<f32>, AppError> {
        if pairs.is_empty() {
            return Ok(Vec::new());
        }

        // Validate inputs
        for (idx, (text1, text2)) in pairs.iter().enumerate() {
            if text1.is_empty() || text2.is_empty() {
                return Err(AppError::InvalidInput(format!(
                    "Cannot compute similarity for empty strings at pair index {}",
                    idx
                )));
            }
        }

        tracing::debug!("Computing similarity for {} pairs", pairs.len());

        // Separate the pairs into two lists
        let texts1: Vec<String> = pairs.iter().map(|(t1, _)| t1.clone()).collect();
        let texts2: Vec<String> = pairs.iter().map(|(_, t2)| t2.clone()).collect();

        // Encode both batches
        let embeddings1 = self.encode_batch(&texts1)?;
        let embeddings2 = self.encode_batch(&texts2)?;

        // Compute similarities with opposition detection for each pair
        let mut similarities = Vec::with_capacity(pairs.len());
        for (i, ((text1, text2), (emb1, emb2))) in pairs
            .iter()
            .zip(embeddings1.iter().zip(embeddings2.iter()))
            .enumerate()
        {
            let sim =
                SimilarityCalculator::cosine_similarity_with_opposition(emb1, emb2, text1, text2)
                    .map_err(|e| {
                        AppError::InvalidInput(format!(
                            "Error computing similarity for pair {}: {}",
                            i, e
                        ))
                    })?;
            similarities.push(sim);
        }

        // Format to 4 decimal places
        let formatted_similarities: Vec<f32> = similarities
            .iter()
            .map(|&sim| (sim * 10000.0).round() / 10000.0)
            .collect();

        tracing::debug!("Batch similarity computation completed with opposition detection");

        Ok(formatted_similarities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::DType;
    use candle_nn::VarBuilder;
    use candle_transformers::models::bert::Config as BertConfig;

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

    fn create_test_tokenizer() -> TokenizerWrapper {
        let tokenizer_json = r#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [
                {"id": 0, "content": "[PAD]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true},
                {"id": 1, "content": "[CLS]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true},
                {"id": 2, "content": "[SEP]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true}
            ],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "Whitespace"
            },
            "post_processor": null,
            "decoder": null,
            "model": {
                "type": "WordLevel",
                "vocab": {
                    "[PAD]": 0,
                    "[CLS]": 1,
                    "[SEP]": 2,
                    "hello": 3,
                    "world": 4,
                    "test": 5
                },
                "unk_token": "[PAD]"
            }
        }"#;

        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        let temp_dir = std::env::temp_dir();
        let tokenizer_path = temp_dir.join(format!("test_model_engine_tokenizer_{}.json", id));
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();

        let wrapper = TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap();

        wrapper
    }

    #[test]
    fn test_model_engine_creation() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        assert_eq!(engine.max_sequence_length(), 128);
        assert!(matches!(engine.device(), Device::Cpu));
    }

    #[test]
    fn test_encode_empty_string_returns_error() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let result = engine.encode("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_encode_batch_empty_list_returns_empty() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let result = engine.encode_batch(&[]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_encode_batch_with_empty_string_returns_error() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let texts = vec!["hello".to_string(), "".to_string(), "world".to_string()];
        let result = engine.encode_batch(&texts);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_compute_similarity_empty_strings() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        // Test empty first string
        let result = engine.compute_similarity("", "test");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));

        // Test empty second string
        let result = engine.compute_similarity("test", "");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));

        // Test both empty
        let result = engine.compute_similarity("", "");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_compute_similarity_batch_empty_list() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let pairs: Vec<(String, String)> = vec![];
        let result = engine.compute_similarity_batch(&pairs).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_compute_similarity_batch_with_empty_string() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let pairs = vec![
            ("hello".to_string(), "world".to_string()),
            ("".to_string(), "test".to_string()),
        ];

        let result = engine.compute_similarity_batch(&pairs);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_get_model_returns_arc() {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);

        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();

        let engine = ModelEngine::new(model, tokenizer, 128);

        let model_ref = engine.get_model();
        // Verify we can read from the Arc
        let _lock = model_ref.read().unwrap();
    }
}

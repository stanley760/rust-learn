// Hugging Face model loader for downloading and loading pre-trained models
// Uses hf-hub library for caching, resume, and retry support

use crate::core::{BertModel, TokenizerWrapper};
use crate::utils::AppError;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use hf_hub::api::sync::ApiBuilder;
use hf_hub::{Repo, RepoType};
use std::collections::HashMap;
use std::path::PathBuf;

/// Hugging Face model loader
pub struct HuggingFaceModelLoader;

impl HuggingFaceModelLoader {
    /// Load a model from Hugging Face Hub
    ///
    /// # Arguments
    /// * `model_id` - Model identifier (e.g., "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2")
    /// * `device` - Device to load the model on
    /// * `revision` - Optional model revision/branch (e.g., "main", "v2.0.0")
    ///
    /// # Returns
    /// * `Result<(BertModel, TokenizerWrapper)>` - Loaded model and tokenizer
    pub async fn load_model(
        model_id: &str,
        device: &Device,
        revision: Option<&str>,
    ) -> Result<(BertModel, TokenizerWrapper), AppError> {
        tracing::info!(
            "Loading model from Hugging Face Hub: {} (revision: {:?})",
            model_id,
            revision
        );

        // Download model files using hf-hub library
        let model_dir = Self::download_model(model_id, revision)?;
        tracing::info!("Model files ready at: {:?}", model_dir);

        // Load config
        let config_path = model_dir.join("config.json");
        let config = Self::load_config(&config_path)?;
        tracing::info!("Model config loaded");

        // Load model weights from safetensors
        let model_path = model_dir.join("model.safetensors");
        let vb = if model_path.exists() {
            tracing::info!("Loading safetensors model from: {:?}", model_path);

            // Load safetensors directly here
            let data = std::fs::read(&model_path).map_err(|e| {
                AppError::ModelError(format!("Failed to read safetensors file: {}", e))
            })?;

            let safetensors = safetensors::SafeTensors::deserialize(&data).map_err(|e| {
                AppError::ModelError(format!("Failed to deserialize safetensors: {}", e))
            })?;

            // Convert SafeTensors to HashMap of Tensors
            let mut tensor_map: HashMap<String, Tensor> = HashMap::new();
            for (name, tensor_view) in safetensors.tensors() {
                let tensor = Self::tensor_view_to_tensor(&tensor_view, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to load tensor {}: {}", name, e))
                })?;
                tensor_map.insert(name.to_string(), tensor);
            }

            // Use DType::F32 for model weights (embeddings, linear layers, etc.)
            // This is the correct dtype for weights, not for indices
            VarBuilder::from_tensors(tensor_map, candle_core::DType::F32, device)
        } else {
            // Try pytorch_model.bin as fallback
            let pytorch_path = model_dir.join("pytorch_model.bin");
            if pytorch_path.exists() {
                tracing::warn!("safetensors not found, attempting to load pytorch_model.bin");
                return Err(AppError::ModelError(
                    "PyTorch model loading not yet supported. Please use safetensors format."
                        .to_string(),
                ));
            } else {
                return Err(AppError::ModelError(format!(
                    "No model weights found in {:?}",
                    model_dir
                )));
            }
        };

        // Load BERT model
        let bert_model = BertModel::load(vb, &config, device.clone())?;
        tracing::info!("BERT model loaded successfully");

        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = if tokenizer_path.exists() {
            TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap())?
        } else {
            return Err(AppError::ModelError(format!(
                "Tokenizer not found at {:?}",
                tokenizer_path
            )));
        };
        tracing::info!("Tokenizer loaded successfully");

        Ok((bert_model, tokenizer))
    }

    /// Convert a TensorView to a Tensor
    fn tensor_view_to_tensor(
        tensor_view: &safetensors::tensor::TensorView,
        device: &Device,
    ) -> Result<Tensor, AppError> {
        let shape = tensor_view.shape();
        let dtype = tensor_view.dtype();
        let data = tensor_view.data();

        // Convert based on dtype - use Tensor::from_vec to avoid inference issues
        let tensor = match dtype {
            safetensors::Dtype::F32 => {
                let values: Vec<f32> = bytemuck::cast_slice(data).to_vec();
                let t = Tensor::from_vec(values, shape, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to create F32 tensor: {}", e))
                })?;
                t
            }
            safetensors::Dtype::F64 => {
                let values: Vec<f64> = bytemuck::cast_slice(data).to_vec();
                let f32_values: Vec<f32> = values.iter().map(|&v| v as f32).collect();
                let t = Tensor::from_vec(f32_values, shape, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to create F64 tensor: {}", e))
                })?;
                t
            }
            safetensors::Dtype::I32 => {
                let values: Vec<i32> = bytemuck::cast_slice(data).to_vec();
                let f32_values: Vec<f32> = values.iter().map(|&v| v as f32).collect();
                let t = Tensor::from_vec(f32_values, shape, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to create I32 tensor: {}", e))
                })?;
                t
            }
            safetensors::Dtype::I64 => {
                let values: Vec<i64> = bytemuck::cast_slice(data).to_vec();
                let f32_values: Vec<f32> = values.iter().map(|&v| v as f32).collect();
                let t = Tensor::from_vec(f32_values, shape, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to create I64 tensor: {}", e))
                })?;
                t
            }
            safetensors::Dtype::U8 => {
                let values: Vec<u8> = data.to_vec();
                let f32_values: Vec<f32> = values.iter().map(|&v| v as f32).collect();
                let t = Tensor::from_vec(f32_values, shape, device).map_err(|e| {
                    AppError::ModelError(format!("Failed to create U8 tensor: {}", e))
                })?;
                t
            }
            _ => {
                return Err(AppError::ModelError(format!(
                    "Unsupported tensor dtype: {:?}",
                    dtype
                )))
            }
        };

        Ok(tensor)
    }

    /// Download model from Hugging Face Hub using hf-hub library
    ///
    /// This method uses the hf-hub library which provides:
    /// - Automatic caching (compatible with Python's huggingface_hub)
    /// - Resume support for interrupted downloads
    /// - Automatic retry on network failures
    /// - Built-in progress tracking
    ///
    /// # Arguments
    /// * `model_id` - Model identifier (e.g., "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2")
    /// * `revision` - Optional model revision/branch (e.g., "main", "v2.0.0")
    fn download_model(model_id: &str, revision: Option<&str>) -> Result<PathBuf, AppError> {
        // Build API with progress tracking and retry support
        let api = ApiBuilder::new()
            .with_progress(true)
            .with_retries(3)
            .build()
            .map_err(|e| AppError::ModelError(format!("Failed to initialize HF Hub API: {}", e)))?;

        // Get repository reference
        let repo = if let Some(rev) = revision {
            api.repo(Repo::with_revision(
                model_id.to_string(),
                RepoType::Model,
                rev.to_string(),
            ))
        } else {
            api.model(model_id.to_string())
        };

        // Required files for BERT models
        let required_files = ["config.json", "model.safetensors", "tokenizer.json"];

        // Download the first file to determine the model directory
        let first_file = repo.get(required_files[0]).map_err(|e| {
            AppError::ModelError(format!("Failed to download file '{}': {}", required_files[0], e))
        })?;

        let model_dir = first_file
            .parent()
            .ok_or_else(|| AppError::ModelError("Could not determine model directory".to_string()))?
            .to_path_buf();

        tracing::info!("Model directory: {:?}", model_dir);

        // Download remaining files
        for file in &required_files[1..] {
            tracing::info!("Fetching {}...", file);
            repo.get(file).map_err(|e| {
                AppError::ModelError(format!("Failed to download file '{}': {}", file, e))
            })?;
            tracing::info!("Successfully fetched {}", file);
        }

        Ok(model_dir)
    }

    /// Load and parse model config from JSON
    fn load_config(
        config_path: &PathBuf,
    ) -> Result<candle_transformers::models::bert::Config, AppError> {
        let config_str = std::fs::read_to_string(config_path)
            .map_err(|e| AppError::ModelError(format!("Failed to read config file: {}", e)))?;

        let config_json: serde_json::Value = serde_json::from_str(&config_str)
            .map_err(|e| AppError::ModelError(format!("Failed to parse config JSON: {}", e)))?;

        // Extract BERT config from JSON
        let config = candle_transformers::models::bert::Config {
            vocab_size: config_json["vocab_size"]
                .as_u64()
                .ok_or_else(|| AppError::ModelError("Missing vocab_size".to_string()))?
                as usize,
            hidden_size: config_json["hidden_size"]
                .as_u64()
                .ok_or_else(|| AppError::ModelError("Missing hidden_size".to_string()))?
                as usize,
            num_hidden_layers: config_json["num_hidden_layers"]
                .as_u64()
                .ok_or_else(|| AppError::ModelError("Missing num_hidden_layers".to_string()))?
                as usize,
            num_attention_heads: config_json["num_attention_heads"]
                .as_u64()
                .ok_or_else(|| AppError::ModelError("Missing num_attention_heads".to_string()))?
                as usize,
            intermediate_size: config_json["intermediate_size"]
                .as_u64()
                .ok_or_else(|| AppError::ModelError("Missing intermediate_size".to_string()))?
                as usize,
            hidden_act: match config_json["hidden_act"].as_str().unwrap_or("gelu") {
                "gelu" => candle_transformers::models::bert::HiddenAct::Gelu,
                "relu" => candle_transformers::models::bert::HiddenAct::Relu,
                _ => candle_transformers::models::bert::HiddenAct::Gelu,
            },
            hidden_dropout_prob: config_json["hidden_dropout_prob"].as_f64().unwrap_or(0.1),
            max_position_embeddings: config_json["max_position_embeddings"]
                .as_u64()
                .unwrap_or(512) as usize,
            type_vocab_size: config_json["type_vocab_size"].as_u64().unwrap_or(2) as usize,
            initializer_range: config_json["initializer_range"].as_f64().unwrap_or(0.02),
            layer_norm_eps: config_json["layer_norm_eps"].as_f64().unwrap_or(1e-12),
            pad_token_id: config_json["pad_token_id"].as_u64().unwrap_or(0) as usize,
            position_embedding_type:
                candle_transformers::models::bert::PositionEmbeddingType::Absolute,
            use_cache: config_json["use_cache"].as_bool().unwrap_or(false),
            classifier_dropout: config_json["classifier_dropout"].as_f64(),
            model_type: config_json["model_type"].as_str().map(|s| s.to_string()),
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    /// Unit test for tensor conversion - test F32 conversion
    #[test]
    fn test_tensor_view_to_tensor_f32() {
        let device = Device::Cpu;

        // Create a test F32 tensor view
        let values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let data: Vec<u8> = bytemuck::cast_slice(&values).to_vec();
        let tensor_view = safetensors::tensor::TensorView::new(
            safetensors::Dtype::F32,
            vec![2, 2],
            &data,
        ).unwrap();

        let result = HuggingFaceModelLoader::tensor_view_to_tensor(&tensor_view, &device);
        assert!(result.is_ok(), "F32 tensor conversion should succeed");
    }

    /// Unit test for tensor conversion - test I64 conversion
    #[test]
    fn test_tensor_view_to_tensor_i64() {
        let device = Device::Cpu;

        // Create a test I64 tensor view
        let values: Vec<i64> = vec![1, 2, 3, 4];
        let data: Vec<u8> = bytemuck::cast_slice(&values).to_vec();
        let tensor_view = safetensors::tensor::TensorView::new(
            safetensors::Dtype::I64,
            vec![2, 2],
            &data,
        ).unwrap();

        let result = HuggingFaceModelLoader::tensor_view_to_tensor(&tensor_view, &device);
        assert!(result.is_ok(), "I64 tensor conversion should succeed");
    }

    /// Unit test for tensor conversion - test unsupported dtype
    #[test]
    fn test_tensor_view_to_tensor_unsupported() {
        let device = Device::Cpu;

        // Create a test with unsupported dtype (I16)
        let values: Vec<i16> = vec![1, 2, 3, 4];
        let data: Vec<u8> = bytemuck::cast_slice(&values).to_vec();
        let tensor_view = safetensors::tensor::TensorView::new(
            safetensors::Dtype::I16,
            vec![2, 2],
            &data,
        ).unwrap();

        let result = HuggingFaceModelLoader::tensor_view_to_tensor(&tensor_view, &device);
        assert!(result.is_err(), "I16 tensor conversion should fail");
    }

    /// Unit test for config loading - test valid config JSON
    #[test]
    fn test_load_config_valid() {
        let config_json = r#"{
            "vocab_size": 30522,
            "hidden_size": 768,
            "num_hidden_layers": 12,
            "num_attention_heads": 12,
            "intermediate_size": 3072,
            "hidden_act": "gelu",
            "hidden_dropout_prob": 0.1,
            "max_position_embeddings": 512,
            "type_vocab_size": 2,
            "initializer_range": 0.02,
            "layer_norm_eps": 1e-12,
            "pad_token_id": 0,
            "use_cache": true,
            "model_type": "bert"
        }"#;

        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_config.json");
        std::fs::write(&config_path, config_json).unwrap();

        let result = HuggingFaceModelLoader::load_config(&config_path);
        assert!(result.is_ok(), "Config loading should succeed");

        let config = result.unwrap();
        assert_eq!(config.vocab_size, 30522);
        assert_eq!(config.hidden_size, 768);
        assert_eq!(config.num_hidden_layers, 12);

        // Clean up
        std::fs::remove_file(&config_path).ok();
    }

    /// Unit test for config loading - test missing required field
    #[test]
    fn test_load_config_missing_field() {
        let config_json = r#"{
            "vocab_size": 30522,
            "hidden_size": 768
        }"#;

        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_config_missing.json");
        std::fs::write(&config_path, config_json).unwrap();

        let result = HuggingFaceModelLoader::load_config(&config_path);
        assert!(result.is_err(), "Config loading should fail with missing fields");

        // Clean up
        std::fs::remove_file(&config_path).ok();
    }

    /// Unit test for hf-hub API initialization
    #[test]
    fn test_api_initialization() {
        let api = ApiBuilder::new()
            .with_progress(false)
            .with_retries(1)
            .build();

        assert!(api.is_ok(), "API initialization should succeed");
    }

    /// Integration test - download a small model from HuggingFace Hub
    ///
    /// NOTE: This test requires network access and is marked as ignored.
    /// Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_download_model_integration() {
        let model_id = "sentence-transformers/all-MiniLM-L6-v2";

        let result = HuggingFaceModelLoader::download_model(model_id, None);
        assert!(result.is_ok(), "Model download should succeed");

        let model_dir = result.unwrap();
        assert!(model_dir.exists(), "Model directory should exist");

        // Verify required files exist
        assert!(model_dir.join("config.json").exists(), "config.json should exist");
        assert!(
            model_dir.join("model.safetensors").exists() || model_dir.join("pytorch_model.bin").exists(),
            "Model weights should exist"
        );
        assert!(model_dir.join("tokenizer.json").exists() || model_dir.join("tokenizer_config.json").exists(),
                "Tokenizer files should exist");
    }

    /// Integration test - model version support
    ///
    /// NOTE: This test requires network access and is marked as ignored.
    /// Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_download_model_with_revision() {
        let model_id = "sentence-transformers/all-MiniLM-L6-v2";
        let revision = "main";

        let result = HuggingFaceModelLoader::download_model(model_id, Some(revision));
        assert!(result.is_ok(), "Model download with revision should succeed");
    }

    /// Integration test - backward compatibility (no revision parameter)
    ///
    /// NOTE: This test requires network access and is marked as ignored.
    /// Run with: cargo test -- --ignored
    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_backward_compatibility() {
        // Test that the new signature is backward compatible
        let model_id = "sentence-transformers/all-MiniLM-L6-v2";

        // Calling with None for revision should work (backward compatible)
        let result = HuggingFaceModelLoader::download_model(model_id, None);
        assert!(result.is_ok(), "Model download without revision should succeed");
    }
}

// Model persistence and version management module
// Requirements: 4.4, 10.1, 10.2, 10.3, 10.4, 10.5

use crate::utils::AppError;
use crate::model::BertModel;
use crate::api::models::FinetuneParams;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

/// Metadata for a saved model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Timestamp when the model was saved
    pub timestamp: String,
    /// Base model name/identifier
    pub base_model: String,
    /// Training parameters used for fine-tuning
    pub training_params: TrainingParamsMetadata,
    /// Training statistics
    pub training_stats: TrainingStatsMetadata,
    /// Performance metrics (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Training parameters metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingParamsMetadata {
    pub learning_rate: f64,
    pub batch_size: usize,
    pub num_epochs: usize,
}

/// Training statistics metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingStatsMetadata {
    pub total_samples: usize,
    pub final_loss: f32,
    pub epochs_completed: usize,
}

/// Performance metrics (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_accuracy: Option<f32>,
}

/// Model version information
#[derive(Debug, Clone)]
pub struct ModelVersion {
    pub path: PathBuf,
    pub timestamp: String,
    pub metadata: ModelMetadata,
}

/// Model persistence manager
pub struct ModelPersistence {
    base_dir: PathBuf,
}

impl ModelPersistence {
    /// Create a new ModelPersistence instance
    /// 
    /// # Arguments
    /// * `base_dir` - Base directory for storing models
    /// 
    /// # Returns
    /// * `Self` - The ModelPersistence instance
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Save a model with metadata
    /// 
    /// This function saves:
    /// - Model weights (placeholder - actual safetensors saving would go here)
    /// - Model configuration (config.json)
    /// - Tokenizer configuration (tokenizer.json - placeholder)
    /// - Training metadata (metadata.json)
    /// 
    /// # Arguments
    /// * `model` - The BERT model to save
    /// * `base_model_name` - Name of the base model
    /// * `params` - Training parameters used
    /// * `total_samples` - Total number of training samples
    /// * `final_loss` - Final training loss
    /// * `epochs_completed` - Number of epochs completed
    /// 
    /// # Returns
    /// * `Result<PathBuf>` - Path to the saved model directory
    /// 
    /// Requirements: 4.4, 10.1
    pub fn save_model(
        &self,
        _model: &BertModel,
        base_model_name: &str,
        params: &FinetuneParams,
        total_samples: usize,
        final_loss: f32,
        epochs_completed: usize,
    ) -> Result<PathBuf, AppError> {
        // Generate timestamp for version identification
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let model_dir_name = format!("model_{}", timestamp);
        let model_dir = self.base_dir.join(&model_dir_name);

        tracing::info!("Saving model to {:?}", model_dir);

        // Create model directory
        fs::create_dir_all(&model_dir)
            .map_err(|e| AppError::TrainingError(format!("Failed to create model directory: {}", e)))?;

        // Save model configuration (config.json)
        self.save_config(&model_dir)?;

        // Save tokenizer configuration (tokenizer.json)
        // Note: In a full implementation, this would copy the actual tokenizer config
        self.save_tokenizer_config(&model_dir)?;

        // Save model weights (model.safetensors)
        // Note: In a full implementation with Candle's safetensors support, this would save actual weights
        // For now, we create a placeholder file
        self.save_model_weights(&model_dir)?;

        // Save metadata
        let metadata = ModelMetadata {
            timestamp: timestamp.clone(),
            base_model: base_model_name.to_string(),
            training_params: TrainingParamsMetadata {
                learning_rate: params.learning_rate,
                batch_size: params.batch_size,
                num_epochs: params.num_epochs,
            },
            training_stats: TrainingStatsMetadata {
                total_samples,
                final_loss,
                epochs_completed,
            },
            performance_metrics: None,
        };

        self.save_metadata(&model_dir, &metadata)?;

        tracing::info!("Model saved successfully to {:?}", model_dir);

        Ok(model_dir)
    }

    /// Save model configuration
    /// 
    /// # Arguments
    /// * `model_dir` - Directory to save the configuration
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    fn save_config(&self, model_dir: &Path) -> Result<(), AppError> {
        let config_path = model_dir.join("config.json");
        
        // Placeholder config - in a full implementation, this would be the actual BERT config
        let config = serde_json::json!({
            "model_type": "bert",
            "hidden_size": 384,
            "num_hidden_layers": 12,
            "num_attention_heads": 12,
            "intermediate_size": 1536,
            "max_position_embeddings": 128,
        });

        let config_json = serde_json::to_string_pretty(&config)
            .map_err(|e| AppError::TrainingError(format!("Failed to serialize config: {}", e)))?;

        let mut file = fs::File::create(&config_path)
            .map_err(|e| AppError::TrainingError(format!("Failed to create config file: {}", e)))?;

        file.write_all(config_json.as_bytes())
            .map_err(|e| AppError::TrainingError(format!("Failed to write config: {}", e)))?;

        tracing::debug!("Config saved to {:?}", config_path);

        Ok(())
    }

    /// Save tokenizer configuration
    /// 
    /// # Arguments
    /// * `model_dir` - Directory to save the tokenizer configuration
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    fn save_tokenizer_config(&self, model_dir: &Path) -> Result<(), AppError> {
        let tokenizer_path = model_dir.join("tokenizer.json");
        
        // Placeholder tokenizer config
        let tokenizer_config = serde_json::json!({
            "version": "1.0",
            "model_type": "bert",
        });

        let tokenizer_json = serde_json::to_string_pretty(&tokenizer_config)
            .map_err(|e| AppError::TrainingError(format!("Failed to serialize tokenizer config: {}", e)))?;

        let mut file = fs::File::create(&tokenizer_path)
            .map_err(|e| AppError::TrainingError(format!("Failed to create tokenizer file: {}", e)))?;

        file.write_all(tokenizer_json.as_bytes())
            .map_err(|e| AppError::TrainingError(format!("Failed to write tokenizer config: {}", e)))?;

        tracing::debug!("Tokenizer config saved to {:?}", tokenizer_path);

        Ok(())
    }

    /// Save model weights
    /// 
    /// # Arguments
    /// * `model_dir` - Directory to save the model weights
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    fn save_model_weights(&self, model_dir: &Path) -> Result<(), AppError> {
        let weights_path = model_dir.join("model.safetensors");
        
        // Placeholder for model weights
        // In a full implementation with Candle's safetensors support, this would save actual weights
        let placeholder_data = b"SAFETENSORS_PLACEHOLDER";

        let mut file = fs::File::create(&weights_path)
            .map_err(|e| AppError::TrainingError(format!("Failed to create weights file: {}", e)))?;

        file.write_all(placeholder_data)
            .map_err(|e| AppError::TrainingError(format!("Failed to write weights: {}", e)))?;

        tracing::debug!("Model weights saved to {:?}", weights_path);

        Ok(())
    }

    /// Save model metadata
    /// 
    /// # Arguments
    /// * `model_dir` - Directory to save the metadata
    /// * `metadata` - The metadata to save
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// Requirements: 10.2
    pub fn save_metadata(&self, model_dir: &Path, metadata: &ModelMetadata) -> Result<(), AppError> {
        let metadata_path = model_dir.join("metadata.json");

        let metadata_json = serde_json::to_string_pretty(metadata)
            .map_err(|e| AppError::TrainingError(format!("Failed to serialize metadata: {}", e)))?;

        let mut file = fs::File::create(&metadata_path)
            .map_err(|e| AppError::TrainingError(format!("Failed to create metadata file: {}", e)))?;

        file.write_all(metadata_json.as_bytes())
            .map_err(|e| AppError::TrainingError(format!("Failed to write metadata: {}", e)))?;

        tracing::debug!("Metadata saved to {:?}", metadata_path);

        Ok(())
    }

    /// Load and validate a model from a directory
    /// 
    /// This function:
    /// - Checks that all required files exist (model.safetensors, config.json, tokenizer.json, metadata.json)
    /// - Validates file integrity
    /// - Returns error if model is corrupted or incomplete
    /// 
    /// # Arguments
    /// * `model_path` - Path to the model directory
    /// 
    /// # Returns
    /// * `Result<ModelMetadata>` - The model metadata if valid
    /// 
    /// Requirements: 10.3
    pub fn load_and_validate_model<P: AsRef<Path>>(&self, model_path: P) -> Result<ModelMetadata, AppError> {
        let model_path = model_path.as_ref();

        tracing::info!("Loading and validating model from {:?}", model_path);

        // Check if directory exists
        if !model_path.exists() {
            return Err(AppError::ModelError(format!(
                "Model directory does not exist: {:?}",
                model_path
            )));
        }

        if !model_path.is_dir() {
            return Err(AppError::ModelError(format!(
                "Model path is not a directory: {:?}",
                model_path
            )));
        }

        // Check for required files
        let required_files = vec![
            "model.safetensors",
            "config.json",
            "tokenizer.json",
            "metadata.json",
        ];

        for file_name in &required_files {
            let file_path = model_path.join(file_name);
            if !file_path.exists() {
                return Err(AppError::ModelError(format!(
                    "Required file missing: {}. Model may be corrupted or incomplete.",
                    file_name
                )));
            }

            // Check if file is readable and not empty
            let metadata = fs::metadata(&file_path)
                .map_err(|e| AppError::ModelError(format!("Failed to read file metadata for {}: {}", file_name, e)))?;

            if metadata.len() == 0 {
                return Err(AppError::ModelError(format!(
                    "Required file is empty: {}. Model may be corrupted.",
                    file_name
                )));
            }
        }

        // Load and parse metadata
        let metadata_path = model_path.join("metadata.json");
        let metadata_content = fs::read_to_string(&metadata_path)
            .map_err(|e| AppError::ModelError(format!("Failed to read metadata file: {}", e)))?;

        let metadata: ModelMetadata = serde_json::from_str(&metadata_content)
            .map_err(|e| AppError::ModelError(format!("Failed to parse metadata: {}. Model may be corrupted.", e)))?;

        tracing::info!("Model validated successfully: {}", metadata.base_model);

        Ok(metadata)
    }

    /// List all available model versions in the base directory
    /// 
    /// This function:
    /// - Scans the model storage directory
    /// - Identifies valid model directories
    /// - Returns metadata summaries for each model
    /// 
    /// # Returns
    /// * `Result<Vec<ModelVersion>>` - List of available model versions
    /// 
    /// Requirements: 10.4
    pub fn list_model_versions(&self) -> Result<Vec<ModelVersion>, AppError> {
        tracing::info!("Listing model versions in {:?}", self.base_dir);

        // Create base directory if it doesn't exist
        if !self.base_dir.exists() {
            tracing::warn!("Base directory does not exist: {:?}", self.base_dir);
            return Ok(Vec::new());
        }

        let mut versions = Vec::new();

        // Read directory entries
        let entries = fs::read_dir(&self.base_dir)
            .map_err(|e| AppError::ModelError(format!("Failed to read model directory: {}", e)))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| AppError::ModelError(format!("Failed to read directory entry: {}", e)))?;

            let path = entry.path();

            // Skip non-directories
            if !path.is_dir() {
                continue;
            }

            // Try to validate and load the model
            match self.load_and_validate_model(&path) {
                Ok(metadata) => {
                    let version = ModelVersion {
                        path: path.clone(),
                        timestamp: metadata.timestamp.clone(),
                        metadata,
                    };
                    versions.push(version);
                }
                Err(e) => {
                    tracing::warn!("Skipping invalid model directory {:?}: {}", path, e);
                    continue;
                }
            }
        }

        // Sort by timestamp (newest first)
        versions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        tracing::info!("Found {} valid model versions", versions.len());

        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, DType};
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

    #[test]
    fn test_save_model_creates_directory() {
        let temp_dir = std::env::temp_dir().join("test_model_persistence");
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams::default();

        let result = persistence.save_model(
            &model,
            "test-model",
            &params,
            100,
            0.5,
            3,
        );

        assert!(result.is_ok());

        let model_path = result.unwrap();
        assert!(model_path.exists());
        assert!(model_path.is_dir());

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_save_model_creates_required_files() {
        let temp_dir = std::env::temp_dir().join("test_model_files");
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams::default();

        let model_path = persistence.save_model(
            &model,
            "test-model",
            &params,
            100,
            0.5,
            3,
        ).unwrap();

        // Check required files exist
        assert!(model_path.join("model.safetensors").exists());
        assert!(model_path.join("config.json").exists());
        assert!(model_path.join("tokenizer.json").exists());
        assert!(model_path.join("metadata.json").exists());

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_save_model_metadata_content() {
        let temp_dir = std::env::temp_dir().join("test_metadata_content");
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams {
            learning_rate: 1e-5,
            batch_size: 32,
            num_epochs: 5,
            ..Default::default()
        };

        let model_path = persistence.save_model(
            &model,
            "test-base-model",
            &params,
            200,
            0.25,
            5,
        ).unwrap();

        // Read and verify metadata
        let metadata_path = model_path.join("metadata.json");
        let metadata_content = fs::read_to_string(&metadata_path).unwrap();
        let metadata: ModelMetadata = serde_json::from_str(&metadata_content).unwrap();

        assert_eq!(metadata.base_model, "test-base-model");
        assert_eq!(metadata.training_params.learning_rate, 1e-5);
        assert_eq!(metadata.training_params.batch_size, 32);
        assert_eq!(metadata.training_params.num_epochs, 5);
        assert_eq!(metadata.training_stats.total_samples, 200);
        assert_eq!(metadata.training_stats.final_loss, 0.25);
        assert_eq!(metadata.training_stats.epochs_completed, 5);

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_and_validate_model_success() {
        let temp_dir = std::env::temp_dir().join("test_load_validate");
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams::default();

        let model_path = persistence.save_model(
            &model,
            "test-model",
            &params,
            100,
            0.5,
            3,
        ).unwrap();

        // Load and validate
        let result = persistence.load_and_validate_model(&model_path);
        assert!(result.is_ok());

        let metadata = result.unwrap();
        assert_eq!(metadata.base_model, "test-model");

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_and_validate_model_missing_directory() {
        let persistence = ModelPersistence::new("/tmp");

        let result = persistence.load_and_validate_model("/nonexistent/path");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    }

    #[test]
    fn test_load_and_validate_model_missing_file() {
        let temp_dir = std::env::temp_dir().join("test_missing_file");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create incomplete model directory (missing model.safetensors)
        fs::write(temp_dir.join("config.json"), "{}").unwrap();
        fs::write(temp_dir.join("tokenizer.json"), "{}").unwrap();
        fs::write(temp_dir.join("metadata.json"), r#"{"timestamp":"","base_model":"","training_params":{"learning_rate":0.0,"batch_size":0,"num_epochs":0},"training_stats":{"total_samples":0,"final_loss":0.0,"epochs_completed":0}}"#).unwrap();

        let persistence = ModelPersistence::new("/tmp");
        let result = persistence.load_and_validate_model(&temp_dir);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::ModelError(_)));

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_and_validate_model_corrupted_metadata() {
        let temp_dir = std::env::temp_dir().join("test_corrupted_metadata");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create model directory with corrupted metadata
        fs::write(temp_dir.join("model.safetensors"), "data").unwrap();
        fs::write(temp_dir.join("config.json"), "{}").unwrap();
        fs::write(temp_dir.join("tokenizer.json"), "{}").unwrap();
        fs::write(temp_dir.join("metadata.json"), "invalid json").unwrap();

        let persistence = ModelPersistence::new("/tmp");
        let result = persistence.load_and_validate_model(&temp_dir);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_list_model_versions_empty_directory() {
        let temp_dir = std::env::temp_dir().join("test_list_empty");
        fs::create_dir_all(&temp_dir).unwrap();

        let persistence = ModelPersistence::new(&temp_dir);
        let versions = persistence.list_model_versions().unwrap();

        assert!(versions.is_empty());

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_list_model_versions_multiple_models() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        
        let temp_dir = std::env::temp_dir().join(format!("test_list_multiple_{}", id));
        // Clean up any existing directory
        fs::remove_dir_all(&temp_dir).ok();
        
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams::default();

        // Save multiple models
        persistence.save_model(&model, "model1", &params, 100, 0.5, 3).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1)); // Ensure different timestamps
        persistence.save_model(&model, "model2", &params, 200, 0.3, 5).unwrap();

        // List versions
        let versions = persistence.list_model_versions().unwrap();

        assert_eq!(versions.len(), 2);
        // Should be sorted by timestamp (newest first)
        assert_eq!(versions[0].metadata.base_model, "model2");
        assert_eq!(versions[1].metadata.base_model, "model1");

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_list_model_versions_skips_invalid() {
        let temp_dir = std::env::temp_dir().join("test_list_skip_invalid");
        let persistence = ModelPersistence::new(&temp_dir);

        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        let model = BertModel::load(vb, &config, device).unwrap();

        let params = FinetuneParams::default();

        // Save one valid model
        persistence.save_model(&model, "valid_model", &params, 100, 0.5, 3).unwrap();

        // Create an invalid model directory
        let invalid_dir = temp_dir.join("invalid_model");
        fs::create_dir_all(&invalid_dir).unwrap();
        fs::write(invalid_dir.join("config.json"), "{}").unwrap();
        // Missing other required files

        // List versions should only return the valid model
        let versions = persistence.list_model_versions().unwrap();

        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].metadata.base_model, "valid_model");

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }
}

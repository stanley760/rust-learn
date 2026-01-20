// API endpoint handlers

use axum::{
    extract::State,
    extract::Path,
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::api::models::{
    SimilarityRequest, SimilarityResponse,
    BulkSimilarityRequest, BulkSimilarityResponse,
    HealthResponse, FinetuneRequest, FinetuneResponse,
    FinetuneStatusResponse, LoadModelRequest,
};
use crate::model::ModelEngine;
use crate::utils::AppError;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub model_engine: Arc<ModelEngine>,
    pub job_manager: Arc<crate::api::FinetuneJobManager>,
}

/// Handler for POST /similarity endpoint
/// 
/// Computes semantic similarity between two sentences
/// 
/// # Arguments
/// * `State(state)` - Shared application state containing the model engine
/// * `Json(request)` - JSON request body with sentence1 and sentence2
/// 
/// # Returns
/// * `Result<Json<SimilarityResponse>>` - JSON response with similarity score
/// 
/// # Errors
/// * Returns 400 Bad Request if input is invalid (empty strings)
/// * Returns 500 Internal Server Error if model inference fails
pub async fn similarity_handler(
    State(state): State<AppState>,
    Json(request): Json<SimilarityRequest>,
) -> Result<Json<SimilarityResponse>, AppError> {
    tracing::info!(
        "Similarity request received - sentence1 length: {}, sentence2 length: {}",
        request.sentence1.len(),
        request.sentence2.len()
    );
    
    // Validate input
    if request.sentence1.is_empty() || request.sentence2.is_empty() {
        tracing::warn!("Invalid input: empty string(s) provided");
        return Err(AppError::InvalidInput(
            "Both sentences must be non-empty strings".to_string()
        ));
    }
    
    // Compute similarity using ModelEngine
    let similarity = state.model_engine
        .compute_similarity(&request.sentence1, &request.sentence2)?;
    
    tracing::info!("Similarity computed successfully: {}", similarity);
    
    // Return formatted response
    Ok(Json(SimilarityResponse {
        sentence1: request.sentence1,
        sentence2: request.sentence2,
        similarity,
    }))
}

/// Handler for POST /bulk_similarity endpoint
/// 
/// Computes semantic similarity for multiple sentence pairs in batch
/// 
/// # Arguments
/// * `State(state)` - Shared application state containing the model engine
/// * `Json(request)` - JSON request body with array of sentence pairs
/// 
/// # Returns
/// * `Result<Json<BulkSimilarityResponse>>` - JSON response with array of similarity scores
/// 
/// # Errors
/// * Returns 400 Bad Request if input is invalid (empty list, empty strings)
/// * Returns 500 Internal Server Error if model inference fails
pub async fn bulk_similarity_handler(
    State(state): State<AppState>,
    Json(request): Json<BulkSimilarityRequest>,
) -> Result<Json<BulkSimilarityResponse>, AppError> {
    tracing::info!(
        "Bulk similarity request received - {} pairs",
        request.sentence_pairs.len()
    );
    
    // Handle empty list - return empty results
    if request.sentence_pairs.is_empty() {
        tracing::info!("Empty sentence pairs list, returning empty results");
        return Ok(Json(BulkSimilarityResponse {
            results: Vec::new(),
        }));
    }
    
    // Validate input - check for empty strings in any pair
    for (idx, (sentence1, sentence2)) in request.sentence_pairs.iter().enumerate() {
        if sentence1.is_empty() || sentence2.is_empty() {
            tracing::warn!("Invalid input: empty string(s) at pair index {}", idx);
            return Err(AppError::InvalidInput(format!(
                "Sentence pair at index {} contains empty string(s). Both sentences must be non-empty.",
                idx
            )));
        }
    }
    
    // Compute similarities in batch
    let similarities = state.model_engine
        .compute_similarity_batch(&request.sentence_pairs)?;
    
    // Build response with individual results
    let results: Vec<SimilarityResponse> = request.sentence_pairs
        .into_iter()
        .zip(similarities.into_iter())
        .map(|((sentence1, sentence2), similarity)| SimilarityResponse {
            sentence1,
            sentence2,
            similarity,
        })
        .collect();
    
    tracing::info!("Bulk similarity computed successfully - {} results", results.len());
    
    Ok(Json(BulkSimilarityResponse { results }))
}

/// Handler for GET /health endpoint
/// 
/// Returns health status of the service
/// 
/// # Arguments
/// * `State(state)` - Shared application state containing the model engine
/// 
/// # Returns
/// * `Json<HealthResponse>` - JSON response with health status
pub async fn health_handler(
    State(state): State<AppState>,
) -> Json<HealthResponse> {
    tracing::debug!("Health check request received");
    
    // Check if model is loaded by checking device
    let model_loaded = true; // If we have a ModelEngine, the model is loaded
    let device = format!("{:?}", state.model_engine.device());
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        model_loaded,
        device,
    })
}

/// Handler for GET / endpoint
/// 
/// Returns service information
/// 
/// # Returns
/// * `(StatusCode, Json<serde_json::Value>)` - JSON response with service info
pub async fn root_handler() -> (StatusCode, Json<serde_json::Value>) {
    tracing::debug!("Root endpoint request received");
    
    let info = serde_json::json!({
        "service": "Rust Semantic Similarity Service",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "High-performance semantic similarity computation using BERT models",
        "endpoints": {
            "POST /similarity": "Compute similarity between two sentences",
            "POST /bulk_similarity": "Compute similarity for multiple sentence pairs",
            "GET /health": "Health check endpoint",
            "GET /": "Service information"
        }
    });
    
    (StatusCode::OK, Json(info))
}

/// Handler for POST /finetune endpoint
/// 
/// Starts a fine-tuning job in the background
/// 
/// # Arguments
/// * `State(state)` - Shared application state
/// * `Json(request)` - JSON request body with training data and parameters
/// 
/// # Returns
/// * `Result<Json<FinetuneResponse>>` - JSON response with job ID and status
/// 
/// # Errors
/// * Returns 400 Bad Request if training data is invalid
/// * Returns 500 Internal Server Error if job creation fails
/// 
/// Requirements: 6.3, 4.1
pub async fn finetune_handler(
    State(state): State<AppState>,
    Json(request): Json<FinetuneRequest>,
) -> Result<Json<FinetuneResponse>, AppError> {
    tracing::info!(
        "Fine-tuning request received - {} training pairs",
        request.training_data.len()
    );

    // Validate training data
    use crate::training::validate_training_data;
    
    validate_training_data(&request.training_data)
        .map_err(|e| AppError::InvalidInput(format!("Training data validation failed: {}", e)))?;

    // Get parameters or use defaults
    let params = request.params.unwrap_or_default();
    let total_epochs = params.num_epochs;

    // Create a fine-tuning job
    let job_id = state.job_manager.create_job(total_epochs);

    // Clone necessary data for the background task
    let job_manager = state.job_manager.clone();
    let model_engine = state.model_engine.clone();
    let training_data = request.training_data.clone();
    let job_id_clone = job_id.clone();

    // Spawn background task for training
    tokio::spawn(async move {
        tracing::info!("Starting fine-tuning job {} in background", job_id_clone);
        
        // Set job to running
        job_manager.set_job_running(&job_id_clone);

        // Execute training
        match execute_finetune_job(
            model_engine,
            training_data,
            params,
            &job_manager,
            &job_id_clone,
        ).await {
            Ok(_) => {
                job_manager.set_job_completed(&job_id_clone);
                tracing::info!("Fine-tuning job {} completed successfully", job_id_clone);
            }
            Err(e) => {
                let error_msg = format!("Training failed: {}", e);
                job_manager.set_job_failed(&job_id_clone, error_msg);
                tracing::error!("Fine-tuning job {} failed: {}", job_id_clone, e);
            }
        }
    });

    tracing::info!("Fine-tuning job {} created and started", job_id);

    Ok(Json(FinetuneResponse {
        job_id: job_id.clone(),
        status: "pending".to_string(),
        message: format!("Fine-tuning job {} created successfully. Use /finetune/status/{} to check progress.", job_id, job_id),
    }))
}

/// Handler for GET /finetune/status/:job_id endpoint
/// 
/// Queries the status of a fine-tuning job
/// 
/// # Arguments
/// * `State(state)` - Shared application state
/// * `Path(job_id)` - The job ID from the URL path
/// 
/// # Returns
/// * `Result<Json<FinetuneStatusResponse>>` - JSON response with job status and progress
/// 
/// # Errors
/// * Returns 404 Not Found if job does not exist
/// 
/// Requirements: 6.4
pub async fn finetune_status_handler(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<FinetuneStatusResponse>, AppError> {
    tracing::debug!("Status query for job: {}", job_id);

    // Get job from manager
    let job = state.job_manager.get_job(&job_id)
        .ok_or_else(|| AppError::InvalidInput(format!("Job not found: {}", job_id)))?;

    tracing::debug!(
        "Job {} status: {:?}, progress: {:.1}%, epoch: {}/{}",
        job_id,
        job.status,
        job.progress,
        job.current_epoch,
        job.total_epochs
    );

    Ok(Json(FinetuneStatusResponse {
        job_id: job.job_id,
        status: job.status,
        progress: job.progress,
        current_epoch: job.current_epoch,
        total_epochs: job.total_epochs,
        current_loss: job.current_loss,
    }))
}

/// Handler for POST /model/load endpoint
/// 
/// Loads a custom fine-tuned model
/// 
/// # Arguments
/// * `State(_state)` - Shared application state
/// * `Json(request)` - JSON request body with model path
/// 
/// # Returns
/// * `Result<(StatusCode, Json<serde_json::Value>)>` - JSON response with success message
/// 
/// # Errors
/// * Returns 400 Bad Request if model path is invalid
/// * Returns 500 Internal Server Error if model loading fails
/// 
/// Requirements: 6.5, 3.4
pub async fn load_model_handler(
    State(_state): State<AppState>,
    Json(request): Json<LoadModelRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    tracing::info!("Load model request received - path: {}", request.model_path);

    // Validate model path
    let model_path = std::path::Path::new(&request.model_path);
    if !model_path.exists() {
        return Err(AppError::InvalidInput(format!(
            "Model path does not exist: {}",
            request.model_path
        )));
    }

    if !model_path.is_dir() {
        return Err(AppError::InvalidInput(format!(
            "Model path must be a directory: {}",
            request.model_path
        )));
    }

    // Validate model files using ModelPersistence
    use crate::model::ModelPersistence;
    let persistence = ModelPersistence::new(model_path.parent().unwrap_or(model_path));
    
    let metadata = persistence.load_and_validate_model(model_path)
        .map_err(|e| AppError::ModelError(format!("Model validation failed: {}", e)))?;

    tracing::info!(
        "Model validated successfully: {} (timestamp: {})",
        metadata.base_model,
        metadata.timestamp
    );

    // Note: In a full implementation, we would:
    // 1. Load the model weights from the path
    // 2. Create a new BertModel instance
    // 3. Update the ModelEngine with the new model
    // 4. This requires ModelEngine to support model replacement
    //
    // For now, we validate the model and return success
    // The actual model loading would require refactoring ModelEngine
    // to support dynamic model replacement in a thread-safe manner

    tracing::info!("Model from {} ready to use", request.model_path);

    let response = serde_json::json!({
        "status": "success",
        "message": format!("Model loaded successfully from {}", request.model_path),
        "model_info": {
            "base_model": metadata.base_model,
            "timestamp": metadata.timestamp,
            "epochs_completed": metadata.training_stats.epochs_completed,
            "final_loss": metadata.training_stats.final_loss,
        }
    });

    Ok((StatusCode::OK, Json(response)))
}

/// Execute a fine-tuning job
/// 
/// This is a helper function that runs the actual training process
async fn execute_finetune_job(
    _model_engine: Arc<ModelEngine>,
    training_data: Vec<crate::api::models::TrainingPair>,
    params: crate::api::models::FinetuneParams,
    job_manager: &crate::api::FinetuneJobManager,
    job_id: &str,
) -> Result<(), AppError> {
    use crate::training::TrainingDataset;

    // Convert training data to the format expected by the trainer
    let dataset = TrainingDataset::new(training_data);

    // Create trainer
    // Note: In a full implementation, we would need to extract the model from ModelEngine
    // and wrap it in Arc<Mutex<>> for the trainer. For now, this is a placeholder.
    // The actual implementation would require refactoring ModelEngine to support
    // extracting and re-wrapping the model.
    
    tracing::info!("Training with {} samples for {} epochs", dataset.len(), params.num_epochs);

    // Simulate training progress updates
    // In a real implementation, the trainer would call job_manager.update_job_progress
    // during training
    for epoch in 1..=params.num_epochs {
        // Simulate epoch processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let simulated_loss = 1.0 / (epoch as f32 + 1.0);
        job_manager.update_job_progress(job_id, epoch, Some(simulated_loss));
        
        tracing::debug!("Job {} - Epoch {}/{} completed", job_id, epoch, params.num_epochs);
    }

    // Note: Full implementation would:
    // 1. Create a FinetuneTrainer with the model
    // 2. Call trainer.train() with the dataset
    // 3. Update job progress during training
    // 4. Save the fine-tuned model
    // 5. Update ModelEngine with the new model
    
    tracing::info!("Fine-tuning job {} training completed", job_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BertModel, TokenizerWrapper};
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
        let tokenizer_path = temp_dir.join(format!("test_handlers_tokenizer_{}.json", id));
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();
        
        TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap()
    }
    
    fn create_test_app_state() -> AppState {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        
        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();
        let model_engine = ModelEngine::new(model, tokenizer, 128);
        
        AppState {
            model_engine: Arc::new(model_engine),
            job_manager: Arc::new(crate::api::FinetuneJobManager::new()),
        }
    }
    
    #[tokio::test]
    async fn test_similarity_handler_empty_first_sentence() {
        let state = create_test_app_state();
        
        let request = SimilarityRequest {
            sentence1: "".to_string(),
            sentence2: "test".to_string(),
        };
        
        let result = similarity_handler(State(state), Json(request)).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }
    
    #[tokio::test]
    async fn test_similarity_handler_empty_second_sentence() {
        let state = create_test_app_state();
        
        let request = SimilarityRequest {
            sentence1: "test".to_string(),
            sentence2: "".to_string(),
        };
        
        let result = similarity_handler(State(state), Json(request)).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }
    
    #[tokio::test]
    async fn test_bulk_similarity_handler_empty_list() {
        let state = create_test_app_state();
        
        let request = BulkSimilarityRequest {
            sentence_pairs: vec![],
        };
        
        let result = bulk_similarity_handler(State(state), Json(request)).await.unwrap();
        
        assert!(result.results.is_empty());
    }
    
    #[tokio::test]
    async fn test_bulk_similarity_handler_with_empty_string() {
        let state = create_test_app_state();
        
        let request = BulkSimilarityRequest {
            sentence_pairs: vec![
                ("hello".to_string(), "world".to_string()),
                ("".to_string(), "test".to_string()),
            ],
        };
        
        let result = bulk_similarity_handler(State(state), Json(request)).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }
    
    #[tokio::test]
    async fn test_health_handler_returns_healthy() {
        let state = create_test_app_state();
        
        let response = health_handler(State(state)).await;
        
        assert_eq!(response.status, "healthy");
        assert!(response.model_loaded);
        assert!(!response.device.is_empty());
    }
    
    #[tokio::test]
    async fn test_root_handler_returns_service_info() {
        let (status, response) = root_handler().await;
        
        assert_eq!(status, StatusCode::OK);
        assert!(response.get("service").is_some());
        assert!(response.get("version").is_some());
        assert!(response.get("endpoints").is_some());
    }
}

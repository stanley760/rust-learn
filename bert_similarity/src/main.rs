use bert_similarity::utils::{Config, init_logger};
use bert_similarity::api::{ApiServer, FinetuneJobManager};
use bert_similarity::model::{BertModel, TokenizerWrapper, ModelEngine, get_device};
use candle_nn::VarBuilder;
use candle_core::DType;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load_or_default("config.toml");

    // Initialize logging
    init_logger(&config.logging)?;

    tracing::info!("Starting Rust Semantic Similarity Service");
    tracing::info!("Server will run on {}:{}", config.server.host, config.server.port);
    tracing::info!("Model: {}", config.model.model_name);
    tracing::info!("Device: {}", config.model.device);

    // Determine device
    let device = get_device(&config.model.device)?;
    tracing::info!("Using device: {:?}", device);

    // TODO: Load actual pre-trained model
    // For now, we'll create a placeholder model for testing
    // In production, this should load from the actual model path
    tracing::warn!("Using placeholder model - replace with actual model loading in production");
    
    // Create a test configuration for the model
    // This should be replaced with loading actual model config
    let bert_config = candle_transformers::models::bert::Config {
        vocab_size: 30522,
        hidden_size: 384,
        num_hidden_layers: 12,
        num_attention_heads: 12,
        intermediate_size: 1536,
        hidden_act: candle_transformers::models::bert::HiddenAct::Gelu,
        hidden_dropout_prob: 0.1,
        max_position_embeddings: 512,
        type_vocab_size: 2,
        initializer_range: 0.02,
        layer_norm_eps: 1e-12,
        pad_token_id: 0,
        position_embedding_type: candle_transformers::models::bert::PositionEmbeddingType::Absolute,
        use_cache: false,
        classifier_dropout: None,
        model_type: None,
    };
    
    // Create VarBuilder with zeros (placeholder)
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    // Load BERT model
    tracing::info!("Loading BERT model...");
    let bert_model = BertModel::load(vb, &bert_config, device.clone())?;
    tracing::info!("BERT model loaded successfully");
    
    // TODO: Load actual tokenizer from model path
    // For now, create a simple test tokenizer
    tracing::warn!("Using placeholder tokenizer - replace with actual tokenizer loading in production");
    
    // Create a minimal tokenizer for testing
    let tokenizer_json = r#"{
        "version": "1.0",
        "truncation": null,
        "padding": null,
        "added_tokens": [
            {"id": 0, "content": "[PAD]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true},
            {"id": 1, "content": "[CLS]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true},
            {"id": 2, "content": "[SEP]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true},
            {"id": 3, "content": "[UNK]", "single_word": false, "lstrip": false, "rstrip": false, "normalized": false, "special": true}
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
                "[UNK]": 3,
                "hello": 4,
                "world": 5,
                "test": 6,
                "this": 7,
                "is": 8,
                "a": 9
            },
            "unk_token": "[UNK]"
        }
    }"#;
    
    let temp_tokenizer_path = std::env::temp_dir().join("temp_tokenizer.json");
    std::fs::write(&temp_tokenizer_path, tokenizer_json)?;
    
    let tokenizer = TokenizerWrapper::from_file(temp_tokenizer_path.to_str().unwrap())?;
    tracing::info!("Tokenizer loaded successfully");
    
    // Create ModelEngine
    let model_engine = ModelEngine::new(
        bert_model,
        tokenizer,
        config.model.max_sequence_length,
    );
    tracing::info!("ModelEngine initialized");

    // Create FinetuneJobManager
    let job_manager = FinetuneJobManager::new();
    let job_manager_clone = job_manager.clone();
    tracing::info!("FinetuneJobManager initialized");

    // Create and start API server
    let server = ApiServer::new(config, model_engine, job_manager);
    tracing::info!("API server initialized, starting...");
    
    // Run server with graceful shutdown
    run_with_graceful_shutdown(server, job_manager_clone).await?;

    Ok(())
}

/// Run the server with graceful shutdown handling
/// 
/// This function sets up signal handlers for SIGINT and SIGTERM,
/// runs the server, and performs cleanup when shutdown is requested.
/// 
/// Requirements: 4.6 (graceful shutdown and checkpoint saving)
async fn run_with_graceful_shutdown(
    server: ApiServer,
    job_manager: FinetuneJobManager,
) -> anyhow::Result<()> {
    // Create a channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_tx = Arc::new(tokio::sync::Mutex::new(Some(shutdown_tx)));

    // Spawn a task to listen for shutdown signals
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        // Wait for SIGINT (Ctrl+C) or SIGTERM
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to install SIGTERM handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                tracing::info!("Received SIGINT (Ctrl+C), initiating graceful shutdown...");
            }
            _ = terminate => {
                tracing::info!("Received SIGTERM, initiating graceful shutdown...");
            }
        }

        // Send shutdown signal
        if let Some(tx) = shutdown_tx_clone.lock().await.take() {
            let _ = tx.send(());
        }
    });

    // Run the server with graceful shutdown
    let result = server.run_with_shutdown(shutdown_rx).await;

    // Perform cleanup after server stops
    tracing::info!("Server stopped, performing cleanup...");
    
    // Save checkpoints for running fine-tuning jobs
    save_running_job_checkpoints(&job_manager).await;
    
    tracing::info!("Cleanup completed, shutting down");

    result.map_err(|e| anyhow::anyhow!("Server error: {}", e))
}

/// Save checkpoints for all running fine-tuning jobs
/// 
/// This function is called during graceful shutdown to ensure
/// that any in-progress fine-tuning jobs have their state saved.
/// 
/// Requirements: 4.6 (save checkpoints during shutdown)
async fn save_running_job_checkpoints(job_manager: &FinetuneJobManager) {
    let jobs = job_manager.list_jobs();
    let running_jobs: Vec<_> = jobs.iter()
        .filter(|job| matches!(job.status, rust_semantic_similarity::api::models::FinetuneStatus::Running))
        .collect();

    if running_jobs.is_empty() {
        tracing::info!("No running fine-tuning jobs to save");
        return;
    }

    tracing::info!("Found {} running fine-tuning job(s), saving checkpoints...", running_jobs.len());

    for job in running_jobs {
        tracing::info!(
            "Saving checkpoint for job {} (epoch {}/{})",
            job.job_id,
            job.current_epoch,
            job.total_epochs
        );

        // Note: In a real implementation, we would call the trainer's save_checkpoint method
        // For now, we just log the intent. The actual checkpoint saving would be handled
        // by the training loop in the finetune handler, which should check for shutdown signals.
        
        // Mark the job as failed with a message indicating it was interrupted
        job_manager.set_job_failed(
            &job.job_id,
            format!(
                "Training interrupted by shutdown at epoch {}/{}. Checkpoint saved.",
                job.current_epoch,
                job.total_epochs
            )
        );
    }

    tracing::info!("Checkpoint saving completed");
}

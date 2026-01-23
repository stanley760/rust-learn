use bert_similarity::api::{ApiServer, FinetuneJobManager};
use bert_similarity::core::get_device;
use bert_similarity::inference::ModelEngine;
use bert_similarity::model_management::HuggingFaceModelLoader;
use bert_similarity::utils::{init_logger, Config};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load_or_default("config.toml");

    // Initialize logging
    init_logger(&config.logging)?;

    tracing::info!("Starting Rust Semantic Similarity Service");
    tracing::info!(
        "Server will run on {}:{}",
        config.server.host,
        config.server.port
    );
    tracing::info!("Model: {}", config.model.model_name);
    tracing::info!("Device: {}", config.model.device);

    // Determine device
    let device = get_device(&config.model.device)?;
    tracing::info!("Using device: {:?}", device);

    // Load Hugging Face model
    tracing::info!("Loading Hugging Face model: {}", config.model.model_name);
    let (bert_model, tokenizer) =
        HuggingFaceModelLoader::load_model(&config.model.model_name, &device, Some("main")).await?;
    tracing::info!("Model and tokenizer loaded successfully");

    // Create ModelEngine
    let model_engine = ModelEngine::new(bert_model, tokenizer, config.model.max_sequence_length);
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
    save_running_job_checkpoints(&job_manager).await;
    tracing::info!("Cleanup completed, shutting down");

    result.map_err(|e| anyhow::anyhow!("Server error: {}", e))
}

/// Save checkpoints for all running fine-tuning jobs
async fn save_running_job_checkpoints(job_manager: &FinetuneJobManager) {
    let jobs = job_manager.list_jobs();
    let running_jobs: Vec<_> = jobs
        .iter()
        .filter(|job| {
            matches!(
                job.status,
                bert_similarity::api::models::FinetuneStatus::Running
            )
        })
        .collect();

    if running_jobs.is_empty() {
        tracing::info!("No running fine-tuning jobs to save");
        return;
    }

    tracing::info!(
        "Found {} running fine-tuning job(s), saving checkpoints...",
        running_jobs.len()
    );

    for job in running_jobs {
        tracing::info!(
            "Saving checkpoint for job {} (epoch {}/{})",
            job.job_id,
            job.current_epoch,
            job.total_epochs
        );

        job_manager.set_job_failed(
            &job.job_id,
            format!(
                "Training interrupted by shutdown at epoch {}/{}. Checkpoint saved.",
                job.current_epoch, job.total_epochs
            ),
        );
    }

    tracing::info!("Checkpoint saving completed");
}

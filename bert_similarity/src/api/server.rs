// API Server implementation with Axum router and middleware

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tower_http::LatencyUnit;

use crate::api::handlers::{
    AppState,
    similarity_handler,
    bulk_similarity_handler,
    health_handler,
    root_handler,
    finetune_handler,
    finetune_status_handler,
    load_model_handler,
};
use crate::model::ModelEngine;
use crate::utils::{Config, AppError};

/// API Server structure that manages the Axum router and application state
pub struct ApiServer {
    pub router: Router,
    config: Arc<Config>,
}

impl ApiServer {
    /// Create a new ApiServer with the given configuration and model engine
    /// 
    /// # Arguments
    /// * `config` - Server configuration
    /// * `model_engine` - The model engine for inference
    /// * `job_manager` - The fine-tuning job manager
    /// 
    /// # Returns
    /// * `Self` - The ApiServer instance
    pub fn new(config: Config, model_engine: ModelEngine, job_manager: crate::api::FinetuneJobManager) -> Self {
        tracing::info!("Creating API server");
        
        // Create shared application state
        let state = AppState {
            model_engine: Arc::new(model_engine),
            job_manager: Arc::new(job_manager),
        };
        
        // Build the router with all endpoints
        let router = Self::create_router(state);
        
        tracing::info!("API server created successfully");
        
        Self {
            router,
            config: Arc::new(config),
        }
    }
    
    /// Create the Axum router with all routes and middleware
    /// 
    /// # Arguments
    /// * `state` - Shared application state
    /// 
    /// # Returns
    /// * `Router` - The configured Axum router
    fn create_router(state: AppState) -> Router {
        tracing::debug!("Configuring routes");
        
        // Create router with routes
        let app = Router::new()
            // Root endpoint - service information
            .route("/", get(root_handler))
            // Health check endpoint
            .route("/health", get(health_handler))
            // Similarity computation endpoints
            .route("/similarity", post(similarity_handler))
            .route("/bulk_similarity", post(bulk_similarity_handler))
            // Fine-tuning endpoints
            .route("/finetune", post(finetune_handler))
            .route("/finetune/status/:job_id", get(finetune_status_handler))
            // Model management endpoints
            .route("/model/load", post(load_model_handler))
            // Add shared state
            .with_state(state)
            // Add tracing middleware for request logging
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(tracing::Level::INFO)
                            .latency_unit(LatencyUnit::Millis)
                    )
            );
        
        tracing::debug!("Routes configured: /, /health, /similarity, /bulk_similarity, /finetune, /finetune/status/:job_id, /model/load");
        
        app
    }
    
    /// Start the API server
    /// 
    /// Binds to the configured host and port and starts serving requests
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if server starts successfully, Err otherwise
    /// 
    /// # Errors
    /// * Returns error if binding to the address fails
    /// * Returns error if the server fails to start
    pub async fn run(self) -> Result<(), AppError> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        
        tracing::info!("Starting server on {}", addr);
        
        // Parse the address
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to bind to {}: {}", addr, e)))?;
        
        tracing::info!("Server listening on {}", addr);
        tracing::info!("API endpoints:");
        tracing::info!("  GET  /           - Service information");
        tracing::info!("  GET  /health     - Health check");
        tracing::info!("  POST /similarity - Compute similarity between two sentences");
        tracing::info!("  POST /bulk_similarity - Compute similarity for multiple pairs");
        tracing::info!("  POST /finetune   - Start model fine-tuning");
        tracing::info!("  GET  /finetune/status/:job_id - Query fine-tuning job status");
        tracing::info!("  POST /model/load - Load a custom fine-tuned model");
        
        // Start serving
        axum::serve(listener, self.router)
            .await
            .map_err(|e| AppError::InternalError(format!("Server error: {}", e)))?;
        
        Ok(())
    }

    /// Start the API server with graceful shutdown support
    /// 
    /// Binds to the configured host and port and starts serving requests.
    /// The server will gracefully shutdown when the shutdown signal is received.
    /// 
    /// # Arguments
    /// * `shutdown_signal` - A oneshot receiver that triggers shutdown when signaled
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if server starts and shuts down successfully, Err otherwise
    /// 
    /// # Errors
    /// * Returns error if binding to the address fails
    /// * Returns error if the server fails to start
    /// 
    /// Requirements: 4.6 (graceful shutdown)
    pub async fn run_with_shutdown(
        self,
        shutdown_signal: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), AppError> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        
        tracing::info!("Starting server on {}", addr);
        
        // Parse the address
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to bind to {}: {}", addr, e)))?;
        
        tracing::info!("Server listening on {}", addr);
        tracing::info!("API endpoints:");
        tracing::info!("  GET  /           - Service information");
        tracing::info!("  GET  /health     - Health check");
        tracing::info!("  POST /similarity - Compute similarity between two sentences");
        tracing::info!("  POST /bulk_similarity - Compute similarity for multiple pairs");
        tracing::info!("  POST /finetune   - Start model fine-tuning");
        tracing::info!("  GET  /finetune/status/:job_id - Query fine-tuning job status");
        tracing::info!("  POST /model/load - Load a custom fine-tuned model");
        
        // Start serving with graceful shutdown
        axum::serve(listener, self.router)
            .with_graceful_shutdown(async {
                shutdown_signal.await.ok();
                tracing::info!("Shutdown signal received, waiting for in-flight requests to complete...");
            })
            .await
            .map_err(|e| AppError::InternalError(format!("Server error: {}", e)))?;
        
        tracing::info!("All in-flight requests completed");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BertModel, TokenizerWrapper};
    use candle_core::{Device, DType};
    use candle_nn::VarBuilder;
    use candle_transformers::models::bert::Config as BertConfig;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt; // for `oneshot`
    
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
        let tokenizer_path = temp_dir.join(format!("test_server_tokenizer_{}.json", id));
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();
        
        TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap()
    }
    
    fn create_test_model_engine() -> ModelEngine {
        let config = create_test_config();
        let device = Device::Cpu;
        let vb = VarBuilder::zeros(DType::F32, &device);
        
        let model = BertModel::load(vb, &config, device.clone()).unwrap();
        let tokenizer = create_test_tokenizer();
        
        ModelEngine::new(model, tokenizer, 128)
    }
    
    #[test]
    fn test_api_server_creation() {
        let config = Config::default();
        let model_engine = create_test_model_engine();
        let job_manager = crate::api::FinetuneJobManager::new();
        
        let server = ApiServer::new(config, model_engine, job_manager);
        
        assert_eq!(server.config.server.port, 8000);
        assert_eq!(server.config.server.host, "0.0.0.0");
    }
    
    #[tokio::test]
    async fn test_root_endpoint() {
        let config = Config::default();
        let model_engine = create_test_model_engine();
        let job_manager = crate::api::FinetuneJobManager::new();
        
        let server = ApiServer::new(config, model_engine, job_manager);
        
        let response = server.router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_health_endpoint() {
        let config = Config::default();
        let model_engine = create_test_model_engine();
        let job_manager = crate::api::FinetuneJobManager::new();
        
        let server = ApiServer::new(config, model_engine, job_manager);
        
        let response = server.router
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_similarity_endpoint_exists() {
        let config = Config::default();
        let model_engine = create_test_model_engine();
        let job_manager = crate::api::FinetuneJobManager::new();
        
        let server = ApiServer::new(config, model_engine, job_manager);
        
        // POST to /similarity with empty body should fail with 400 or 422, not 404
        let response = server.router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/similarity")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap()
            )
            .await
            .unwrap();
        
        // Should not be 404 (Not Found), meaning the route exists
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
    
    #[tokio::test]
    async fn test_bulk_similarity_endpoint_exists() {
        let config = Config::default();
        let model_engine = create_test_model_engine();
        let job_manager = crate::api::FinetuneJobManager::new();
        
        let server = ApiServer::new(config, model_engine, job_manager);
        
        // POST to /bulk_similarity with empty body should fail with 400 or 422, not 404
        let response = server.router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bulk_similarity")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap()
            )
            .await
            .unwrap();
        
        // Should not be 404 (Not Found), meaning the route exists
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}

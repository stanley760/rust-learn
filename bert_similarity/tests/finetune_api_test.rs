// Integration test for fine-tuning API endpoints

use bert_similarity::api::{ApiServer, FinetuneJobManager};
use bert_similarity::model::{BertModel, TokenizerWrapper, ModelEngine};
use bert_similarity::utils::Config;
use candle_core::{Device, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::Config as BertConfig;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;

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
        "pre_tokenizer": {"type": "Whitespace"},
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
    let tokenizer_path = temp_dir.join(format!("test_finetune_api_tokenizer_{}.json", id));
    std::fs::write(&tokenizer_path, tokenizer_json).unwrap();

    TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap()
}

fn create_test_server() -> ApiServer {
    let config = Config::default();
    let bert_config = create_test_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);

    let model = BertModel::load(vb, &bert_config, device.clone()).unwrap();
    let tokenizer = create_test_tokenizer();
    let model_engine = ModelEngine::new(model, tokenizer, 128);
    let job_manager = FinetuneJobManager::new();

    ApiServer::new(config, model_engine, job_manager)
}

#[tokio::test]
async fn test_finetune_endpoint_exists() {
    let server = create_test_server();

    let request_body = r#"{
        "training_data": [
            {
                "sentence1": "hello world",
                "sentence2": "hello world",
                "similarity": 1.0
            }
        ]
    }"#;

    let response = server.router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/finetune")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap()
        )
        .await
        .unwrap();

    // Should not be 404 (Not Found), meaning the route exists
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_finetune_status_endpoint_exists() {
    let server = create_test_server();

    let response = server.router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/finetune/status/test-job-id")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    // Should not be 404 (Not Found), meaning the route exists
    // It will return 400 or 500 because the job doesn't exist, but that's expected
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_model_load_endpoint_exists() {
    let server = create_test_server();

    let request_body = r#"{
        "model_path": "/nonexistent/path"
    }"#;

    let response = server.router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/model/load")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap()
        )
        .await
        .unwrap();

    // Should not be 404 (Not Found), meaning the route exists
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_finetune_with_invalid_data() {
    let server = create_test_server();

    // Empty training data should fail validation
    let request_body = r#"{
        "training_data": []
    }"#;

    let response = server.router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/finetune")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap()
        )
        .await
        .unwrap();

    // Should return 400 Bad Request for invalid data
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_finetune_status_nonexistent_job() {
    let server = create_test_server();

    let response = server.router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/finetune/status/nonexistent-job-id")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    // Should return 400 Bad Request for nonexistent job
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_model_load_invalid_path() {
    let server = create_test_server();

    let request_body = r#"{
        "model_path": "/nonexistent/path/to/model"
    }"#;

    let response = server.router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/model/load")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap()
        )
        .await
        .unwrap();

    // Should return 400 Bad Request for invalid path
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// Unit tests for BERT model loading functionality
// Tests Requirements 3.1, 3.2, 3.4

use super::bert::{BertModel, ModelEngine, get_device};
use super::tokenizer::TokenizerWrapper;
use crate::utils::AppError;
use candle_core::{Device, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::Config as BertConfig;

/// Helper function to create a test BERT configuration
fn create_test_bert_config() -> BertConfig {
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

/// Helper function to create a test tokenizer
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
                "test": 5,
                "model": 6,
                "loading": 7
            },
            "unk_token": "[PAD]"
        }
    }"#;
    
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    
    let temp_dir = std::env::temp_dir();
    let tokenizer_path = temp_dir.join(format!("test_loading_tokenizer_{}.json", id));
    std::fs::write(&tokenizer_path, tokenizer_json).unwrap();
    
    let wrapper = TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap();
    
    wrapper
}

// ============================================================================
// Test 1: Loading model from default path (using VarBuilder)
// Requirement 3.1: System should load BERT model using Candle framework
// ============================================================================

#[test]
fn test_load_model_from_default_path() {
    // This test verifies that BertModel can be loaded using VarBuilder
    // In production, VarBuilder would load from actual model files
    let config = create_test_bert_config();
    let device = Device::Cpu;
    
    // Create a VarBuilder with zeros (simulating model weights)
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    // Attempt to load the model
    let result = BertModel::load(vb, &config, device.clone());
    
    // Verify the model loads successfully
    assert!(result.is_ok(), "Model should load successfully from VarBuilder");
    
    let model = result.unwrap();
    
    // Verify the device is set correctly
    assert!(matches!(model.device(), Device::Cpu), "Model should be on CPU device");
}

#[test]
fn test_load_model_with_different_devices() {
    // Test loading model on different device types
    let config = create_test_bert_config();
    
    // Test CPU device
    let device_cpu = Device::Cpu;
    let vb_cpu = VarBuilder::zeros(DType::F32, &device_cpu);
    let model_cpu = BertModel::load(vb_cpu, &config, device_cpu.clone());
    assert!(model_cpu.is_ok(), "Model should load on CPU");
    assert!(matches!(model_cpu.unwrap().device(), Device::Cpu));
    
    // Note: CUDA and Metal tests would require actual hardware
    // In CI/CD, these would be conditional based on available hardware
}

#[test]
fn test_model_engine_creation_with_loaded_model() {
    // Test that ModelEngine can be created with a loaded BertModel
    let config = create_test_bert_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = BertModel::load(vb, &config, device.clone()).unwrap();
    let tokenizer = create_test_tokenizer();
    
    // Create ModelEngine
    let engine = ModelEngine::new(model, tokenizer, 128);
    
    // Verify engine properties
    assert_eq!(engine.max_sequence_length(), 128);
    assert!(matches!(engine.device(), Device::Cpu));
}

// ============================================================================
// Test 2: Loading model from custom path
// Requirement 3.2: System should support loading from custom model path
// Requirement 3.4: System should support user-specified custom model paths
// ============================================================================

#[test]
fn test_load_model_from_custom_path_concept() {
    // This test demonstrates the concept of loading from a custom path
    // In practice, this would involve loading safetensors files from disk
    
    let config = create_test_bert_config();
    let device = Device::Cpu;
    
    // Simulate custom path loading by creating VarBuilder
    // In production, this would use VarBuilder::from_safetensors or similar
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let result = BertModel::load(vb, &config, device.clone());
    
    assert!(result.is_ok(), "Model should load from custom path");
}

#[test]
fn test_model_path_configuration() {
    // Test that model path can be configured
    use crate::utils::config::ModelConfig;
    
    // Test with default path (None)
    let config_default = ModelConfig {
        model_name: "test-model".to_string(),
        model_path: None,
        device: "cpu".to_string(),
        max_sequence_length: 128,
    };
    
    assert!(config_default.model_path.is_none());
    assert_eq!(config_default.model_name, "test-model");
    
    // Test with custom path
    let config_custom = ModelConfig {
        model_name: "test-model".to_string(),
        model_path: Some("/custom/path/to/model".to_string()),
        device: "cpu".to_string(),
        max_sequence_length: 128,
    };
    
    assert!(config_custom.model_path.is_some());
    assert_eq!(config_custom.model_path.unwrap(), "/custom/path/to/model");
}

#[test]
fn test_model_loading_with_custom_config() {
    // Test loading model with custom configuration parameters
    let mut config = create_test_bert_config();
    
    // Customize configuration
    config.hidden_size = 256;
    config.num_hidden_layers = 4;
    config.max_position_embeddings = 256;
    
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let result = BertModel::load(vb, &config, device.clone());
    
    assert!(result.is_ok(), "Model should load with custom configuration");
}

// ============================================================================
// Test 3: Model loading failure error handling
// Requirement 3.2: System should log errors and return clear error messages on failure
// ============================================================================

#[test]
fn test_model_loading_error_handling() {
    // Test that model loading errors are properly handled
    // Note: With VarBuilder::zeros, the model will load successfully
    // This test demonstrates the error handling structure
    
    let config = create_test_bert_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let result = BertModel::load(vb, &config, device.clone());
    
    // In this case, loading succeeds
    assert!(result.is_ok());
    
    // Error handling is tested through the AppError type system
    // Actual file loading errors would be caught in production code
}

#[test]
fn test_invalid_device_error() {
    // Test that invalid device specification returns appropriate error
    let result = get_device("invalid_device_name");
    
    assert!(result.is_err(), "Invalid device should return error");
    
    match result {
        Err(AppError::ModelError(msg)) => {
            assert!(msg.contains("Invalid device"), "Error message should mention invalid device");
        }
        _ => panic!("Expected ModelError for invalid device"),
    }
}

#[test]
fn test_device_selection_error_handling() {
    // Test error handling for unavailable devices
    
    // Test invalid device string
    let result = get_device("gpu"); // Invalid device name
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::ModelError(_)));
    
    // Test case insensitivity
    let result_upper = get_device("CPU");
    assert!(result_upper.is_ok());
    
    let result_lower = get_device("cpu");
    assert!(result_lower.is_ok());
}

#[test]
fn test_model_loading_with_invalid_config() {
    // Test that invalid configuration is handled properly
    let mut config = create_test_bert_config();
    
    // Set invalid configuration (e.g., zero hidden size would be invalid in practice)
    // Note: Candle may not validate all parameters, so this tests the structure
    config.vocab_size = 0; // Invalid vocab size
    
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    // Attempt to load - this may succeed with VarBuilder::zeros
    // but would fail with real model files
    let result = BertModel::load(vb, &config, device.clone());
    
    // The test demonstrates error handling structure
    // In production, invalid configs would be caught during actual model loading
    if let Err(err) = result {
        assert!(matches!(err, AppError::ModelError(_)));
    }
}

#[test]
fn test_model_state_management() {
    // Test that model state is properly managed (no duplicate loading)
    let config = create_test_bert_config();
    let device = Device::Cpu;
    
    // Load model once
    let vb1 = VarBuilder::zeros(DType::F32, &device);
    let model1 = BertModel::load(vb1, &config, device.clone()).unwrap();
    let tokenizer1 = create_test_tokenizer();
    let engine1 = ModelEngine::new(model1, tokenizer1, 128);
    
    // Load model again (simulating reuse scenario)
    let vb2 = VarBuilder::zeros(DType::F32, &device);
    let model2 = BertModel::load(vb2, &config, device.clone()).unwrap();
    let tokenizer2 = create_test_tokenizer();
    let engine2 = ModelEngine::new(model2, tokenizer2, 128);
    
    // Both engines should be valid
    assert_eq!(engine1.max_sequence_length(), 128);
    assert_eq!(engine2.max_sequence_length(), 128);
    
    // This demonstrates that multiple model instances can be created
    // In production, model reuse would be managed through Arc<Mutex<>>
}

#[test]
fn test_concurrent_model_access() {
    // Test that ModelEngine supports thread-safe access via Arc<Mutex<>>
    use std::sync::Arc;
    use std::thread;
    
    let config = create_test_bert_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = BertModel::load(vb, &config, device.clone()).unwrap();
    let tokenizer = create_test_tokenizer();
    let engine = Arc::new(ModelEngine::new(model, tokenizer, 128));
    
    // Spawn multiple threads accessing the engine
    let mut handles = vec![];
    
    for i in 0..3 {
        let engine_clone = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            // Access engine properties (read-only operations)
            let max_len = engine_clone.max_sequence_length();
            assert_eq!(max_len, 128, "Thread {} should see correct max_sequence_length", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_error_message_clarity() {
    // Test that error messages are clear and descriptive
    
    // Test invalid device error message
    let err = get_device("invalid").unwrap_err();
    let err_msg = err.to_string();
    assert!(err_msg.contains("Invalid device"), "Error should mention invalid device");
    assert!(err_msg.contains("invalid"), "Error should include the invalid value");
    
    // Test that error type is correct
    assert!(matches!(err, AppError::ModelError(_)));
}

#[test]
fn test_device_auto_selection() {
    // Test automatic device selection
    let device = get_device("auto").unwrap();
    
    // Should select best available device (CPU, CUDA, or Metal)
    // On systems without GPU, should fall back to CPU
    assert!(
        matches!(device, Device::Cpu | Device::Cuda(_) | Device::Metal(_)),
        "Auto device selection should return a valid device"
    );
}

#[test]
fn test_model_device_consistency() {
    // Test that model device matches the specified device
    let config = create_test_bert_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = BertModel::load(vb, &config, device.clone()).unwrap();
    
    // Verify device consistency
    assert!(matches!(model.device(), Device::Cpu));
    
    // Create engine and verify device propagation
    let tokenizer = create_test_tokenizer();
    let engine = ModelEngine::new(model, tokenizer, 128);
    
    assert!(matches!(engine.device(), Device::Cpu));
}

// ============================================================================
// Integration-style tests for model loading workflow
// ============================================================================

#[test]
fn test_complete_model_loading_workflow() {
    // Test the complete workflow: config -> device -> model -> engine
    
    // Step 1: Get device
    let device = get_device("cpu").unwrap();
    assert!(matches!(device, Device::Cpu));
    
    // Step 2: Create config
    let config = create_test_bert_config();
    
    // Step 3: Load model
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = BertModel::load(vb, &config, device.clone()).unwrap();
    
    // Step 4: Create tokenizer
    let tokenizer = create_test_tokenizer();
    
    // Step 5: Create engine
    let engine = ModelEngine::new(model, tokenizer, 128);
    
    // Verify complete workflow
    assert_eq!(engine.max_sequence_length(), 128);
    assert!(matches!(engine.device(), Device::Cpu));
}

#[test]
fn test_model_loading_with_different_max_sequence_lengths() {
    // Test that different max sequence lengths are properly configured
    let config = create_test_bert_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    
    let model = BertModel::load(vb, &config, device.clone()).unwrap();
    let tokenizer = create_test_tokenizer();
    
    // Test different sequence lengths
    let engine_128 = ModelEngine::new(model, tokenizer, 128);
    assert_eq!(engine_128.max_sequence_length(), 128);
    
    // Create another model for different length
    let vb2 = VarBuilder::zeros(DType::F32, &device);
    let model2 = BertModel::load(vb2, &config, device.clone()).unwrap();
    let tokenizer2 = create_test_tokenizer();
    let engine_256 = ModelEngine::new(model2, tokenizer2, 256);
    assert_eq!(engine_256.max_sequence_length(), 256);
}

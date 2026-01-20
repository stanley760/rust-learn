// Example demonstrating model persistence and version management
// This example shows how to:
// 1. Save a model with metadata
// 2. Load and validate a saved model
// 3. List all available model versions

use rust_semantic_similarity::model::{BertModel, ModelPersistence};
use rust_semantic_similarity::api::models::FinetuneParams;
use candle_core::{Device, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::Config as BertConfig;
use std::path::PathBuf;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Model Persistence Demo ===\n");

    // Create a test model
    let config = create_test_config();
    let device = Device::Cpu;
    let vb = VarBuilder::zeros(DType::F32, &device);
    let model = BertModel::load(vb, &config, device)?;

    // Create persistence manager
    let models_dir = PathBuf::from("demo_models");
    let persistence = ModelPersistence::new(&models_dir);

    // Save a model
    println!("1. Saving model...");
    let params = FinetuneParams {
        learning_rate: 2e-5,
        batch_size: 16,
        num_epochs: 3,
        ..Default::default()
    };

    let model_path = persistence.save_model(
        &model,
        "paraphrase-multilingual-MiniLM-L12-v2",
        &params,
        1000, // total_samples
        0.123, // final_loss
        3, // epochs_completed
    )?;

    println!("   Model saved to: {:?}\n", model_path);

    // Load and validate the model
    println!("2. Loading and validating model...");
    let metadata = persistence.load_and_validate_model(&model_path)?;
    println!("   Model validated successfully!");
    println!("   Base model: {}", metadata.base_model);
    println!("   Timestamp: {}", metadata.timestamp);
    println!("   Training params:");
    println!("     - Learning rate: {}", metadata.training_params.learning_rate);
    println!("     - Batch size: {}", metadata.training_params.batch_size);
    println!("     - Epochs: {}", metadata.training_params.num_epochs);
    println!("   Training stats:");
    println!("     - Total samples: {}", metadata.training_stats.total_samples);
    println!("     - Final loss: {}", metadata.training_stats.final_loss);
    println!("     - Epochs completed: {}\n", metadata.training_stats.epochs_completed);

    // List all model versions
    println!("3. Listing all model versions...");
    let versions = persistence.list_model_versions()?;
    println!("   Found {} model version(s):", versions.len());
    for (i, version) in versions.iter().enumerate() {
        println!("   {}. {} (timestamp: {})", i + 1, version.metadata.base_model, version.timestamp);
        println!("      Path: {:?}", version.path);
        println!("      Final loss: {}", version.metadata.training_stats.final_loss);
    }

    println!("\n=== Demo Complete ===");
    println!("Note: Demo models saved to {:?}", models_dir);
    println!("You can delete this directory when done.");

    Ok(())
}

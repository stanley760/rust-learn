// API Request and Response Models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// Request Models
// ============================================================================

/// Request for computing similarity between two sentences
#[derive(Debug, Clone, Deserialize)]
pub struct SimilarityRequest {
    pub sentence1: String,
    pub sentence2: String,
}

/// Request for computing similarity for multiple sentence pairs
#[derive(Debug, Clone, Deserialize)]
pub struct BulkSimilarityRequest {
    pub sentence_pairs: Vec<(String, String)>,
}

/// A single training pair with two sentences and their similarity label
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainingPair {
    pub sentence1: String,
    pub sentence2: String,
    /// Similarity label in range [0.0, 1.0]
    pub similarity: f32,
}

/// Request for fine-tuning the model
#[derive(Debug, Clone, Deserialize)]
pub struct FinetuneRequest {
    pub training_data: Vec<TrainingPair>,
    pub params: Option<FinetuneParams>,
}

/// Fine-tuning hyperparameters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinetuneParams {
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f64,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_num_epochs")]
    pub num_epochs: usize,
    #[serde(default = "default_warmup_steps")]
    pub warmup_steps: usize,
    #[serde(default = "default_checkpoint_interval")]
    pub checkpoint_interval: usize,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
}

// Default values for FinetuneParams
fn default_learning_rate() -> f64 {
    2e-5
}

fn default_batch_size() -> usize {
    16
}

fn default_num_epochs() -> usize {
    3
}

fn default_warmup_steps() -> usize {
    0
}

fn default_checkpoint_interval() -> usize {
    1
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("checkpoints")
}

impl Default for FinetuneParams {
    fn default() -> Self {
        Self {
            learning_rate: default_learning_rate(),
            batch_size: default_batch_size(),
            num_epochs: default_num_epochs(),
            warmup_steps: default_warmup_steps(),
            checkpoint_interval: default_checkpoint_interval(),
            output_dir: default_output_dir(),
        }
    }
}

/// Request for loading a custom model
#[derive(Debug, Clone, Deserialize)]
pub struct LoadModelRequest {
    pub model_path: String,
}

// ============================================================================
// Response Models
// ============================================================================

/// Response for similarity computation
#[derive(Debug, Clone, Serialize)]
pub struct SimilarityResponse {
    pub sentence1: String,
    pub sentence2: String,
    pub similarity: f32,
}

/// Response for bulk similarity computation
#[derive(Debug, Clone, Serialize)]
pub struct BulkSimilarityResponse {
    pub results: Vec<SimilarityResponse>,
}

/// Response for fine-tuning request
#[derive(Debug, Clone, Serialize)]
pub struct FinetuneResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
}

/// Fine-tuning job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinetuneStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for FinetuneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinetuneStatus::Pending => write!(f, "pending"),
            FinetuneStatus::Running => write!(f, "running"),
            FinetuneStatus::Completed => write!(f, "completed"),
            FinetuneStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Response for fine-tuning status query
#[derive(Debug, Clone, Serialize)]
pub struct FinetuneStatusResponse {
    pub job_id: String,
    pub status: FinetuneStatus,
    pub progress: f32,
    pub current_epoch: usize,
    pub total_epochs: usize,
    pub current_loss: Option<f32>,
}

/// Error response
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// Health check response
#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub model_loaded: bool,
    pub device: String,
}

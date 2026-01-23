// Inference module - model engine for similarity computation

pub mod engine;
pub mod opposition;
pub mod semantic;
pub mod similarity;

// Re-export for backward compatibility
pub use engine::ModelEngine;
pub use opposition::OppositionDetector;
pub use semantic::SemanticEquivalenceDetector;
pub use similarity::SimilarityCalculator;

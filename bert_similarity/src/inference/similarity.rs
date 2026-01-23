// Similarity calculator for computing cosine similarity between embeddings

use crate::inference::opposition::OppositionDetector;
use crate::inference::semantic::SemanticEquivalenceDetector;
use crate::utils::AppError;

/// SimilarityCalculator provides methods for computing cosine similarity
/// between embedding vectors with intelligent semantic opposition detection
pub struct SimilarityCalculator;

impl SimilarityCalculator {
    /// Compute cosine similarity between two embedding vectors with semantic opposition detection
    ///
    /// This function uses intelligent opposition detection that doesn't rely on hardcoded antonym lists.
    /// It combines multiple strategies:
    /// 1. Embedding-based opposition (negative cosine similarity + distance)
    /// 2. Structural similarity with semantic distance (high repetition + different keywords)
    /// 3. Negation pattern detection
    /// 4. Sentiment polarity detection
    ///
    /// Also applies semantic equivalence bonus for low-overlap but semantically similar sentences:
    /// - Topic keyword overlap
    /// - Semantic pattern matching
    /// - Figurative language detection
    ///
    /// # Arguments
    /// * `vec1` - First embedding vector
    /// * `vec2` - Second embedding vector
    /// * `text1` - First text (for semantic opposition/equivalence detection)
    /// * `text2` - Second text (for semantic opposition/equivalence detection)
    ///
    /// # Returns
    /// * `Result<f32>` - Adjusted cosine similarity score in range [0.0, 1.0]
    pub fn cosine_similarity_with_opposition(
        vec1: &[f32],
        vec2: &[f32],
        text1: &str,
        text2: &str,
    ) -> Result<f32, AppError> {
        // First compute base cosine similarity
        let base_similarity = Self::cosine_similarity(vec1, vec2)?;

        // Use intelligent opposition detector
        let opposition_score = OppositionDetector::detect_opposition(text1, text2, vec1, vec2)?;

        // Check for semantic equivalence (bonus for low-overlap but similar meaning)
        let semantic_bonus = SemanticEquivalenceDetector::detect_semantic_bonus(
            text1,
            text2,
            base_similarity,
        ).unwrap_or(0.0);

        // Apply stronger penalty for high opposition scores
        // Use a graduated curve that increases more sharply at higher opposition levels
        let adjusted_similarity = if opposition_score > 0.5 {
            // Very strong opposition: 50-100% reduction
            let normalized = (opposition_score - 0.5) / 0.5; // 0 to 1
            let reduction_factor = 0.4 + normalized * 0.5; // 40% to 90% reduction
            base_similarity * (1.0 - reduction_factor)
        } else if opposition_score > 0.25 {
            // Moderate opposition: 10-40% reduction
            let normalized = (opposition_score - 0.25) / 0.25; // 0 to 1
            let reduction_factor = 0.1 + normalized * 0.3; // 10% to 40% reduction
            base_similarity * (1.0 - reduction_factor)
        } else {
            base_similarity
        };

        // Apply semantic equivalence bonus if detected
        let final_similarity = if semantic_bonus > 0.0 {
            (adjusted_similarity + semantic_bonus).min(1.0)
        } else {
            adjusted_similarity
        };

        Ok(final_similarity.max(0.0))
    }

    /// Compute cosine similarity between two embedding vectors
    ///
    /// Cosine similarity is calculated as: cos(θ) = (A · B) / (||A|| * ||B||)
    /// where A and B are the embedding vectors
    ///
    /// # Arguments
    /// * `vec1` - First embedding vector
    /// * `vec2` - Second embedding vector
    ///
    /// # Returns
    /// * `Result<f32>` - Cosine similarity score in range [0.0, 1.0]
    ///
    /// # Errors
    /// * Returns error if vectors have different lengths
    /// * Returns error if vectors are empty
    /// * Returns error if vectors have zero magnitude
    pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> Result<f32, AppError> {
        if vec1.is_empty() || vec2.is_empty() {
            return Err(AppError::InvalidInput(
                "Cannot compute similarity for empty vectors".to_string(),
            ));
        }

        if vec1.len() != vec2.len() {
            return Err(AppError::InvalidInput(format!(
                "Vector dimensions must match: {} vs {}",
                vec1.len(),
                vec2.len()
            )));
        }

        let dot = Self::dot_product(vec1, vec2);
        let mag1 = Self::magnitude(vec1);
        let mag2 = Self::magnitude(vec2);

        if mag1 == 0.0 || mag2 == 0.0 {
            return Err(AppError::InvalidInput(
                "Cannot compute similarity for zero-magnitude vectors".to_string(),
            ));
        }

        let similarity = dot / (mag1 * mag2);

        // Clamp to [0.0, 1.0] to handle floating point precision issues
        // Cosine similarity is naturally in [-1, 1], but for sentence embeddings
        // from BERT models, we typically see values in [0, 1]
        Ok(similarity.clamp(0.0, 1.0))
    }

    /// Compute cosine similarity for multiple pairs of vectors in batch
    ///
    /// # Arguments
    /// * `vecs1` - First set of embedding vectors
    /// * `vecs2` - Second set of embedding vectors
    ///
    /// # Returns
    /// * `Result<Vec<f32>>` - Vector of cosine similarity scores
    ///
    /// # Errors
    /// * Returns error if the two sets have different lengths
    /// * Returns error if any pair of vectors has mismatched dimensions
    pub fn cosine_similarity_batch(
        vecs1: &[Vec<f32>],
        vecs2: &[Vec<f32>],
    ) -> Result<Vec<f32>, AppError> {
        if vecs1.len() != vecs2.len() {
            return Err(AppError::InvalidInput(format!(
                "Batch sizes must match: {} vs {}",
                vecs1.len(),
                vecs2.len()
            )));
        }

        let mut similarities = Vec::with_capacity(vecs1.len());

        for (i, (v1, v2)) in vecs1.iter().zip(vecs2.iter()).enumerate() {
            let sim = Self::cosine_similarity(v1, v2).map_err(|e| {
                AppError::InvalidInput(format!("Error computing similarity for pair {}: {}", i, e))
            })?;
            similarities.push(sim);
        }

        Ok(similarities)
    }

    /// Compute dot product of two vectors
    ///
    /// # Arguments
    /// * `vec1` - First vector
    /// * `vec2` - Second vector
    ///
    /// # Returns
    /// * `f32` - Dot product value
    fn dot_product(vec1: &[f32], vec2: &[f32]) -> f32 {
        vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum()
    }

    /// Compute magnitude (L2 norm) of a vector
    ///
    /// # Arguments
    /// * `vec` - The vector
    ///
    /// # Returns
    /// * `f32` - Magnitude value
    fn magnitude(vec: &[f32]) -> f32 {
        vec.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    /// Normalize a vector to unit length (in-place)
    ///
    /// # Arguments
    /// * `vec` - The vector to normalize (modified in-place)
    pub fn normalize(vec: &mut [f32]) {
        let mag = Self::magnitude(vec);
        if mag > 0.0 {
            for x in vec.iter_mut() {
                *x /= mag;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical_vectors() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0];

        let similarity = SimilarityCalculator::cosine_similarity(&vec1, &vec2).unwrap();

        assert!(
            (similarity - 1.0).abs() < 1e-6,
            "Identical vectors should have similarity 1.0, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_orthogonal_vectors() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![0.0, 1.0, 0.0];

        let similarity = SimilarityCalculator::cosine_similarity(&vec1, &vec2).unwrap();

        assert!(
            similarity.abs() < 1e-6,
            "Orthogonal vectors should have similarity 0.0, got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_opposite_vectors() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![-1.0, -2.0, -3.0];

        let similarity = SimilarityCalculator::cosine_similarity(&vec1, &vec2).unwrap();

        // Opposite vectors have similarity -1.0, but we clamp to [0.0, 1.0]
        assert!(
            (0.0..=1.0).contains(&similarity),
            "Similarity should be clamped to [0.0, 1.0], got {}",
            similarity
        );
    }

    #[test]
    fn test_cosine_similarity_empty_vectors() {
        let vec1: Vec<f32> = vec![];
        let vec2: Vec<f32> = vec![];

        let result = SimilarityCalculator::cosine_similarity(&vec1, &vec2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_cosine_similarity_mismatched_dimensions() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0];

        let result = SimilarityCalculator::cosine_similarity(&vec1, &vec2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_cosine_similarity_zero_magnitude() {
        let vec1 = vec![0.0, 0.0, 0.0];
        let vec2 = vec![1.0, 2.0, 3.0];

        let result = SimilarityCalculator::cosine_similarity(&vec1, &vec2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_cosine_similarity_batch_success() {
        let vecs1 = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![1.0, 0.0, 0.0],
        ];
        let vecs2 = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![0.0, 1.0, 0.0],
        ];

        let similarities = SimilarityCalculator::cosine_similarity_batch(&vecs1, &vecs2).unwrap();

        assert_eq!(similarities.len(), 3);
        assert!((similarities[0] - 1.0).abs() < 1e-6);
        assert!((similarities[1] - 1.0).abs() < 1e-6);
        assert!(similarities[2].abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_batch_mismatched_sizes() {
        let vecs1 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let vecs2 = vec![vec![1.0, 2.0, 3.0]];

        let result = SimilarityCalculator::cosine_similarity_batch(&vecs1, &vecs2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_cosine_similarity_batch_empty() {
        let vecs1: Vec<Vec<f32>> = vec![];
        let vecs2: Vec<Vec<f32>> = vec![];

        let similarities = SimilarityCalculator::cosine_similarity_batch(&vecs1, &vecs2).unwrap();

        assert!(similarities.is_empty());
    }

    #[test]
    fn test_dot_product() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];

        let dot = SimilarityCalculator::dot_product(&vec1, &vec2);

        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_magnitude() {
        let vec = vec![3.0, 4.0];

        let mag = SimilarityCalculator::magnitude(&vec);

        assert_eq!(mag, 5.0); // sqrt(3^2 + 4^2) = sqrt(9 + 16) = sqrt(25) = 5
    }

    #[test]
    fn test_normalize() {
        let mut vec = vec![3.0, 4.0];

        SimilarityCalculator::normalize(&mut vec);

        assert!((vec[0] - 0.6).abs() < 1e-6);
        assert!((vec[1] - 0.8).abs() < 1e-6);

        let mag = SimilarityCalculator::magnitude(&vec);
        assert!((mag - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let mut vec = vec![0.0, 0.0, 0.0];

        SimilarityCalculator::normalize(&mut vec);

        // Should remain zero
        assert_eq!(vec, vec![0.0, 0.0, 0.0]);
    }
}

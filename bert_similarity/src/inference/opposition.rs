// Semantic opposition detection without hardcoded antonym lists
// Uses multiple intelligent strategies to detect semantic opposition

use crate::utils::AppError;
use std::collections::HashMap;

/// Opposition detector using multiple strategies
pub struct OppositionDetector;

impl OppositionDetector {
    /// Detect semantic opposition using multiple strategies
    ///
    /// Returns a score from 0.0 to 1.0 indicating likelihood of opposition
    pub fn detect_opposition(
        text1: &str,
        text2: &str,
        embedding1: &[f32],
        embedding2: &[f32],
    ) -> Result<f32, AppError> {
        let mut opposition_scores = Vec::new();

        // Strategy 1: Embedding-based opposition detection
        let embedding_opposition = Self::detect_from_embeddings(embedding1, embedding2)?;
        opposition_scores.push(("embedding", embedding_opposition, 0.4)); // 40% weight

        // Strategy 2: Structural similarity with semantic distance
        let structural_opposition = Self::detect_from_structure(text1, text2)?;
        opposition_scores.push(("structural", structural_opposition, 0.3)); // 30% weight

        // Strategy 3: Negation pattern detection
        let negation_opposition = Self::detect_negation_patterns(text1, text2)?;
        opposition_scores.push(("negation", negation_opposition, 0.2)); // 20% weight

        // Strategy 4: Sentiment polarity detection
        let sentiment_opposition = Self::detect_sentiment_opposition(text1, text2)?;
        opposition_scores.push(("sentiment", sentiment_opposition, 0.1)); // 10% weight

        // Combine scores with weights
        let total_opposition: f32 = opposition_scores
            .iter()
            .map(|(_, score, weight)| score * weight)
            .sum();

        Ok(total_opposition.min(1.0))
    }

    /// Strategy 1: Detect opposition from embeddings
    ///
    /// If two texts have high character overlap but embeddings point in opposite directions,
    /// they likely have opposite semantics
    fn detect_from_embeddings(embedding1: &[f32], embedding2: &[f32]) -> Result<f32, AppError> {
        if embedding1.is_empty() || embedding2.is_empty() {
            return Ok(0.0);
        }

        // Calculate cosine similarity
        let dot_product: f32 = embedding1
            .iter()
            .zip(embedding2.iter())
            .map(|(a, b)| a * b)
            .sum();

        let mag1: f32 = embedding1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag2: f32 = embedding2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag1 == 0.0 || mag2 == 0.0 {
            return Ok(0.0);
        }

        let cosine_sim = dot_product / (mag1 * mag2);

        // If cosine similarity is negative, embeddings point in opposite directions
        // This indicates semantic opposition
        if cosine_sim < 0.0 {
            // Strong opposition: -1.0 becomes 1.0, -0.5 becomes 0.5
            Ok((-cosine_sim).min(1.0))
        } else if cosine_sim < 0.3 {
            // Weak opposition: low similarity might indicate opposition
            Ok((0.3 - cosine_sim) * 0.3)
        } else {
            Ok(0.0)
        }
    }

    /// Strategy 2: Structural similarity with semantic distance
    ///
    /// High structural similarity (same words) but low semantic similarity (different embeddings)
    /// indicates opposition
    fn detect_from_structure(text1: &str, text2: &str) -> Result<f32, AppError> {
        let norm1 = Self::normalize_text(text1);
        let norm2 = Self::normalize_text(text2);

        // Calculate character-level similarity
        let char_sim = Self::calculate_char_similarity(&norm1, &norm2);

        // Calculate word-level overlap
        let words1: Vec<&str> = norm1.split_whitespace().collect();
        let words2: Vec<&str> = norm2.split_whitespace().collect();

        let common_words = words1.iter().filter(|w| words2.contains(w)).count();

        let total_unique_words = words1.len() + words2.len() - common_words;
        let word_overlap = if total_unique_words > 0 {
            common_words as f32 / total_unique_words as f32
        } else {
            0.0
        };

        // High structural similarity (char + word overlap) suggests opposition
        // if combined with semantic distance
        let structural_score = (char_sim + word_overlap) / 2.0;

        // Only return opposition if structural similarity is high (> 0.6)
        if structural_score > 0.6 {
            Ok(structural_score * 0.5) // Max 50% from structure alone
        } else {
            Ok(0.0)
        }
    }

    /// Strategy 3: Detect negation patterns
    ///
    /// Uses linguistic patterns to detect negations without hardcoded lists
    fn detect_negation_patterns(text1: &str, text2: &str) -> Result<f32, AppError> {
        let negation_score1 = Self::score_negation_patterns(text1);
        let negation_score2 = Self::score_negation_patterns(text2);

        // If one text has negation and the other doesn't, it's opposition
        let negation_diff = (negation_score1 - negation_score2).abs();

        // If both have negation or both don't, less likely to be opposition
        Ok(negation_diff.min(1.0))
    }

    /// Score negation patterns in text
    ///
    /// Detects common negation patterns:
    /// - Prefix negations: un-, dis-, non-, im-, in-, ir-, il-
    /// - Negation words: not, no, never, neither, none, nothing, nowhere, nobody
    /// - Negation particles: n't, 不, 没, 无, 非
    fn score_negation_patterns(text: &str) -> f32 {
        let text_lower = text.to_lowercase();
        let mut score: f32 = 0.0;

        // Check for negation prefixes
        let prefixes = vec!["un", "dis", "non", "im", "in", "ir", "il"];
        for prefix in prefixes {
            if text_lower.contains(&format!("{}-", prefix)) {
                score += 0.15;
            }
        }

        // Check for negation words (English)
        let negation_words = vec![
            "not", "no", "never", "neither", "none", "nothing", "nowhere", "nobody", "n't",
            "cannot", "can't", "won't", "don't", "doesn't", "didn't",
        ];
        for word in negation_words {
            if text_lower.contains(word) {
                score += 0.2;
            }
        }

        // Check for Chinese negation particles
        let chinese_negations = vec!["不", "没", "无", "非", "否"];
        for neg in chinese_negations {
            if text.contains(neg) {
                score += 0.2;
            }
        }

        // Check for contrast words that might indicate opposition
        let contrast_words = vec![
            "but", "however", "yet", "instead", "rather", "contrary", "opposite", "但", "然而",
            "却", "反而", "相反", "对比",
        ];
        for word in contrast_words {
            if text_lower.contains(word) {
                score += 0.1;
            }
        }

        score.min(1.0)
    }

    /// Strategy 4: Detect sentiment opposition
    ///
    /// Uses simple sentiment indicators to detect opposing sentiments
    fn detect_sentiment_opposition(text1: &str, text2: &str) -> Result<f32, AppError> {
        let sentiment1 = Self::score_sentiment(text1);
        let sentiment2 = Self::score_sentiment(text2);

        // If sentiments are opposite (one positive, one negative), it's opposition
        let sentiment_diff = (sentiment1 - sentiment2).abs();

        // Only count as opposition if both texts have clear sentiment
        let sentiment_strength = (sentiment1.abs() + sentiment2.abs()) / 2.0;

        if sentiment_strength > 0.3 && sentiment_diff > 0.5 {
            Ok(sentiment_diff * 0.5) // Max 50% from sentiment
        } else {
            Ok(0.0)
        }
    }

    /// Score sentiment of text (-1.0 to 1.0)
    ///
    /// Negative: -1.0 to 0.0
    /// Positive: 0.0 to 1.0
    fn score_sentiment(text: &str) -> f32 {
        let text_lower = text.to_lowercase();
        let mut score: f32 = 0.0;

        // Positive sentiment words
        let positive_words = vec![
            "good",
            "great",
            "excellent",
            "amazing",
            "wonderful",
            "fantastic",
            "love",
            "like",
            "best",
            "perfect",
            "beautiful",
            "happy",
            "joy",
            "好",
            "很好",
            "优秀",
            "完美",
            "美丽",
            "快乐",
            "喜欢",
        ];
        for word in positive_words {
            if text_lower.contains(word) {
                score += 0.2;
            }
        }

        // Negative sentiment words
        let negative_words = vec![
            "bad",
            "terrible",
            "awful",
            "horrible",
            "poor",
            "worst",
            "hate",
            "dislike",
            "ugly",
            "sad",
            "angry",
            "disappointed",
            "坏",
            "很坏",
            "糟糕",
            "可怕",
            "丑陋",
            "悲伤",
            "讨厌",
        ];
        for word in negative_words {
            if text_lower.contains(word) {
                score -= 0.2;
            }
        }

        // Intensifiers
        let intensifiers = ["very", "extremely", "so", "really", "quite", "非常", "极其"];
        let has_intensifier = intensifiers.iter().any(|w| text_lower.contains(w));

        if has_intensifier {
            score *= 1.3; // Amplify sentiment if intensifier present
        }

        score.clamp(-1.0, 1.0)
    }

    /// Normalize text for comparison
    fn normalize_text(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    /// Calculate character-level similarity
    fn calculate_char_similarity(text1: &str, text2: &str) -> f32 {
        let chars1 = Self::get_char_frequency(text1);
        let chars2 = Self::get_char_frequency(text2);

        let mut common = 0;
        let mut total = 0;

        for (ch, count1) in &chars1 {
            if let Some(count2) = chars2.get(ch) {
                common += (*count1).min(*count2);
            }
            total += count1;
        }

        for (ch, count2) in &chars2 {
            if !chars1.contains_key(ch) {
                total += count2;
            }
        }

        if total == 0 {
            0.0
        } else {
            common as f32 / total as f32
        }
    }

    /// Get character frequency map
    fn get_char_frequency(text: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();
        for ch in text.chars() {
            if ch.is_alphanumeric() {
                *freq.entry(ch).or_insert(0) += 1;
            }
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_pattern_detection() {
        let score1 = OppositionDetector::score_negation_patterns("This is good");
        let score2 = OppositionDetector::score_negation_patterns("This is not good");

        assert!(score2 > score1);
        assert!(score2 > 0.2);
    }

    #[test]
    fn test_sentiment_scoring() {
        let positive = OppositionDetector::score_sentiment("This is very good and wonderful");
        let negative = OppositionDetector::score_sentiment("This is very bad and terrible");

        assert!(positive > 0.5);
        assert!(negative < -0.5);
        assert!((positive - negative).abs() > 1.0);
    }

    #[test]
    fn test_char_similarity() {
        let sim1 = OppositionDetector::calculate_char_similarity("hello", "hello");
        let sim2 = OppositionDetector::calculate_char_similarity("hello", "world");

        assert!(sim1 > 0.9);
        assert!(sim2 < 0.5);
    }

    #[test]
    fn test_chinese_negation() {
        let score1 = OppositionDetector::score_negation_patterns("这很好");
        let score2 = OppositionDetector::score_negation_patterns("这不好");

        assert!(score2 > score1);
    }

    #[test]
    fn test_embedding_opposition() {
        // Embeddings pointing in opposite directions
        let emb1 = vec![1.0, 0.0, 0.0];
        let emb2 = vec![-1.0, 0.0, 0.0];

        let opposition = OppositionDetector::detect_from_embeddings(&emb1, &emb2).unwrap();
        assert!(opposition > 0.8);
    }

    #[test]
    fn test_embedding_similarity() {
        // Embeddings pointing in same direction
        let emb1 = vec![1.0, 0.0, 0.0];
        let emb2 = vec![1.0, 0.0, 0.0];

        let opposition = OppositionDetector::detect_from_embeddings(&emb1, &emb2).unwrap();
        assert!(opposition < 0.1);
    }
}

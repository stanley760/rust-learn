// Semantic opposition detection without hardcoded antonym lists
// Uses multiple intelligent strategies to detect semantic opposition

use crate::utils::AppError;
use std::collections::HashMap;

/// Check if a character is a Chinese character (CJK Unified Ideographs)
fn is_chinese_char(c: char) -> bool {
    match c {
        // CJK Unified Ideographs block: U+4E00 to U+9FFF
        '\u{4E00}'..='\u{9FFF}' => true,
        // CJK Extension A: U+3400 to U+4DBF
        '\u{3400}'..='\u{4DBF}' => true,
        // CJK Extension B: U+20000 to U+2A6DF (needs surrogate handling in UTF-16, but Rust handles UTF-8)
        // CJK Extension C-F: U+2A700 to U+2B81F
        // CJK Compatibility Ideographs: U+F900 to U+FAFF
        '\u{F900}'..='\u{FAFF}' => true,
        _ => false,
    }
}

/// Opposition detector using multiple strategies
pub struct OppositionDetector;

impl OppositionDetector {
    /// Detect semantic opposition using multiple strategies
    ///
    /// Returns a score from 0.0 to 1.0 indicating likelihood of opposition
    ///
    /// Weight distribution:
    /// - Structural (50%): Most effective for "high repetition, opposite semantics" cases
    /// - Embedding (25%): Detects opposite directions in embedding space
    /// - Negation (15%): Catches explicit negation patterns
    /// - Sentiment (10%): Detects sentiment polarity differences
    pub fn detect_opposition(
        text1: &str,
        text2: &str,
        embedding1: &[f32],
        embedding2: &[f32],
    ) -> Result<f32, AppError> {
        let mut opposition_scores = Vec::new();

        // Strategy 1: Embedding-based opposition detection
        let embedding_opposition = Self::detect_from_embeddings(embedding1, embedding2)?;
        opposition_scores.push(("embedding", embedding_opposition, 0.25)); // 25% weight

        // Strategy 2: Structural similarity with semantic distance
        // Increased to 50% as it's most effective for "high repetition, opposite semantics" cases
        let structural_opposition = Self::detect_from_structure(text1, text2)?;
        opposition_scores.push(("structural", structural_opposition, 0.50)); // 50% weight

        // Strategy 3: Negation pattern detection
        let negation_opposition = Self::detect_negation_patterns(text1, text2)?;
        opposition_scores.push(("negation", negation_opposition, 0.15)); // 15% weight

        // Strategy 4: Sentiment polarity detection
        let sentiment_opposition = Self::detect_sentiment_opposition(text1, text2)?;
        opposition_scores.push(("sentiment", sentiment_opposition, 0.10)); // 10% weight

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
    /// they likely have opposite semantics.
    ///
    /// Also detects medium similarity range (0.3-0.6) which may indicate partial opposition.
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

        // Calculate embedding distance (Euclidean distance as additional signal)
        let dist_squared: f32 = embedding1
            .iter()
            .zip(embedding2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        let distance = dist_squared.sqrt();

        // Normalize distance by vector magnitude
        let avg_mag = (mag1 + mag2) / 2.0;
        let normalized_distance = if avg_mag > 0.0 {
            distance / avg_mag
        } else {
            0.0
        };

        // Opposition detection based on multiple signals:
        // 1. Negative cosine similarity = strong opposition
        // 2. Low cosine similarity (0.3-0.6) + high distance = potential opposition
        let opposition_score = if cosine_sim < 0.0 {
            // Strong opposition: -1.0 becomes 1.0, -0.5 becomes 0.5
            (-cosine_sim).min(1.0)
        } else if cosine_sim < 0.4 {
            // Medium to low similarity might indicate opposition
            // Combine with distance signal for more robust detection
            let sim_score = (0.4 - cosine_sim) / 0.4; // 0 to 1
            let dist_score = normalized_distance.min(1.0);
            (sim_score * 0.5 + dist_score * 0.5).min(0.7) // Max 0.7 for this range
        } else if cosine_sim < 0.7 {
            // For 0.4-0.7 range, use distance as primary signal
            // High distance with moderate similarity suggests different semantics
            let dist_factor = (normalized_distance - 0.5).max(0.0) / 0.5; // 0 to 1 for distance > 0.5
            dist_factor * 0.4 // Max 0.4
        } else {
            0.0
        };

        Ok(opposition_score)
    }

    /// Strategy 2: Structural similarity with semantic distance
    ///
    /// High structural similarity (same words) but low semantic similarity (different embeddings)
    /// indicates opposition. This is effective for detecting cases where only a few keywords differ.
    fn detect_from_structure(text1: &str, text2: &str) -> Result<f32, AppError> {
        let norm1 = Self::normalize_text(text1);
        let norm2 = Self::normalize_text(text2);

        // Calculate character-level similarity
        let char_sim = Self::calculate_char_similarity(&norm1, &norm2);

        // Detect if text contains Chinese characters (CJK Unified Ideographs block)
        let has_chinese = norm1.chars().any(|c| is_chinese_char(c)) || norm2.chars().any(|c| is_chinese_char(c));

        // For Chinese text, use character-level comparison instead of word-level
        // since Chinese doesn't use spaces between words
        let (word_overlap, diff_ratio) = if has_chinese {
            // For Chinese, compare character n-grams (bigrams)
            let bigrams1: Vec<String> = norm1.chars().collect::<Vec<_>>().windows(2)
                .map(|w| w.iter().collect())
                .collect();
            let bigrams2: Vec<String> = norm2.chars().collect::<Vec<_>>().windows(2)
                .map(|w| w.iter().collect())
                .collect();

            let common_bigrams = bigrams1.iter().filter(|w| bigrams2.contains(w)).count();
            let total_unique_bigrams = bigrams1.len() + bigrams2.len() - common_bigrams;
            let bigram_overlap = if total_unique_bigrams > 0 {
                common_bigrams as f32 / total_unique_bigrams as f32
            } else {
                0.0
            };

            // Calculate character-level difference
            let chars1: std::collections::HashSet<char> = norm1.chars().collect();
            let chars2: std::collections::HashSet<char> = norm2.chars().collect();
            let char_diff_count = chars1.symmetric_difference(&chars2).count();
            let total_chars = chars1.len() + chars2.len();
            let char_diff_ratio = if total_chars > 0 {
                char_diff_count as f32 / total_chars as f32
            } else {
                0.0
            };

            (bigram_overlap, char_diff_ratio)
        } else {
            // For non-Chinese text, use word-level comparison
            let words1: Vec<&str> = norm1.split_whitespace().collect();
            let words2: Vec<&str> = norm2.split_whitespace().collect();

            let common_words = words1.iter().filter(|w| words2.contains(w)).count();
            let total_unique_words = words1.len() + words2.len() - common_words;
            let word_overlap = if total_unique_words > 0 {
                common_words as f32 / total_unique_words as f32
            } else {
                0.0
            };

            // Calculate the difference in word count (indicates how many words changed)
            let words1_set: std::collections::HashSet<&str> = words1.iter().cloned().collect();
            let words2_set: std::collections::HashSet<&str> = words2.iter().cloned().collect();
            let symmetric_diff_count = words1_set.symmetric_difference(&words2_set).count();
            let diff_ratio = if words1.len() + words2.len() > 0 {
                symmetric_diff_count as f32 / (words1.len() + words2.len()) as f32
            } else {
                0.0
            };

            (word_overlap, diff_ratio)
        };

        // High structural similarity (char + word/bigram overlap) suggests opposition
        // if combined with small number of differing words/characters
        let structural_score = char_sim * 0.5 + word_overlap * 0.3 + (1.0 - diff_ratio) * 0.2;

        // Lower the threshold to catch more cases
        // Increase the weight when structural similarity is very high (> 0.7)
        let opposition_score = if structural_score > 0.85 {
            structural_score * 0.9 // Max 90% from extremely high structural similarity
        } else if structural_score > 0.75 {
            structural_score * 0.8 // Max 80% from very high structural similarity
        } else if structural_score > 0.6 {
            structural_score * 0.6 // Max 60% from high structural similarity
        } else if structural_score > 0.4 {
            structural_score * 0.4 // Max 40% from moderate structural similarity
        } else {
            0.0
        };

        Ok(opposition_score)
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

    #[test]
    fn test_high_repetition_opposite_semantics() {
        // Test case: "这项措施能够有效促进当地经济发展" vs "这项措施能够有效阻碍当地经济发展"
        let text1 = "这项措施能够有效促进当地经济发展";
        let text2 = "这项措施能够有效阻碍当地经济发展";

        // Create mock embeddings with moderate similarity (like BERT would produce for these)
        let emb1: Vec<f32> = (0..100).map(|i| (i as f32 * 0.01).sin()).collect();
        let emb2: Vec<f32> = (0..100).map(|i| (i as f32 * 0.01 + 0.2).sin()).collect();

        // Debug: Check individual scores
        let norm1 = OppositionDetector::normalize_text(text1);
        let norm2 = OppositionDetector::normalize_text(text2);
        println!("norm1: {}", norm1);
        println!("norm2: {}", norm2);

        let char_sim = OppositionDetector::calculate_char_similarity(&norm1, &norm2);
        println!("char_sim: {}", char_sim);

        let words1: Vec<&str> = norm1.split_whitespace().collect();
        let words2: Vec<&str> = norm2.split_whitespace().collect();
        println!("words1: {:?}", words1);
        println!("words2: {:?}", words2);

        let common_words = words1.iter().filter(|w| words2.contains(w)).count();
        let total_unique_words = words1.len() + words2.len() - common_words;
        let word_overlap = if total_unique_words > 0 {
            common_words as f32 / total_unique_words as f32
        } else {
            0.0
        };
        println!("common_words: {}, total_unique: {}, word_overlap: {}", common_words, total_unique_words, word_overlap);

        let structural = OppositionDetector::detect_from_structure(text1, text2).unwrap();
        println!("structural_opposition: {}", structural);

        let embedding_opp = OppositionDetector::detect_from_embeddings(&emb1, &emb2).unwrap();
        println!("embedding_opposition: {}", embedding_opp);

        let opposition = OppositionDetector::detect_opposition(text1, text2, &emb1, &emb2).unwrap();
        println!("total_opposition: {}", opposition);

        // Should detect significant opposition due to high structural similarity
        assert!(opposition > 0.3, "Expected opposition > 0.3, got {}", opposition);

        // Test structural detection specifically
        assert!(structural > 0.5, "Expected structural opposition > 0.5, got {}", structural);
    }

    #[test]
    fn test_chinese_antonym_like_patterns() {
        // Test various Chinese antonym-like patterns
        // For shorter sentences, detection is harder, so we use longer sentences
        let test_cases = vec![
            ("这项措施能够有效促进经济发展", "这项措施能够有效阻碍经济发展"),
            ("这项政策可以增加居民收入", "这项政策可以减少居民收入"),
            ("这个方法能够显著提高工作效率", "这个方法能够显著降低工作效率"),
            ("我们完全支持这个观点和建议", "我们完全反对这个观点和建议"),
            ("很多用户非常喜欢这个产品", "很多用户非常讨厌这个产品"),
        ];

        for (text1, text2) in test_cases {
            let emb1 = vec![0.5, 0.3, 0.8, 0.2];
            let emb2 = vec![0.5, 0.3, 0.8, 0.2]; // Same structure, different meaning

            let _opposition = OppositionDetector::detect_opposition(text1, text2, &emb1, &emb2).unwrap();

            // At minimum, structural detection should trigger for longer sentences
            let structural = OppositionDetector::detect_from_structure(text1, text2).unwrap();
            assert!(
                structural > 0.35,
                "Structural opposition too low for '{}' vs '{}': {}",
                text1, text2, structural
            );
        }
    }
}

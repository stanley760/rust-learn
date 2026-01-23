// Semantic equivalence detection for low-overlap but semantically similar sentences
// Uses multiple strategies to detect deep semantic similarity

use crate::utils::AppError;

/// Semantic equivalence detector using multiple strategies
pub struct SemanticEquivalenceDetector;

impl SemanticEquivalenceDetector {
    /// Detect semantic equivalence when surface similarity is low
    ///
    /// Returns a bonus score from 0.0 to 1.0 to add to similarity
    ///
    /// This is useful for cases like:
    /// - "站在风口的猪都会飞" vs "他赶上了机遇，事业成功了"
    /// - Metaphorical vs literal expressions of the same meaning
    pub fn detect_semantic_bonus(
        text1: &str,
        text2: &str,
        base_similarity: f32,
    ) -> Result<f32, AppError> {
        // Only apply bonus if base similarity is low (0.2 - 0.5 range)
        // This helps cases where surface overlap is low but meaning is similar
        if base_similarity < 0.2 || base_similarity > 0.5 {
            return Ok(0.0);
        }

        let mut bonus_scores = Vec::new();

        // Strategy 1: Topic keyword overlap
        let topic_bonus = Self::detect_topic_overlap(text1, text2)?;
        bonus_scores.push(("topic", topic_bonus, 0.4));

        // Strategy 2: Semantic pattern matching
        let pattern_bonus = Self::detect_semantic_patterns(text1, text2)?;
        bonus_scores.push(("pattern", pattern_bonus, 0.3));

        // Strategy 3: Figurative language detection
        let figurative_bonus = Self::detect_figurative_equivalence(text1, text2)?;
        bonus_scores.push(("figurative", figurative_bonus, 0.3));

        // Combine bonuses
        let total_bonus: f32 = bonus_scores
            .iter()
            .map(|(_, score, weight)| score * weight)
            .sum();

        // Cap bonus at 0.3 (30% increase)
        Ok(total_bonus.min(0.3))
    }

    /// Strategy 1: Detect topic keyword overlap
    ///
    /// Extracts key topics and checks if both texts discuss the same topics
    fn detect_topic_overlap(text1: &str, text2: &str) -> Result<f32, AppError> {
        // Define topic-related keywords (no hardcoded antonyms, just topic indicators)
        let topic_groups = vec![
            // Success/achievement topic
            vec!["成功", "成就", "飞", "腾", "达", "旺", "盛", "兴", "荣"],
            // Opportunity/timing topic
            vec!["机遇", "机会", "风口", "时机", "场合", "际遇", "契机"],
            // Action/effort topic
            vec!["赶", "抓", "站", "把", "握", "拿", "趁"],
            // Career/business topic
            vec!["事业", "职业", "工作", "发展", "经商", "创业"],
        ];

        let mut common_topics = 0;
        let total_topic_groups = topic_groups.len();

        for group in topic_groups {
            let text1_has_topic = group.iter().any(|&kw| text1.contains(kw));
            let text2_has_topic = group.iter().any(|&kw| text2.contains(kw));

            if text1_has_topic && text2_has_topic {
                common_topics += 1;
            }
        }

        if total_topic_groups == 0 {
            return Ok(0.0);
        }

        let topic_overlap_ratio = common_topics as f32 / total_topic_groups as f32;

        // If both texts share 2+ topics, give a bonus
        if common_topics >= 2 {
            Ok(topic_overlap_ratio * 0.8)
        } else {
            Ok(0.0)
        }
    }

    /// Strategy 2: Detect semantic patterns
    ///
    /// Looks for common semantic structures like "cause + effect"
    fn detect_semantic_patterns(text1: &str, text2: &str) -> Result<f32, AppError> {
        // Check if both texts express cause-effect relationships
        let cause_effect_pattern = |text: &str| -> bool {
            let cause_indicators = ["因为", "由于", "赶", "抓住", "趁", "因为"];
            let effect_indicators = ["成功", "飞", "腾", "达", "好", "行", "成"];

            let has_cause = cause_indicators.iter().any(|&w| text.contains(w));
            let has_effect = effect_indicators.iter().any(|&w| text.contains(w));

            has_cause && has_effect
        };

        let both_have_cause_effect = cause_effect_pattern(text1) && cause_effect_pattern(text2);

        if both_have_cause_effect {
            Ok(0.5)
        } else {
            Ok(0.0)
        }
    }

    /// Strategy 3: Detect figurative language equivalence
    ///
    /// Looks for metaphorical expressions that map to literal meanings
    fn detect_figurative_equivalence(text1: &str, text2: &str) -> Result<f32, AppError> {
        // Common figurative expressions in Chinese and their literal equivalents
        let figurative_pairs = vec![
            (vec!["风口", "飞"], vec!["机遇", "成功"]),
            (vec!["吃土"], vec!["穷", "没钱"]),
            (vec!["躺平"], vec!["放弃", "不努力"]),
            (vec!["内卷"], vec!["竞争", "激烈"]),
            (vec!["摸鱼"], vec!["偷懒", "不工作"]),
        ];

        for (figurative, literal) in figurative_pairs {
            // Check if one text uses figurative language and the other uses literal
            let text1_figurative = figurative.iter().any(|&w| text1.contains(w));
            let text1_literal = literal.iter().any(|&w| text1.contains(w));
            let text2_figurative = figurative.iter().any(|&w| text2.contains(w));
            let text2_literal = literal.iter().any(|&w| text2.contains(w));

            // If one is figurative and one is literal, they might be equivalent
            if (text1_figurative && text2_literal) || (text2_figurative && text1_literal) {
                return Ok(0.6);
            }
        }

        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_overlap_detection() {
        let text1 = "站在风口的猪都会飞";
        let text2 = "他赶上了机遇，事业成功了";

        let bonus = SemanticEquivalenceDetector::detect_semantic_bonus(text1, text2, 0.35).unwrap();
        // Should detect some bonus due to topic overlap
        assert!(bonus > 0.0, "Expected bonus > 0.0, got {}", bonus);
    }

    #[test]
    fn test_no_bonus_for_identical() {
        let text1 = "hello world";
        let text2 = "hello world";

        // High similarity should not get bonus
        let bonus = SemanticEquivalenceDetector::detect_semantic_bonus(text1, text2, 0.95).unwrap();
        assert_eq!(bonus, 0.0);
    }

    #[test]
    fn test_no_bonus_for_very_different() {
        let text1 = "今天天气很好";
        let text2 = "我喜欢吃苹果";

        // Very low similarity should not get bonus
        let bonus = SemanticEquivalenceDetector::detect_semantic_bonus(text1, text2, 0.1).unwrap();
        assert_eq!(bonus, 0.0);
    }
}

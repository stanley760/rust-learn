// Training data validation module

use crate::api::models::TrainingPair;
use crate::utils::error::AppError;

/// Validation error details for training data
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub index: usize,
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation error at index {}: {} - {}",
            self.index, self.field, self.message
        )
    }
}

/// Result of training data validation
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }

    pub fn error_summary(&self) -> String {
        if self.is_valid {
            return "No validation errors".to_string();
        }

        let mut summary = format!("Found {} validation error(s):\n", self.errors.len());
        for error in &self.errors {
            summary.push_str(&format!("  - {}\n", error));
        }
        summary
    }
}

/// Validates a single training pair
///
/// Requirements: 5.1, 5.2, 5.3
///
/// Checks:
/// - Both sentences are non-empty
/// - Similarity label is in range [0.0, 1.0]
pub fn validate_training_pair(pair: &TrainingPair, index: usize) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Validate sentence1 is non-empty
    if pair.sentence1.trim().is_empty() {
        errors.push(ValidationError {
            index,
            field: "sentence1".to_string(),
            message: "Sentence1 cannot be empty".to_string(),
        });
    }

    // Validate sentence2 is non-empty
    if pair.sentence2.trim().is_empty() {
        errors.push(ValidationError {
            index,
            field: "sentence2".to_string(),
            message: "Sentence2 cannot be empty".to_string(),
        });
    }

    // Validate similarity label is in range [0.0, 1.0]
    if pair.similarity < 0.0 || pair.similarity > 1.0 {
        errors.push(ValidationError {
            index,
            field: "similarity".to_string(),
            message: format!(
                "Similarity label {} is out of range [0.0, 1.0]",
                pair.similarity
            ),
        });
    }

    // Check for NaN or infinite values
    if pair.similarity.is_nan() {
        errors.push(ValidationError {
            index,
            field: "similarity".to_string(),
            message: "Similarity label cannot be NaN".to_string(),
        });
    }

    if pair.similarity.is_infinite() {
        errors.push(ValidationError {
            index,
            field: "similarity".to_string(),
            message: "Similarity label cannot be infinite".to_string(),
        });
    }

    errors
}

/// Validates a collection of training pairs
///
/// Requirements: 5.1, 5.2, 5.3, 5.5
///
/// Returns a ValidationResult with all validation errors found
pub fn validate_training_data(training_data: &[TrainingPair]) -> Result<ValidationResult, AppError> {
    // Check if training data is empty
    if training_data.is_empty() {
        return Err(AppError::InvalidInput(
            "Training data cannot be empty".to_string(),
        ));
    }

    let mut all_errors = Vec::new();

    // Validate each training pair
    for (index, pair) in training_data.iter().enumerate() {
        let errors = validate_training_pair(pair, index);
        all_errors.extend(errors);
    }

    if all_errors.is_empty() {
        Ok(ValidationResult::valid())
    } else {
        Ok(ValidationResult::invalid(all_errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_training_pair() {
        let pair = TrainingPair {
            sentence1: "This is a test".to_string(),
            sentence2: "This is another test".to_string(),
            similarity: 0.8,
        };

        let errors = validate_training_pair(&pair, 0);
        assert!(errors.is_empty(), "Valid pair should have no errors");
    }

    #[test]
    fn test_validate_empty_sentence1() {
        let pair = TrainingPair {
            sentence1: "".to_string(),
            sentence2: "This is a test".to_string(),
            similarity: 0.8,
        };

        let errors = validate_training_pair(&pair, 0);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "sentence1");
        assert!(errors[0].message.contains("empty"));
    }

    #[test]
    fn test_validate_empty_sentence2() {
        let pair = TrainingPair {
            sentence1: "This is a test".to_string(),
            sentence2: "   ".to_string(), // whitespace only
            similarity: 0.8,
        };

        let errors = validate_training_pair(&pair, 0);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "sentence2");
        assert!(errors[0].message.contains("empty"));
    }

    #[test]
    fn test_validate_similarity_out_of_range_low() {
        let pair = TrainingPair {
            sentence1: "Test 1".to_string(),
            sentence2: "Test 2".to_string(),
            similarity: -0.5,
        };

        let errors = validate_training_pair(&pair, 0);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "similarity");
        assert!(errors[0].message.contains("out of range"));
    }

    #[test]
    fn test_validate_similarity_out_of_range_high() {
        let pair = TrainingPair {
            sentence1: "Test 1".to_string(),
            sentence2: "Test 2".to_string(),
            similarity: 1.5,
        };

        let errors = validate_training_pair(&pair, 0);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "similarity");
        assert!(errors[0].message.contains("out of range"));
    }

    #[test]
    fn test_validate_similarity_boundary_values() {
        // Test 0.0
        let pair1 = TrainingPair {
            sentence1: "Test 1".to_string(),
            sentence2: "Test 2".to_string(),
            similarity: 0.0,
        };
        assert!(validate_training_pair(&pair1, 0).is_empty());

        // Test 1.0
        let pair2 = TrainingPair {
            sentence1: "Test 1".to_string(),
            sentence2: "Test 2".to_string(),
            similarity: 1.0,
        };
        assert!(validate_training_pair(&pair2, 0).is_empty());
    }

    #[test]
    fn test_validate_multiple_errors() {
        let pair = TrainingPair {
            sentence1: "".to_string(),
            sentence2: "".to_string(),
            similarity: 2.0,
        };

        let errors = validate_training_pair(&pair, 0);
        assert_eq!(errors.len(), 3); // empty sentence1, empty sentence2, invalid similarity
    }

    #[test]
    fn test_validate_training_data_valid() {
        let data = vec![
            TrainingPair {
                sentence1: "Test 1".to_string(),
                sentence2: "Test 2".to_string(),
                similarity: 0.5,
            },
            TrainingPair {
                sentence1: "Test 3".to_string(),
                sentence2: "Test 4".to_string(),
                similarity: 0.9,
            },
        ];

        let result = validate_training_data(&data).unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_training_data_empty() {
        let data: Vec<TrainingPair> = vec![];
        let result = validate_training_data(&data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_validate_training_data_with_errors() {
        let data = vec![
            TrainingPair {
                sentence1: "Valid".to_string(),
                sentence2: "Valid".to_string(),
                similarity: 0.5,
            },
            TrainingPair {
                sentence1: "".to_string(),
                sentence2: "Invalid".to_string(),
                similarity: 0.8,
            },
            TrainingPair {
                sentence1: "Invalid".to_string(),
                sentence2: "Invalid".to_string(),
                similarity: 1.5,
            },
        ];

        let result = validate_training_data(&data).unwrap();
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 2); // One error at index 1, one at index 2
    }

    #[test]
    fn test_validation_result_error_summary() {
        let errors = vec![
            ValidationError {
                index: 0,
                field: "sentence1".to_string(),
                message: "Cannot be empty".to_string(),
            },
            ValidationError {
                index: 1,
                field: "similarity".to_string(),
                message: "Out of range".to_string(),
            },
        ];

        let result = ValidationResult::invalid(errors);
        let summary = result.error_summary();
        assert!(summary.contains("2 validation error"));
        assert!(summary.contains("index 0"));
        assert!(summary.contains("index 1"));
    }
}

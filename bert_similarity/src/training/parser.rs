// Training data parser module

use crate::api::models::TrainingPair;
use crate::utils::error::AppError;
use serde::Deserialize;
use std::io::Read;

/// Training dataset structure
#[derive(Debug, Clone)]
pub struct TrainingDataset {
    pub pairs: Vec<TrainingPair>,
}

impl TrainingDataset {
    pub fn new(pairs: Vec<TrainingPair>) -> Self {
        Self { pairs }
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

/// Data format for training data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    Csv,
}

impl DataFormat {
    /// Detect format from file extension
    pub fn from_extension(path: &str) -> Option<Self> {
        let path_lower = path.to_lowercase();
        if path_lower.ends_with(".json") {
            Some(DataFormat::Json)
        } else if path_lower.ends_with(".csv") {
            Some(DataFormat::Csv)
        } else {
            None
        }
    }
}

/// JSON wrapper for training data
#[derive(Debug, Deserialize)]
struct JsonTrainingData {
    training_data: Vec<TrainingPair>,
}

/// CSV record structure
#[derive(Debug, Deserialize)]
struct CsvRecord {
    sentence1: String,
    sentence2: String,
    similarity: f32,
}

/// Parse training data from JSON format
///
/// Requirements: 5.4
///
/// Expected JSON format:
/// ```json
/// {
///   "training_data": [
///     {
///       "sentence1": "Example 1",
///       "sentence2": "Example 2",
///       "similarity": 0.95
///     }
///   ]
/// }
/// ```
pub fn parse_json(data: &str) -> Result<TrainingDataset, AppError> {
    // Try to parse as wrapped format first
    match serde_json::from_str::<JsonTrainingData>(data) {
        Ok(wrapped) => Ok(TrainingDataset::new(wrapped.training_data)),
        Err(_) => {
            // Try to parse as direct array
            match serde_json::from_str::<Vec<TrainingPair>>(data) {
                Ok(pairs) => Ok(TrainingDataset::new(pairs)),
                Err(e) => Err(AppError::InvalidInput(format!(
                    "Failed to parse JSON training data: {}",
                    e
                ))),
            }
        }
    }
}

/// Parse training data from CSV format
///
/// Requirements: 5.4
///
/// Expected CSV format:
/// ```csv
/// sentence1,sentence2,similarity
/// Example 1,Example 2,0.95
/// ```
pub fn parse_csv(data: &str) -> Result<TrainingDataset, AppError> {
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut pairs = Vec::new();

    for (index, result) in reader.deserialize().enumerate() {
        match result {
            Ok(record) => {
                let csv_record: CsvRecord = record;
                pairs.push(TrainingPair {
                    sentence1: csv_record.sentence1,
                    sentence2: csv_record.sentence2,
                    similarity: csv_record.similarity,
                });
            }
            Err(e) => {
                return Err(AppError::InvalidInput(format!(
                    "Failed to parse CSV record at line {}: {}",
                    index + 2, // +2 because index is 0-based and we skip header
                    e
                )));
            }
        }
    }

    if pairs.is_empty() {
        return Err(AppError::InvalidInput(
            "CSV file contains no training data".to_string(),
        ));
    }

    Ok(TrainingDataset::new(pairs))
}

/// Parse training data from a reader with automatic format detection
///
/// Requirements: 5.4
pub fn parse_training_data<R: Read>(
    mut reader: R,
    format: DataFormat,
) -> Result<TrainingDataset, AppError> {
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .map_err(|e| AppError::IoError(e))?;

    match format {
        DataFormat::Json => parse_json(&data),
        DataFormat::Csv => parse_csv(&data),
    }
}

/// Parse training data from a string with specified format
pub fn parse_from_string(data: &str, format: DataFormat) -> Result<TrainingDataset, AppError> {
    match format {
        DataFormat::Json => parse_json(data),
        DataFormat::Csv => parse_csv(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_wrapped_format() {
        let json_data = r#"
        {
            "training_data": [
                {
                    "sentence1": "This is a test",
                    "sentence2": "This is another test",
                    "similarity": 0.95
                },
                {
                    "sentence1": "Hello world",
                    "sentence2": "Goodbye world",
                    "similarity": 0.3
                }
            ]
        }
        "#;

        let dataset = parse_json(json_data).unwrap();
        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.pairs[0].sentence1, "This is a test");
        assert_eq!(dataset.pairs[0].similarity, 0.95);
        assert_eq!(dataset.pairs[1].sentence1, "Hello world");
        assert_eq!(dataset.pairs[1].similarity, 0.3);
    }

    #[test]
    fn test_parse_json_direct_array() {
        let json_data = r#"
        [
            {
                "sentence1": "Test 1",
                "sentence2": "Test 2",
                "similarity": 0.8
            }
        ]
        "#;

        let dataset = parse_json(json_data).unwrap();
        assert_eq!(dataset.len(), 1);
        assert_eq!(dataset.pairs[0].sentence1, "Test 1");
        assert_eq!(dataset.pairs[0].similarity, 0.8);
    }

    #[test]
    fn test_parse_json_invalid() {
        let json_data = r#"{ "invalid": "format" }"#;
        let result = parse_json(json_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_csv_valid() {
        let csv_data = "sentence1,sentence2,similarity\n\
                       This is a test,This is another test,0.95\n\
                       Hello world,Goodbye world,0.3\n";

        let dataset = parse_csv(csv_data).unwrap();
        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.pairs[0].sentence1, "This is a test");
        assert_eq!(dataset.pairs[0].similarity, 0.95);
        assert_eq!(dataset.pairs[1].sentence1, "Hello world");
        assert_eq!(dataset.pairs[1].similarity, 0.3);
    }

    #[test]
    fn test_parse_csv_empty() {
        let csv_data = "sentence1,sentence2,similarity\n";
        let result = parse_csv(csv_data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));
    }

    #[test]
    fn test_parse_csv_invalid_format() {
        let csv_data = "sentence1,sentence2,similarity\n\
                       Test 1,Test 2,invalid_number\n";
        let result = parse_csv(csv_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_csv_missing_column() {
        let csv_data = "sentence1,sentence2\n\
                       Test 1,Test 2\n";
        let result = parse_csv(csv_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_format_from_extension() {
        assert_eq!(
            DataFormat::from_extension("data.json"),
            Some(DataFormat::Json)
        );
        assert_eq!(
            DataFormat::from_extension("data.JSON"),
            Some(DataFormat::Json)
        );
        assert_eq!(
            DataFormat::from_extension("data.csv"),
            Some(DataFormat::Csv)
        );
        assert_eq!(
            DataFormat::from_extension("data.CSV"),
            Some(DataFormat::Csv)
        );
        assert_eq!(DataFormat::from_extension("data.txt"), None);
    }

    #[test]
    fn test_parse_from_string_json() {
        let json_data = r#"[{"sentence1": "A", "sentence2": "B", "similarity": 0.5}]"#;
        let dataset = parse_from_string(json_data, DataFormat::Json).unwrap();
        assert_eq!(dataset.len(), 1);
    }

    #[test]
    fn test_parse_from_string_csv() {
        let csv_data = "sentence1,sentence2,similarity\nA,B,0.5\n";
        let dataset = parse_from_string(csv_data, DataFormat::Csv).unwrap();
        assert_eq!(dataset.len(), 1);
    }

    #[test]
    fn test_training_dataset_methods() {
        let pairs = vec![TrainingPair {
            sentence1: "Test".to_string(),
            sentence2: "Test".to_string(),
            similarity: 1.0,
        }];
        let dataset = TrainingDataset::new(pairs);
        assert_eq!(dataset.len(), 1);
        assert!(!dataset.is_empty());

        let empty_dataset = TrainingDataset::new(vec![]);
        assert_eq!(empty_dataset.len(), 0);
        assert!(empty_dataset.is_empty());
    }

    #[test]
    fn test_parse_csv_with_quotes() {
        let csv_data = r#"sentence1,sentence2,similarity
"This is a test, with comma","Another test, with comma",0.85
"#;
        let dataset = parse_csv(csv_data).unwrap();
        assert_eq!(dataset.len(), 1);
        assert_eq!(dataset.pairs[0].sentence1, "This is a test, with comma");
        assert_eq!(dataset.pairs[0].sentence2, "Another test, with comma");
        assert_eq!(dataset.pairs[0].similarity, 0.85);
    }

    #[test]
    fn test_parse_json_empty_array() {
        let json_data = r#"{"training_data": []}"#;
        let dataset = parse_json(json_data).unwrap();
        assert_eq!(dataset.len(), 0);
        assert!(dataset.is_empty());
    }
}

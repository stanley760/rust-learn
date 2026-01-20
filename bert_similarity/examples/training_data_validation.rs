// Example demonstrating training data validation and parsing

use rust_semantic_similarity::training::{
    parse_from_string, validate_training_data, DataFormat,
};

fn main() {
    println!("=== Training Data Validation and Parsing Example ===\n");

    // Example 1: Parse and validate JSON data
    println!("1. Parsing JSON training data:");
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

    match parse_from_string(json_data, DataFormat::Json) {
        Ok(dataset) => {
            println!("   ✓ Successfully parsed {} training pairs", dataset.len());
            
            // Validate the data
            match validate_training_data(&dataset.pairs) {
                Ok(result) => {
                    if result.is_valid {
                        println!("   ✓ All training data is valid");
                    } else {
                        println!("   ✗ Validation errors found:");
                        println!("{}", result.error_summary());
                    }
                }
                Err(e) => {
                    println!("   ✗ Validation error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Failed to parse JSON: {}", e);
        }
    }

    println!();

    // Example 2: Parse and validate CSV data
    println!("2. Parsing CSV training data:");
    let csv_data = "sentence1,sentence2,similarity\n\
                   The weather is nice,It's a beautiful day,0.85\n\
                   I love programming,Coding is fun,0.9\n";

    match parse_from_string(csv_data, DataFormat::Csv) {
        Ok(dataset) => {
            println!("   ✓ Successfully parsed {} training pairs", dataset.len());
            
            match validate_training_data(&dataset.pairs) {
                Ok(result) => {
                    if result.is_valid {
                        println!("   ✓ All training data is valid");
                    } else {
                        println!("   ✗ Validation errors found:");
                        println!("{}", result.error_summary());
                    }
                }
                Err(e) => {
                    println!("   ✗ Validation error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Failed to parse CSV: {}", e);
        }
    }

    println!();

    // Example 3: Invalid data (similarity out of range)
    println!("3. Validating invalid training data:");
    let invalid_json = r#"
    [
        {
            "sentence1": "Valid sentence",
            "sentence2": "Another valid sentence",
            "similarity": 1.5
        },
        {
            "sentence1": "",
            "sentence2": "Valid sentence",
            "similarity": 0.5
        }
    ]
    "#;

    match parse_from_string(invalid_json, DataFormat::Json) {
        Ok(dataset) => {
            println!("   ✓ Successfully parsed {} training pairs", dataset.len());
            
            match validate_training_data(&dataset.pairs) {
                Ok(result) => {
                    if result.is_valid {
                        println!("   ✓ All training data is valid");
                    } else {
                        println!("   ✗ Validation errors found:");
                        println!("{}", result.error_summary());
                    }
                }
                Err(e) => {
                    println!("   ✗ Validation error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ✗ Failed to parse JSON: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
}

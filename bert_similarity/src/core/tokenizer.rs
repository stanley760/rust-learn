// Tokenizer wrapper for BERT model text processing

use crate::utils::AppError;
use tokenizers::{Tokenizer, Encoding};

/// Wrapper around the tokenizers::Tokenizer for easier integration
#[derive(Debug, Clone)]
pub struct TokenizerWrapper {
    tokenizer: Tokenizer,
}

impl TokenizerWrapper {
    /// Load a pretrained tokenizer from Hugging Face
    ///
    /// # Arguments
    /// * `model_name` - The name of the model on Hugging Face (e.g., "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2")
    ///
    /// # Returns
    /// * `Result<Self>` - The tokenizer wrapper or an error
    ///
    /// # Note
    /// This method attempts to download the tokenizer from Hugging Face Hub.
    /// For offline usage, use `from_file` instead with a local tokenizer.json file.
    pub fn from_pretrained(model_name: &str) -> Result<Self, AppError> {
        tracing::info!("Loading tokenizer from: {}", model_name);

        // Construct the Hugging Face Hub URL for the tokenizer
        // Format: https://huggingface.co/{model_name}/resolve/main/tokenizer.json
        let url = format!(
            "https://huggingface.co/{}/resolve/main/tokenizer.json",
            model_name
        );

        tracing::debug!("Attempting to download tokenizer from: {}", url);

        // For now, we'll return an error suggesting to use from_file
        // In a production environment, you'd want to implement HTTP download
        Err(AppError::TokenizationError(format!(
            "Direct download from Hugging Face is not yet implemented. \
             Please download the tokenizer.json file from {} and use from_file() instead.",
            url
        )))
    }

    /// Load a tokenizer from a local file path
    ///
    /// # Arguments
    /// * `path` - Path to the tokenizer.json file
    ///
    /// # Returns
    /// * `Result<Self>` - The tokenizer wrapper or an error
    pub fn from_file(path: &str) -> Result<Self, AppError> {
        tracing::info!("Loading tokenizer from file: {}", path);

        let tokenizer = Tokenizer::from_file(path).map_err(|e| {
            AppError::TokenizationError(format!(
                "Failed to load tokenizer from file '{}': {}",
                path, e
            ))
        })?;

        tracing::info!("Tokenizer loaded successfully from file");
        Ok(Self { tokenizer })
    }

    /// Encode a single text string into token IDs
    ///
    /// # Arguments
    /// * `text` - The text to encode
    /// * `add_special_tokens` - Whether to add special tokens (CLS, SEP, etc.)
    ///
    /// # Returns
    /// * `Result<Encoding>` - The encoding containing token IDs, attention mask, etc.
    pub fn encode(&self, text: &str, add_special_tokens: bool) -> Result<Encoding, AppError> {
        if text.is_empty() {
            return Err(AppError::InvalidInput("Cannot encode empty string".to_string()));
        }

        self.tokenizer
            .encode(text, add_special_tokens)
            .map_err(|e| AppError::TokenizationError(format!("Failed to encode text: {}", e)))
    }

    /// Encode multiple text strings in batch
    ///
    /// # Arguments
    /// * `texts` - A slice of text strings to encode
    /// * `add_special_tokens` - Whether to add special tokens (CLS, SEP, etc.)
    ///
    /// # Returns
    /// * `Result<Vec<Encoding>>` - Vector of encodings for each input text
    pub fn encode_batch(
        &self,
        texts: &[String],
        add_special_tokens: bool,
    ) -> Result<Vec<Encoding>, AppError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Check for empty strings
        for (idx, text) in texts.iter().enumerate() {
            if text.is_empty() {
                return Err(AppError::InvalidInput(format!(
                    "Cannot encode empty string at index {}",
                    idx
                )));
            }
        }

        // Convert to Vec<&str> for the tokenizer API
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

        self.tokenizer
            .encode_batch(text_refs, add_special_tokens)
            .map_err(|e| AppError::TokenizationError(format!("Failed to encode batch: {}", e)))
    }

    /// Decode token IDs back to text
    ///
    /// # Arguments
    /// * `ids` - The token IDs to decode
    /// * `skip_special_tokens` - Whether to skip special tokens in the output
    ///
    /// # Returns
    /// * `Result<String>` - The decoded text
    pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> Result<String, AppError> {
        self.tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| AppError::TokenizationError(format!("Failed to decode token IDs: {}", e)))
    }

    /// Get the vocabulary size of the tokenizer
    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }

    /// Get the padding token ID if available
    pub fn pad_token_id(&self) -> Option<u32> {
        self.tokenizer.get_padding().map(|padding| padding.pad_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty_string_returns_error() {
        // Create a simple tokenizer for testing
        let tokenizer_json = r#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "Whitespace"
            },
            "post_processor": null,
            "decoder": null,
            "model": {
                "type": "WordLevel",
                "vocab": {
                    "hello": 0,
                    "world": 1
                },
                "unk_token": "[UNK]"
            }
        }"#;

        let temp_dir = std::env::temp_dir();
        let tokenizer_path = temp_dir.join("test_tokenizer.json");
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();

        let wrapper = TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap();
        let result = wrapper.encode("", true);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));

        // Cleanup
        std::fs::remove_file(tokenizer_path).ok();
    }

    #[test]
    fn test_encode_batch_empty_list_returns_empty() {
        let tokenizer_json = r#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "Whitespace"
            },
            "post_processor": null,
            "decoder": null,
            "model": {
                "type": "WordLevel",
                "vocab": {
                    "hello": 0,
                    "world": 1
                },
                "unk_token": "[UNK]"
            }
        }"#;

        let temp_dir = std::env::temp_dir();
        let tokenizer_path = temp_dir.join("test_tokenizer_batch.json");
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();

        let wrapper = TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap();
        let result = wrapper.encode_batch(&[], true).unwrap();

        assert!(result.is_empty());

        // Cleanup
        std::fs::remove_file(tokenizer_path).ok();
    }

    #[test]
    fn test_encode_batch_with_empty_string_returns_error() {
        let tokenizer_json = r#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "Whitespace"
            },
            "post_processor": null,
            "decoder": null,
            "model": {
                "type": "WordLevel",
                "vocab": {
                    "hello": 0,
                    "world": 1
                },
                "unk_token": "[UNK]"
            }
        }"#;

        let temp_dir = std::env::temp_dir();
        let tokenizer_path = temp_dir.join("test_tokenizer_batch_empty.json");
        std::fs::write(&tokenizer_path, tokenizer_json).unwrap();

        let wrapper = TokenizerWrapper::from_file(tokenizer_path.to_str().unwrap()).unwrap();
        let texts = vec!["hello".to_string(), "".to_string(), "world".to_string()];
        let result = wrapper.encode_batch(&texts, true);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidInput(_)));

        // Cleanup
        std::fs::remove_file(tokenizer_path).ok();
    }

    #[test]
    fn test_from_file_nonexistent_returns_error() {
        let result = TokenizerWrapper::from_file("/nonexistent/path/tokenizer.json");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::TokenizationError(_)));
    }
}

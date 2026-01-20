use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid port number: {0}")]
    InvalidPort(u16),
    
    #[error("Invalid max_connections: must be greater than 0")]
    InvalidMaxConnections,
    
    #[error("Invalid device: {0}. Must be one of: auto, cuda, metal, cpu")]
    InvalidDevice(String),
    
    #[error("Invalid max_sequence_length: must be greater than 0")]
    InvalidMaxSequenceLength,
    
    #[error("Invalid log level: {0}. Must be one of: debug, info, warn, error")]
    InvalidLogLevel(String),
    
    #[error("Invalid learning rate: must be greater than 0")]
    InvalidLearningRate,
    
    #[error("Invalid batch size: must be greater than 0")]
    InvalidBatchSize,
    
    #[error("Invalid epochs: must be greater than 0")]
    InvalidEpochs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub model: ModelConfig,
    pub logging: LoggingConfig,
    pub finetune: FinetuneConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl ServerConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::InvalidPort(self.port));
        }
        if self.max_connections == 0 {
            return Err(ConfigError::InvalidMaxConnections);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub model_name: String,
    pub model_path: Option<String>,
    pub device: String,
    pub max_sequence_length: usize,
}

impl ModelConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let valid_devices = ["auto", "cuda", "metal", "cpu"];
        if !valid_devices.contains(&self.device.as_str()) {
            return Err(ConfigError::InvalidDevice(self.device.clone()));
        }
        if self.max_sequence_length == 0 {
            return Err(ConfigError::InvalidMaxSequenceLength);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

impl LoggingConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let valid_levels = ["debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.level.as_str()) {
            return Err(ConfigError::InvalidLogLevel(self.level.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinetuneConfig {
    pub default_learning_rate: f64,
    pub default_batch_size: usize,
    pub default_epochs: usize,
    pub checkpoint_dir: String,
}

impl FinetuneConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.default_learning_rate <= 0.0 {
            return Err(ConfigError::InvalidLearningRate);
        }
        if self.default_batch_size == 0 {
            return Err(ConfigError::InvalidBatchSize);
        }
        if self.default_epochs == 0 {
            return Err(ConfigError::InvalidEpochs);
        }
        Ok(())
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration with fallback to default if file doesn't exist
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        match Self::from_file(&path) {
            Ok(config) => {
                tracing::info!("Configuration loaded from {:?}", path.as_ref());
                config
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to load configuration from {:?}: {}. Using default configuration.",
                    path.as_ref(),
                    e
                );
                Self::default()
            }
        }
    }
    
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.server.validate()?;
        self.model.validate()?;
        self.logging.validate()?;
        self.finetune.validate()?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8000,
                max_connections: 100,
            },
            model: ModelConfig {
                model_name: "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
                    .to_string(),
                model_path: None,
                device: "auto".to_string(),
                max_sequence_length: 128,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
            finetune: FinetuneConfig {
                default_learning_rate: 2e-5,
                default_batch_size: 16,
                default_epochs: 3,
                checkpoint_dir: "checkpoints".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    
    #[test]
    fn test_load_valid_config() {
        let config_content = r#"
[server]
host = "127.0.0.1"
port = 9000
max_connections = 50

[model]
model_name = "test-model"
device = "cpu"
max_sequence_length = 256

[logging]
level = "debug"

[finetune]
default_learning_rate = 1e-5
default_batch_size = 32
default_epochs = 5
checkpoint_dir = "test_checkpoints"
"#;
        
        let temp_file = std::env::temp_dir().join("test_config.toml");
        let mut file = std::fs::File::create(&temp_file).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();
        
        let config = Config::from_file(&temp_file).unwrap();
        
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 9000);
        assert_eq!(config.server.max_connections, 50);
        assert_eq!(config.model.model_name, "test-model");
        assert_eq!(config.model.device, "cpu");
        assert_eq!(config.model.max_sequence_length, 256);
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.finetune.default_learning_rate, 1e-5);
        assert_eq!(config.finetune.default_batch_size, 32);
        assert_eq!(config.finetune.default_epochs, 5);
        
        std::fs::remove_file(temp_file).ok();
    }
    
    #[test]
    fn test_load_nonexistent_config_uses_default() {
        let config = Config::load_or_default("nonexistent_config.toml");
        
        // Should use default values
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8000);
        assert_eq!(config.model.device, "auto");
        assert_eq!(config.logging.level, "info");
    }
    
    #[test]
    fn test_load_malformed_config_uses_default() {
        let malformed_content = "this is not valid toml [[[";
        
        let temp_file = std::env::temp_dir().join("malformed_config.toml");
        let mut file = std::fs::File::create(&temp_file).unwrap();
        file.write_all(malformed_content.as_bytes()).unwrap();
        
        let config = Config::load_or_default(&temp_file);
        
        // Should use default values
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8000);
        
        std::fs::remove_file(temp_file).ok();
    }
    
    #[test]
    fn test_validate_invalid_port() {
        let config = ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 0,
            max_connections: 100,
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidPort(0)));
    }
    
    #[test]
    fn test_validate_invalid_max_connections() {
        let config = ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8000,
            max_connections: 0,
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidMaxConnections));
    }
    
    #[test]
    fn test_validate_invalid_device() {
        let config = ModelConfig {
            model_name: "test".to_string(),
            model_path: None,
            device: "invalid_device".to_string(),
            max_sequence_length: 128,
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidDevice(_)));
    }
    
    #[test]
    fn test_validate_invalid_log_level() {
        let config = LoggingConfig {
            level: "invalid_level".to_string(),
            file: None,
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidLogLevel(_)));
    }
    
    #[test]
    fn test_validate_invalid_learning_rate() {
        let config = FinetuneConfig {
            default_learning_rate: 0.0,
            default_batch_size: 16,
            default_epochs: 3,
            checkpoint_dir: "checkpoints".to_string(),
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidLearningRate));
    }
    
    #[test]
    fn test_validate_invalid_batch_size() {
        let config = FinetuneConfig {
            default_learning_rate: 2e-5,
            default_batch_size: 0,
            default_epochs: 3,
            checkpoint_dir: "checkpoints".to_string(),
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidBatchSize));
    }
    
    #[test]
    fn test_validate_invalid_epochs() {
        let config = FinetuneConfig {
            default_learning_rate: 2e-5,
            default_batch_size: 16,
            default_epochs: 0,
            checkpoint_dir: "checkpoints".to_string(),
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidEpochs));
    }
}

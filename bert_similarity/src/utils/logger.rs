use crate::utils::config::LoggingConfig;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

/// A thread-safe writer wrapper for file logging
#[derive(Clone)]
struct FileWriter {
    inner: Arc<Mutex<io::BufWriter<std::fs::File>>>,
}

impl FileWriter {
    fn new(file: std::fs::File) -> Self {
        Self {
            inner: Arc::new(Mutex::new(io::BufWriter::new(file))),
        }
    }
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.lock().unwrap().flush()
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for FileWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

/// Initialize the logging system based on configuration
pub fn init_logger(config: &LoggingConfig) -> anyhow::Result<()> {
    let level = parse_log_level(&config.level);

    // Create environment filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("rust_semantic_similarity={}", level)));

    // Create console layer with structured format
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_filter(env_filter.clone());

    // Initialize subscriber with console layer
    let subscriber = tracing_subscriber::registry().with(console_layer);

    // Add file logging if configured
    if let Some(file_path) = &config.file {
        // Create parent directories if they don't exist
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open or create log file in append mode
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let file_writer = FileWriter::new(file);

        // Create file layer with structured format
        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_file(true)
            .with_ansi(false) // Disable ANSI colors in file output
            .with_filter(env_filter);

        subscriber.with(file_layer).init();
        tracing::info!("Logger initialized with level: {} (file: {})", level, file_path);
    } else {
        subscriber.init();
        tracing::info!("Logger initialized with level: {}", level);
    }

    Ok(())
}

fn parse_log_level(level: &str) -> &str {
    match level.to_lowercase().as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => {
            tracing::warn!("Unknown log level '{}', defaulting to 'info'", level);
            "info"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level_valid() {
        assert_eq!(parse_log_level("debug"), "debug");
        assert_eq!(parse_log_level("info"), "info");
        assert_eq!(parse_log_level("warn"), "warn");
        assert_eq!(parse_log_level("error"), "error");
    }

    #[test]
    fn test_parse_log_level_case_insensitive() {
        assert_eq!(parse_log_level("DEBUG"), "debug");
        assert_eq!(parse_log_level("Info"), "info");
        assert_eq!(parse_log_level("WARN"), "warn");
        assert_eq!(parse_log_level("ErRoR"), "error");
    }

    #[test]
    fn test_parse_log_level_invalid_defaults_to_info() {
        assert_eq!(parse_log_level("invalid"), "info");
        assert_eq!(parse_log_level("trace"), "info");
        assert_eq!(parse_log_level(""), "info");
    }

    #[test]
    fn test_init_logger_console_only() {
        let config = LoggingConfig {
            level: "info".to_string(),
            file: None,
        };

        // This test just verifies that init_logger doesn't panic
        // Note: This will succeed only once per process
        let _ = init_logger(&config);
    }

    #[test]
    fn test_file_writer_creation() {
        let temp_dir = std::env::temp_dir();
        let log_file = temp_dir.join("test_file_writer.log");

        // Clean up any existing log file
        let _ = std::fs::remove_file(&log_file);

        // Create a file and FileWriter
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .unwrap();

        let mut writer = FileWriter::new(file);

        // Test writing
        let result = writer.write(b"test log message\n");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 17);

        // Test flushing
        let flush_result = writer.flush();
        assert!(flush_result.is_ok());

        // Clean up
        let _ = std::fs::remove_file(&log_file);
    }

    #[test]
    fn test_log_file_creation() {
        let temp_dir = std::env::temp_dir();
        let log_file = temp_dir.join("test_log_creation.log");

        // Clean up any existing log file
        let _ = std::fs::remove_file(&log_file);

        // Test file creation logic directly
        if let Some(parent) = log_file.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .unwrap();

        // Verify the file was created
        assert!(log_file.exists(), "Log file should be created");

        // Clean up
        drop(file);
        let _ = std::fs::remove_file(&log_file);
    }

    #[test]
    fn test_parent_directory_creation() {
        let temp_dir = std::env::temp_dir();
        let nested_log_file = temp_dir.join("test_nested").join("logs").join("test.log");

        // Clean up any existing directories
        let _ = std::fs::remove_dir_all(temp_dir.join("test_nested"));

        // Test parent directory creation logic directly
        if let Some(parent) = nested_log_file.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        // Verify the parent directories were created
        assert!(
            nested_log_file.parent().unwrap().exists(),
            "Parent directories should be created"
        );

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir.join("test_nested"));
    }
}

# Rust Semantic Similarity Service

A high-performance semantic similarity service built with Rust, using the Candle ML framework to run BERT models for multilingual text similarity computation and model fine-tuning.

## Features

- **Semantic Similarity Computation**: Calculate similarity scores between text pairs
- **Batch Processing**: Efficiently process multiple text pairs in parallel
- **Model Fine-tuning**: Customize models with domain-specific training data
- **Multilingual Support**: Works with Chinese, English, Japanese, Korean, and more
- **High Performance**: Built with Rust for speed and memory efficiency
- **REST API**: Easy-to-use HTTP endpoints

## Project Structure

```
.
├── src/
│   ├── api/           # HTTP API endpoints and request/response models
│   ├── model/         # BERT model loading, inference, and embeddings
│   ├── training/      # Model fine-tuning and training logic
│   ├── utils/         # Configuration, logging, and error handling
│   └── main.rs        # Application entry point
├── config.toml        # Configuration file
├── Cargo.toml         # Rust dependencies
└── README.md          # This file
```

## Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-semantic-similarity
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

Edit `config.toml` to customize the service:

```toml
[server]
host = "0.0.0.0"
port = 8000
max_connections = 100

[model]
model_name = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
device = "auto"  # Options: auto, cuda, metal, cpu
max_sequence_length = 128

[logging]
level = "info"  # Options: debug, info, warn, error

[finetune]
default_learning_rate = 2e-5
default_batch_size = 16
default_epochs = 3
checkpoint_dir = "checkpoints"
```

## Usage

### Running the Service

```bash
cargo run --release
```

The service will start on the configured host and port (default: `http://0.0.0.0:8000`).

### API Endpoints

#### Health Check
```bash
curl http://localhost:8000/health
```

#### Compute Similarity
```bash
curl -X POST http://localhost:8000/similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence1": "这是一个例子",
    "sentence1": "这是一个例子",
  }'
```

#### Batch Similarity
```bash
curl -X POST http://localhost:8000/bulk_similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence_pairs": [
      ["Hello world", "Hi there"],
      ["Good morning", "Good evening"]
    ]
  }'
```

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Technology Stack

- **Candle**: Rust ML framework for running BERT models
- **Axum**: Modern async web framework
- **Tokio**: Async runtime
- **Tokenizers**: Fast text tokenization
- **Tracing**: Structured logging
- **Serde**: Serialization/deserialization

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

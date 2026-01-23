# Usage Examples

## Starting the Server

```bash
# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will:
1. Load configuration from `config.toml`
2. Download the Hugging Face model (first run only)
3. Initialize the model and tokenizer
4. Start listening on `http://0.0.0.0:8000`

## API Examples

### 1. Basic Similarity Computation

**Request:**
```bash
curl -X POST http://localhost:8000/similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence1": "The cat is on the mat",
    "sentence2": "A cat is sitting on a mat"
  }'
```

**Response:**
```json
{
  "sentence1": "The cat is on the mat",
  "sentence2": "A cat is sitting on a mat",
  "similarity": 0.9234
}
```

### 2. Detecting Semantic Opposition

**Request:**
```bash
curl -X POST http://localhost:8000/similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence1": "This product is good",
    "sentence2": "This product is not good"
  }'
```

**Response:**
```json
{
  "sentence1": "This product is good",
  "sentence2": "This product is not good",
  "similarity": 0.4521
}
```

Note: Similarity is reduced because of the "not" negation marker despite high character overlap.

### 3. Chinese Text with Opposition

**Request:**
```bash
curl -X POST http://localhost:8000/similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence1": "这个产品很好",
    "sentence2": "这个产品不好"
  }'
```

**Response:**
```json
{
  "sentence1": "这个产品很好",
  "sentence2": "这个产品不好",
  "similarity": 0.4234
}
```

### 4. Bulk Similarity Computation

**Request:**
```bash
curl -X POST http://localhost:8000/bulk_similarity \
  -H "Content-Type: application/json" \
  -d '{
    "sentence_pairs": [
      ["I love this movie", "I hate this movie"],
      ["The weather is sunny", "The weather is cloudy"],
      ["She is happy", "She is very happy"]
    ]
  }'
```

**Response:**
```json
{
  "results": [
    {
      "sentence1": "I love this movie",
      "sentence2": "I hate this movie",
      "similarity": 0.3456
    },
    {
      "sentence1": "The weather is sunny",
      "sentence2": "The weather is cloudy",
      "similarity": 0.6789
    },
    {
      "sentence1": "She is happy",
      "sentence2": "She is very happy",
      "similarity": 0.9123
    }
  ]
}
```

### 5. Health Check

**Request:**
```bash
curl http://localhost:8000/health
```

**Response:**
```json
{
  "status": "ok",
  "model_loaded": true,
  "device": "cuda"
}
```

## Python Client Example

```python
import requests
import json

BASE_URL = "http://localhost:8000"

def compute_similarity(text1, text2):
    """Compute similarity between two texts"""
    response = requests.post(
        f"{BASE_URL}/similarity",
        json={
            "sentence1": text1,
            "sentence2": text2
        }
    )
    return response.json()

def compute_bulk_similarity(pairs):
    """Compute similarity for multiple pairs"""
    response = requests.post(
        f"{BASE_URL}/bulk_similarity",
        json={"sentence_pairs": pairs}
    )
    return response.json()

# Example usage
if __name__ == "__main__":
    # Single pair
    result = compute_similarity(
        "The cat is sleeping",
        "The cat is resting"
    )
    print(f"Similarity: {result['similarity']}")
    
    # Multiple pairs
    pairs = [
        ["I like this", "I don't like this"],
        ["Good morning", "Good evening"],
        ["She is tall", "She is very tall"]
    ]
    results = compute_bulk_similarity(pairs)
    for r in results['results']:
        print(f"{r['sentence1']} <-> {r['sentence2']}: {r['similarity']}")
```

## JavaScript/Node.js Client Example

```javascript
const axios = require('axios');

const BASE_URL = 'http://localhost:8000';

async function computeSimilarity(text1, text2) {
  try {
    const response = await axios.post(`${BASE_URL}/similarity`, {
      sentence1: text1,
      sentence2: text2
    });
    return response.data;
  } catch (error) {
    console.error('Error:', error.message);
  }
}

async function computeBulkSimilarity(pairs) {
  try {
    const response = await axios.post(`${BASE_URL}/bulk_similarity`, {
      sentence_pairs: pairs
    });
    return response.data;
  } catch (error) {
    console.error('Error:', error.message);
  }
}

// Example usage
(async () => {
  // Single pair
  const result = await computeSimilarity(
    'The cat is sleeping',
    'The cat is resting'
  );
  console.log(`Similarity: ${result.similarity}`);
  
  // Multiple pairs
  const pairs = [
    ['I like this', "I don't like this"],
    ['Good morning', 'Good evening'],
    ['She is tall', 'She is very tall']
  ];
  const results = await computeBulkSimilarity(pairs);
  results.results.forEach(r => {
    console.log(`${r.sentence1} <-> ${r.sentence2}: ${r.similarity}`);
  });
})();
```

## Configuration Examples

### For English-only Applications

```toml
[model]
model_name = "sentence-transformers/all-MiniLM-L6-v2"
device = "auto"
max_sequence_length = 128
```

### For Multilingual Applications

```toml
[model]
model_name = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
device = "auto"
max_sequence_length = 128
```

### For GPU Acceleration

```toml
[model]
model_name = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
device = "cuda"
max_sequence_length = 256
```

### For CPU-only Environments

```toml
[model]
model_name = "sentence-transformers/all-MiniLM-L6-v2"
device = "cpu"
max_sequence_length = 128
```

## Common Use Cases

### 1. Duplicate Detection

```python
def find_duplicates(texts, threshold=0.85):
    """Find duplicate or near-duplicate texts"""
    duplicates = []
    for i, text1 in enumerate(texts):
        for j, text2 in enumerate(texts[i+1:], i+1):
            result = compute_similarity(text1, text2)
            if result['similarity'] >= threshold:
                duplicates.append((i, j, result['similarity']))
    return duplicates
```

### 2. Semantic Search

```python
def semantic_search(query, documents, top_k=5):
    """Find most similar documents to query"""
    pairs = [(query, doc) for doc in documents]
    results = compute_bulk_similarity(pairs)
    
    # Sort by similarity
    sorted_results = sorted(
        results['results'],
        key=lambda x: x['similarity'],
        reverse=True
    )
    
    return sorted_results[:top_k]
```

### 3. Sentiment Contradiction Detection

```python
def detect_contradictions(text1, text2, threshold=0.5):
    """Detect if two texts contradict each other"""
    result = compute_similarity(text1, text2)
    
    # Low similarity with high character overlap = likely contradiction
    if result['similarity'] < threshold:
        return True, result['similarity']
    return False, result['similarity']
```

### 4. Paraphrase Detection

```python
def is_paraphrase(text1, text2, threshold=0.80):
    """Check if two texts are paraphrases"""
    result = compute_similarity(text1, text2)
    return result['similarity'] >= threshold
```

## Performance Tips

1. **Batch Processing**: Use `/bulk_similarity` for multiple pairs
2. **GPU Usage**: Set `device = "cuda"` for faster inference
3. **Sequence Length**: Reduce `max_sequence_length` for faster processing
4. **Model Selection**: Use smaller models for speed, larger for accuracy

## Troubleshooting

### Server won't start
- Check if port 8000 is available
- Verify Hugging Face Hub is accessible
- Check disk space for model download

### Slow inference
- Enable GPU: `device = "cuda"`
- Reduce `max_sequence_length`
- Use smaller model

### High memory usage
- Reduce batch size
- Use smaller model
- Reduce `max_sequence_length`

### Incorrect opposition detection
- Check if negation words are in the opposition marker list
- Verify text encoding (UTF-8)
- Test with simpler examples first

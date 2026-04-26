# Chyren API Documentation

## Overview

Chyren provides a comprehensive API for interacting with the AEGIS (Adaptive Epistemic Guardian & Inference System) and the Chiral Thesis framework. This document outlines the core API endpoints, data structures, and usage patterns.

## Table of Contents

- [Core Components](#core-components)
- [API Endpoints](#api-endpoints)
- [Data Structures](#data-structures)
- [Authentication](#authentication)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Examples](#examples)

---

## Core Components

### AEGIS Core

The AEGIS system provides the foundational intelligence framework.

```rust
use chyren::aegis::Core;

let aegis = Core::new(config)?;
let response = aegis.process_query(query).await?;
```

### Chiral Thesis Engine

Implements the mathematical framework for chiral invariance.

```rust
use chyren::chiral::ThesisEngine;

let engine = ThesisEngine::initialize(params)?;
let result = engine.compute_master_equation(state).await?;
```

---

## API Endpoints

### Query Processing

#### `POST /api/v1/query`

Process a natural language query through AEGIS.

**Request:**
```json
{
  "query": "Explain the Chiral Thesis framework",
  "context": {
    "session_id": "uuid-string",
    "parameters": {
      "temperature": 0.7,
      "max_tokens": 2000
    }
  }
}
```

**Response:**
```json
{
  "id": "response-uuid",
  "result": "The Chiral Thesis framework...",
  "metadata": {
    "processing_time_ms": 245,
    "model_version": "v1.0.0",
    "confidence": 0.94
  }
}
```

### Mathematical Computation

#### `POST /api/v1/compute/master-equation`

Compute the Chiral Master Equation for a given state.

**Request:**
```json
{
  "state": {
    "psi": [0.707, 0.707],
    "operator": "hamiltonian",
    "parameters": {
      "lambda": 1.0,
      "chyren": 2.5
    }
  }
}
```

**Response:**
```json
{
  "evolved_state": [0.6, 0.8],
  "eigenvalues": [1.5, -1.5],
  "observables": {
    "energy": 1.25,
    "entropy": 0.69
  }
}
```

### Embedding Space Operations

#### `POST /api/v1/embedding/transform`

Transform data into the chiral embedding space.

**Request:**
```json
{
  "data": "Input text or numerical data",
  "embedding_type": "chiral_invariant",
  "dimensions": 768
}
```

**Response:**
```json
{
  "embedding": [0.123, -0.456, ...],
  "norm": 1.0,
  "chirality_score": 0.87
}
```

---

## Data Structures

### State Vector

```rust
pub struct StateVector {
    pub psi: Vec<Complex64>,
    pub normalization: f64,
    pub basis: BasisType,
}
```

### Query Context

```rust
pub struct QueryContext {
    pub session_id: Uuid,
    pub parameters: HashMap<String, Value>,
    pub timestamp: DateTime<Utc>,
}
```

### Computational Result

```rust
pub struct ComputationResult<T> {
    pub data: T,
    pub metadata: ResultMetadata,
    pub errors: Vec<ErrorInfo>,
}
```

---

## Authentication

All API requests require authentication using API keys.

### Header-Based Authentication

```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
     https://api.chyren.io/v1/query
```

### Obtaining API Keys

1. Register at the Chyren platform
2. Navigate to Settings > API Keys
3. Generate a new key with appropriate scopes
4. Store securely (keys are shown only once)

---

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_STATE",
    "message": "State vector normalization failed",
    "details": {
      "expected_norm": 1.0,
      "actual_norm": 1.23
    },
    "request_id": "uuid-string"
  }
}
```

### Common Error Codes

- `INVALID_REQUEST` - Malformed request body
- `AUTHENTICATION_FAILED` - Invalid or expired API key
- `RATE_LIMIT_EXCEEDED` - Too many requests
- `COMPUTATION_ERROR` - Mathematical computation failed
- `INVALID_STATE` - State vector validation failed
- `RESOURCE_NOT_FOUND` - Requested resource doesn't exist

---

## Rate Limiting

API requests are rate-limited based on your subscription tier:

| Tier | Requests/Minute | Requests/Day |
|------|-----------------|-------------|
| Free | 10 | 1,000 |
| Pro | 100 | 50,000 |
| Enterprise | Custom | Custom |

### Rate Limit Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640995200
```

---

## Examples

### Python SDK

```python
from chyren import AegisClient

client = AegisClient(api_key="your-api-key")

# Process a query
response = client.query(
    "Compute the master equation for initial state |0⟩",
    parameters={"temperature": 0.7}
)

print(response.result)
print(f"Confidence: {response.metadata.confidence}")
```

### Rust SDK

```rust
use chyren_sdk::AegisClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = AegisClient::new("your-api-key");
    
    let response = client
        .query("Explain chiral symmetry")
        .with_temperature(0.7)
        .execute()
        .await?;
    
    println!("Result: {}", response.result);
    Ok(())
}
```

### cURL Examples

```bash
# Query processing
curl -X POST https://api.chyren.io/v1/query \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is the Chiral Thesis?",
    "context": {
      "session_id": "test-session"
    }
  }'

# Master equation computation
curl -X POST https://api.chyren.io/v1/compute/master-equation \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "state": {
      "psi": [0.707, 0.707],
      "operator": "hamiltonian"
    }
  }'
```

---

## Advanced Features

### WebSocket Streaming

For real-time computation updates:

```javascript
const ws = new WebSocket('wss://api.chyren.io/v1/stream');

ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('Computation progress:', update.progress);
};

ws.send(JSON.stringify({
  type: 'compute',
  payload: { /* computation parameters */ }
}));
```

### Batch Processing

```json
{
  "batch": [
    {"query": "Query 1"},
    {"query": "Query 2"},
    {"query": "Query 3"}
  ],
  "mode": "parallel"
}
```

---

## Webhooks

Register webhooks for asynchronous computation results:

```json
{
  "url": "https://your-app.com/webhook",
  "events": ["computation.completed", "computation.failed"],
  "secret": "webhook-secret"
}
```

---

## SDK Support

Official SDKs are available for:

- **Python** - `pip install chyren-sdk`
- **Rust** - `cargo add chyren-sdk`
- **JavaScript/TypeScript** - `npm install @chyren/sdk`
- **Go** - `go get github.com/chyren/go-sdk`

---

## Support

For API support:
- Documentation: https://docs.chyren.io
- Email: api-support@chyren.io
- Discord: https://discord.gg/chyren
- GitHub Issues: https://github.com/Mega-Therion/Chyren/issues

---

## Changelog

### v1.0.0 (Current)
- Initial API release
- Core AEGIS endpoints
- Master equation computation
- Embedding transformations

### Upcoming Features
- GraphQL API
- gRPC support
- Enhanced batch processing
- Multi-region deployment

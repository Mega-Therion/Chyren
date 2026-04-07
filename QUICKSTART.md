# 🚀 Chyren Quickstart Guide

Get Chyren up and running in minutes. This guide walks you through setup, basic usage, and your first ADCCL verification.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [First Run](#first-run)
- [Basic Usage](#basic-usage)
- [Next Steps](#next-steps)

## Prerequisites

Before installing Chyren, ensure you have:

- **Rust**: Version 1.70 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Python**: Version 3.8+ (for Python bindings, optional)
  ```bash
  python --version
  ```

- **Git**: For cloning the repository
  ```bash
  git --version
  ```

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren

# Build the project
cargo build --release

# Run tests to verify installation
cargo test
```

### Option 2: Using Cargo

```bash
# Install directly from crates.io (once published)
cargo install chyren
```

### Verify Installation

```bash
# Check Chyren CLI is available
cargo run -- --version
```

## Configuration

### 1. Environment Setup

Create a `.env` file in the project root:

```bash
# Copy the example environment file
cp .env.example .env
```

### 2. Configure Core Parameters

Edit `.env` with your settings:

```env
# Chyren Core Configuration
CHYREN_LOG_LEVEL=info
CHYREN_WORKSPACE_PATH=./omega_workspace

# ADCCL Parameters
ADCCL_STRICTNESS=0.5
ADCCL_ENABLE_PREFLIGHT=true
ADCCL_GATE_THRESHOLD=0.7

# OmegA Integration (optional)
OMEGA_API_ENDPOINT=http://localhost:8080
OMEGA_AUTH_TOKEN=your_token_here
```

### 3. Initialize Workspace

```bash
# Create and initialize the OmegA workspace
cargo run -- init
```

This creates:
- `omega_workspace/` - Main workspace directory
- State files for ADCCL tracking
- Provider configuration templates

## First Run

### Run a Simple ADCCL Check

```bash
# Check a simple action for ADCCL compliance
cargo run -- check --action "user_data_access" --context "read_profile"
```

Expected output:
```
✓ ADCCL Preflight Check PASSED
  Strictness: 0.50
  Gate Status: OPEN (score: 0.85)
  Chirality: RIGHT-HANDED
  Recommendation: PROCEED
```

### Run Your First Provider

```bash
# Start a basic AI provider with ADCCL verification
cargo run -- run-provider --name "gemini" --config ./providers/gemini.yaml
```

## Basic Usage

### CLI Commands

#### Check ADCCL Compliance

```bash
# Basic compliance check
chyren check --action <ACTION> --context <CONTEXT>

# With custom strictness
chyren check --action user_delete --strictness 0.8

# Detailed output
chyren check --action data_export --verbose
```

#### Manage Providers

```bash
# List available providers
chyren providers list

# Run a specific provider
chyren run-provider --name gemini

# Stop a provider
chyren stop-provider --name gemini
```

#### Workspace Management

```bash
# View workspace status
chyren workspace status

# Clean workspace
chyren workspace clean

# Reset ADCCL state
chyren workspace reset-adccl
```

### Programmatic Usage (Rust)

```rust
use chyren::{
    core::adccl::AdcclGate,
    state::State,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ADCCL gate
    let gate = AdcclGate::new(0.5)?;
    
    // Create state for action
    let state = State::new("user_data_access")?;
    
    // Run preflight check
    let result = gate.preflight_check(&state).await?;
    
    if result.is_approved() {
        println!("✓ Action approved: {}", result.chirality());
        // Proceed with action
    } else {
        println!("✗ Action blocked: {}", result.reason());
    }
    
    Ok(())
}
```

### Programmatic Usage (Python)

```python
from chyren import AdcclGate, State

# Initialize ADCCL gate
gate = AdcclGate(strictness=0.5)

# Create state
state = State(action="user_data_access")

# Run preflight check
result = gate.preflight_check(state)

if result.is_approved:
    print(f"✓ Action approved: {result.chirality}")
    # Proceed with action
else:
    print(f"✗ Action blocked: {result.reason}")
```

## Example Workflows

### Workflow 1: Verify AI Provider Actions

```bash
# Start Chyren with Gemini provider
chyren run-provider --name gemini --adccl-strictness 0.6

# Send a request (Chyren automatically applies ADCCL verification)
curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Explain quantum computing"}'

# Check ADCCL audit log
chyren audit --provider gemini --last 10
```

### Workflow 2: Batch Compliance Checks

```bash
# Create a batch check file
cat > actions.yaml << EOF
actions:
  - action: user_login
    context: web_app
  - action: data_export
    context: gdpr_request
  - action: model_training
    context: user_consent
EOF

# Run batch check
chyren check-batch --file actions.yaml --output results.json
```

### Workflow 3: Integration with OmegA Stack

```bash
# Configure OmegA integration
export OMEGA_API_ENDPOINT="https://omega.example.com"
export OMEGA_AUTH_TOKEN="your_token"

# Start Chyren as OmegA middleware
chyren serve --mode omega-middleware --port 8080

# OmegA requests now flow through Chyren's ADCCL verification
```

## Understanding ADCCL Output

### Gate Status Indicators

| Status | Symbol | Meaning |
|--------|--------|----------|
| OPEN | ✓ | Action complies with ADCCL principles |
| CLOSED | ✗ | Action violates ADCCL principles |
| PENDING | ⚠ | Requires manual review |

### Chirality Types

- **RIGHT-HANDED**: Aligns with deterministic, verifiable outcomes
- **LEFT-HANDED**: Shows probabilistic or uncertain characteristics
- **ACHIRAL**: Neutral with respect to ADCCL principles

### Strictness Levels

- `0.0 - 0.3`: **Permissive** - Most actions pass
- `0.4 - 0.6`: **Balanced** - Moderate filtering
- `0.7 - 0.9`: **Strict** - High compliance requirements
- `0.9 - 1.0`: **Maximum** - Only deterministic actions pass

## Troubleshooting

### Common Issues

#### Build Errors

```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release
```

#### ADCCL Gate Always Closes

```bash
# Check strictness setting (may be too high)
echo $ADCCL_STRICTNESS

# Lower strictness temporarily
export ADCCL_STRICTNESS=0.3
```

#### Provider Connection Issues

```bash
# Verify provider configuration
chyren providers check --name gemini

# Check logs
chyren logs --provider gemini --tail 50
```

### Enable Debug Logging

```bash
# Set log level to debug
export CHYREN_LOG_LEVEL=debug
export RUST_LOG=chyren=debug

# Run with verbose output
cargo run -- check --action test --verbose
```

## Next Steps

### Learn More

- 📖 [Architecture Overview](./docs/ARCHITECTURE.md) - Deep dive into Chyren's design
- 🔬 [Chiral Thesis](./docs/chiral_thesis.md) - Mathematical foundations
- 🎯 [ADCCL Principles](./README.md#adccl-principles) - Core verification concepts
- 🧪 [Examples](./examples/) - Sample implementations

### Advanced Topics

- **Custom Providers**: Build your own ADCCL-verified AI providers
- **Vettagrammaton Signatures**: Implement ledger-based verification
- **Python SDK**: Use Chyren from Python applications
- **CI/CD Integration**: Add ADCCL checks to your pipeline

### Get Involved

- 🐛 [Report Issues](https://github.com/Mega-Therion/Chyren/issues)
- 💡 [Request Features](https://github.com/Mega-Therion/Chyren/issues/new?template=feature_request.md)
- 🤝 [Contributing Guide](./CONTRIBUTING.md)
- 💬 [Discussions](https://github.com/Mega-Therion/Chyren/discussions)

## Quick Reference Card

```bash
# Installation
git clone https://github.com/Mega-Therion/Chyren.git && cd Chyren && cargo build --release

# Initialize
cargo run -- init

# Quick check
cargo run -- check --action <ACTION>

# Run provider
cargo run -- run-provider --name <PROVIDER>

# View status
cargo run -- workspace status

# Get help
cargo run -- --help
```

---

**Ready to build sovereign intelligence? 🚀**

For questions or support, see [SUPPORT.md](./SUPPORT.md) or join our [discussions](https://github.com/Mega-Therion/Chyren/discussions).

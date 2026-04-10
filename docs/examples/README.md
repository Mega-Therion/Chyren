# Chyren Examples

Welcome to the Chyren examples directory! This collection demonstrates how to use Chyren's sovereign intelligence verification and ADCCL (Adaptive Deterministic Causal Compliance Layer) in your applications.

## Directory Structure

```
examples/
├── rust/                  # Rust examples
│   └── basic_adccl_check.rs    # Basic ADCCL compliance checking
├── python/                # Python examples  
│   └── basic_adccl_usage.py    # Python ADCCL API usage
└── README.md              # This file
```

## Quick Start

### Rust Examples

The Rust examples demonstrate using Chyren's core library directly.

#### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren
```

#### Running the Basic ADCCL Check Example

```bash
# Build and run the example
cargo run --example basic_adccl_check

# Or compile first, then run
cargo build --release --example basic_adccl_check
./target/release/examples/basic_adccl_check
```

**What it demonstrates:**
- Initializing ADCCL gate with custom configuration
- Performing preflight compliance checks
- Handling different action risk levels
- Using custom strictness settings
- Detecting chirality (deterministic vs probabilistic operations)

### Python Examples

The Python examples show how to use Chyren through its Python bindings.

#### Prerequisites

```bash
# Install Chyren Python bindings
pip install chyren

# Or install from source
cd chyren_py
pip install -e .
```

#### Running the Basic Usage Example

```bash
# Make the script executable
chmod +x examples/python/basic_adccl_usage.py

# Run the example
python examples/python/basic_adccl_usage.py

# Or use python3 explicitly
python3 examples/python/basic_adccl_usage.py
```

**What it demonstrates:**
- Basic ADCCL compliance checks
- Testing different risk levels
- Custom strictness levels
- Chirality detection
- Batch processing of multiple actions

## Example Categories

### 1. Basic ADCCL Compliance

Learn the fundamentals of ADCCL verification:
- Initializing the ADCCL gate
- Creating action states
- Running preflight checks
- Interpreting results

**Files:**
- `rust/basic_adccl_check.rs`
- `python/basic_adccl_usage.py`

### 2. Risk Level Management

Understand how Chyren evaluates different action risk levels:
- Low-risk operations (data reads)
- Medium-risk operations (data updates)
- High-risk operations (data deletions)
- Critical operations (system-wide changes)

### 3. Strictness Configuration

Control how strictly actions are evaluated:
- **0.0 - 0.3**: Permissive (most actions pass)
- **0.4 - 0.6**: Balanced (moderate filtering)
- **0.7 - 0.9**: Strict (high security)
- **0.9 - 1.0**: Maximum (only deterministic actions)

### 4. Chirality Detection

Identify the nature of operations:
- **RIGHT-HANDED**: Deterministic, verifiable outcomes
- **LEFT-HANDED**: Probabilistic, uncertain outcomes  
- **ACHIRAL**: Neutral operations

## Key Concepts

### ADCCL Gate

The ADCCL gate is the core verification mechanism that:
- Evaluates action compliance with sovereign intelligence principles
- Provides preflight checks before executing sensitive operations
- Returns detailed results including scores and recommendations

### State Management

Each action being verified needs a State object that includes:
- **Action**: The operation being performed (e.g., "user_login")
- **Context**: Additional context (e.g., "web_app")
- **Metadata**: Key-value pairs with relevant information

### Preflight Results

ADCCL checks return comprehensive results:
- **is_approved**: Boolean indicating if action should proceed
- **score**: Numerical compliance score (0.0 - 1.0)
- **chirality**: Type of operation (RIGHT/LEFT/ACHIRAL)
- **gate_status**: Current gate state (OPEN/CLOSED/PENDING)
- **reason**: Explanation if action is blocked

## Integration Patterns

### Pattern 1: API Request Validation

```rust
// Check API request before processing
let state = State::new("api_request")?;
state.set_context("user_endpoint");
state.set_metadata("user_id", user_id);

let result = adccl_gate.preflight_check(&state).await?;

if result.is_approved() {
    // Process the request
} else {
    // Return 403 Forbidden
}
```

### Pattern 2: Batch Operation Gating

```python
# Validate batch operations
for action in batch_actions:
    state = State(action=action)
    result = await gate.preflight_check(state)
    
    if not result.is_approved:
        # Skip or log this action
        continue
    
    # Process approved action
```

### Pattern 3: Dynamic Strictness

```rust
// Adjust strictness based on context
let strictness = if is_production {
    0.8  // Strict in production
} else {
    0.4  // Permissive in development
};

let config = AdcclConfig {
    strictness,
    ..Default::default()
};
let gate = AdcclGate::new(config)?;
```

## Best Practices

1. **Set Appropriate Strictness**: Match strictness to your security requirements
2. **Provide Context**: Include relevant metadata in State objects
3. **Handle Failures Gracefully**: Always check `is_approved()` before proceeding
4. **Log Blocked Actions**: Keep audit trails of blocked operations
5. **Test Different Risk Levels**: Validate behavior across action types
6. **Use Batch Processing**: Efficiently check multiple actions
7. **Monitor Chirality**: Track deterministic vs probabilistic operations

## Common Use Cases

- ✅ **User Authentication**: Verify login attempts
- ✅ **Data Access Control**: Gate sensitive data operations
- ✅ **API Rate Limiting**: Add compliance layer to rate limits
- ✅ **Administrative Actions**: Extra verification for admin operations
- ✅ **AI Model Inference**: Validate AI operation requests
- ✅ **Financial Transactions**: Pre-flight checks for payments
- ✅ **Data Export**: Compliance for GDPR/privacy requests

## Troubleshooting

### Example Won't Compile (Rust)

```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --example basic_adccl_check
```

### Import Error (Python)

```bash
# Verify installation
pip show chyren

# Reinstall if needed
pip uninstall chyren
pip install chyren
```

### ADCCL Gate Always Closes

- Check your strictness setting (may be too high)
- Review State metadata (may be missing required fields)
- Verify action names are correct
- Enable debug logging for details

## Next Steps

- 📖 Read the [QUICKSTART Guide](../QUICKSTART.md)
- 📚 Review [Architecture Documentation](../docs/ARCHITECTURE.md)
- 🔬 Study [Chiral Thesis](../docs/chiral_thesis.md)
- 👥 Join [Discussions](https://github.com/Mega-Therion/Chyren/discussions)
- 🐛 [Report Issues](https://github.com/Mega-Therion/Chyren/issues)

## Contributing Examples

We welcome community examples! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:

- Adding new example categories
- Documenting complex use cases
- Sharing integration patterns
- Improving existing examples

## License

All examples are licensed under the same license as Chyren. See [LICENSE](../LICENSE) for details.

---

**Ready to build sovereign intelligence? Start with the examples above! 🚀**

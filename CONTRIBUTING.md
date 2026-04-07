# Contributing to Chyren

First off, thank you for considering contributing to Chyren! It's people like you that make Chyren a reality for sovereign AI systems.

## 🎯 Vision

Chyren is not just another AI framework — it's a **fundamental rethinking** of how AI systems can maintain integrity, verifiability, and alignment. Every contribution should advance our mission of creating truly sovereign, cryptographically verified intelligence.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Contribution Guidelines](#contribution-guidelines)
- [Architecture Principles](#architecture-principles)
- [Testing Requirements](#testing-requirements)
- [Documentation Standards](#documentation-standards)
- [Pull Request Process](#pull-request-process)
- [Recognition](#recognition)

## 📜 Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to uphold:

### Our Pledge

- **Maintain constitutional alignment**: Respect the Vettagrammaton integrity model
- **Verify before trust**: All claims must be cryptographically provable
- **Embrace scrutiny**: ADCCL threshold 0.7 applies to all contributions
- **Challenge with respect**: Disagreement on technical grounds is encouraged
- **Never compromise the Master Ledger**: Integrity is non-negotiable

### Unacceptable Behavior

- Circumventing verification mechanisms
- Introducing drift or alignment bypasses
- Submitting unverified or malicious code
- Harassing or discriminating against contributors
- Violating cryptographic proof requirements

## 🤝 How Can I Contribute?

### Reporting Bugs

**Security vulnerabilities**: Please email `mega_therion@yahoo.com` instead of filing a public issue.

For bugs:

1. **Check existing issues** to avoid duplicates
2. **Use the bug report template**
3. **Include**:
   - Chyren version
   - Rust/Python versions
   - Operating system
   - Minimal reproduction steps
   - Expected vs actual behavior
   - Relevant logs (with sensitive data redacted)

### Suggesting Enhancements

1. **Open a Discussion** first for major changes
2. **Use the feature request template**
3. **Explain**:
   - The problem you're solving
   - How it aligns with Chyren's principles
   - Impact on ADCCL/verification/phylactery
   - Backward compatibility considerations

### Your First Contribution

Looking to help but don't know where to start?

- Issues labeled `good-first-issue` are perfect for newcomers
- Issues labeled `help-wanted` need community assistance
- Check the [Architecture documentation](./docs/AEGIS.md) to understand the system

## 🛠️ Development Setup

### Prerequisites

- **Rust** 1.75+ with `cargo`
- **Python** 3.12+
- **PostgreSQL** 14+ (for ledger storage)
- **Git** with LFS enabled

### Local Setup

```bash
# Clone the repository
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren

# Install Rust dependencies
cargo build

# Run tests
cargo test

# Install Python dependencies
cd chyren_py
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -e .[dev]

# Run Python tests
pytest
```

### Running Chyren Locally

```bash
# Start the Hub
cargo run --bin chyren-hub

# In another terminal, start a spoke
cargo run --bin chyren-spoke -- --spoke anthropic
```

## ✅ Contribution Guidelines

### Code Standards

**Rust**:
- Follow `rustfmt` formatting: `cargo fmt`
- Pass `clippy` lints: `cargo clippy -- -D warnings`
- Maintain type safety — no `unsafe` without justification
- Document all public APIs with examples

**Python**:
- Follow PEP 8 and Black formatting
- Type hints required for all functions
- Docstrings in Google style format

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) spec:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `chore`: Maintenance (deps, config)
- `test`: Adding/updating tests
- `refactor`: Code restructuring
- `perf`: Performance improvements
- `security`: Security patches

**Examples**:
```
feat(adccl): implement gradient verification thresholds

Added support for configurable ADCCL thresholds (0.5, 0.7, 0.9)
to allow fine-grained control over drift detection.

Closes #142
```

## 🏗️ Architecture Principles

All contributions must respect these core principles:

### 1. Verification First
Every state transition MUST be cryptographically verifiable:
- Generate proofs for all mutations
- Never bypass the Master Ledger
- Maintain Merkle tree integrity

### 2. ADCCL Compliance
All AI interactions must pass through ADCCL:
- Threshold: 0.7 (configurable for experiments)
- Rejection triggers regeneration
- No silent failures

### 3. Phylactery Immutability
The 50,000-entry kernel is sacred:
- Modifications require cryptographic signatures
- Drift detection triggers immediate restoration
- Memory corruption is unacceptable

### 4. Hub-Spoke Isolation
- Spokes never directly communicate
- All routing through Hub
- Provider-agnostic interfaces

## 🧪 Testing Requirements

All PRs must include:

1. **Unit Tests**
   - Rust: `#[cfg(test)]` modules
   - Python: pytest with >80% coverage

2. **Integration Tests**
   - Test full verification flows
   - Include ADCCL threshold scenarios
   - Validate ledger consistency

3. **Property-Based Tests** (for critical components)
   - Use `proptest` (Rust) or `hypothesis` (Python)
   - Test invariants: "all transactions are signed"

4. **Documentation Tests**
   - All code examples in docs must compile/run

### Running Tests

```bash
# Rust unit tests
cargo test

# Rust integration tests
cargo test --test integration_tests

# Python tests with coverage
pytest --cov=chyren_py --cov-report=html

# Documen tests
cargo test --doc
```

## 📚 Documentation Standards

### Code Documentation

**Rust**:
```rust
/// Verifies a state transition using the Master Ledger.
///
/// # Arguments
///
/// * `prev_state` - The previous system state
/// * `next_state` - The proposed next state
/// * `proof` - Cryptographic proof of validity
///
/// # Returns
///
/// `Ok(true)` if verification succeeds, `Err` otherwise.
///
/// # Examples
///
/// ```
/// let result = verify_transition(&prev, &next, &proof)?;
/// assert!(result);
/// ```
pub fn verify_transition(prev_state: &State, next_state: &State, proof: &Proof) -> Result<bool>
```

**Python**:
```python
def verify_transition(prev_state: State, next_state: State, proof: Proof) -> bool:
    """Verifies a state transition using the Master Ledger.
    
    Args:
        prev_state: The previous system state
        next_state: The proposed next state
        proof: Cryptographic proof of validity
    
    Returns:
        True if verification succeeds
    
    Raises:
        VerificationError: If proof is invalid
    
    Example:
        >>> result = verify_transition(prev, next, proof)
        >>> assert result
    """
```

### Architecture Documentation

- Update `docs/` for architectural changes
- Include Mermaid diagrams for flows
- Reference the Chiral Thesis for mathematical foundations

## 🔄 Pull Request Process

### Before Submitting

- [ ] Tests pass locally: `cargo test && pytest`
- [ ] Code is formatted: `cargo fmt && black .`
- [ ] Lints pass: `cargo clippy`
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] Branch is up-to-date with `main`

### PR Template

```markdown
## Description
[Brief description of changes]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
[Describe tests added/modified]

## ADCCL Impact
[Describe impact on verification thresholds]

## Breaking Changes
[List any breaking changes]

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Changelog entry added
```

### Review Process

1. **Automated checks** must pass (CI/CD)
2. **Code review** by at least one maintainer
3. **Architecture review** for significant changes
4. **Security review** for cryptographic changes

### Merge Criteria

- All checks green ✅
- Approved by maintainer(s)
- No unresolved conversations
- Branch up-to-date with `main`

## 🏆 Recognition

### Contributors

All contributors are recognized in:
- `CONTRIBUTORS.md` (automatically updated)
- GitHub contributors graph
- Monthly community calls

### Special Recognition

- **🥇 Core Contributors**: Significant architectural contributions
- **🔒 Security Researchers**: Responsible vulnerability disclosure
- **📚 Documentation Heroes**: Major documentation improvements
- **🧪 Test Champions**: Extensive test coverage additions

## 📞 Getting Help

- **GitHub Discussions**: General questions and ideas
- **GitHub Issues**: Bugs and feature requests
- **Discord**: Real-time chat (link in README)
- **Monthly Dev Calls**: Community sync (calendar in README)

## 📄 License

By contributing to Chyren, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](./LICENSE)).

---

**Remember**: Every line of code in Chyren is part of a cryptographically verified chain of truth. Your contribution becomes part of history the moment it's merged into the Master Ledger.

**Routes intelligence. Verifies truth. Remembers everything.**

```
##  Ω "Truth is not negotiated. It is verified." Ω
```

Thank you for helping build the future of sovereign AI! 🚀

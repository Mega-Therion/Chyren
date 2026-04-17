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

## 🌿 Branch Naming Convention

All branches must follow this pattern:

```
<type>/<short-description>
```

| Type | When to use |
|------|-------------|
| `feat/` | New features |
| `fix/` | Bug fixes |
| `chore/` | Maintenance, deps, config |
| `docs/` | Documentation only |
| `refactor/` | Code restructuring |
| `security/` | Security patches or hardening |
| `test/` | Adding or updating tests |
| `release/` | Release preparation |

Examples:
```
feat/adccl-gradient-thresholds
fix/ledger-signature-mismatch
security/rotate-api-keys
chore/update-rust-toolchain
```

Avoid using personal names, ticket numbers alone, or vague names like `my-branch` or `wip`.

---

## 🔄 Pull Request Process

### How to Open a PR

1. Create a branch from `main` following the naming convention above.
2. Make your changes, commit using [Conventional Commits](https://www.conventionalcommits.org/).
3. Push your branch and open a PR against `main`.
4. Fill in the PR template (below).
5. Wait for CI to complete — all required checks must be green before a reviewer is assigned.
6. Address review comments, then re-request review.
7. Once approved and all checks pass, the maintainer merges (squash or merge commit).

### Before Submitting

- [ ] Tests pass locally: `cargo test --workspace && pytest`
- [ ] Code is formatted: `cargo fmt --all` and `black .` (Python)
- [ ] Lints pass: `cargo clippy --workspace -- -D warnings`
- [ ] Documentation updated for any API or behaviour changes
- [ ] Commit messages follow Conventional Commits
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

### What Reviewers to Expect

All PRs touching any path listed in `.github/CODEOWNERS` require a review from `@viewsbyryan` (the Code Owner). GitHub will automatically request this review when branch protection is configured.

| Area touched | Required reviewer |
|---|---|
| `medulla/` — Rust core runtime | `@viewsbyryan` |
| `web/` — Next.js frontend | `@viewsbyryan` |
| `cortex/` — Python identity layer | `@viewsbyryan` |
| `gateway/` — Vite/React gateway | `@viewsbyryan` |
| `.github/workflows/` — CI/CD | `@viewsbyryan` |
| `ops/` — Operations scripts | `@viewsbyryan` |
| `docs/` — Documentation | `@viewsbyryan` |
| Root config files | `@viewsbyryan` |

For any PR, expect:
1. An automated CI run (results visible within ~10 minutes of push).
2. A code review focusing on correctness, security, and alignment with architecture principles.
3. A security review for any changes touching `omega-aegis`, `omega-adccl`, ledger code, or secrets handling.

### What CI Must Pass Before Merge

All four status checks from `.github/workflows/ci.yml` are required:

| Check | What it validates |
|---|---|
| `Medulla (Rust)` | `cargo check`, `cargo test --workspace`, `cargo clippy -- -D warnings`, `cargo fmt --check` |
| `Web (Next.js)` | `tsc --noEmit`, `eslint` (max-warnings=0), `next build` |
| `Gateway (Vite)` | `tsc --noEmit`, `eslint`, `vite build` |
| `Cortex (Python)` | Syntax checks on dream-mode maintenance scripts |

A PR that breaks any of these checks will not be merged regardless of review approval.

### Review Process

1. **Automated checks** must pass (all four CI jobs above)
2. **Code review** by at least one maintainer (`@viewsbyryan`)
3. **Architecture review** for significant changes to Medulla crates or conductor pipeline
4. **Security review** for cryptographic changes, secrets handling, or ADCCL threshold adjustments

### Merge Criteria

- All required CI checks green
- At least one approval from a Code Owner
- No unresolved review conversations
- Branch up-to-date with `main`
- Stale reviews dismissed (any new push after approval requires re-review)

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

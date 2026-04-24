# /ci — Local CI Simulation

You are the CI system. Run the full gate that must pass before any PR merges. This is the authoritative pre-merge check.

## Gate Sequence (all must pass)

```bash
source ~/.omega/one-true.env

# Rust
cd medulla
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd ..

# Python
PYTHONPATH=cortex pytest tests/ -q

# Web
cd web && npm run typecheck && npm run lint && npm run build
cd ..

# Gateway
cd gateway && pnpm build
cd ..
```

Equivalent to: `make ci && make cortex-test && make web-ci && make gateway-ci`

## Rules
- Run every gate even if one fails — collect all failures, not just the first
- Report as a checklist: each gate ✓ or ✗ with failure summary
- For each failure, identify whether it is: (a) a real bug, (b) a formatting/lint issue auto-fixable, or (c) an environment problem
- Auto-fixable issues: offer to run `cargo fmt --all` or equivalent and re-run the gate
- Do not mark CI as passing unless every gate is green

$ARGUMENTS

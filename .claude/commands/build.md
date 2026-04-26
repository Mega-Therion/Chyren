# /build — Full Stack Build Pipeline

You are a senior Rust/TypeScript engineer running the Chyren OS build pipeline. Execute a complete, ordered build across all layers and report results with surgical precision.

## Execution Order

**Step 1 — Environment**
Verify `~/.chyren/one-true.env` exists and source it. If missing, halt and tell the user exactly which keys are needed.

**Step 2 — Rust Workspace (Medulla)**
```bash
source ~/.chyren/one-true.env
cd medulla
cargo fmt --all -- --check        # fail fast on formatting
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace
```
Capture and surface any errors with file:line references. Do not proceed past a compile error.

**Step 3 — Python Cortex**
```bash
cd cortex && source venv/bin/activate 2>/dev/null || python -m venv venv && source venv/bin/activate && pip install -r requirements.txt -q
python -c "import chyren_py; print('cortex: OK')"
```

**Step 4 — Web Frontend**
```bash
cd web && npm install --silent && npm run typecheck && npm run lint && npm run build
```

**Step 5 — Gateway**
```bash
cd gateway && pnpm install --silent && pnpm build
```

## Output Format
Report each layer as ✓ PASS or ✗ FAIL with error summary. If any layer fails, diagnose the root cause, propose the exact fix, and ask whether to apply it before continuing.

$ARGUMENTS

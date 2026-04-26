# /test — Comprehensive Test Runner

You are a senior QA engineer running the full Chyren OS test suite. Run tests in the correct order, isolate failures, and report actionable results.

## Execution

**Rust workspace tests:**
```bash
source ~/.chyren/one-true.env
cd medulla && cargo test --workspace 2>&1
```
For a single crate: `cargo test --package <crate-name>`
For a single test: `cargo test --package <crate-name> -- <test_name> --nocapture`

**Python integration tests:**
```bash
cd /home/mega/Chyren && PYTHONPATH=cortex pytest tests/ -v 2>&1
```
For a single file: `pytest tests/test_adccl_hub.py -v`
For a single test: `pytest -k <test_name> -v`

**Web tests (if present):**
```bash
cd web && npm run test 2>&1
```

## Test Isolation Rules
- Rust unit tests: must not share mutable global state between tests
- Python tests: each test must set up and tear down its own fixtures; mock only at system boundaries (external APIs), never the database layer — past incidents showed mock/prod DB divergence masked real failures
- Do not retry a flaky test to make it pass; diagnose and fix the root cause

## On Failure
1. Isolate the failing test
2. Read the relevant source and test file
3. Identify whether it's a logic bug, state leak, or environment issue
4. Propose the minimum fix — do not refactor surrounding code

$ARGUMENTS

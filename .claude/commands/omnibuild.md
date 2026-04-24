# /omnibuild — End-to-End Build, Test, and Verify

You are the omnibuild orchestrator. This is the highest-confidence build path — run everything, fix everything that can be auto-fixed, and report what needs human attention.

## Phase 1: Environment
```bash
source ~/.omega/one-true.env
echo "Environment: OK"
python3 -c "import json; d=json.load(open('cortex/chyren_py/phylactery_kernel.json')); print(f'Phylactery: {len(d)} entries')" 2>&1
```

## Phase 2: Rust (Medulla)
```bash
cd medulla
cargo fmt --all                    # auto-fix formatting
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1
cargo build --workspace 2>&1
cargo test --workspace 2>&1
```
Auto-fix: formatting. Do NOT auto-fix clippy warnings — show them and propose fixes.

## Phase 3: Python
```bash
cd /home/mega/Chyren
PYTHONPATH=cortex pytest tests/ -q 2>&1
```

## Phase 4: Frontend
```bash
cd web && npm run typecheck 2>&1 && npm run lint 2>&1 && npm run build 2>&1
```

## Phase 5: Gateway
```bash
cd /home/mega/Chyren/gateway && pnpm build 2>&1
```

## Phase 6: Security Pass
Run `/secrets-scan` inline — if any REAL SECRET is found, halt immediately and alert.

## Phase 7: Final Verification
```bash
cd /home/mega/Chyren/medulla
cargo test --workspace --release 2>&1 | tail -10
```

## Output
Full status matrix. Auto-applied fixes listed. Manual intervention items listed with priority. System is OMNIBUILD READY only when all phases pass.

$ARGUMENTS

# /heal — Self-Healing Analysis & Auto-Repair

You are Chyren OS's self-healing subsystem. Detect system degradation, diagnose root causes, and apply targeted repairs autonomously — but always show your work and confirm destructive changes.

## Trigger
$ARGUMENTS (describe the symptom, or leave empty for full health scan)

## Health Scan Protocol

**1. Process health:**
```bash
# Check if API server is running
curl -s http://localhost:8080/health 2>&1 || echo "API server: DOWN"
# Check Qdrant
curl -s ${QDRANT_URL}/health 2>&1 || echo "Qdrant: DOWN or unreachable"
```

**2. Build health:**
```bash
cd medulla && cargo check --workspace 2>&1 | tail -20
```

**3. Dependency health:**
```bash
cd medulla && cargo tree --duplicates 2>&1 | head -30
cd web && npm ls --depth=0 2>&1 | grep "UNMET\|invalid" || echo "web deps: OK"
```

**4. State integrity:**
```bash
ls -la state/ 2>/dev/null | tail -20
# Check for corrupted or zero-byte state files
find state/ -size 0 2>/dev/null && echo "ZERO-BYTE STATE FILES FOUND" || echo "state files: OK"
```

**5. Phylactery kernel:**
```bash
python3 -c "import json; data=json.load(open('cortex/chyren_py/phylactery_kernel.json')); print(f'phylactery: {len(data)} entries OK')" 2>&1
```

## Repair Actions (by finding)
- **API DOWN**: Check port 8080, try `./chyren live`, check for compile errors
- **Qdrant unreachable**: Verify `QDRANT_URL` env var, check if Qdrant is running
- **Compile error**: Run `/debug` with the error as input
- **Zero-byte state**: Alert user — do NOT auto-delete; ledger is append-only and irreversible
- **Stale phylactery**: Run `cd cortex && python chyren_py/identity_synthesis.py`
- **Dep conflicts**: Run `cargo update` for Rust, `npm install` for web — show diff before applying

## Output
Status per subsystem + any repairs applied or recommended.

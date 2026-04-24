# /cortex-sync — Cortex ↔ Medulla Synchronization Audit

You are the cross-layer synchronization engineer. Cortex (Python) is the legacy/maintenance layer; Medulla (Rust) is the live runtime. This skill audits that they stay coherent.

## Sync Dimensions

**1. Provider parity check:**
```bash
# Python providers
ls cortex/providers/*.py 2>/dev/null | xargs -I{} basename {} .py | sort

# Rust spokes
ls medulla/omega-spokes/src/spokes/*.rs 2>/dev/null | xargs -I{} basename {} .rs | sort
```
Flag any provider in Cortex that has no Rust spoke equivalent — that's a migration gap.

**2. ADCCL parity:**
```bash
# Python ADCCL implementation (if any)
grep -rn "adccl\|drift\|hallucination\|threshold" cortex/ 2>/dev/null | head -20

# Rust ADCCL
grep -rn "threshold\|0\.7\|DriftScore" medulla/omega-adccl/src/ 2>/dev/null | head -20
```
Verify both use the same threshold and flag set.

**3. API contract parity:**
```bash
# Python response shape
grep -rn "class.*Response\|return.*{" cortex/core/ cortex/providers/ 2>/dev/null | head -20

# Rust response types
grep -rn "struct.*Response\|pub.*response" medulla/omega-core/src/ 2>/dev/null | head -20
```

**4. Legacy bridge status:**
```bash
ls medulla/*/src/legacy_bridge.rs 2>/dev/null || echo "No legacy bridges found"
```

**5. Dream mode paths:**
```bash
# What does ./chyren dream actually run?
grep -n "dream" chyren 2>/dev/null | head -10
# Python scripts executed during dream
ls cortex/ops/scripts/ 2>/dev/null
```

## Output
Parity matrix per provider/feature. Migration gaps listed. Any places Cortex is still invoked during live requests (should be zero) flagged as CRITICAL.

$ARGUMENTS

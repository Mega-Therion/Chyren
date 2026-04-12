# Statsig Bootstrap Blueprint for Chyren

Current connector state: no gates, no experiments configured.

## 1) First Gates (Feature Flags)
Create these first:
1. `adccl_pre_persist_gate_enabled`
2. `reject_repair_path_enabled`
3. `myelin_persist_on_pass_only`
4. `proof_pack_trend_export_enabled`

Purpose: ensure verification-before-persistence behavior can be toggled and audited safely.

## 2) First Dynamic Configs
1. `adccl_threshold_config`
   - `accept_threshold`: `0.7`
   - `max_repair_attempts`: `2`
2. `runtime_routing_config`
   - `prefer_medulla_for`: `["benchmark","telemetry","cli"]`
   - `prefer_cortex_for`: `["policy","verification","routing"]`

Purpose: externalize sensitive control constants and routing knobs.

## 3) First Experiments
1. `exp_adccl_threshold_070_vs_075`
   - Control: `0.70`
   - Test: `0.75`
   - Primary metrics: reject rate, false reject proxy, latency delta
2. `exp_repair_budget_2_vs_3`
   - Control: repair budget `2`
   - Test: repair budget `3`
   - Primary metrics: completion rate, mean response latency

## 4) Metrics to Register
- `verification_pass_rate`
- `verification_fail_rate`
- `reject_repair_recovery_rate`
- `end_to_end_latency_ms`
- `persist_after_pass_compliance_rate`

## 5) Rollout Sequence
1. Create gates and configs first.
2. Validate on non-production environment.
3. Launch one experiment at a time.
4. Record deltas into `docs/evidence/v0.2/summary.md`.

## 6) Guardrails
- Never run overlapping experiments on the same threshold parameter.
- Keep one source of truth for ADCCL constants (Statsig config or repo config, not both).
- Tie every experiment result to a proof-pack run ID before publishing claims.

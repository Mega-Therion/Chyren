# SPEC-001: The Sovereign Observer (Special Assistant)

## Overview
The Sovereign Observer is an asynchronous, event-driven agent designed to monitor Chyren’s cognitive state. It functions as an autonomous "debugger" that records systemic failures and performance anomalies, allowing the orchestrator to perform meta-learning on its own internal state.

## Architecture
- **Passive Telemetry Listener**: Operates on a dedicated, non-blocking `tokio` thread.
- **Event-Driven**: Only triggers on specific hooks:
    1. `ADCCL_REJECTION` (Score < 0.7).
    2. `CRITICAL_SYS_ERROR`.
    3. `PERFORMANCE_LATENCY_SPIKE` (> 2s response).
- **Storage**: Append-only `state/observer_journal.json`.

## Responsibilities
1. **Anomaly Logging**: Captures the input task, the model output (or failure), the ADCCL rejection reason, and the emotional vector snapshot at time of failure.
2. **Evidence Matrix Maintenance**: Updates a structured matrix of "Failure Contexts" — identifying patterns like "Chyren fails when asked to perform multi-step math on DeepSeek provider."
3. **Autonomous Meta-Querying**: Exposes a tool for Chyren to ask: "QueryEvidenceMatrix(problem_type: 'math_scaling')"

## Performance Constraints
- **Zero-Blocking**: The primary cognitive loop *never* waits for the Observer.
- **Eviction Policy**: Successful routine task logs are purged after 24 hours. Only "Rejection" and "Error" data is retained indefinitely.

## Implementation Plan
1. `medulla/omega-conductor/src/agents/observer.rs`: Implement the listener.
2. `omega-telemetry`: Add event-broadcasting for `ADCCL_REJECTION`.
3. `omega-myelin`: Add storage support for the `EvidenceMatrix`.

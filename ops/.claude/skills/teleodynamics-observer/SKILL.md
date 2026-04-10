---
name: teleodynamics-observer
description: Real-time system integrity monitor. Aggregates trace events across all layers to identify systemic 'authority shrink' before it causes failure.
---

# Governance
- Enforce AEGIS Run Envelope.
- Emit Teleodynamics traces.

# Core Logic
Streams `TELEODYNAMICS_TRACE` from all logs. Alerts on drift clusters and authority-shrink spikes.
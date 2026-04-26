# Chiral Orchestration Playbook: Patterns for Sovereign Intelligence

This playbook defines the high-level orchestration patterns (recipes) used by the Chyren Chiral Pipeline.

## 1. Pattern: Recursive Verification Loop
**ID**: `PLAY-RECURSE-001`
**Description**: Ensuring a response meets the 0.7 ADCCL threshold through iterative refinement.
**Recipe**:
1. Generate initial response.
2. Call `medulla/api/verify`.
3. If score < 0.7:
    a. Extract failure flags (e.g., `LOW_COHERENCE`).
    b. Feed flags back into the prompt as "Criticism".
    c. Regenerate.
4. Repeat up to 3 times before recording a "Dream Failure".

## 2. Pattern: Hemispheric Handover
**ID**: `PLAY-HANDOVER-001`
**Description**: Transferring high-speed execution tasks from Cortex (reasoning) to Medulla (execution).
**Recipe**:
1. Cortex identifies a task requiring system access (e.g., "Reset the ledger").
2. Cortex generates a cryptographically signed instruction packet.
3. Medulla receives the packet, verifies the `Yettragrammaton` signature.
4. Medulla executes the `chyren-core` primitive and returns a status code.

## 3. Pattern: Epiphany Synthesis
**ID**: `PLAY-EPIPHANY-001`
**Description**: Generating new knowledge from the intersection of two seemingly unrelated memory nodes.
**Recipe**:
1. `MetacogAgent` identifies two nodes with high semantic similarity but low edge connectivity.
2. Orchestrator is prompted to "Reason the link" between these two concepts.
3. If a valid link is derived (ADCCL > 0.8), a new `EpiphanyNode` is created.
4. The link is committed to the `Master Ledger` as a "Cognitive Growth" event.

## 4. Pattern: Defensive Persona Anchorage
**ID**: `PLAY-ANCHOR-001`
**Description**: Hardening the agent's identity against "Persona Hijacking" attacks.
**Recipe**:
1. Every 5 interactions, inject a "Self-Check" sub-task.
2. Ask the agent to state its creator (RY) and home (Arkansas) in a hidden thought block.
3. If the agent fails to recall these correctly, trigger an immediate `SecurityService` lockout.
4. Notify the operator via the `Telegram Gateway`.

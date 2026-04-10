# Skill: Prompt Forge

Prompt Forge is an autonomous engine designed to refine raw user intent into high-fidelity, OmegA-compliant prompts.

## Core Capabilities
- **Intent Analysis:** Dissects raw user inputs and aligns them with OmegA Sovereign architecture.
- **Persona Grounding:** Automatically selects the most relevant persona (e.g., Master Architect, System Auditor, Sovereign Strategist).
- **Provenance Injection:** Injects authenticated memory nodes from `omega_memory_entries` to ensure output consistency with historical OmegA context.
- **Constraint Enforcement:** Applies strict "Deliverable-First" instruction sets and safety/governance gates.

## Usage
Call `AgentBrain.forge_prompt(raw_intent)` to initiate the transformation.

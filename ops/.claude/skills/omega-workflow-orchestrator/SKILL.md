---
name: omega-workflow-orchestrator
description: Automate complex, multi-step development workflows for OmegA by analyzing codebase state and orchestrating other skills. Use for high-level tasks like "implement feature X", "refactor module Y", "audit system Z", or "upgrade dependency W".
---

# Omega Workflow Orchestrator

## Overview

This skill functions as a high-level orchestrator for the OmegA system. It bridges the gap between vague user intents (e.g., "improve the logging system") and concrete execution steps. It analyzes the current codebase state, consults the `SKILLS_ROADMAP.md`, and generates execution plans that may involve invoking other specialized skills or running specific scripts.

## Core Capabilities

1.  **Workflow Generation**: Decomposes high-level goals into actionable steps.
2.  **Codebase Analysis**: Uses `grep`, `glob`, and `read_file` to understand the current state of the project before acting.
3.  **Skill Orchestration**: Identifies and activates other skills (like `backend-development`, `neon-postgres`, `omega-web-tools`) as needed.
4.  **Roadmap Alignment**: Ensures actions are consistent with `OmegA-Architecture/SKILLS_ROADMAP.md`.

## Workflows

### 1. Feature Implementation
**Trigger**: "Add feature X", "Implement Y"

1.  **Context Gathering**:
    - Search for existing related code.
    - Read `OmegA-Architecture/SKILLS_ROADMAP.md` to see if the feature is planned.
    - Check `OmegA-Architecture/.md` for architectural constraints.

2.  **Plan Formulation**:
    - Draft a step-by-step plan.
    - Identify required skills.

3.  **Execution**:
    - Activate necessary skills.
    - Execute steps sequentially.
    - Verify each step.

### 2. System Audit & Refactoring
**Trigger**: "Audit X", "Refactor Y", "Clean up Z"

1.  **Mapping**:
    - Map the target directory structure.
    - Identify stale files or dependencies.

2.  **Analysis**:
    - Check for specific patterns (e.g., "TODO", deprecated API usage).
    - Compare against "Spec-to-Code" standards.

3.  **Remediation**:
    - Propose changes.
    - Execute refactoring.

### 3. Automation Setup
**Trigger**: "Automate X", "Create a script for Y"

1.  **Requirement Analysis**:
    - Determine input/output requirements.
    - Identify environment constraints.

2.  **Script Generation**:
    - Write a standalone script (Python/Bash) to perform the task.
    - Ensure the script is idempotent and has error handling.
    - Place the script in `OmegA-Architecture/scripts/` or `~/bin/`.

3.  **Validation**:
    - Run the script with a test case.
    - Verify the output.

## Reference Material

-   **`references/skill_registry.md`**: A list of available skills and their primary use cases.

## Usage Guidelines

-   **Always** verify the current state of `OmegA-Architecture` before proposing changes.
-   **Prefer** creating reusable scripts over one-off manual commands.
-   **Maintain** the "Sovereign" nature of OmegA—keep dependencies local and controllable where possible.

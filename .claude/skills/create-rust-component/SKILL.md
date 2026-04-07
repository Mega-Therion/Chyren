---
name: create-rust-component
description: Scaffolds a new Rust component within the OmegA-Next workspace.
disable-model-invocation: true
---

## Create Rust Component

This skill helps you scaffold a new Rust component within the `omega_workspace/workspace/OmegA-Next/` directory.

### Usage

To create a new Rust component, run this skill and provide the `component_name` and `component_type` (e.g., `lib` or `bin`).

```bash
/create-rust-component component_name:<name> component_type:<type>
```

### Example

```bash
/create-rust-component component_name:omega-new-feature component_type:lib
```

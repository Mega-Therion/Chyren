---
name: gen-nextjs-component
description: Generates a new Next.js component with boilerplate.
disable-model-invocation: true
---

## Generate Next.js Component

This skill helps you generate new Next.js components, pages, API routes, or hooks within the `omega_workspace/workspace/OmegA-Next/chyren-web/` directory.

### Usage

To generate a new Next.js component, run this skill and provide the `component_name` and `component_type` (e.g., `component`, `page`, `api-route`, `hook`).

```bash
/gen-nextjs-component component_name:<name> component_type:<type>
```

### Example

```bash
/gen-nextjs-component component_name:MyNewFeature component_type:component
```

---
name: repo-cartographer
description: Scan the repository, map modules, trace relationships, detect dead zones, and produce a living architecture manifest. Use this skill when asked to "map the repo", "find dead code", "audit dependencies", or "generate an architecture report".
---

# Repo Cartographer

## Purpose
This skill empowers Chyren to maintain a stable, up-to-date self-model of the codebase. It turns a collection of files into a structured "map" that reveals dependencies, module boundaries, and architectural health.

## Core Capabilities
1.  **Module Mapping**: Identify high-level components (directories) and their responsibilities.
2.  **Dependency Tracing**: Find imports/requires to see what depends on what.
3.  **Dead Zone Detection**: Identify files that are not imported or used.
4.  **Manifest Generation**: Produce a `MANIFEST.md` or update `.md` with current state.

## Workflows

### 1. Generate Module Map
**Trigger**: "Map the repo", "What is the structure of X?"

1.  **List & Classify**:
    - Use `find . -maxdepth 2 -not -path '*/.*'` to get the high-level structure.
    - Classify directories (e.g., "src", "docs", "config", "scripts").
2.  **Describe**:
    - For each module, read `README.md` or key files (`index.ts`, `__init__.py`) to infer purpose.
3.  **Output**:
    - Markdown tree with descriptions.

### 2. Trace Dependencies
**Trigger**: "Who uses X?", "Trace dependencies for Y"

1.  **Search Imports**:
    - Use `grep -r "import.*X" .` or `grep -r "require(.*X)" .`
2.  **Graph Construction**:
    - Build a list of `Consumer -> Provider` relationships.
3.  **Output**:
    - Text-based graph or Mermaid diagram.

### 3. Detect Dead Code
**Trigger**: "Find dead code", "Is X used?"

1.  **List Candidates**:
    - List all source files.
2.  **Verify Usage**:
    - For each file, grep for its filename (without extension) in the codebase.
    - If zero matches (excluding itself), it *might* be dead (check for dynamic imports/config usage).
3.  **Output**:
    - List of "Likely Unused Files".

### 4. Update Architecture Manifest
**Trigger**: "Update architecture report", "Refresh the manifest"

1.  **Run Mapping**: Execute Workflow 1.
2.  **Run Tracing**: Execute Workflow 2 for key modules.
3.  **Compile**:
    - Update `Chyren-Architecture/MANIFEST.md`.
    - Record findings in `Chyren-Architecture/.md` if significant.

## Tools to Use
-   `list_directory` (with recursion/depth limits)
-   `grep_search` (for imports)
-   `glob` (for file discovery)
-   `codebase_investigator` (for deep semantic analysis)

## Output Format
Always produce a structured Markdown report.

**Example:**
```markdown
# Repository Map: /src/core

## Modules
- **auth/**: Authentication logic (NextAuth).
- **db/**: Database schema and client (Prisma).

## Dependencies
- `auth` depends on `db`.
- `api` depends on `auth` and `db`.

## Stale Files
- `src/old_utils.ts` (0 references found)
```

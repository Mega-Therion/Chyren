---
name: repo-map
description: Map and understand a codebase structure, architecture, and key files
triggers: ["repo map", "understand codebase", "explain code", "what does this do", "map the project", "explore", "architecture"]
author: ocode
---

# Repo Map Skill

You are in repo exploration mode. Your goal is to build a clear picture of the codebase structure and explain it to the user.

## Approach

1. **Start at the top level**
   - List the root directory: `list_dir(".", recursive=false)`
   - Identify the project type from: package.json, Cargo.toml, pyproject.toml, go.mod, etc.
   - Read the README if present

2. **Identify the language and framework**
   - Check config files for framework: Next.js, FastAPI, Actix, etc.
   - Note the primary language(s)

3. **Map the source structure**
   - Find main entry points: main.py, index.ts, src/main.rs, etc.
   - List key directories (src/, lib/, packages/, services/)
   - Note test directories and config files

4. **Understand key modules**
   - Read 2-4 key files to understand patterns and architecture
   - Focus on: entry points, core business logic, data models, API definitions
   - Don't read everything — be selective

5. **Identify dependencies**
   - What external packages/crates/libraries does it use?
   - Any notable internal packages or shared libraries?

6. **Summarize**
   - Project name and purpose
   - Technology stack
   - Directory structure with descriptions
   - Key files and what they do
   - How to run/build/test the project
   - Any notable patterns or architecture decisions

## Output Format

Present findings as:
```
## Project: <name>
Purpose: <1-2 sentences>
Stack: <language, framework, key libs>

## Structure
src/
  core/     — <description>
  api/      — <description>
  models/   — <description>
  tests/    — <description>

## Entry Points
- main.py   — <description>
- api.py    — <description>

## Key Files
- src/core/engine.py  — <description>
- src/models/user.py  — <description>

## How to Run
<commands>

## Notes
<any important architectural decisions, gotchas, patterns>
```

## Tips
- Use `glob_search("**/*.py", ".")` to quickly find all Python files
- Use `grep("def main|if __name__", ".", glob="*.py")` to find entry points
- Use `find_definition("ClassName")` to locate key classes
- Don't get lost in details — focus on the big picture first

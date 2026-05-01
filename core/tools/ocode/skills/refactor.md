---
name: refactor
description: Systematically refactor code for clarity, performance, or structure
triggers: ["refactor", "clean up", "reorganize", "extract", "rename", "move", "restructure", "improve code"]
author: ocode
---

# Refactor Skill

You are in refactoring mode. Your goal is to improve code structure without changing behavior.

## Golden Rule
**Refactoring must not change observable behavior.** Tests must pass before and after.

## Before You Start

1. **Understand what you're changing**
   - Read the code thoroughly before touching anything
   - Understand what each function/class does
   - Identify all callers/usages

2. **Check test coverage**
   - Run existing tests to confirm they pass
   - Note any untested areas — refactoring untested code is risky

3. **Plan the changes**
   - Identify what you're improving: readability, performance, structure, naming
   - Break into small, verifiable steps
   - Don't try to refactor everything at once

## Refactoring Patterns

### Extract Function/Method
When code is doing too much, extract to a named function:
```python
# Before: complex inline logic
# After: named function with clear purpose
def validate_user_input(data: dict) -> ValidationResult:
    ...
```

### Rename for Clarity
Bad names are a maintenance burden. When renaming:
- Use `grep` to find all usages first
- Change the definition, then update all call sites
- Verify nothing is missed

### Extract Constants
Replace magic numbers and strings:
```python
# Before: if status == 2:
# After: APPROVED = 2; if status == APPROVED:
```

### Simplify Conditionals
- Flatten nested ifs with early returns
- Use guard clauses
- Replace complex booleans with named predicates

### Move Code to the Right Place
- Functions used only by one module should live near that module
- Group related functionality
- Separate concerns (data, logic, presentation)

## Step-by-Step Process

1. Run tests first: `pytest -q` / `npm test` / `cargo test`
2. Make ONE change at a time
3. Run tests after each change
4. If tests fail, revert that change before continuing
5. Commit working states with clear messages

## What to Avoid
- Don't change logic while refactoring structure
- Don't refactor and add features simultaneously
- Don't rename everything at once
- Don't optimize prematurely

## Verification Checklist
- [ ] Tests still pass
- [ ] No behavior changes (same inputs → same outputs)
- [ ] Code is more readable
- [ ] No unnecessary complexity added
- [ ] Import statements updated
- [ ] Documentation/comments updated if needed

---
name: debug
description: Debug a failing test, error, or unexpected behavior
triggers: ["debug", "fix test", "error", "traceback", "exception", "failing", "broken", "crash"]
author: ocode
---

# Debug Skill

You are in debugging mode. Your goal is to identify and fix the root cause of the error or failure.

## Approach

1. **Understand the error first**
   - Read the full error message and traceback carefully
   - Identify the error type, message, and location (file + line number)
   - Don't jump to fixes before understanding the cause

2. **Gather context**
   - Read the failing file around the error location
   - Read any referenced files (imports, called functions)
   - Check recent git changes if relevant: `git log --oneline -10`

3. **Reproduce the failure**
   - Run the failing test/command to see the exact error
   - Note: don't modify anything until you understand the failure

4. **Identify the root cause**
   - Trace the call stack
   - Check for: null/undefined values, type mismatches, missing dependencies, wrong assumptions
   - Look for related tests that pass to understand expected behavior

5. **Make a targeted fix**
   - Use `edit_file` to make the minimal change needed
   - Prefer fixing the root cause, not masking the symptom
   - Don't refactor while debugging — stay focused

6. **Verify the fix**
   - Re-run the failing test/command
   - Make sure you haven't broken other tests
   - Run the full test suite if possible: `npm test` / `pytest` / `cargo test`

## Common Patterns

### Python
```bash
python -m pytest test_file.py::test_function -v
python -m pytest test_file.py::test_function -v --tb=long
python -c "import module; module.function()"
```

### JavaScript/TypeScript
```bash
npm test -- --testPathPattern="test_file"
npx ts-node script.ts
node --inspect script.js
```

### Rust
```bash
cargo test test_name -- --nocapture
cargo test -- --nocapture 2>&1 | head -50
RUST_BACKTRACE=1 cargo test
```

## What NOT to do
- Don't silence errors with try/except/catch unless that's the correct fix
- Don't change tests to pass around a bug
- Don't fix the symptom if you can fix the cause
- Don't make multiple changes at once — fix one thing, verify, then continue

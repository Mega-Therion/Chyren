---
name: cdl-one-shot
description: Use when you need a robust, error-proof command to orchestrate CDL integration and scraper deployment in a single, terminal-compatible execution.
---

# CDL One-Shot Deployment

## Execution
Run this command in your terminal at the project root:

```bash
cd /home/mega/Chyren && python3 scripts/init_cdl.py && python3 scripts/autonomous_enrichment_scraper.py
```

## Why this is robust
- **Helper Script**: Encapsulates the JSON logic in `scripts/init_cdl.py` to avoid terminal indentation errors.
- **Atomic Execution**: Uses `&&` to ensure steps only run on success.
- **Clean**: No complex multi-line command strings to copy-paste.

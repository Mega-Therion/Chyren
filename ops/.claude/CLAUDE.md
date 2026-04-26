# Claude Code Global Configuration

Canonical workspace: `/home/mega/CHYREN_WORKSPACE/`
Full system map: `/home/mega/CHYREN_WORKSPACE/CLAUDE.md`

Active Chyren repo: `/home/mega/CHYREN_WORKSPACE/CANON/Chyren-Architecture`
Archive attic: `/home/mega/CHYREN_WORKSPACE/DOCS/archive`

Rules:
- Treat CANON/Chyren-Architecture as the only live Chyren source of truth.
- All ingest work lives in INGEST/ — use venv at INGEST/venv/bin/python.
- New downloads land in INBOX/ — deduper runs there automatically.
- VAULT/ contains secrets — never log or expose.

Shell protocol:
1. Write scripts to a file first.
2. Give one copy-paste command to run.
3. Do not paste raw multi-line shell sequences for manual entry.

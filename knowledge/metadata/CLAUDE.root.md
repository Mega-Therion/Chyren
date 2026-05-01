# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Orientation

`/home/mega` is the operator's home directory, not a single repository (the `.git/` here is inert). The active codebase is **Chyren** at `~/Chyren/`, which has its own canonical `CLAUDE.md` — read that first for build/test/lint commands, the Medulla (Rust, 17 crates) ↔ Cortex (Python) split, the ADCCL 0.7 verification gate, the Master Ledger / Phylactery / Myelin data layer, and `chyren-os/` layer status.

`.bashrc` auto-`cd`s into `~/Chyren` on shell start and sources `~/.chyren/one-true.env`. Most work happens there.

## Layout of the home directory

| Path | What it is |
|---|---|
| `~/Chyren/` | **Primary project.** Read `~/Chyren/CLAUDE.md` and `~/Chyren/.cursorrules`. |
| `~/.chyren` → `~/Chyren/state/vault/` | Secrets vault. `one-true.env` lives here and is sourced by `.bashrc` automatically. Never commit. |
| `~/bin/chyren` → `~/Chyren/medulla/target/release/chyren` | Symlink to the Rust release binary; **broken until `cargo build --release` runs in `~/Chyren/medulla/`**. `.bashrc` exports `~/bin` on `PATH`. |
| `~/bin/chyren.python_legacy` | Fallback Python wrapper; do not extend — Rust is canonical. |
| `~/bin/omega` | Standalone Bash launcher targeting `chyren_GATEWAY_URL` (default `http://127.0.0.1:8787`). |
| `~/bin/yettragrammaton.py` | Standalone integrity/signing utility (related to the Yettragrammaton hash referenced in Chyren). |
| `~/scripts/` | Loose Python utilities: `validate_chyren_env.py`, `verify_env.py`, `test_chyren.py`, `run_qa.py`, `run_qa_final.py`, `ingest_schemas.py`. They presume `~/.chyren/one-true.env` is sourced. |
| `~/docs/` | Mostly publication artifacts (Millennium Prize submission, Riemann run logs, narrative drafts). Has its own `CLAUDE.md` and `GEMINI.md`. **Not source code** — treat as write-rarely. |
| `~/supabase/functions/` | Supabase edge functions (separate from the `~/Chyren/supabase/` deployment if any). |
| `~/.omega/` | Runtime state for the omega launcher (`dreams.json`, `study-cron.log`). Don't hand-edit. |
| `~/OMEGA_WORKSPACE/` | Sparse — currently only `Documents/`. |
| `~/backups/` | Snapshot/archive directory (`DEPRECATED_MIGRATE/`, `Archive/`, `MEGA/`, etc.). **Read-only by default** — duplicate stale copies of `.cursorrules` and old code live here and will mislead grep. Exclude from searches. |
| `~/.cursor/` | Cursor IDE settings, not project rules. The real Cursor rules are at `~/Chyren/.cursorrules`. |
| `~/GEMINI.md` | Empty placeholder. |

## Working conventions

- **Always source secrets before running anything that touches Chyren**: `source ~/.chyren/one-true.env`. Interactive shells get this for free; non-interactive subshells and `make` targets do not.
- **`.megaignore`** (`node_modules/`, `.cache/`, `*.log`, `*.bak`) controls MEGA cloud sync, not git. Don't repurpose it.
- **`.neon`** holds the Neon org id (`org-nameless-queen-58294501`) used by the Master Ledger Postgres connection.
- **Searches**: scope to `~/Chyren/` for code work. Globbing from `~/` will hit `backups/`, `.cache/`, virtualenvs (`.venv/`, `.venvs/`, `aws-venv/`, `docx_env/`), `node_modules/`, and many AI-tool config dirs (`.codex/`, `.gemini/`, `.cursor/`, `.opencode/`, `.augment/`, `.aider/`, etc.) and bury the signal.
- **Do not delete**: `state/`, `.omega/`, `~/Chyren/state/vault/`, anything under `backups/`. Ledger-style state in this system is append-only by design.
- **Shell aliases** worth knowing (from `.bashrc`): `ari` = `chyren ask`; `gsmart`/`gflash`/`glite`/`gpro` = Gemini CLI variants; `telegram-send` = `~/.gemini/skills/telegram-sender/send.sh`; `aws` resolves to `~/aws-venv/bin/aws`.

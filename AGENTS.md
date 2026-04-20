# Repository Guidelines

## Project Structure & Module Organization
Chyren is a multi-runtime monorepo:
- `medulla/`: Rust workspace (`omega-*` crates) for core execution, verification, and CLI binaries.
- `cortex/`: higher-level reasoning and orchestration logic.
- `web/`: Next.js 15 frontend (`chyren-web`).
- `gateway/`: Vite/React gateway UI and routing surface.
- `tests/`: Python integration and system tests (`test_*.py`).
- `docs/`: architecture, contribution policy, and examples.

Keep changes scoped to one subsystem where possible, and avoid cross-layer coupling unless required by design.

## Build, Test, and Development Commands
Run commands from the relevant directory:
- `cargo build` (repo root or `medulla/`): build Rust workspace.
- `cargo test`: run Rust tests.
- `cargo fmt` and `cargo clippy -- -D warnings`: enforce Rust formatting/lints.
- `cd web && npm run dev`: start Next.js app locally.
- `cd web && npm run build && npm run lint && npm run typecheck`: production checks for web.
- `cd gateway && npm run dev`: run gateway in development.
- `pytest` (repo root): run Python test suite in `tests/`.
- `pytest --cov=chyren_py --cov-report=html`: coverage output (as referenced in docs).

## Coding Style & Naming Conventions
- **Rust**: `rustfmt` clean, clippy-clean, avoid `unsafe` unless justified.
- **Python**: PEP 8 + Black style, type hints required, Google-style docstrings.
- **TypeScript/React**: follow ESLint defaults in each app.
- **Auditory Persona**: Chyren identifies as a sophisticated, warm British male (e.g., ElevenLabs "Brian"). All TTS configuration must prioritize British male voices.
- Test files follow `test_*.py`; keep names behavior-focused (e.g., `test_ledger_hub.py`).
- Prefer clear crate/module naming consistent with existing `omega-*` patterns.

## Testing Guidelines
Add tests with every behavior change:
- Unit tests near implementation (Rust `#[cfg(test)]`, Python module tests).
- Integration tests for cross-component flows (verification, ledger consistency, ADCCL paths).
- For critical invariants, prefer property-based tests (`proptest` / `hypothesis`).
Target minimum Python coverage of 80% per `docs/CONTRIBUTING.md`.

## Commit & Pull Request Guidelines
Use Conventional Commits, as seen in history:
- `feat(scope): ...`, `fix: ...`, `docs: ...`, `test: ...`, `chore: ...`.
- Keep subject imperative and concise; add scope when it improves traceability.

PRs should include:
- clear problem statement and change summary,
- linked issue/discussion when relevant,
- test evidence (commands + result),
- screenshots for UI changes (`web/` or `gateway/`),
- notes on architectural or security impact if touching verification/integrity paths.

## Security & Configuration Tips
Never commit secrets (`.env.local`, API keys, tokens). Report vulnerabilities privately per `docs/CONTRIBUTING.md` (email security contact). Redact sensitive data from logs and test artifacts.

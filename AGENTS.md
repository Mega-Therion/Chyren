# Repository Guidelines

## Project Structure & Module Organization
Chyren is a mixed-language monorepo. `medulla/` contains the Rust workspace and `omega-*` crates that implement the kernel, CLI, telemetry, and integrations. `chyren-os/interface/` is the main Next.js 15 frontend, while `gateway/` contains a separate TypeScript gateway app. Python orchestration and CLI support live under `cortex/`, with repo-level Python tests in `tests/` and additional module tests in `cortex/tests/`. Keep docs and examples in `docs/`, and treat generated folders such as `.next/`, `target/`, and caches as non-source.

## Build, Test, and Development Commands
Use the top-level `Makefile` for common checks:
- `make fmt`: runs `cargo fmt --all` in `medulla/`.
- `make lint`: runs `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- `make test`: runs the Rust workspace tests.
- `make ci`: local Rust CI equivalent.
- `make cortex-test`: runs `PYTHONPATH=cortex pytest tests/`.
- `make web-ci`: installs deps and runs `typecheck`, `lint`, and `build` in `web/`.
- `make gateway-ci`: installs deps and runs TypeScript, lint, and build checks in `gateway/`.

For local development, use `cd web && npm run dev` or `cd gateway && pnpm dev` when working on those apps.

## Coding Style & Naming Conventions
Let the repo tools define style. Rust must stay `rustfmt`-clean and clippy-clean. TypeScript in `web/` and `gateway/` follows ESLint flat configs, uses 2-space indentation, and prefers descriptive component and utility names. Python tests use `test_*.py`; TypeScript tests use `*.test.ts` or `*.spec.ts`. Match existing crate names like `omega-core` and keep new package or module names lowercase and hyphenated where applicable.

## Testing Guidelines
Add or update tests with every behavior change. Keep Rust unit tests close to implementation, Python integration coverage in `tests/` or `cortex/tests/`, and frontend tests in `web/__tests__/` or `web/tests/`. Run the narrowest relevant command first, then the broader check before opening a PR.

## Commit & Pull Request Guidelines
Recent history follows Conventional Commits such as `feat: ...` and `fix: ...`; continue that pattern and keep subjects imperative. PRs should include a short problem statement, a concise summary of changes, linked issues when relevant, test evidence, and screenshots for `web/` or `gateway/` UI work.

## Security & Configuration Tips
Do not commit secrets, `.env*` files, or production tokens. Review logs and fixtures for sensitive data before pushing, especially when touching telemetry, gateways, or external-provider integrations.

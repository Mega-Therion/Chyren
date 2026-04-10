# Repository Guidelines

## Project Structure & Module Organization
This repository is a mixed Python/Rust workspace. Core Python modules live in `core/` and `providers/`, with supporting identity code in `chyren_py/`. The CLI package is isolated in `chyren_cli/` (`core/`, `providers/`, `ui/`, and `tests/`). Rust and web assets are under `omega_workspace/workspace/OmegA-Next/`, including the `chyren-web/` app. Shared scripts live in `scripts/`, documentation in `docs/`, examples in `examples/`, and repository-level tests in `tests/`.

## Build, Test, and Development Commands
Use the commands CI already exercises:
- `pytest tests/` runs the main Python test suite.
- `python -m pytest chyren_cli/tests/` runs CLI-specific tests.
- `ruff check .` lints Python code.
- `mypy core/ providers/ chyren_py/ --ignore-missing-imports` type-checks the Python core.
- `cd omega_workspace/workspace/OmegA-Next && cargo build --workspace` builds the Rust workspace.
- `cd omega_workspace/workspace/OmegA-Next && cargo test --workspace` runs Rust tests.
- `./scripts/docker-manager.sh up|down|build|logs|status` manages the Docker stack via `omega_workspace/workspace/OmegA-Next/docker-compose.yml`.

## Coding Style & Naming Conventions
Follow standard Python formatting: 4-space indentation, `snake_case` for functions and modules, `PascalCase` for classes. Keep modules small and import paths explicit. Rust code should follow `cargo fmt` output and pass `cargo clippy --workspace -D warnings`. Use descriptive names that match existing conventions such as `adccl`, `ledger`, `alignment`, and provider modules like `openai.py` or `gemini.py`.

## Testing Guidelines
Tests use `pytest`. Name files `test_*.py` and keep focused unit tests close to the code they exercise; CLI tests belong in `chyren_cli/tests/`, broader integration coverage in `tests/`. CI treats some tests as environment-dependent, so note required API keys or fixtures in the test or README when a test cannot run offline.

## Commit & Pull Request Guidelines
Recent history uses scoped, imperative commits such as `fix(chyren-web): ...` and `feat(omega-adccl): ...`. Follow that pattern when practical. PRs should include a short summary, the commands used to verify the change, and screenshots for UI work. If the change touches Rust, Python, and web layers, call out which layer changed and any environment variables or Docker steps needed to reproduce it.

## Configuration & Secrets
Do not commit local secrets. The repo expects runtime configuration in `~/.omega/one-true.env` and uses environment variables for provider credentials. Keep generated state, caches, and virtual environments out of version control.

.PHONY: help fmt lint test ci medulla-fmt medulla-lint medulla-test cortex-test web-ci gateway-ci

help:
	@echo "Common commands:"
	@echo "  make fmt        - format Rust (workspace)"
	@echo "  make lint       - lint Rust (clippy -D warnings)"
	@echo "  make test       - run Rust tests"
	@echo "  make ci         - run Rust fmt+clippy+test (local CI equivalent)"
	@echo "  make cortex-test- run Python tests"
	@echo "  make web-ci     - run Next.js checks (lint/typecheck/build)"
	@echo "  make gateway-ci - run gateway checks (tsc/lint/build)"

fmt: medulla-fmt
lint: medulla-lint
test: medulla-test

ci: medulla-fmt medulla-lint medulla-test

medulla-fmt:
	cd medulla && cargo fmt --all

medulla-lint:
	cd medulla && cargo clippy --workspace --all-targets --all-features -- -D warnings

medulla-test:
	cd medulla && cargo test --workspace

cortex-test:
	PYTHONPATH=cortex pytest tests/

web-ci:
	cd chyren-os/interface && npm ci && npm run typecheck && npm run lint && npm run build

gateway-ci:
	cd gateway && pnpm install && npx tsc --noEmit && pnpm lint && pnpm build


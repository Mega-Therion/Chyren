export RUSTUP_HOME := $(CURDIR)/environment/rustup
export CARGO_HOME := $(CURDIR)/environment/cargo
export PATH := $(CARGO_HOME)/bin:$(PATH)

.PHONY: help fmt lint test ci medulla-fmt medulla-lint medulla-test cortex-test web-ci gateway-ci

help:
	@echo "Common commands:"
	@echo "  make fmt        - format Rust (workspace)"
	@echo "  make lint       - lint Rust (clippy -D warnings)"
	@echo "  make test       - run Rust tests"
	@echo "  make ci         - run Rust fmt+clippy+test (local CI equivalent)"
	@echo "  make cortex-test- run Python tests"
	@echo "  make jwst-query - query MAST for JWST data"
	@echo "  make jwst-ingest- parse and ingest JWST data"
	@echo "  make web-ci     - run Next.js checks (lint/typecheck/build)"
	@echo "  make gateway-ci - run gateway checks (tsc/lint/build)"

fmt: medulla-fmt
lint: medulla-lint
test: medulla-test

ci: medulla-fmt medulla-lint medulla-test

medulla-fmt:
	cd src/medulla/kernel && cargo fmt --all

medulla-lint:
	cd src/medulla/kernel && cargo clippy --workspace --all-targets --all-features -- -D warnings

medulla-test:
	cd src/medulla/kernel && cargo test --workspace

cortex-test:
	./src/cortex/venv/bin/python -m pytest src/cortex/tests

jwst-query:
	cd src/research/jwst_pipeline && ./env/bin/python pipeline/query.py

jwst-ingest:
	cd src/research/jwst_pipeline && ./env/bin/python pipeline/ingest.py

web-ci:
	cd src/medulla/interface && npm ci && npm run typecheck && npm run lint && npm run build

gateway-ci:
	cd src/gateways && pnpm install && npx tsc --noEmit && pnpm lint && pnpm build



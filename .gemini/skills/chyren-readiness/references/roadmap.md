# Production Readiness Roadmap (Prioritized)

### Phase 1: Foundation Stability
1. **Fix Dependency Conflicts**: Resolve `duplicate key` in `medulla/omega-cli/Cargo.toml`.
2. **Deterministic Build**: Ensure `cargo build --workspace` succeeds without errors.
3. **CI Integration**: Validate `make ci` and integrate into GitHub Actions.

### Phase 2: Operational Security
1. **Secrets Management**: Replace direct env file reliance with a robust secrets management provider (e.g., HashiCorp Vault or similar).
2. **Infrastructure as Code**: Finalize `docker-compose.yml` configurations for production environments.

### Phase 3: Observability
1. **Logging**: Centralize logs via `omega-telemetry`.
2. **Monitoring**: Standardize metrics collection and alerting (e.g., Prometheus/Grafana).

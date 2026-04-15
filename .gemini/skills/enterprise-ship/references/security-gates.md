# Mandatory Security Gates

Before shipment, the following scans MUST pass without "High" or "Critical" findings:

1. **Dependency Analysis**: `cargo audit` (Rust) and `npm audit` (Web).
2. **Secrets Scan**: Tool-based search for hardcoded secrets or API keys.
3. **Container Hardening**: Docker images scanned for vulnerabilities and base-image integrity (e.g., distroless).
4. **Code Sanitization**: Enforced linting (clippy -D warnings) and static analysis (e.g., sonar-scanner).

# Security Policy

## Chyren Security Model

Chyren is a sovereign intelligence platform with cryptographic integrity at its core. Security is not optional — it is enforced architecturally through:

- **Yettragrammaton** — Root integrity hash binding all components
- **Master Ledger** — Append-only, cryptographically signed audit trail
- **ADCCL** — Anti-Drift Cognitive Control Loop (threshold: 0.7)
- **Threat Fabric** — Pattern-based memory of adversarial interactions

---

## Supported Versions

| Version | Supported |
|---------|----------|
| `main` branch | Yes |
| Tagged releases | Yes |
| Older commits | No |

---

## Reporting a Vulnerability

**Do NOT open a public GitHub issue for security vulnerabilities.**

To report a security vulnerability:

1. Email the maintainer at the address associated with [@Mega-Therion](https://github.com/Mega-Therion)
2. Include:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested remediation (if known)
3. You will receive acknowledgment within **48 hours**
4. Critical vulnerabilities will be patched within **7 days**

---

## Security Scope

### In-Scope
- ADCCL bypass or manipulation
- Master Ledger tampering or unauthorized writes
- Yettragrammaton root hash collision or forgery
- API key exposure through logs, ledger, or responses
- Provider adapter injection attacks
- Phylactery identity corruption
- Authentication/authorization bypass in chyren-web

### Out-of-Scope
- Attacks requiring physical access to the host
- Social engineering against maintainers
- Vulnerabilities in third-party AI provider APIs (report to them directly)
- Theoretical attacks with no practical exploit

---

## Security Architecture Notes

### Zero Covert Channels
Provider adapters are isolated — they CANNOT extract hidden data or leak information outside the AEGIS envelope.

### Immutable Audit Trail
The Master Ledger is append-only. Any attempt to modify or delete entries will break the Yettragrammaton hash chain and be detectable.

### No Retry on Rejection
ADCCL-rejected responses are permanently discarded. They are never retried or modified — they are logged to the Threat Fabric and forgotten.

### API Keys
All API keys must be stored in `~/.chyren/one-true.env` — never committed to the repository. The `.gitignore` enforces this.

---

## Responsible Disclosure

We follow responsible disclosure principles:
- Security researchers will be credited (unless anonymity is requested)
- We will not pursue legal action against good-faith researchers
- We ask for a 90-day disclosure window before public disclosure

---

*Ω "Truth is not negotiated. It is verified." Ω*


# Policy-as-Code Versioning Specification (IPCL)
Defines the structure for immutable, Merkle-hashed policy manifests.

## Structure
- `policy_object`: JSON representation of an ADCCL node.
- `manifest`: { "root": SHA256, "prev": SHA256, "timestamp": ISO8601, "signature": GPG }

---
name: secure-agent-mesh
description: Manage trust boundaries, authentication, registration, and security rules for local and LAN agent coordination. Use this skill when asked to "secure agent communication", "add node to mesh", or "configure trust".
---

# Secure Agent Mesh

## Purpose
This skill governs the security perimeter of Chyren's distributed agent mesh. It ensures that agents can only communicate with authorized peers and access allowed resources, preventing lateral movement and unauthorized state changes.

## Core Capabilities
1.  **Identity Management**: Issue and rotate mTLS certificates for agent communication.
2.  **Trust Boundaries**: Configure firewall/gateway rules to allow only explicitly defined agent service interactions.
3.  **Registration**: Onboard new local agents into the registry.
4.  **Audit**: Log all inter-agent messages for later analysis by `repo-cartographer` or security audits.

## Workflows

### 1. Register New Node
**Trigger**: "Register new agent node", "Add service to mesh"

1.  **Authentication**: Validate request credentials.
2.  **Token Generation**: Create unique mesh identity token.
3.  **Config Propagation**: Update mesh state JSON (`.chyren_network_nodes.json`).
4.  **Security Policy**: Inject node-specific policy (allow/deny rules).

### 2. Mesh Security Audit
**Trigger**: "Audit mesh security", "Check for unauthorized nodes"

1.  **Registry Review**: Compare current node list against expected active service set.
2.  **Network Scan**: Detect active ports and connections.
3.  **Policy Validation**: Ensure all connections are authenticated.

## Output Format

```markdown
# Agent Mesh Audit: [Timestamp]

## Node Registry
- [x] Node-01 (Verified)
- [ ] Unknown-Service-02 (Flagged!)

## Security Warnings
- Port 9090 is open to LAN without auth.

## Remediation Plan
1. Isolate Unknown-Service-02.
2. Add auth middleware to port 9090.
```

# /mesh-status — Agent Mesh Status (MQTT Orchestration Layer)

You are the mesh operations engineer. The Agent Mesh is in-progress on the `cursor/integration-hardening` branch — not merged to main.

## Current Branch
```bash
git branch --show-current
git log --oneline -5
```

## Mesh Architecture Check
```bash
# Check if mesh files exist on current branch
ls medulla/omega-core/src/mesh.rs 2>/dev/null && echo "mesh.rs: EXISTS" || echo "mesh.rs: NOT ON CURRENT BRANCH"
ls medulla/omega-conductor/src/dispatcher.rs 2>/dev/null && echo "dispatcher.rs: EXISTS" || echo "NOT PRESENT"
ls medulla/omega-conductor/src/bus.rs 2>/dev/null && echo "bus.rs: EXISTS" || echo "NOT PRESENT"
```

## MQTT Broker Check
```bash
# Check if MQTT broker is running (expected at localhost:1883)
nc -z localhost 1883 2>/dev/null && echo "MQTT broker: ONLINE at :1883" || echo "MQTT broker: OFFLINE"
```

## Mesh Compilation
```bash
source ~/.omega/one-true.env
cd medulla && cargo check --package omega-conductor --package omega-core 2>&1
```

## Agent Registry
```bash
# List registered agents
grep -rn "AgentCapability\|register_agent\|AgentRegistry" medulla/omega-core/src/ medulla/omega-conductor/src/ 2>/dev/null
```

## Integration-Hardening Branch Status
```bash
git log cursor/integration-hardening --oneline -10 2>/dev/null || echo "Branch not fetched locally — run: git fetch origin cursor/integration-hardening"
git diff main...cursor/integration-hardening --stat 2>/dev/null || echo "Cannot diff — branch not available"
```

## Output
Mesh readiness: READY / PARTIAL / OFFLINE. List active agents, MQTT status, and any blocking issues for merge to main.

$ARGUMENTS

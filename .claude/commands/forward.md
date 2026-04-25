---
name: forward
description: Forward a task to another agent (Aider for implementation,  for strategy, Codex for architecture) via the OMEGA orchestrator
argument-hint: [task description]
allowed-tools: Bash(omega forward:*), Bash(~/.local/bin/omega forward:*), Bash(aider:*), Bash(~/.local/bin/omega chat:*)
---

## Forward to Agent

Task to forward: $ARGUMENTS

### Step 1: Determine the best agent

| Agent | Best for |
|-------|----------|
| **Aider** | Hands-on implementation, writing/editing files, committing code |
| **** | Strategy, operations, multi-agent coordination |
| **Codex** | Technical architecture, infrastructure design |
| **Claude** | Deep reasoning, research, complex analysis |

### Step 2: Prepare an implementation brief

Before forwarding, write a clear brief that includes:
- What needs to be done (specific, actionable)
- Relevant file paths and context
- Constraints and conventions to follow (from CLAUDE.md or project docs)
- Expected output / definition of done

### Step 3: Forward

Try these in order:
```bash
# Via omega orchestrator
omega forward "$ARGUMENTS"
# or
~/.local/bin/omega forward "$ARGUMENTS"

# Direct to Aider (for implementation)
aider --message "$ARGUMENTS"
```

### Step 4: Log the delegation
Append to `~/NEXUS/ERGON.md`:
```
Forwarded to [Agent]: [task summary]
```

### Note
If omega/aider isn't available, output the prepared brief for the user to copy manually.

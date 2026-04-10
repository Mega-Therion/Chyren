---
name: task
description: Create a task in the ORYAN task queue — routes to the appropriate agent (claude, , codex, aider) based on task type
argument-hint: [task description]
allowed-tools: Bash(~/NEXUS/OmegA/OmegA-SI/tools/ORYAN/oryan *), Bash(oryan *), Read
---

## Create ORYAN Task

Task: $ARGUMENTS

### Step 1: Route to the right agent

Analyze the task and choose:

| Agent | Route when task involves |
|-------|--------------------------|
| `claude` | Research, analysis, reasoning, documentation, review |
| `` | Strategy, planning, coordination, operations |
| `codex` | Architecture, infrastructure, system design |
| `aider` | Writing code, editing files, implementation |

### Step 2: Create the task

```bash
# From ORYAN directory
cd ~/NEXUS/OmegA/OmegA-SI/tools/ORYAN
./oryan task create --agent <agent> --prompt "$ARGUMENTS"

# Verify it was created
./oryan task list
```

### Step 3: Log to ERGON.md

Append to `~/NEXUS/ERGON.md`:
```
Created ORYAN task → [agent]: [task summary]
```

### Fallback

If ORYAN is not available, suggest using:
```bash
omega chat  # Interactive multi-agent chat
# or
omega ask-<agent> "$ARGUMENTS"
```

If $ARGUMENTS is ambiguous, ask what the task is before routing.

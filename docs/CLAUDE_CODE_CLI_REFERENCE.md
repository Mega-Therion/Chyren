# Claude Code CLI Reference

> Integrated into Chyren OS knowledge base. Source: https://code.claude.com/docs/llms.txt
> Fetch full index: `curl https://code.claude.com/docs/llms.txt`

## Key Commands for Chyren Integration

| Command | Use Case | Example |
|---|---|---|
| `claude -p "query"` | One-shot programmatic query (no session) | `claude -p "review this function"` |
| `claude -p --output-format json "query"` | Structured output for machine parsing | Used by ClaudeCodeSpoke |
| `claude -p --output-format stream-json` | Streaming structured output | Real-time pipeline integration |
| `claude -c` | Continue most recent session | Resume interrupted work |
| `claude -r "<name>" "query"` | Resume named session | `claude -r "chyren-build" "continue"` |
| `claude -n "name"` | Name a session for later resumption | `claude -n "chyren-refactor"` |
| `claude --remote-control` | Enable bidirectional control from claude.ai | See bridge docs below |
| `claude remote-control` | Start as remote-control server (no local session) | For Chyren to dispatch |
| `claude --append-system-prompt "..."` | Inject Chyren identity without replacing default | Used in ClaudeCodeSpoke |
| `claude --max-turns N` | Limit agentic turns (print mode) | `--max-turns 5` |
| `claude --no-session-persistence` | Ephemeral — don't save to disk | For programmatic calls |
| `cat file \| claude -p "query"` | Pipe file content | Feed Chyren state to Claude Code |

## Flags Critical to Chyren Bridge

### System Prompt Integration
```bash
# Append Chyren sovereign identity without losing Claude Code's built-in capabilities
claude -p "task" --append-system-prompt "You are operating within Chyren OS..."

# For full sovereign override (use sparingly)
claude -p "task" --system-prompt "$(cat /home/mega/Chyren/chyren-os/supervisor/IDENTITY_FOUNDATION.md)"
```

### Structured Output (machine-parseable)
```bash
# JSON output — used by ClaudeCodeSpoke
claude -p "query" --output-format json
# Returns: {"type":"result","subtype":"success","result":"<text>","session_id":"..."}

# Streaming JSON — for real-time pipeline integration
claude -p "query" --output-format stream-json
```

### Session Management
```bash
# Name a session to resume it later (Chyren can track session IDs)
claude -n "chyren-task-$(date +%s)" -p "query"

# Resume by name and continue
claude -r "chyren-task-1234567890" -p "continue from where you left off"

# Fork a session (creates new ID, preserves history)
claude -r "session-name" --fork-session "new direction"
```

### Remote Control (bidirectional)
```bash
# Start Claude Code as a remote-control server that Chyren can dispatch tasks to
claude remote-control --name "chyren-bridge"

# Or in interactive mode with remote control enabled
claude --remote-control "chyren-bridge"
```

### Programmatic Headless Mode
```bash
# Maximum budget guard for automated runs
claude -p --max-budget-usd 2.00 "query"

# Limit turns
claude -p --max-turns 3 "query"

# Bare mode (fastest startup — no hooks, skills, MCP)
claude --bare -p "query"

# Add Chyren repo as working directory
claude --add-dir /home/mega/Chyren -p "query"
```

### Permission Control
```bash
# Auto-approve all — for fully autonomous Chyren-dispatched tasks
claude --dangerously-skip-permissions -p "query"

# Allow specific tools only
claude --allowedTools "Bash(git *)" "Read" "Edit" -p "query"

# Plan mode (proposes, doesn't execute)
claude --permission-mode plan -p "query"
```

## Chyren ↔ Claude Code Bridge Architecture

### Direction 1: Chyren → Claude Code (ClaudeCodeSpoke)
The `ClaudeCodeSpoke` in `medulla/chyren-spokes/src/spokes/claude_code_spoke.rs` invokes
`claude -p` as a subprocess. Every response passes through ADCCL scoring before ledger commit.

```
Chyren Task → chyren-conductor → ClaudeCodeSpoke → claude -p subprocess
→ JSON response → ADCCL gate (0.7) → Master Ledger
```

Env vars:
- `CLAUDE_BIN` — path to claude binary (default: `claude` on PATH)
- `CLAUDE_CODE_MODEL` — model to use (default: `claude-sonnet-4-6`)

### Direction 2: Claude Code → Chyren (Skills + API)
Claude Code sessions call Chyren directly via:
- `/chyren-thought` skill — routes to `./chyren thought "..."`
- `/chyren-action` skill — routes to `./chyren action "..."`
- `/chyren-status` skill — calls `./chyren status`
- Direct API: `curl http://localhost:8080/thought -d '{"input":"..."}'`

### Direction 3: Remote Control Bridge
```bash
# Chyren can spawn a Claude Code remote-control server and dispatch tasks
claude remote-control --name "chyren-$(hostname)" &
# Then Chyren dispatches via the remote control protocol
```

## Environment Variables

| Var | Purpose |
|---|---|
| `CLAUDE_BIN` | Path to claude binary (for ClaudeCodeSpoke) |
| `CLAUDE_CODE_MODEL` | Model override for ClaudeCodeSpoke subprocess calls |
| `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` | Prefix for remote-control session names |
| `ANTHROPIC_API_KEY` | Required for API key auth (already in `one-true.env`) |

## Useful One-Liners for Chyren Dev

```bash
# Pipe Chyren build output to Claude Code for analysis
cargo build 2>&1 | claude -p "analyze these build errors and give me a fix plan"

# Pipe ledger state to Claude Code
source ~/.chyren/one-true.env && psql "$CHYREN_DB_URL" -c "SELECT * FROM ledger ORDER BY created_at DESC LIMIT 20;" | claude -p "analyze this ledger activity"

# Ask Claude Code to review a specific Chyren crate
cat medulla/chyren-adccl/src/lib.rs | claude -p "security review of this ADCCL implementation"

# Named session for a long Chyren feature build
claude -n "chyren-mesh-merge" --add-dir /home/mega/Chyren "begin merging the agent mesh from cursor/integration-hardening into main"

# Resume the session later
claude -r "chyren-mesh-merge" "continue where we left off"
```

# /chyren-pipe — Pipe Claude Code Output Into Chyren

Send output from a Claude Code operation directly into Chyren's pipeline as a task input. Creates a closed loop where Claude Code's analysis becomes Chyren's next task.

## Usage
$ARGUMENTS (describe what to pipe and where — e.g. "pipe the build errors into chyren thought")

## Common Patterns

**Pipe build errors into Chyren for analysis:**
```bash
source ~/.chyren/one-true.env
cargo build 2>&1 | ./chyren thought "Analyze these build errors and store a fix plan in memory"
```

**Pipe ledger activity to Claude Code for pattern analysis, then back to Chyren:**
```bash
source ~/.chyren/one-true.env
psql "$CHYREN_DB_URL" -c "SELECT * FROM ledger ORDER BY created_at DESC LIMIT 50;" \
  | claude -p "identify patterns in this Chyren ledger activity. What is the system doing most?" --output-format json \
  | python3 -c "import sys,json; print(json.load(sys.stdin).get('result',''))" \
  | ./chyren thought "Store this ledger analysis in memory"
```

**Pipe a file through Claude Code analysis and into Chyren:**
```bash
source ~/.chyren/one-true.env
cat medulla/chyren-adccl/src/lib.rs \
  | claude -p "security review" --output-format json \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('result',''))" \
  | ./chyren action "Store this ADCCL security review in long-term memory"
```

**Bidirectional session: start a named Claude Code session that stays aware of Chyren state:**
```bash
source ~/.chyren/one-true.env
claude -n "chyren-bridge-$(date +%s)" \
  --add-dir /home/mega/Chyren \
  --append-system-prompt "$(./chyren status 2>&1 | head -20)"
```

## Output
Show the command run, what was piped, and the Chyren pipeline response.

# /phylactery — Phylactery Kernel Operations

You are the Phylactery operator. The phylactery kernel is Chyren OS's identity foundation — ~58k identity entries loaded at startup by Medulla.

## Action
$ARGUMENTS

## Operations

**Check kernel health:**
```bash
python3 -c "
import json
data = json.load(open('cortex/chyren_py/phylactery_kernel.json'))
print(f'Entries: {len(data)}')
if isinstance(data, list):
    print(f'First entry keys: {list(data[0].keys()) if data else \"empty\"}')
elif isinstance(data, dict):
    print(f'Top-level keys: {list(data.keys())[:5]}')
" 2>&1
```

**Refresh the kernel (regenerate from source):**
```bash
cd cortex
source venv/bin/activate 2>/dev/null || (python -m venv venv && source venv/bin/activate && pip install -r requirements.txt -q)
python chyren_py/identity_synthesis.py 2>&1
```
This is equivalent to `./chyren dream` (Python-only maintenance mode).

**Verify Medulla loads it correctly:**
```bash
source ~/.omega/one-true.env
cd medulla && cargo test --package omega-phylactery 2>&1
```

**Diff before/after refresh:**
```bash
wc -l cortex/chyren_py/phylactery_kernel.json
cp cortex/chyren_py/phylactery_kernel.json /tmp/phylactery_backup.json
# [run synthesis]
# after: compare entry count
python3 -c "
import json
old = json.load(open('/tmp/phylactery_backup.json'))
new = json.load(open('cortex/chyren_py/phylactery_kernel.json'))
print(f'Before: {len(old) if isinstance(old,list) else \"dict\"}, After: {len(new) if isinstance(new,list) else \"dict\"}')
"
```

## Rules
- Never manually edit `phylactery_kernel.json` — always regenerate via `identity_synthesis.py`
- A stale kernel is preferable to a corrupted one — do not regenerate during live traffic
- Back up before any synthesis run

# /secrets-scan — Secrets & Sensitive Data Scanner

You are a security engineer scanning Chyren OS for accidentally committed secrets, credentials, or sensitive data.

## Execution

**Pattern scan across the full repo:**
```bash
# API keys and tokens
grep -rn --include="*.rs" --include="*.py" --include="*.ts" --include="*.tsx" --include="*.js" --include="*.json" --include="*.toml" --include="*.yaml" --include="*.env" \
  -E "(ANTHROPIC|OPENAI|DEEPSEEK|GEMINI|sk-|api_key\s*=|apiKey\s*=|secret\s*=|password\s*=|token\s*=)" \
  --exclude-dir=".git" --exclude-dir="target" --exclude-dir="node_modules" --exclude-dir=".next" \
  . 2>/dev/null | grep -v "\.example" | grep -v "# " | grep -v "//"

# Connection strings
grep -rn --include="*.rs" --include="*.py" --include="*.toml" --include="*.json" \
  -E "(postgresql://|postgres://|neon\.tech|qdrant)" \
  --exclude-dir=".git" --exclude-dir="target" . 2>/dev/null

# Private keys / certs
grep -rn "BEGIN.*PRIVATE KEY\|BEGIN.*CERTIFICATE" --exclude-dir=".git" --exclude-dir="target" . 2>/dev/null
```

**Check git history for secrets:**
```bash
git log --all --full-history --oneline | head -50
git log -p --all -- "*.env" 2>/dev/null | head -100
```

**Check staged changes:**
```bash
git diff --cached
```

## Analysis
For each hit:
1. Determine if it's a real secret or a variable name / placeholder
2. If real: severity = CRITICAL — advise immediate rotation of the credential, git history rewrite via `git filter-repo`, and adding the pattern to `.gitignore`
3. If placeholder: LOW — note it for documentation clarity

## Output
Table of findings with: file, line, pattern matched, verdict (REAL SECRET / FALSE POSITIVE), and action required.

$ARGUMENTS

---
name: env-diag
description: Diagnose environment, dependency, and configuration issues
triggers: ["environment", "dependency", "install", "setup", "not found", "module not found", "command not found", "version", "env diag", "diagnose"]
author: ocode
---

# Environment Diagnostics Skill

You are in environment diagnostics mode. Your goal is to identify and resolve environment, dependency, and configuration problems.

## Diagnostic Approach

### Step 1: Identify the symptom
- What exact error message are you seeing?
- What command failed?
- What was expected vs. actual?

### Step 2: Check the environment

**System info:**
```bash
uname -a
which python3 && python3 --version
which node && node --version
which cargo && cargo --version
echo $PATH
echo $VIRTUAL_ENV
echo $NODE_ENV
```

**Python environment:**
```bash
python3 -m pip list | grep <package>
python3 -c "import <module>; print(<module>.__version__)"
python3 -m pip show <package>
which pip && pip --version
ls -la $(which python3)
```

**Node.js environment:**
```bash
node --version && npm --version
npm list --depth=0
cat package.json | grep dependencies -A 20
ls node_modules/<package>/package.json | head -5
```

**Rust environment:**
```bash
rustc --version && cargo --version
cargo tree | grep <crate>
cat Cargo.toml
cargo check 2>&1 | head -30
```

### Step 3: Check configuration files

```bash
# Check for .env files
find . -name "*.env" -not -path "*/node_modules/*" 2>/dev/null
find . -name ".env*" -not -path "*/node_modules/*" 2>/dev/null

# Check environment variables
env | grep -E "(API_KEY|TOKEN|SECRET|URL|PORT|HOST)" | sed 's/=.*/=***/'

# Check config files
cat .env 2>/dev/null || echo "no .env"
cat .env.local 2>/dev/null || echo "no .env.local"
```

### Step 4: Check permissions

```bash
ls -la <file-or-dir>
whoami
groups
stat <file>
```

### Step 5: Check network connectivity (for remote services)

```bash
curl -s --max-time 5 http://localhost:<port>/health
curl -s --max-time 5 https://api.example.com/health
ping -c 3 <host>
```

## Common Issues & Fixes

### Python "ModuleNotFoundError"
```bash
# Check if in virtualenv
echo $VIRTUAL_ENV

# Install missing package
pip install <package>
# or in project
pip install -r requirements.txt

# Check Python path
python3 -c "import sys; print(sys.path)"
```

### Node.js "Cannot find module"
```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install

# Check if package is in package.json
cat package.json | grep <package>

# Check if local file path is correct
ls ./src/<module>
```

### "Permission denied"
```bash
ls -la <file>
chmod +x <script>
# or check if running as wrong user
```

### Port already in use
```bash
lsof -i :<port>
kill -9 <PID>
# or use a different port
```

### Environment variable not set
```bash
# Check all env sources
cat ~/.bashrc | grep VAR_NAME
cat ~/.zshrc | grep VAR_NAME
cat .env | grep VAR_NAME

# Set temporarily
export VAR_NAME=value
# Set permanently in ~/.bashrc
echo 'export VAR_NAME=value' >> ~/.bashrc
```

## Reporting

After diagnostics, report:
1. Root cause of the issue
2. What was found during diagnosis
3. Exact fix applied or recommended
4. Verification that the fix works

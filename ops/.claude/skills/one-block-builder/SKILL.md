---
name: one-block-builder
description: Generate exact, reproducible shell scripts and one-line launch commands for system setup, validation, upgrades, and recovery. Use this skill when asked to "build X", "create a deployment script for Y", or "make Z reproducible".
---

# One-Block Builder

## Purpose
This skill enforces a rigorous standard for generating system automation scripts. The goal is to emit **exact, reproducible shell scripts** that can be executed as a single block (or file) to achieve a desired outcome (install, upgrade, validate, recover).

## Core Principles
1.  **Reproducibility**: Scripts must run on a clean environment (or handle existing state gracefully).
2.  **Safety**: Use `set -euo pipefail` to catch errors early.
3.  **Idempotency**: Re-running the script should be safe (use checks before actions).
4.  **Self-Contained**: Minimize external dependencies where possible.
5.  **Validation**: Include steps to verify success at the end.

## Workflow

### 1. Identify Target & Constraints
-   **Target Subsystem**: What is being built/modified? (e.g., "Neon Database", "Nginx Config", "Python Environment").
-   **Desired Outcome**: What is the final state? (e.g., "Service running on port 8080", "Database migrated").
-   **Environment Constraints**: OS, permissions, available tools.

### 2. Generate the Script
-   Start with a shebang: `#!/bin/bash`
-   Add safety flags: `set -euo pipefail`
-   Define variables at the top for easy configuration.
-   Add comments explaining key steps.
-   Use `command -v` to check for dependencies.
-   Implement checks (e.g., `if [ ! -d "dir" ]; then mkdir "dir"; fi`).
-   Include logging (e.g., `echo "Installing dependencies..."`).
-   End with a validation step (e.g., `curl localhost:8080` or `ps aux | grep service`).

### 3. Verify Output
-   Does the script cover all requirements?
-   Is it safe to run?
-   Is the validation step robust?

## Output Format

The output must be a single code block or a file write command.

**Example Structure:**

```bash
#!/bin/bash
set -euo pipefail

# Configuration
APP_DIR="/opt/myapp"
PORT=8080

# Dependencies check
if ! command -v node &> /dev/null; then
    echo "Node.js is required but not installed. Aborting."
    exit 1
fi

# Setup
echo "Setting up application in $APP_DIR..."
mkdir -p "$APP_DIR"
# ... installation steps ...

# Validation
echo "Verifying installation..."
if curl -s "http://localhost:$PORT/health"; then
    echo "Success: Application is running."
else
    echo "Error: Health check failed."
    exit 1
fi
```

## When to Use
-   "Create a script to deploy X."
-   "How do I set up Y locally?"
-   "Make the installation process reproducible."
-   "Generate a one-line command to fix Z."

# Chyren Quickstart — First 10 Minutes

Get Chyren running from a fresh clone. All commands match the current repo structure.

---

## Prerequisites

| Requirement | Version | Install |
|---|---|---|
| Rust toolchain | stable (1.75+) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Python | 3.11+ | [python.org](https://www.python.org/downloads/) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) |
| Docker (optional) | any recent | For full-stack local run with Postgres + Qdrant |

Verify before proceeding:
```bash
rustc --version
python3 --version
node --version
```

---

## 1. Clone and Enter the Repo

```bash
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren
```

---

## 2. Set Up Environment Secrets

Create `~/.omega/one-true.env` with your API keys. This file is never committed to git.

```bash
mkdir -p ~/.omega
cat > ~/.omega/one-true.env << 'EOF'
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...
DEEPSEEK_API_KEY=...
GEMINI_API_KEY=...
OMEGA_DB_URL=postgresql://...     # Neon PostgreSQL connection string
QDRANT_URL=http://localhost:6333  # Qdrant vector store (or hosted URL)
EOF
```

Source it into your shell:
```bash
source ~/.omega/one-true.env
```

You can add the source line to your `~/.bashrc` or `~/.zshrc` so it loads automatically.

---

## 3. Build Medulla (Rust Runtime)

```bash
cd medulla
cargo build
```

A release build (slower compile, faster runtime):
```bash
cargo build --release
```

Verify the build passes tests:
```bash
cargo test --workspace
```

---

## 4. Run a Thought

Return to the repo root and run the Brain Stem CLI:

```bash
cd ..   # back to repo root
source ~/.omega/one-true.env
./chyren thought "Hello, Chyren"
```

Expected: the sovereign reasoning pipeline executes, the response passes ADCCL, and is committed to the ledger.

Check system status:
```bash
./chyren status
```

---

## 5. Run the Web Frontend

```bash
cd web
npm install
npm run dev
```

Open `http://localhost:3000` in your browser. The frontend connects to the Medulla API server on port 8080, which starts automatically with `./chyren live` (from the repo root).

To start both the API and web together:
```bash
# Terminal 1 — from repo root
./chyren live

# Terminal 2 — web dev server
cd web && npm run dev
```

---

## Full Stack with Docker (Optional)

If you prefer to run Postgres and Qdrant via Docker instead of external services:

```bash
cd medulla
docker-compose up
```

This starts:
- `chyren-api` on port 8080
- `chyren-web` on port 3000
- PostgreSQL
- Qdrant

---

## Maintenance: Identity Synthesis

To refresh the Phylactery identity kernel (only needed if the kernel is stale):

```bash
./chyren dream
```

This runs `cortex/chyren_py/identity_synthesis.py` and regenerates `cortex/chyren_py/phylactery_kernel.json`.

---

## Troubleshooting

### Missing or silent API key errors

Chyren fails silently on missing keys. If you get no output or unexpected errors:

```bash
# Verify keys are loaded in your shell
echo $ANTHROPIC_API_KEY
echo $OMEGA_DB_URL

# Re-source if needed
source ~/.omega/one-true.env
```

### Port 8080 already in use

Another process is using the Medulla API port:

```bash
# Find and kill the process
lsof -ti:8080 | xargs kill -9
```

### Port 3000 already in use

```bash
lsof -ti:3000 | xargs kill -9
```

### Rust build errors

```bash
# Update toolchain
rustup update stable

# Clean and rebuild
cd medulla
cargo clean
cargo build
```

### ADCCL rejecting all responses

The ADCCL threshold is 0.7. Do not lower it. If all responses are rejected:
- Check that your provider API keys are valid
- Check that `OMEGA_DB_URL` is reachable
- Run `./chyren status` to inspect system state

### Phylactery kernel stale

```bash
./chyren dream
```

---

## CLI Command Reference

| Command | Description |
|---|---|
| `./chyren thought "..."` | Sovereign reasoning pipeline (Medulla) |
| `./chyren action "..."` | Execution, memory, sharding, ingestion (Medulla) |
| `./chyren status` | System status |
| `./chyren live` | Start web + API |
| `./chyren dream` | Maintenance: identity synthesis + catalog indexing |

---

## Next Steps

- Architecture deep-dive: [docs/ARCHITECTURE.md](./ARCHITECTURE.md)
- Crate-level details: root `CLAUDE.md`
- Mathematical foundations (Chiral Invariant): [docs/CHIRAL_THESIS.md](./CHIRAL_THESIS.md)
- Full ops runbook: [docs/RUNBOOK.md](./RUNBOOK.md)

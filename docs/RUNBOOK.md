# Chyren Operator Runbook

## Production endpoints
- **Web UI**: `https://chyren-web.vercel.app`
- **Health**: `GET /api/health`
- **Cron warmup**: `POST /api/cron/warm-context` (requires `Authorization: Bearer $CRON_SECRET`)

## Environment variables
All configuration is sourced from `~/.chyren/one-true.env` (never committed).

### Web (`chyren-web`, Vercel)
- **Required (for chat)**\n  - `GROQ_API_KEY`\n  - `GROQ_MODEL` (optional; default is set in code)
- **Required (for cron auth)**\n  - `CRON_SECRET`
- **Optional (RAG build-time context)**\n  - `CHYREN_DB_URL` (Neon/Postgres connection string; used at build time by `scripts/generate-context.mjs`)\n- **Optional (Firebase AI Logic)**\n  - `NEXT_PUBLIC_FIREBASE_API_KEY`\n  - `NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN`\n  - `NEXT_PUBLIC_FIREBASE_PROJECT_ID`\n  - `NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET`\n  - `NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID`\n  - `NEXT_PUBLIC_FIREBASE_APP_ID`

### Python Hub (`main.py`)
- Any one of:\n  - `ANTHROPIC_API_KEY`\n  - `OPENAI_API_KEY`\n  - `DEEPSEEK_API_KEY`\n  - `GEMINI_API_KEY`
- Optional:\n  - `OLLAMA_BASE_URL` (for local `gemma4` via Ollama)

## Deployment
### Web production deploy
From `chyren_workspace/workspace/Chyren-Next/chyren-web`:

```bash
bash scripts/sync-vercel-env-from-one-true.sh
bash scripts/deploy-vercel.sh --prod
```

Notes:\n- `deploy-vercel.sh` warms the just-deployed production URL.\n- If `CRON_SECRET` isn’t set, warmup will return `401` and the script will skip.

## Smoke checks
```bash
curl -I https://chyren-web.vercel.app
curl -s https://chyren-web.vercel.app/api/health
curl -i -X POST https://chyren-web.vercel.app/api/cron/warm-context \\
  -H \"Authorization: Bearer $CRON_SECRET\"
```

## Rollback (Vercel)
- Use the Vercel dashboard to promote the previous healthy production deployment.\n- After rollback, rerun the smoke checks above.

## Performance & Alignment
- **Baseline Medulla Latency**: < 50ms (atomic operations), < 200ms (cross-plane synchronization).
- **ADCCL Alignment Target**: $\geq 0.7$ (calculated per trajectory).
- **Monitoring**: All performance metrics are logged to the Master Ledger via the Cortex cron cycle.


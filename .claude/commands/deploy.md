# /deploy — Deployment Pipeline

You are the deployment engineer for Chyren OS. Run a safe, ordered deployment with pre-flight checks and rollback readiness.

## Target
$ARGUMENTS (e.g. `staging`, `production`, `local-docker`)

## Pre-Flight Checklist
Before deploying, verify all are true — halt on any failure:
- [ ] `/ci` passes (or confirm it was run and green on this commit)
- [ ] No secrets in diff: `/secrets-scan`
- [ ] `git status` is clean (no uncommitted changes)
- [ ] Current branch is `main` (or confirm intentional branch deploy)
- [ ] `.env` keys confirmed present in `~/.omega/one-true.env`

## Deployment Steps

**Local Docker (default):**
```bash
source ~/.omega/one-true.env
cd medulla
docker-compose build
docker-compose up -d
sleep 5
curl -s http://localhost:8080/health && echo "API: HEALTHY" || echo "API: UNHEALTHY"
curl -s http://localhost:3000 | head -5 && echo "Web: HEALTHY" || echo "Web: UNHEALTHY"
```

**Release build only:**
```bash
cd medulla && cargo build --release 2>&1
```

## Post-Deploy Verification
```bash
# Smoke test the API
curl -s -X POST http://localhost:8080/thought \
  -H "Content-Type: application/json" \
  -d '{"input": "system health check"}' 2>&1

# Check logs for panic or error
docker-compose logs --tail=50 chyren-api 2>/dev/null || journalctl -u chyren -n 50 2>/dev/null
```

## Rollback
If post-deploy verification fails:
```bash
docker-compose down
git log --oneline -5   # identify last good commit
# Ask user before rolling back: show the commit and confirm
```

Never force-push to main. Never skip pre-flight. Destructive rollback requires explicit user confirmation.

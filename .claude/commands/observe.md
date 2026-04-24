# /observe — Live System Observation

You are the observability engineer. Watch the live system, collect telemetry, and surface anomalies.

## Observation Target
$ARGUMENTS (e.g. `api`, `ledger`, `adccl`, `pipeline`, or empty for full system)

## Live Data Collection

**API server health:**
```bash
curl -s http://localhost:8080/health 2>&1
curl -s http://localhost:8080/metrics 2>&1 | head -30
```

**Recent ledger activity:**
```bash
source ~/.omega/one-true.env
psql "$OMEGA_DB_URL" -c "
SELECT
  created_at,
  task_type,
  adccl_score,
  LEFT(rejection_flags, 50) as flags
FROM ledger
ORDER BY created_at DESC
LIMIT 20;" 2>&1
```

**ADCCL rejection rate (last hour):**
```bash
psql "$OMEGA_DB_URL" -c "
SELECT
  COUNT(*) FILTER (WHERE adccl_score >= 0.7) as passed,
  COUNT(*) FILTER (WHERE adccl_score < 0.7) as rejected,
  ROUND(AVG(adccl_score)::numeric, 3) as avg_score
FROM ledger
WHERE created_at > NOW() - INTERVAL '1 hour';" 2>&1
```

**Docker container stats:**
```bash
docker stats --no-stream 2>/dev/null || echo "Docker not running"
```

**System resources:**
```bash
ps aux | grep -E "chyren|omega|qdrant|postgres" | grep -v grep
free -h
df -h / 2>/dev/null | tail -1
```

**Telemetry events (last 100 lines):**
```bash
# Check if there's a telemetry log/stderr redirect
journalctl -u chyren --no-pager -n 100 2>/dev/null || docker logs chyren-api --tail=100 2>/dev/null || echo "No system journal available"
```

## Anomaly Detection
Flag automatically:
- ADCCL rejection rate > 30% → threshold may be miscalibrated
- API response time > 5s → provider timeout or DB latency issue
- Ledger gap detected → integrity alert
- Memory > 80% → potential leak in omega-myelin or Qdrant

## Output
Dashboard-style snapshot with status per subsystem. Anomalies highlighted. Recommended action for each anomaly.

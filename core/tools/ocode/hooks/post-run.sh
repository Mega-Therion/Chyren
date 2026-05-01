#!/bin/bash
# ocode post-run hook — logs session summary to ERGON.md
# Called by ocode after each agent session completes.
# Env vars available: OCODE_WORKSPACE, OCODE_TURNS, OCODE_STATUS, OCODE_SUMMARY

ERGON="$HOME/NEXUS/ERGON.md"
TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M UTC")
WORKSPACE="${OCODE_WORKSPACE:-$(pwd)}"
TURNS="${OCODE_TURNS:-?}"
STATUS="${OCODE_STATUS:-completed}"
SUMMARY="${OCODE_SUMMARY:-ocode session ended}"

# Only log if there was actual work done (more than 1 turn)
if [[ "${TURNS}" -gt 1 ]] 2>/dev/null; then
  echo "" >> "$ERGON"
  echo "- **[$TIMESTAMP]** ocode/OmegA (${WORKSPACE##*/}): $SUMMARY [${TURNS} turns, $STATUS]" >> "$ERGON"
fi

# Also push to gateway memory if it's up
GATEWAY="http://localhost:8787"
TOKEN="${OMEGA_API_BEARER_TOKEN:-}"

if curl -sf "$GATEWAY/health" > /dev/null 2>&1 && [[ "${TURNS}" -gt 1 ]] 2>/dev/null; then
  python3 -c "
import urllib.request, json, sys
payload = json.dumps({
    'content': f'OmegA session: {sys.argv[1]} in {sys.argv[2]}',
    'importance': 0.6,
    'source': 'ocode_session',
    'namespace': 'default'
}).encode()
req = urllib.request.Request('$GATEWAY/api/v1/memory',
    data=payload,
    headers={'Content-Type': 'application/json'${TOKEN:+, 'Authorization': 'Bearer $TOKEN'}})
urllib.request.urlopen(req, timeout=3)
" "$SUMMARY" "${WORKSPACE##*/}" 2>/dev/null || true
fi

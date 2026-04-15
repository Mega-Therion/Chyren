#!/bin/bash
# scripts/sync-vercel-env.sh
# Pushes local environment secrets to Vercel production

echo "Starting Chyren environment sync to Vercel..."

# List of keys to sync
KEYS=("OMEGA_DB_URL" "SUPABASE_URL" "SUPABASE_SERVICE_KEY" "ANTHROPIC_API_KEY" "GEMINI_API_KEY")

for var in "${KEYS[@]}"; do
  # Extract value from your local source of truth
  val=$(grep "^$var=" ~/.omega/one-true.env | cut -d '=' -f2)
  
  if [ -z "$val" ]; then
    echo "  [SKIP] $var not found in ~/.omega/one-true.env"
    continue
  fi

  echo "  [SYNC] $var"
  echo "$val" | vercel env add "$var" production
done

echo "Sync complete."

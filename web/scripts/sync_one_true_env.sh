#!/usr/bin/env bash
# This script copies selected keys from the one‑true env file into a local .env.local for development.
# It is tolerant of missing keys – if a key is not present the script simply skips it.

ONE_TRUE="${HOME}/.omega/one-true.env"
TARGET=".env.local"

# Keys the app expects (add/remove as needed)
keys=(
  OPENAI_API_KEY
  KV_URL
  KV_TOKEN
  GROQ_API_KEY
  ELEVENLABS_API_KEY
  ELEVENLABS_VOICE_ID
)

# Overwrite (or create) the target file
> "$TARGET"

for k in "${keys[@]}"; do
  # Use grep quietly; if no match we just skip – this avoids exiting due to set -e
  val=$(grep -E "^${k}=" "$ONE_TRUE" | cut -d'=' -f2- | tr -d '\r' || true)
  if [[ -n "$val" ]]; then
    echo "${k}=${val}" >> "$TARGET"
  fi
done

echo "✅ .env.local created from $ONE_TRUE"

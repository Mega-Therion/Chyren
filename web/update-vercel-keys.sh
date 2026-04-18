#!/bin/bash
set -e

echo "Removing old keys..."
npx vercel env rm GROQ_API_KEY production --yes || true
npx vercel env rm GEMINI_API_KEY production --yes || true
npx vercel env rm ANTHROPIC_API_KEY production --yes || true
npx vercel env rm GROK_API_KEY production --yes || true
npx vercel env rm OPENAI_API_KEY production --yes || true
npx vercel env rm OPENAI_API_BASE production --yes || true
npx vercel env rm OPENAI_MODEL production --yes || true

echo "Adding Gemini keys..."
if [ -z "${GEMINI_API_KEY:-}" ]; then
  echo "GEMINI_API_KEY is not set in your shell. Export it first and rerun."
  exit 1
fi
echo -n "${GEMINI_API_KEY}" | npx vercel env add GEMINI_API_KEY production
echo -n "gemini-3.1-flash-live-preview" | npx vercel env add GEMINI_MODEL production

echo "Env variables updated successfully."

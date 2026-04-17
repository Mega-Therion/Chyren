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

echo "Adding OpenRouter keys..."
if [ -z "${OPENAI_API_KEY:-}" ]; then
  echo "OPENAI_API_KEY is not set in your shell. Export it first and rerun."
  exit 1
fi
echo -n "${OPENAI_API_KEY}" | npx vercel env add OPENAI_API_KEY production
echo -n "${OPENAI_API_BASE:-https://openrouter.ai/api/v1}" | npx vercel env add OPENAI_API_BASE production
echo -n "${OPENAI_MODEL:-meta-llama/llama-3.3-70b-instruct:free}" | npx vercel env add OPENAI_MODEL production

echo "Env variables updated successfully."

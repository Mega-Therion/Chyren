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
echo -n "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" | npx vercel env add OPENAI_API_KEY production
echo -n "https://openrouter.ai/api/v1" | npx vercel env add OPENAI_API_BASE production
echo -n "meta-llama/llama-3.3-70b-instruct:free" | npx vercel env add OPENAI_MODEL production

echo "Env variables updated successfully."

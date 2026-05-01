KEY="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"
PROJECT_DIR="$HOME/Chyren"

echo "🔍 Finding Chyren provider config in $PROJECT_DIR..."
ENV_FILE=$(find "$PROJECT_DIR" -name ".env" -not -path "*/node_modules/*" -not -path "*/venv/*" | head -1)

echo "📝 Updating provider settings in ${ENV_FILE:-$PROJECT_DIR/.env}..."

if [ -n "$ENV_FILE" ]; then
  sed -i '/OPENAI_BASE_URL/d; /OPENAI_API_KEY/d; /MODEL/d' "$ENV_FILE"
  echo "OPENAI_BASE_URL=https://openrouter.ai/api/v1" >> "$ENV_FILE"
  echo "OPENAI_API_KEY=$KEY" >> "$ENV_FILE"
  echo "MODEL=meta-llama/llama-3.3-70b-instruct:free" >> "$ENV_FILE"
  echo "✅ Updated $ENV_FILE"
else
  mkdir -p "$PROJECT_DIR"
  echo "OPENAI_BASE_URL=https://openrouter.ai/api/v1" > "$PROJECT_DIR/.env"
  echo "OPENAI_API_KEY=$KEY" >> "$PROJECT_DIR/.env"
  echo "MODEL=meta-llama/llama-3.3-70b-instruct:free" >> "$PROJECT_DIR/.env"
  echo "✅ Created $PROJECT_DIR/.env"
fi

echo "🧪 Testing OpenRouter connection..."
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" https://openrouter.ai/api/v1/models \
  -H "Authorization: Bearer $KEY")

if [ "$RESPONSE" = "200" ]; then
  echo "✅ OpenRouter API key is valid and working!"
else
  echo "❌ API key test failed (HTTP $RESPONSE). Double-check your key at openrouter.ai"
fi

echo ""
echo "🎉 Done! Chyren is now configured to use OpenRouter."
echo "   Model: meta-llama/llama-3.3-70b-instruct:free"
echo "   Base URL: https://openrouter.ai/api/v1"

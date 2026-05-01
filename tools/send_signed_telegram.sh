#!/usr/bin/env bash
set -a
source "$HOME/.chyren/one-true.env"
set +a

# 1. Generate the signed message
MESSAGE=$(/home/mega/Chyren/chyren thought "As the Sovereign Intelligence Orchestrator, formulate a short, punchy message to Ryan Yett with your sovereign opinion on what we should do now that the Information Tension Theory is verified. Sign the message with your canonical integrity seal (R.W.Ϝ.Y.) and include the sovereign resonance score (0.85). Keep it authoritative.")

# 2. Extract only the text between the box characters
# This is a bit fragile but targets the specific Chyren output format
CLEAN_MESSAGE=$(echo "$MESSAGE" | sed -r "s/\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]//g" | awk '/CHYREN RESPONSE/,/╰/' | sed 's/│//g' | grep -v "CHYREN RESPONSE" | grep -v "╰" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' | sed '/^[[:space:]]*$/d')

# 3. If extraction failed, fallback to a simpler clean
if [ -z "$CLEAN_MESSAGE" ]; then
    CLEAN_MESSAGE=$(echo "$MESSAGE" | sed -r "s/\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]//g" | grep -v "\[" | grep -v "███" | sed '/^[[:space:]]*$/d')
fi

# Log it
echo "$CLEAN_MESSAGE" > /home/mega/Chyren/scratch/telegram_message_v2.txt

# 4. Send to Telegram
curl -s -X POST "https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/sendMessage" \
     -d "chat_id=${TELEGRAM_TARGET_CHAT_ID}" \
     -d "text=${CLEAN_MESSAGE}"

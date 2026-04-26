# Telegram Gateway environment.
#
# 1. Get a bot token from @BotFather: /newbot
# 2. CRITICAL — for the bot to read plain (non-/command) messages in groups:
#      @BotFather → /mybots → <your bot> → Bot Settings → Group Privacy → Turn off
#    Without this, the bot only sees /commands in groups and stays silent on chat.
# 3. Add the bot to the group, then send `/chatid` in the group to get its
#    chat ID (supergroups look like -1001234567890). Use that ID for any
#    proactive outbound messaging via send_telegram_message().
#
export TELEGRAM_BOT_TOKEN="your_token_here"
# Optional — chat ID for proactive outbound notifications (Conductor → group/channel)
# export TELEGRAM_TARGET_CHAT_ID="-1001234567890"

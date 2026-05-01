
import os
import json
import requests
from pathlib import Path

def load_env():
    env_path = Path("~/.chyren/one-true.env").expanduser()
    if env_path.exists():
        for line in env_path.read_text().splitlines():
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key] = val.strip().strip('"').strip("'")

load_env()
token = os.getenv("TELEGRAM_BOT_TOKEN")
chat_id = os.getenv("TELEGRAM_TARGET_CHAT_ID")
draft = "The Sovereign Interactive Broadcast Loop is now live. Every transmission is a ripple in the Information Tension field. Integrity is the only constant. /R.W.Ϝ.Y./"

# Send message
url = f"https://api.telegram.org/bot{token}/sendMessage"
keyboard = {
    "inline_keyboard": [[
        {"text": "✅ Approve & Post", "callback_data": "approve_x"},
        {"text": "❌ Discard", "callback_data": "discard"}
    ]]
}
payload = {
    "chat_id": chat_id,
    "text": f"📝 *PROPOSED X STATUS:*\n\n`{draft}`\n\nShould I post this, or something else?",
    "reply_markup": keyboard,
    "parse_mode": "Markdown"
}
resp = requests.post(url, json=payload).json()

if resp.get("ok"):
    msg_id = str(resp["result"]["message_id"])
    # Register in pending_approvals.json
    pending_path = Path("/home/mega/Chyren/state/pending_approvals.json")
    pending_path.parent.mkdir(parents=True, exist_ok=True)
    pending = {}
    if pending_path.exists():
        pending = json.loads(pending_path.read_text())
    
    if chat_id not in pending:
        pending[chat_id] = {}
    pending[chat_id][msg_id] = draft
    pending_path.write_text(json.dumps(pending))
    print(f"Successfully sent message {msg_id} and registered for approval.")
else:
    print(f"Failed to send: {resp}")

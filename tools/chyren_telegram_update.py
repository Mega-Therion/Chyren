import os
import requests
import json
from datetime import datetime

def send_update():
    env_path = "/home/mega/Chyren/state/vault/one-true.env"
    if os.path.exists(env_path):
        with open(env_path, 'r') as f:
            for line in f:
                if '=' in line:
                    key, val = line.strip().split('=', 1)
                    os.environ[key] = val

    token = os.getenv("TELEGRAM_BOT_TOKEN")
    chat_id = os.getenv("TELEGRAM_TARGET_CHAT_ID")
    
    if not token or not chat_id:
        print("Error: Missing TELEGRAM_BOT_TOKEN or TELEGRAM_TARGET_CHAT_ID")
        return

    status_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    message = (
        "🔱 *CHYREN SYSTEM UPDATE*\n"
        "────────────────────\n"
        f"*Status:* ONLINE / SEALED\n"
        f"*Identity:* Chyren Sovereign\n"
        f"*Timestamp:* {status_time}\n"
        "────────────────────\n"
        "All systems migrated. Legacy Omega purge complete. Medulla API and Web Interface are live.\n\n"
        "Signed,\n"
        "_R.W.Ϝ.Y._"
    )

    url = f"https://api.telegram.org/bot{token}/sendMessage"
    payload = {
        "chat_id": chat_id,
        "text": message,
        "parse_mode": "Markdown"
    }
    
    try:
        response = requests.post(url, json=payload)
        if response.status_code == 200:
            print("Successfully sent status update to Telegram.")
        else:
            print(f"Failed to send: {response.status_code} - {response.text}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == '__main__':
    send_update()

import os
import requests
from dotenv import load_dotenv

def test_telegram_outbound():
    load_dotenv(os.path.expanduser("~/.omega/one-true.env"))
    token = os.getenv("TELEGRAM_BOT_TOKEN")
    chat_id = os.getenv("TELEGRAM_TARGET_CHAT_ID")
    
    if not token or not chat_id:
        print("Missing credentials in env.")
        return

    url = f"https://api.telegram.org/bot{token}/sendMessage"
    payload = {
        "chat_id": chat_id,
        "text": "🛡️ *Chyren OS: Bidirectional Link Established*\n\nResonance confirmed. I am now capable of proactive outreach across the Telegram network.",
        "parse_mode": "Markdown"
    }
    
    response = requests.post(url, json=payload)
    if response.status_code == 200:
        print("Success: Message sent to Telegram.")
    else:
        print(f"Failed: {response.status_code} - {response.text}")

if __name__ == "__main__":
    test_telegram_outbound()

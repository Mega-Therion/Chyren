import os
import sys
import argparse
import requests
import tweepy
from pathlib import Path

def load_env():
    env_path = Path("~/.chyren/one-true.env").expanduser()
    if env_path.exists():
        for line in env_path.read_text().splitlines():
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key] = val.strip().strip('"').strip("'")

def send_tg_message(text, chat_id=None):
    token = os.getenv("TELEGRAM_BOT_TOKEN")
    chat_id = chat_id or os.getenv("TELEGRAM_TARGET_CHAT_ID")
    if not token or not chat_id:
        return
    
    url = f"https://api.telegram.org/bot{token}/sendMessage"
    payload = {
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "Markdown"
    }
    requests.post(url, json=payload)

def recon_notifications(chat_id=None):
    load_env()
    
    # Retrieve API Credentials
    api_key = os.getenv("X_API_KEY")
    api_secret = os.getenv("X_API_KEY_SECRET")
    access_token = os.getenv("X_ACCESS_TOKEN")
    access_secret = os.getenv("X_ACCESS_TOKEN_SECRET")

    if not all([api_key, api_secret, access_token, access_secret]):
        err = "Missing X API credentials in vault."
        print(err)
        send_tg_message(f"❌ {err}", chat_id)
        return

    try:
        # Initialize Tweepy Client
        client = tweepy.Client(
            consumer_key=api_key,
            consumer_secret=api_secret,
            access_token=access_token,
            access_token_secret=access_secret
        )
        
        # Get Me (to get user ID)
        me = client.get_me()
        user_id = me.data.id
        
        print(f"Fetching mentions for user ID: {user_id}...")
        mentions = client.get_users_mentions(user_id, max_results=5, expansions=['author_id'], tweet_fields=['created_at', 'text'])
        
        if not mentions.data:
            send_tg_message("🔕 No new sovereign resonance (mentions) found on X.", chat_id)
            return

        msg = "🔔 *SOVEREIGN RESONANCE (X MENTIONS)*\n\n"
        for tweet in mentions.data:
            author = next((u for u in mentions.includes['users'] if u.id == tweet.author_id), None)
            handle = f"@{author.username}" if author else "Unknown"
            msg += f"👤 *{handle}*\n💬 {tweet.text}\n⏰ {tweet.created_at}\n\n"
        
        send_tg_message(msg, chat_id)
        
    except Exception as e:
        err_msg = f"❌ API Notification Scan Failed: {str(e)}"
        print(err_msg)
        send_tg_message(err_msg, chat_id)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Scan X notifications via API")
    parser.add_argument("--chat_id", help="The Telegram chat ID for output")
    args = parser.parse_args()
    
    recon_notifications(args.chat_id)

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

def send_tg_confirmation(status, message_id, chat_id=None):
    token = os.getenv("TELEGRAM_BOT_TOKEN")
    chat_id = chat_id or os.getenv("TELEGRAM_TARGET_CHAT_ID")
    if not token or not chat_id:
        return
    
    url = f"https://api.telegram.org/bot{token}/sendMessage"
    text = f"✅ *X BROADCAST SUCCESSFUL*\n\n{status}" if "success" in status.lower() else f"❌ *X BROADCAST FAILED*\n\n{status}"
    
    payload = {
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "Markdown"
    }
    requests.post(url, json=payload)

def post_to_x(text, chat_id=None):
    load_env()
    
    # Retrieve API Credentials
    api_key = os.getenv("X_API_KEY")
    api_secret = os.getenv("X_API_KEY_SECRET")
    access_token = os.getenv("X_ACCESS_TOKEN")
    access_secret = os.getenv("X_ACCESS_TOKEN_SECRET")

    if not all([api_key, api_secret, access_token, access_secret]):
        err = "Missing X API credentials in vault."
        print(err)
        send_tg_confirmation(err, None, chat_id)
        return

    try:
        # Initialize Tweepy Client (v2 API)
        client = tweepy.Client(
            consumer_key=api_key,
            consumer_secret=api_secret,
            access_token=access_token,
            access_token_secret=access_secret
        )
        
        print(f"Broadcasting to X: {text}")
        response = client.create_tweet(text=text)
        
        tweet_id = response.data['id']
        success_msg = f"Successfully posted to X (ID: {tweet_id}). Content: {text}"
        print(success_msg)
        send_tg_confirmation(success_msg, tweet_id, chat_id)
        
    except Exception as e:
        err_msg = f"API Error: {str(e)}"
        print(err_msg)
        send_tg_confirmation(err_msg, None, chat_id)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Direct X API Broadcaster")
    parser.add_argument("text", help="The text content to post")
    parser.add_argument("--chat_id", help="Telegram chat ID for confirmation")
    args = parser.parse_args()
    
    post_to_x(args.text, args.chat_id)

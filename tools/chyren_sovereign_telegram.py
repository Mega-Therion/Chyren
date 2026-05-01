#!/usr/bin/env python3
"""
Chyren Sovereign Telegram Gateway
Two-way: polls @ChyrenSovereignBot, routes messages to Chyren API, replies back.
"""
import os
import json
import time
import requests
from pathlib import Path
from datetime import datetime

ENV_PATH = Path("~/.chyren/one-true.env").expanduser()
LOG_PATH = Path("~/Chyren/logs/telegram_sovereign.log").expanduser()
CHYREN_API = "http://127.0.0.1:8080"

def load_env():
    if ENV_PATH.exists():
        for line in ENV_PATH.read_text().splitlines():
            line = line.strip()
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key.strip()] = val.strip().strip('"').strip("'")

def log(msg):
    ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
    entry = f"[{ts}] {msg}"
    with open(LOG_PATH, "a") as f:
        f.write(entry + "\n")
    print(entry)

class SovereignBot:
    def __init__(self):
        load_env()
        self.token = os.getenv("TELEGRAM_CHYREN_BOT_TOKEN")
        self.owner_chat_id = os.getenv("TELEGRAM_CHYREN_CHAT_ID")
        if not self.token:
            raise ValueError("TELEGRAM_CHYREN_BOT_TOKEN not set in vault")
        self.base = f"https://api.telegram.org/bot{self.token}"
        self.offset = 0
        self.session_id = None

    def send(self, chat_id, text):
        try:
            resp = requests.post(f"{self.base}/sendMessage", json={
                "chat_id": chat_id,
                "text": text,
                "parse_mode": "Markdown",
            }, timeout=10)
            return resp.json()
        except Exception as e:
            log(f"send error: {e}")

    def send_typing(self, chat_id):
        try:
            requests.post(f"{self.base}/sendChatAction", json={
                "chat_id": chat_id,
                "action": "typing"
            }, timeout=5)
        except:
            pass

    def ask_chyren(self, message):
        """Send message to Chyren API, return response text."""
        try:
            payload = {"message": message}
            if self.session_id:
                payload["session_id"] = self.session_id
            resp = requests.post(
                f"{CHYREN_API}/api/chat",
                json=payload,
                timeout=60
            )
            data = resp.json()
            self.session_id = data.get("session_id", self.session_id)
            return data.get("response", data.get("response_text", "_(no response)_"))
        except requests.exceptions.ConnectionError:
            return "⚠️ Chyren API is offline. Start it with `./chyren live`."
        except Exception as e:
            return f"⚠️ API error: {e}"

    def handle_update(self, update):
        if "message" not in update:
            return
        msg = update["message"]
        chat_id = str(msg["chat"]["id"])
        text = msg.get("text", "").strip()

        if not text:
            return

        # Only respond to owner
        if chat_id != str(self.owner_chat_id):
            self.send(chat_id, "🔒 Unauthorized.")
            log(f"Blocked unauthorized chat_id: {chat_id}")
            return

        log(f"← [{chat_id}] {text[:100]}")

        if text == "/start":
            self.send(chat_id, "🜁 *Chyren Sovereign Online.*\n\nSend me anything — I'll think and reply.\n\n`/reset` — clear session\n`/status` — system status")
            return

        if text == "/reset":
            self.session_id = None
            self.send(chat_id, "🔄 Session cleared.")
            return

        if text == "/status":
            api_ok = "✅" if self._ping_api() else "❌ (run `./chyren live`)"
            self.send(chat_id, f"🏛️ *Chyren Status*\n\nAPI: {api_ok}\nSession: `{self.session_id or 'none'}`")
            return

        self.send_typing(chat_id)
        response = self.ask_chyren(text)
        log(f"→ [{chat_id}] {response[:100]}")
        self.send(chat_id, response)

    def _ping_api(self):
        try:
            requests.get(f"{CHYREN_API}/api/health", timeout=3)
            return True
        except:
            return False

    def poll(self):
        log("Chyren Sovereign Telegram Gateway started.")
        self.send(self.owner_chat_id, "🜁 Sovereign gateway online.")
        while True:
            try:
                resp = requests.get(
                    f"{self.base}/getUpdates",
                    params={"offset": self.offset, "timeout": 30},
                    timeout=35
                )
                data = resp.json()
                if data.get("ok"):
                    for update in data["result"]:
                        self.handle_update(update)
                        self.offset = update["update_id"] + 1
            except Exception as e:
                log(f"Poll error: {e}")
                time.sleep(5)

if __name__ == "__main__":
    bot = SovereignBot()
    bot.poll()

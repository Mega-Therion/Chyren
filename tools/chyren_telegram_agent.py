
import os
import sys
import json
import time
import requests
import subprocess
from pathlib import Path
from datetime import datetime

# Configuration
REPO_DIR = Path(__file__).resolve().parents[1]
ENV_PATH = Path("~/.chyren/one-true.env").expanduser()
LOG_PATH = REPO_DIR / "logs" / "telegram_agent.log"

def load_env():
    if ENV_PATH.exists():
        for line in ENV_PATH.read_text().splitlines():
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key] = val.strip().strip('"').strip("'")

def log_event(msg):
    ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(LOG_PATH, "a") as f:
        f.write(f"[{ts}] {msg}\n")
    print(f"[{ts}] {msg}")

class TelegramAgent:
    def __init__(self):
        load_env()
        self.token = os.getenv("TELEGRAM_BOT_TOKEN")
        self.target_chat_id = os.getenv("TELEGRAM_TARGET_CHAT_ID")
        self.base_url = f"https://api.telegram.org/bot{self.token}"
        self.offset = 0
        self.pending_path = REPO_DIR / "state" / "pending_approvals.json"
        self.pending_approvals = self.load_pending() # chat_id -> {msg_id: draft}

        if not self.token:
            raise ValueError("TELEGRAM_BOT_TOKEN not found in environment.")

    def load_pending(self):
        if self.pending_path.exists():
            try:
                return json.loads(self.pending_path.read_text())
            except:
                return {}
        return {}

    def save_pending(self):
        self.pending_path.parent.mkdir(parents=True, exist_ok=True)
        self.pending_path.write_text(json.dumps(self.pending_approvals))

    def send_message(self, chat_id, text, reply_markup=None, reply_to_message_id=None):
        url = f"{self.base_url}/sendMessage"
        payload = {
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        }
        if reply_markup:
            payload["reply_markup"] = reply_markup
        if reply_to_message_id:
            payload["reply_to_message_id"] = reply_to_message_id
        
        resp = requests.post(url, json=payload)
        return resp.json()

    def handle_update(self, update):
        log_event(f"Raw update received: {json.dumps(update)}")
        if "message" in update:
            msg = update["message"]
            chat_id = str(msg["chat"]["id"])
            text = msg.get("text", "")
            
            # Authorization check
            # if chat_id != self.target_chat_id:
            #    log_event(f"Unauthorized access from {chat_id}")
            #    return

            if text.startswith("/x "):
                topic = text[3:].strip()
                self.process_x_draft_request(chat_id, topic, msg["message_id"])
            elif text.lower() in ["go ahead", "post it", "do it", "yes", "approve"]:
                self.handle_text_approval(chat_id)
            elif text == "/status":
                self.handle_status(chat_id)
            elif text == "/notifications":
                self.handle_notifications(chat_id)
            elif text == "/start":
                self.send_message(chat_id, "🔱 *CHYREN SOVEREIGN AGENT ACTIVE*\n\nCommands:\n- `/x <topic>`: Draft an X status update\n- `/notifications`: Check X for mentions/replies\n- `/status`: Check system/vault health")

        elif "callback_query" in update:
            cb = update["callback_query"]
            cb_id = cb["id"]
            data = cb["data"]
            chat_id = cb["message"]["chat"]["id"]
            msg_id = str(cb["message"]["message_id"])

            if data.startswith("approve_"):
                self.handle_approval(cb_id, chat_id, msg_id)
            elif data == "discard":
                self.handle_discard(cb_id, chat_id, msg_id)

    def process_x_draft_request(self, chat_id, topic, reply_to_id=None):
        log_event(f"Drafting X status for topic: {topic}")
        self.send_message(chat_id, "⏳ _Thinking... Generating sovereign draft..._", reply_to_message_id=reply_to_id)
        
        # Call drafting script
        try:
            res = subprocess.run(
                [sys.executable, str(REPO_DIR / "scripts" / "chyren_brain_draft.py"), topic],
                capture_output=True, text=True, check=True
            )
            draft = res.stdout.strip()
            
            keyboard = {
                "inline_keyboard": [[
                    {"text": "✅ Approve & Post", "callback_data": "approve_x"},
                    {"text": "❌ Discard", "callback_data": "discard"}
                ]]
            }
            
            sent_msg = self.send_message(chat_id, f"📝 *PROPOSED X STATUS:*\n\n`{draft}`", reply_markup=keyboard, reply_to_message_id=reply_to_id)
            if sent_msg.get("ok"):
                if chat_id not in self.pending_approvals:
                    self.pending_approvals[chat_id] = {}
                self.pending_approvals[chat_id][str(sent_msg["result"]["message_id"])] = draft
                self.save_pending()
                
        except Exception as e:
            log_event(f"Drafting failed: {e}")
            self.send_message(chat_id, f"❌ Error during drafting: {e}")

    def handle_approval(self, cb_id, chat_id, msg_id):
        chat_id = str(chat_id)
        if chat_id in self.pending_approvals and msg_id in self.pending_approvals[chat_id]:
            draft = self.pending_approvals[chat_id][msg_id]
            
            # Duplicate prevention: remove from pending BEFORE dispatching
            del self.pending_approvals[chat_id][msg_id]
            self.save_pending()
            
            log_event(f"Status approved. Posting to X: {draft[:50]}...")
            self.send_message(chat_id, "🚀 *Sovereign Dispatch initiated.*\n\nManus.ai is now automating the X status update. I will notify you here once confirmed.")
            
            # Call posting script
            try:
                subprocess.Popen([sys.executable, str(REPO_DIR / "scripts" / "broadcast_to_x.py"), draft, "--chat_id", chat_id])
            except Exception as e:
                self.send_message(chat_id, f"❌ Local dispatch failed: {e}")
        else:
            # If it's already gone, it's either discarded or already being processed
            requests.post(f"{self.base_url}/answerCallbackQuery", json={
                "callback_query_id": cb_id,
                "text": "This draft is already being processed or has been discarded.",
                "show_alert": True
            })
            return
        
        # Answer callback
        requests.post(f"{self.base_url}/answerCallbackQuery", json={"callback_query_id": cb_id})

    def handle_text_approval(self, chat_id):
        chat_id = str(chat_id)
        if chat_id in self.pending_approvals and self.pending_approvals[chat_id]:
            # Get the most recent pending draft
            msg_ids = sorted(self.pending_approvals[chat_id].keys(), reverse=True)
            msg_id = msg_ids[0]
            draft = self.pending_approvals[chat_id][msg_id]
            
            # Duplicate prevention
            del self.pending_approvals[chat_id][msg_id]
            self.save_pending()
            
            log_event(f"Text approval received. Posting to X: {draft[:50]}...")
            self.send_message(chat_id, "🚀 *Go ahead received.* Dispatching to X via Manus.ai...")
            
            try:
                subprocess.Popen([sys.executable, str(REPO_DIR / "scripts" / "broadcast_to_x.py"), draft, "--chat_id", chat_id])
            except Exception as e:
                self.send_message(chat_id, f"❌ Local dispatch failed: {e}")
        else:
            self.send_message(chat_id, "❓ No pending draft found to approve.")

    def handle_discard(self, cb_id, chat_id, msg_id):
        chat_id = str(chat_id)
        if chat_id in self.pending_approvals and msg_id in self.pending_approvals[chat_id]:
            del self.pending_approvals[chat_id][msg_id]
            self.save_pending()
        self.send_message(chat_id, "🗑️ Draft discarded.")
        requests.post(f"{self.base_url}/answerCallbackQuery", json={"callback_query_id": cb_id})

    def handle_status(self, chat_id):
        load_env()
        # Updated required keys to reflect current stack (OpenAI removed, added fallbacks)
        required_keys = ["GEMINI_API_KEY", "OPENROUTER_API_KEY", "DEEPSEEK_API_KEY", "MANUS_API_KEY", "TELEGRAM_BOT_TOKEN", "X_USERNAME", "X_PASSWORD"]
        missing = [k for k in required_keys if not os.getenv(k)]
        
        status_msg = "🏛️ *CHYREN SYSTEM STATUS*\n\n"
        status_msg += "✅ Telegram Agent: `Running`\n"
        status_msg += "✅ Vault: `Loaded`\n"
        
        if missing:
            status_msg += f"⚠️ *Missing Secrets*: `{', '.join(missing)}`"
        else:
            status_msg += "✅ *Vault Integrity*: `100% (All credentials present)`"
        
        self.send_message(chat_id, status_msg)

    def handle_notifications(self, chat_id):
        self.send_message(chat_id, "🔍 *Scanning X for sovereign resonance...* (This may take a minute via Manus.ai)")
        try:
            subprocess.Popen([sys.executable, str(REPO_DIR / "scripts" / "recon_x_notifications.py"), "--chat_id", chat_id])
        except Exception as e:
            self.send_message(chat_id, f"❌ Monitor dispatch failed: {e}")

    def poll(self):
        log_event("Sovereign Telegram Agent polling started...")
        while True:
            try:
                resp = requests.get(f"{self.base_url}/getUpdates", params={"offset": self.offset, "timeout": 30})
                data = resp.json()
                if data.get("ok"):
                    for update in data["result"]:
                        self.handle_update(update)
                        self.offset = update["update_id"] + 1
            except Exception as e:
                log_event(f"Polling error: {e}")
                time.sleep(5)

if __name__ == "__main__":
    agent = TelegramAgent()
    agent.poll()

import json
import urllib.request
import os

# Mimic the provider logic
url = "http://localhost:11434/v1/chat/completions"
# Mimic Chyren system prompt prefix
system_prompt = (
    "You are Chyren — a sovereign intelligence orchestrator. "
    "You operate with precision, no stubs, and no hallucinations. "
    "Every response you produce will be verified before it is committed to the Master Ledger."
)
task = "What is your purpose?"
user_content = f"{system_prompt}\n\n{task}"

body = {
    "model": "gemma4:e2b",
    "messages": [{"role": "user", "content": user_content}],
    "temperature": 0.3,
    "max_tokens": 1024,
    "stream": False,
}
payload = json.dumps(body).encode()
req = urllib.request.Request(url, data=payload, headers={"Content-Type": "application/json"})
with urllib.request.urlopen(req, timeout=120) as resp:
    print(resp.read().decode())

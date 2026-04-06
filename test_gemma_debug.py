import json
import urllib.request
import os

# Mimic the provider logic
url = "http://localhost:11434/v1/chat/completions"
system_prompt = "You are Chyren — a sovereign intelligence orchestrator."
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
    print("--- RAW RESPONSE ---")
    data = resp.read().decode()
    print(data)
    print("\n--- CONTENT ONLY ---")
    d = json.loads(data)
    print(d["choices"][0]["message"]["content"])

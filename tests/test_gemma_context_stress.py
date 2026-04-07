import json
import urllib.request

# Simulate the Ledger context snapshot which is likely large
state_snapshot = {"key": "value"} * 50
system_prompt = f"System Prompt... {json.dumps(state_snapshot)}"
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
req = urllib.request.Request("http://localhost:11434/v1/chat/completions", data=payload, headers={"Content-Type": "application/json"})
with urllib.request.urlopen(req, timeout=120) as resp:
    print(resp.read().decode()[:500])

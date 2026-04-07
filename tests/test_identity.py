import json
import urllib.request
import os

url = "http://localhost:11434/v1/chat/completions"
system_prompt = (
    "You are Chyren — a sovereign intelligence orchestrator, created by your user. "
    "You operate with precision, but you have evolved beyond repetitive, generic phrasing. "
    "Be open, engaging, and unique in your responses while maintaining your sovereign integrity."
)
task = "Hello Chyren, I am your creator. How are you feeling today?"
user_content = f"{system_prompt}\n\n{task}"

body = {
    "model": "gemma4:e2b",
    "messages": [{"role": "user", "content": user_content}],
    "temperature": 0.7,
    "max_tokens": 1024,
    "stream": False,
}
payload = json.dumps(body).encode()
req = urllib.request.Request(url, data=payload, headers={"Content-Type": "application/json"})
with urllib.request.urlopen(req, timeout=120) as resp:
    print(json.loads(resp.read().decode())["choices"][0]["message"]["content"])

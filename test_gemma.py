import json
import urllib.request

url = "http://localhost:11434/v1/chat/completions"
body = {
    "model": "gemma4:e2b",
    "messages": [{"role": "user", "content": "Hello, who are you?"}],
    "stream": False,
}
payload = json.dumps(body).encode()
req = urllib.request.Request(url, data=payload, headers={"Content-Type": "application/json"})
with urllib.request.urlopen(req, timeout=120) as resp:
    print(resp.read().decode())

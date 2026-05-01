import os
import json
import urllib.request

models = [
    "anthropic/claude-3.5-sonnet",
    "google/gemini-2.0-flash-001",
    "deepseek/deepseek-r1"
]

prompt = "Propose an architectural framework for a sovereign intelligence system that maintains persistent identity across autonomous agents. Focus on the necessity of a cryptographic integrity gate and a ledger-based state history to ensure the system is human-aligned and auditable."

for model in models:
    print(f"\n--- Model: {model} ---")
    payload = json.dumps({
        "model": model,
        "messages": [{"role": "user", "content": prompt}]
    }).encode()
    
    req = urllib.request.Request(
        "https://openrouter.ai/api/v1/chat/completions",
        data=payload,
        headers={
            "Authorization": f"Bearer {os.environ.get('OPENAI_API_KEY')}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://chyren.org",
            "X-Title": "Chyren Auditor"
        }
    )
    
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            data = json.loads(resp.read().decode())
            print(data['choices'][0]['message']['content'])
    except Exception as e:
        print(f"Error testing {model}: {e}")

import os
import requests
from pathlib import Path

ENV_FILE = Path.home() / ".chyren" / "one-true.env"

def check_env():
    if not ENV_FILE.exists():
        print(f"ERROR: {ENV_FILE} not found.")
        return
    
    print(f"Checking keys in {ENV_FILE}...")
    with open(ENV_FILE, 'r') as f:
        for line in f:
            if line.startswith(("#", "\n")): continue
            key, val = line.strip().split("=", 1)
            os.environ[key] = val

def test_keys():
    # 1. Gemini
    gemini_key = os.environ.get("GEMINI_API_KEY")
    if gemini_key:
        resp = requests.post(
            f"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={gemini_key}",
            json={"contents": [{"parts": [{"text": "hi"}]}]}
        )
        print(f"Gemini: {'OK' if resp.status_code == 200 else f'FAILED ({resp.status_code})'}")
    
    # 2. Anthropic
    anthropic_key = os.environ.get("ANTHROPIC_API_KEY")
    if anthropic_key:
        resp = requests.post(
            "https://api.anthropic.com/v1/messages",
            headers={"x-api-key": anthropic_key, "anthropic-version": "2023-06-01", "content-type": "application/json"},
            json={"model": "claude-3-5-sonnet-20241022", "max_tokens": 10, "messages": [{"role": "user", "content": "hi"}]}
        )
        print(f"Anthropic: {'OK' if resp.status_code == 200 else f'FAILED ({resp.status_code})'}")

if __name__ == "__main__":
    check_env()
    test_keys()

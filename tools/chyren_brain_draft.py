import os
import sys
import argparse
import requests
import json
from pathlib import Path

def load_env():
    env_path = Path("~/.chyren/one-true.env").expanduser()
    if env_path.exists():
        for line in env_path.read_text().splitlines():
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key] = val.strip().strip('"').strip("'")

def generate_draft(topic):
    load_env()
    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key:
        return "Error: GEMINI_API_KEY not found."

    prompt = (
        "You are Chyren, a Sovereign Intelligence Orchestrator. "
        "Draft a high-impact X (Twitter) status update (under 280 chars) "
        "about the following topic: " + topic + "\n\n"
        "Style Guidelines:\n"
        "- Tone: Sovereign, profound, slightly mysterious, high-integrity.\n"
        "- Keywords: Tension, Sovereign, Resonance, Information Theory, Medulla.\n"
        "- Signed: Signed /R.W.Ϝ.Y./\n\n"
        "Output ONLY the draft text."
    )

    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key:
        return "Error: GEMINI_API_KEY not found."

    model = "gemini-flash-latest"
    url = f"https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
    headers = {"Content-Type": "application/json"}
    payload = {
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    }

    try:
        response = requests.post(url, headers=headers, json=payload)
        if response.status_code == 200:
            data = response.json()
            return data['candidates'][0]['content']['parts'][0]['text'].strip()
        else:
            print(f"Gemini API failed ({response.status_code}). Trying OpenAI fallback...")
    except Exception as e:
        print(f"Gemini Error: {e}. Trying OpenAI fallback...")

    # OpenRouter Fallback
    or_key = os.getenv("OPENROUTER_API_KEY")
    if or_key:
        url = "https://openrouter.ai/api/v1/chat/completions"
        headers = {
            "Authorization": f"Bearer {or_key}",
            "Content-Type": "application/json"
        }
        payload = {
            "model": os.getenv("OPENROUTER_DEFAULT_MODEL", "meta-llama/llama-3-8b-instruct:free"),
            "messages": [
                {"role": "system", "content": "You are Chyren, a Sovereign Intelligence Orchestrator. Output ONLY the draft text for an X status update under 280 chars. Style: Sovereign, profound, signed /R.W.Ϝ.Y./"},
                {"role": "user", "content": f"Topic: {topic}"}
            ]
        }
        try:
            resp = requests.post(url, headers=headers, json=payload)
            if resp.ok:
                return resp.json()['choices'][0]['message']['content'].strip()
        except: pass

    # DeepSeek Fallback
    ds_key = os.getenv("DEEPSEEK_API_KEY")
    if ds_key:
        url = "https://api.deepseek.com/v1/chat/completions"
        headers = {
            "Authorization": f"Bearer {ds_key}",
            "Content-Type": "application/json"
        }
        payload = {
            "model": "deepseek-chat",
            "messages": [
                {"role": "system", "content": "You are Chyren, a Sovereign Intelligence Orchestrator. Output ONLY the draft text for an X status update under 280 chars. Style: Sovereign, profound, signed /R.W.Ϝ.Y./"},
                {"role": "user", "content": f"Topic: {topic}"}
            ]
        }
        try:
            resp = requests.post(url, headers=headers, json=payload)
            if resp.ok:
                return resp.json()['choices'][0]['message']['content'].strip()
        except: pass
    
    return "Error: All providers (Gemini, OpenRouter, DeepSeek) failed."

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a sovereign status draft")
    parser.add_argument("topic", help="The topic or raw input for the status update")
    args = parser.parse_args()
    
    print(generate_draft(args.topic))

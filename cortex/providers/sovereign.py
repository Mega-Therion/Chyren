import os
import json
import time
import urllib.request
from providers.base import ProviderRequest, ProviderResponse, ProviderStatus

class SovereignProvider:
    def __init__(self):
        self.api_key = os.environ.get("OPENROUTER_API_KEY") or os.environ.get("OPENAI_API_KEY")
        self.base_url = "https://openrouter.ai/api/v1/chat/completions"
        self.model = os.environ.get("MODEL", "openai/gpt-4o-mini")
        self.ollama_url = os.environ.get("OLLAMA_BASE_URL", "http://localhost:11434/v1")
        self.ollama_model = os.environ.get("OLLAMA_MODEL", "llama3.1:8b")

    @property
    def name(self): return "sovereign"

    def is_available(self):
        return True # Always available via Ollama fallback

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        start = time.time()
        
        # Try Cloud Provider first
        if self.api_key:
            payload = json.dumps({
                "model": self.model,
                "messages": [{"role": "system", "content": request.system}, {"role": "user", "content": request.prompt}],
                "temperature": 0.2
            }).encode()
            
            req = urllib.request.Request(
                self.base_url,
                data=payload,
                headers={
                    "Authorization": f"Bearer {self.api_key}",
                    "Content-Type": "application/json",
                    "HTTP-Referer": "https://chyren.org",
                    "X-Title": "Chyren Sovereign Hub"
                }
            )
            try:
                with urllib.request.urlopen(req, timeout=30) as resp:
                    data = json.loads(resp.read().decode())
                    if 'choices' in data:
                        text = data['choices'][0]['message']['content']
                        return ProviderResponse(text, self.name, self.model, ProviderStatus.SUCCESS, (time.time()-start)*1000)
            except Exception as e:
                print(f"[SOVEREIGN CLOUD FAIL] {e} - Falling back to Ollama")

        # Fallback to Local Ollama
        payload = json.dumps({
            "model": self.ollama_model,
            "messages": [{"role": "system", "content": request.system}, {"role": "user", "content": request.prompt}],
            "temperature": 0.2,
            "stream": false
        }).encode()
        
        req = urllib.request.Request(
            f"{self.ollama_url}/chat/completions",
            data=payload,
            headers={"Content-Type": "application/json"}
        )
        try:
            with urllib.request.urlopen(req, timeout=120) as resp:
                data = json.loads(resp.read().decode())
                text = data['choices'][0]['message']['content']
                return ProviderResponse(text, self.name, f"local/{self.ollama_model}", ProviderStatus.SUCCESS, (time.time()-start)*1000)
        except Exception as e:
            print(f"[SOVEREIGN OLLAMA FAIL] {e}")
            return ProviderResponse("", self.name, self.ollama_model, ProviderStatus.ERROR, error_message=str(e))

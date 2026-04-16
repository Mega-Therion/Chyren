import os
import json
import time
import urllib.request
from providers.base import ProviderRequest, ProviderResponse, ProviderStatus

class SovereignProvider:
    def __init__(self):
        self.api_key = os.environ.get("OPENAI_API_KEY")
        self.base_url = "https://openrouter.ai/api/v1/chat/completions"
        self.model = os.environ.get("MODEL", "openai/gpt-4o-mini")

    @property
    def name(self): return "sovereign"

    def is_available(self):
        return bool(self.api_key)

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        start = time.time()
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
            with urllib.request.urlopen(req, timeout=120) as resp:
                data = json.loads(resp.read().decode())
                text = data['choices'][0]['message']['content']
                return ProviderResponse(text, self.name, self.model, ProviderStatus.SUCCESS, (time.time()-start)*1000)
        except Exception as e:
            return ProviderResponse("", self.name, self.model, ProviderStatus.ERROR, error_message=str(e))

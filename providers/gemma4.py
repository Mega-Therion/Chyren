"""
providers/gemma4.py — Gemma 4 E2B connector via local Ollama (OpenAI-compatible API).
Environment variable: OLLAMA_BASE_URL (default: http://localhost:11434/v1)
Model: gemma4:e2b
"""

import json
import os
import time
import urllib.request
import urllib.error

from providers.base import ProviderRequest, ProviderResponse, ProviderStatus


class Gemma4Provider:
    DEFAULT_MODEL = "gemma4:e2b"

    def __init__(self, base_url: str | None = None, model: str | None = None):
        self._base_url = (base_url or os.environ.get("OLLAMA_BASE_URL", "http://localhost:11434/v1")).rstrip("/")
        self._model = model or self.DEFAULT_MODEL

    @property
    def name(self) -> str:
        return "gemma4"

    def is_available(self) -> bool:
        try:
            req = urllib.request.Request(f"{self._base_url}/models")
            with urllib.request.urlopen(req, timeout=3) as resp:
                data = json.loads(resp.read().decode())
                models = [m.get("id", "") for m in data.get("data", [])]
                return self._model in models
        except Exception:
            return False

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        model = request.model or self._model
        start = time.time()

        # Gemma 4 via ollama returns empty content when given a system role message.
        # Prepend system instructions inline into the user turn instead.
        if request.system:
            user_content = f"{request.system}\n\n{request.prompt}"
        else:
            user_content = request.prompt
        messages = [{"role": "user", "content": user_content}]

        body = {
            "model": model,
            "messages": messages,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": False,
        }

        payload = json.dumps(body).encode()
        req = urllib.request.Request(
            f"{self._base_url}/chat/completions",
            data=payload,
            headers={"Content-Type": "application/json"},
        )

        try:
            with urllib.request.urlopen(req, timeout=600) as resp:
                data = json.loads(resp.read().decode())
                text = data["choices"][0]["message"]["content"]
                usage = data.get("usage", {})
                return ProviderResponse(
                    text=text, provider_name=self.name, model=model,
                    status=ProviderStatus.SUCCESS,
                    latency_ms=(time.time() - start) * 1000,
                    token_count=usage.get("completion_tokens", 0),
                    raw_metadata={"usage": usage},
                )
        except urllib.error.HTTPError as e:
            body_text = e.read().decode() if hasattr(e, "read") else ""
            return ProviderResponse(
                text="", provider_name=self.name, model=model,
                status=ProviderStatus.ERROR,
                latency_ms=(time.time() - start) * 1000,
                error_message=f"HTTP {e.code}: {body_text[:300]}",
            )
        except Exception as e:
            return ProviderResponse(
                text="", provider_name=self.name, model=model,
                status=ProviderStatus.ERROR,
                latency_ms=(time.time() - start) * 1000,
                error_message=str(e),
            )

"""
providers/gemini.py — Google Gemini connector.
Environment variable: GEMINI_API_KEY
Provider initializes safely without a key — is_available() returns False
until the key is set.

Current implementation: Direct REST API (proven stable).
Future: Official google-genai SDK available in venv (requires API adjustment).
"""

import json
import os
import time
import urllib.request
import urllib.error

from providers.base import ProviderRequest, ProviderResponse, ProviderStatus


class GeminiProvider:
    API_BASE = "https://generativelanguage.googleapis.com/v1beta/models"
    DEFAULT_MODEL = "gemini-2.5-flash-lite"

    def __init__(self, api_key: str | None = None, model: str | None = None):
        self._api_key = api_key or os.environ.get("GEMINI_API_KEY", "")
        self._model = model or self.DEFAULT_MODEL

    @property
    def name(self) -> str:
        return "gemini"

    def is_available(self) -> bool:
        return bool(self._api_key)

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        if not self._api_key:
            return ProviderResponse(
                text="", provider_name=self.name,
                model=request.model or self._model,
                status=ProviderStatus.UNAVAILABLE,
                error_message="GEMINI_API_KEY not set",
            )

        model = request.model or self._model
        start = time.time()

        body: dict = {
            "contents": [{"parts": [{"text": request.prompt}]}],
            "generationConfig": {
                "temperature": request.temperature,
                "maxOutputTokens": request.max_tokens,
            },
        }
        if request.system:
            body["systemInstruction"] = {"parts": [{"text": request.system}]}

        payload = json.dumps(body).encode()
        url = f"{self.API_BASE}/{model}:generateContent?key={self._api_key}"

        req = urllib.request.Request(
            url, data=payload,
            headers={"Content-Type": "application/json"},
        )

        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode())
                candidates = data.get("candidates", [])
                text = ""
                if candidates:
                    parts = candidates[0].get("content", {}).get("parts", [])
                    text = "".join(p.get("text", "") for p in parts)
                usage = data.get("usageMetadata", {})
                return ProviderResponse(
                    text=text, provider_name=self.name, model=model,
                    status=ProviderStatus.SUCCESS,
                    latency_ms=(time.time() - start) * 1000,
                    token_count=usage.get("candidatesTokenCount", 0),
                    raw_metadata={"usage": usage},
                )
        except TimeoutError:
            return ProviderResponse(
                text="", provider_name=self.name, model=model,
                status=ProviderStatus.TIMEOUT,
                latency_ms=(time.time() - start) * 1000,
                error_message="Request timed out",
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

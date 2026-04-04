"""
providers/anthropic.py — Anthropic Claude connector.
Environment variable: ANTHROPIC_API_KEY
"""

import json
import os
import time
import urllib.request
import urllib.error

from providers.base import ProviderRequest, ProviderResponse, ProviderStatus


class AnthropicProvider:
    API_URL = "https://api.anthropic.com/v1/messages"
    DEFAULT_MODEL = "claude-sonnet-4-20250514"

    def __init__(self, api_key: str | None = None, model: str | None = None):
        self._api_key = api_key or os.environ.get("ANTHROPIC_API_KEY", "")
        self._model = model or self.DEFAULT_MODEL

    @property
    def name(self) -> str:
        return "anthropic"

    def is_available(self) -> bool:
        return bool(self._api_key)

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        if not self._api_key:
            return ProviderResponse(
                text="", provider_name=self.name,
                model=request.model or self._model,
                status=ProviderStatus.UNAVAILABLE,
                error_message="ANTHROPIC_API_KEY not set",
            )

        model = request.model or self._model
        start = time.time()

        body: dict = {
            "model": model,
            "max_tokens": request.max_tokens,
            "messages": [{"role": "user", "content": request.prompt}],
        }
        if request.system:
            body["system"] = request.system

        payload = json.dumps(body).encode()
        req = urllib.request.Request(
            self.API_URL,
            data=payload,
            headers={
                "Content-Type": "application/json",
                "x-api-key": self._api_key,
                "anthropic-version": "2023-06-01",
            },
        )

        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode())
                text = "".join(
                    b.get("text", "") for b in data.get("content", [])
                    if b.get("type") == "text"
                )
                usage = data.get("usage", {})
                return ProviderResponse(
                    text=text, provider_name=self.name, model=model,
                    status=ProviderStatus.SUCCESS,
                    latency_ms=(time.time() - start) * 1000,
                    token_count=usage.get("output_tokens", 0),
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

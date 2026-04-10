from __future__ import annotations

import asyncio
import json
import os
import urllib.request
from typing import AsyncIterator

from chyren_cli.providers.base import Provider, ProviderEvent, ProviderRequest


class GeminiProvider(Provider):
    """
    Gemini provider for the CLI via the stable REST API.
    """

    API_BASE = "https://generativelanguage.googleapis.com/v1beta/models"
    DEFAULT_MODEL = "gemini-2.5-flash-lite"

    def __init__(self, api_key: str | None = None, model: str | None = None) -> None:
        self._api_key = api_key or os.environ.get("GEMINI_API_KEY", "")
        self._model = model or self.DEFAULT_MODEL

    @property
    def name(self) -> str:
        return "gemini"

    def is_available(self) -> bool:
        return bool(self._api_key)

    async def stream(self, request: ProviderRequest) -> AsyncIterator[ProviderEvent]:
        if not self._api_key:
            yield ProviderEvent(type="error", error_message="GEMINI_API_KEY not set")
            return

        yield ProviderEvent(type="start", raw={"provider": self.name})

        model = request.model or self._model
        url = f"{self.API_BASE}/{model}:generateContent?key={self._api_key}"

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

        def _call() -> str:
            req = urllib.request.Request(
                url,
                data=payload,
                headers={"Content-Type": "application/json"},
            )
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode())
            candidates = data.get("candidates", [])
            if not candidates:
                return ""
            parts = candidates[0].get("content", {}).get("parts", [])
            return "".join(p.get("text", "") for p in parts)

        try:
            text = await asyncio.to_thread(_call)
            if text:
                yield ProviderEvent(type="delta", text=text)
            yield ProviderEvent(type="end")
        except Exception as exc:
            yield ProviderEvent(type="error", error_message=str(exc))
            yield ProviderEvent(type="end")


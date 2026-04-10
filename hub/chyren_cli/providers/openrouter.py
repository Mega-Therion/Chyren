from __future__ import annotations

import asyncio
import json
import os
import urllib.request
from typing import AsyncIterator

from chyren_cli.providers.base import Provider, ProviderEvent, ProviderRequest


class OpenRouterProvider(Provider):
    """
    OpenRouter provider using the OpenAI-compatible chat completions API.
    """

    API_URL = "https://openrouter.ai/api/v1/chat/completions"
    DEFAULT_MODEL = "openai/gpt-4o-mini"

    def __init__(self, api_key: str | None = None, model: str | None = None) -> None:
        self._api_key = api_key or os.environ.get("OPENROUTER_API_KEY", "")
        self._model = model or self.DEFAULT_MODEL

    @property
    def name(self) -> str:
        return "openrouter"

    def is_available(self) -> bool:
        return bool(self._api_key)

    async def stream(self, request: ProviderRequest) -> AsyncIterator[ProviderEvent]:
        if not self._api_key:
            yield ProviderEvent(type="error", error_message="OPENROUTER_API_KEY not set")
            return

        yield ProviderEvent(type="start", raw={"provider": self.name})

        model = request.model or self._model

        messages: list[dict] = []
        if request.system:
            messages.append({"role": "system", "content": request.system})
        messages.append({"role": "user", "content": request.prompt})

        payload = json.dumps(
            {
                "model": model,
                "messages": messages,
                "temperature": request.temperature,
                "max_tokens": request.max_tokens,
                "stream": False,
            }
        ).encode()

        def _call() -> str:
            req = urllib.request.Request(
                self.API_URL,
                data=payload,
                headers={
                    "Content-Type": "application/json",
                    "Authorization": f"Bearer {self._api_key}",
                },
            )
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode())
            return data["choices"][0]["message"]["content"]

        try:
            text = await asyncio.to_thread(_call)
            if text:
                yield ProviderEvent(type="delta", text=text)
            yield ProviderEvent(type="end")
        except Exception as exc:
            yield ProviderEvent(type="error", error_message=str(exc))
            yield ProviderEvent(type="end")


from __future__ import annotations

import asyncio
import abc
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, AsyncIterator, Literal, Sequence


class ProviderStatus(str, Enum):
    SUCCESS = "success"
    ERROR = "error"
    UNAVAILABLE = "unavailable"
    TIMEOUT = "timeout"
    CANCELLED = "cancelled"


@dataclass(frozen=True)
class ProviderRequest:
    """
    Normalized request sent to any provider adapter.

    Notes:
    - `messages` is optional; for chat-native providers it can be used directly.
      If absent, providers should treat `prompt` as the user message.
    - `metadata` is reserved for router/session/context-injection fields that
      should not change provider behavior unless explicitly supported.
    """

    prompt: str
    system: str = ""
    model: str = ""
    temperature: float = 0.3
    top_p: float | None = None
    max_tokens: int = 1024
    run_id: str = ""
    messages: Sequence[dict[str, Any]] | None = None
    metadata: dict[str, Any] = field(default_factory=dict)


ProviderEventType = Literal["start", "delta", "tool", "error", "end"]


@dataclass(frozen=True)
class ProviderEvent:
    """
    Streaming event emitted by a provider.

    - type="delta": incremental text in `text`
    - type="error": error info in `error_message`
    - type="start"/"end": lifecycle markers with optional metadata
    - type="tool": reserved for future tool-call events
    """

    type: ProviderEventType
    text: str = ""
    error_message: str = ""
    raw: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class ProviderResponse:
    """Normalized final response from any provider."""

    text: str
    provider_name: str
    model: str
    status: ProviderStatus
    latency_ms: float = 0.0
    token_count: int = 0
    error_message: str = ""
    raw_metadata: dict[str, Any] = field(default_factory=dict)

    @property
    def ok(self) -> bool:
        return self.status == ProviderStatus.SUCCESS


class Provider(abc.ABC):
    """
    Async-first provider contract for the Chyren CLI.

    Implementers should:
    - be safe when credentials are missing (is_available() gates network calls)
    - support streaming via `stream()` whenever the upstream API allows it
    - keep `generate()` correct even if `stream()` is the primary implementation
    """

    @property
    @abc.abstractmethod
    def name(self) -> str: ...

    @abc.abstractmethod
    def is_available(self) -> bool: ...

    async def generate(self, request: ProviderRequest) -> ProviderResponse:
        """
        Default non-streaming implementation: collect from `stream()`.

        Providers may override this for a more efficient non-streaming call.
        """
        start = time.perf_counter()
        chunks: list[str] = []
        raw_meta: dict[str, Any] = {}

        try:
            async for ev in self.stream(request):
                if ev.type == "delta" and ev.text:
                    chunks.append(ev.text)
                elif ev.type in ("start", "end") and ev.raw:
                    raw_meta.update(ev.raw)
                elif ev.type == "error":
                    latency_ms = (time.perf_counter() - start) * 1000
                    return ProviderResponse(
                        text="".join(chunks),
                        provider_name=self.name,
                        model=request.model,
                        status=ProviderStatus.ERROR,
                        latency_ms=latency_ms,
                        token_count=0,
                        error_message=ev.error_message or "provider error",
                        raw_metadata=raw_meta | (ev.raw or {}),
                    )
        except asyncio.CancelledError:  # type: ignore[name-defined]
            latency_ms = (time.perf_counter() - start) * 1000
            return ProviderResponse(
                text="".join(chunks),
                provider_name=self.name,
                model=request.model,
                status=ProviderStatus.CANCELLED,
                latency_ms=latency_ms,
                token_count=0,
                error_message="cancelled",
                raw_metadata=raw_meta,
            )
        except Exception as exc:
            latency_ms = (time.perf_counter() - start) * 1000
            return ProviderResponse(
                text="".join(chunks),
                provider_name=self.name,
                model=request.model,
                status=ProviderStatus.ERROR,
                latency_ms=latency_ms,
                token_count=0,
                error_message=str(exc),
                raw_metadata=raw_meta,
            )

        latency_ms = (time.perf_counter() - start) * 1000
        return ProviderResponse(
            text="".join(chunks),
            provider_name=self.name,
            model=request.model,
            status=ProviderStatus.SUCCESS,
            latency_ms=latency_ms,
            token_count=0,
            raw_metadata=raw_meta,
        )

    @abc.abstractmethod
    async def stream(self, request: ProviderRequest) -> AsyncIterator[ProviderEvent]:
        """
        Stream provider output as ProviderEvents.

        Must yield:
        - ProviderEvent(type="start") once (recommended)
        - zero or more ProviderEvent(type="delta", text="...")
        - ProviderEvent(type="end") once (recommended)
        """

        raise NotImplementedError


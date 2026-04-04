"""
providers/base.py — Provider interface and routing engine.

All providers normalize their output into ProviderResponse.
The ProviderRouter selects and falls back across registered providers.
No provider crashes the system if credentials are absent — is_available()
gates each provider before any network call is attempted.
"""

import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Protocol


class ProviderStatus(str, Enum):
    SUCCESS    = "success"
    ERROR      = "error"
    UNAVAILABLE = "unavailable"
    TIMEOUT    = "timeout"


@dataclass
class ProviderRequest:
    """Normalized request passed to any provider."""
    prompt: str
    system: str = ""
    model: str = ""
    temperature: float = 0.3
    max_tokens: int = 1024
    run_id: str = ""
    state_context: dict = field(default_factory=dict)  # injected ledger context


@dataclass
class ProviderResponse:
    """Normalized response from any provider."""
    text: str
    provider_name: str
    model: str
    status: ProviderStatus
    latency_ms: float = 0.0
    token_count: int = 0
    error_message: str = ""
    raw_metadata: dict = field(default_factory=dict)

    @property
    def ok(self) -> bool:
        return self.status == ProviderStatus.SUCCESS


class BaseProvider(Protocol):
    """Interface every provider must satisfy."""

    @property
    def name(self) -> str: ...

    def is_available(self) -> bool: ...

    def generate(self, request: ProviderRequest) -> ProviderResponse: ...


class ProviderRouter:
    """
    Routes requests to providers with ordered fallback.
    Providers are tried in preference order; the first successful response wins.
    """

    def __init__(self):
        self._providers: dict[str, BaseProvider] = {}
        self._order: list[str] = []

    def register(self, provider: BaseProvider) -> None:
        self._providers[provider.name] = provider
        if provider.name not in self._order:
            self._order.append(provider.name)

    def set_preference(self, order: list[str]) -> None:
        self._order = order

    def available(self) -> list[str]:
        return [n for n, p in self._providers.items() if p.is_available()]

    def route(
        self,
        request: ProviderRequest,
        preferred: str | None = None,
    ) -> ProviderResponse:
        """
        Route a request through the provider chain.
        If preferred is set, try it first before falling back to the ordered chain.
        Returns the first successful ProviderResponse, or an UNAVAILABLE response
        if all providers fail.
        """
        order = list(self._order)
        if preferred and preferred in self._providers:
            order = [preferred] + [n for n in order if n != preferred]

        errors: list[str] = []
        for name in order:
            provider = self._providers.get(name)
            if not provider or not provider.is_available():
                errors.append(f"{name}: not available")
                continue
            try:
                response = provider.generate(request)
            except Exception as exc:
                errors.append(f"{name}: exception — {exc}")
                continue
            if response.ok:
                return response
            errors.append(f"{name}: {response.error_message}")

        return ProviderResponse(
            text="",
            provider_name="none",
            model=request.model,
            status=ProviderStatus.UNAVAILABLE,
            error_message="All providers failed: " + "; ".join(errors),
        )

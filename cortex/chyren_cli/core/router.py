from __future__ import annotations

from dataclasses import dataclass
from typing import AsyncIterator, Optional

from chyren_cli.providers.base import Provider, ProviderEvent, ProviderRequest, ProviderResponse, ProviderStatus


@dataclass
class ProviderError(Exception):
    message: str


class ProviderRouter:
    """
    Async-first provider router with ordered fallback.
    """

    def __init__(self) -> None:
        self._providers: dict[str, Provider] = {}
        self._order: list[str] = []

    def register(self, provider: Provider) -> None:
        self._providers[provider.name] = provider
        if provider.name not in self._order:
            self._order.append(provider.name)

    def set_preference(self, order: list[str]) -> None:
        self._order = order

    def available(self) -> list[str]:
        return [name for name, p in self._providers.items() if p.is_available()]

    def _route_order(self, preferred: Optional[str]) -> list[str]:
        order = list(self._order)
        if preferred and preferred in self._providers:
            order = [preferred] + [n for n in order if n != preferred]
        return order

    def generate(
        self,
        prompt: str,
        *,
        system: str = "",
        model: str = "",
        preferred: Optional[str] = None,
    ) -> ProviderResponse:
        """
        Non-streaming convenience wrapper (runs provider async generate via event collection).
        """
        # Use the default Provider.generate (collects from stream), but we must run it synchronously.
        import asyncio

        async def _run() -> ProviderResponse:
            async for _ in self.stream(prompt, system=system, model=model, preferred=preferred):
                pass
            # stream() returns deltas; we also want the final text; easiest is call each provider.generate
            # for the first available provider in order.
            for name in self._route_order(preferred):
                p = self._providers.get(name)
                if not p or not p.is_available():
                    continue
                return await p.generate(ProviderRequest(prompt=prompt, system=system, model=model))
            return ProviderResponse(
                text="",
                provider_name="none",
                model=model,
                status=ProviderStatus.UNAVAILABLE,
                error_message="No providers available",
            )

        return asyncio.run(_run())

    def stream(
        self,
        prompt: str,
        *,
        system: str = "",
        model: str = "",
        preferred: Optional[str] = None,
    ) -> AsyncIterator[ProviderEvent]:
        async def _stream() -> AsyncIterator[ProviderEvent]:
            errors: list[str] = []
            req = ProviderRequest(prompt=prompt, system=system, model=model)
            for name in self._route_order(preferred):
                provider = self._providers.get(name)
                if not provider or not provider.is_available():
                    errors.append(f"{name}: unavailable")
                    continue
                try:
                    async for ev in provider.stream(req):
                        yield ev
                    return
                except Exception as exc:
                    errors.append(f"{name}: {exc}")
                    continue

            yield ProviderEvent(
                type="error",
                error_message="All providers failed: " + "; ".join(errors),
            )

        return _stream()


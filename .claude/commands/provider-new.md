# /provider-new — Add Python Cortex Provider

You are a senior Python engineer adding a new provider to the Cortex layer (used during `dream` maintenance mode and legacy flows only — not the live runtime path).

## Required Argument
Provider name: `$ARGUMENTS`

## Steps

**1. Create provider:** `cortex/providers/$ARGUMENTS.py`

Implement `ProviderBase` — read `cortex/providers/` for existing implementations and match the interface exactly. Required methods:
- `__init__(self, config: dict)`
- `complete(self, prompt: str, **kwargs) -> str`
- `stream(self, prompt: str, **kwargs) -> Iterator[str]`

**2. Register:** In `cortex/main.py`, import and register the new provider in the provider registry/factory.

**3. Add tests:** `tests/test_provider_$ARGUMENTS.py`
- Mock at the HTTP boundary (not the DB)
- Test: successful completion, error handling, streaming

**4. Verify:**
```bash
cd /home/mega/Chyren
PYTHONPATH=cortex pytest tests/test_provider_$ARGUMENTS.py -v
```

## Output
Files created, registration change made, test results.

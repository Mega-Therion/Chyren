"""
providers/huggingface.py — Hugging Face Inference API connector.
Environment variable: HF_API_KEY
"""

import json
import os
import time
import urllib.request
import urllib.error

from providers.base import ProviderRequest, ProviderResponse, ProviderStatus


class HuggingFaceProvider:
    # Example for a high-capability model; can be overridden via model request.
    # Base URL for Hugging Face Inference API
    API_URL = "https://api-inference.huggingface.co/models/"
    DEFAULT_MODEL = "mistralai/Mistral-Nemo-Instruct-2407"

    def __init__(self, api_key: str | None = None, model: str | None = None):
        self._api_key = api_key or os.environ.get("HF_API_KEY", "")
        self._model = model or self.DEFAULT_MODEL
        # Use the configured API base or default
        self.base_url = os.environ.get("HF_API_BASE", self.API_URL)

    @property
    def name(self) -> str:
        return "huggingface"

    def is_available(self) -> bool:
        return bool(self._api_key)

    def generate(self, request: ProviderRequest) -> ProviderResponse:
        if not self._api_key:
            return ProviderResponse(
                text="", provider_name=self.name,
                model=request.model or self._model,
                status=ProviderStatus.UNAVAILABLE,
                error_message="HF_API_KEY not set",
            )

        start = time.time()

        # Proper prompt structure for Mistral models
        prompt = f"<s>[INST] {request.system}\n{request.prompt} [/INST]"

        # Construct model URL
        model = request.model or self._model
        url = f"{self.base_url.rstrip('/')}/{model.lstrip('/')}"

        payload = json.dumps({
            "inputs": prompt,
            "parameters": {
                "max_new_tokens": request.max_tokens,
                "temperature": request.temperature,
                "return_full_text": False
            },
        }).encode()

        req = urllib.request.Request(
            url,
            data=payload,
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {self._api_key}",
            },
        )
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode())
                # Handle error if API returns an object with 'error' key
                if isinstance(data, dict) and "error" in data:
                    return ProviderResponse(
                        text="", provider_name=self.name, model=self._model,
                        status=ProviderStatus.ERROR,
                        error_message=data["error"],
                    )
                # HF Inference API returns a list of dicts
                text = data[0]["generated_text"]
                return ProviderResponse(
                    text=text, provider_name=self.name, model=self._model,
                    status=ProviderStatus.SUCCESS,
                    latency_ms=(time.time() - start) * 1000,
                )
        except Exception as e:
            return ProviderResponse(
                text="", provider_name=self.name, model=self._model,
                status=ProviderStatus.ERROR,
                latency_ms=(time.time() - start) * 1000,
                error_message=str(e),
            )

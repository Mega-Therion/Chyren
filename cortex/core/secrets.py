"""
core/secrets.py — Mock secret manager.

Replaces raw environment variable access with a centralized, secure interface.
"""

import os
from typing import Optional

class SecretManager:
    """Mock secret manager, currently wrapping environment variables."""
    
    @staticmethod
    def get(key: str, default: Optional[str] = None) -> str:
        """Retrieve a secret."""
        # Future: Implement vault integration (HashiCorp, AWS, etc.)
        return os.environ.get(key, default or "")

    @staticmethod
    def set(key: str, value: str) -> None:
        """Temporarily set a secret."""
        os.environ[key] = value

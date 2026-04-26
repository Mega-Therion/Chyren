"""
core/secrets.py — Centralized Config Loader.

Provides a unified interface for loading secrets and configuration.
Loads from environment variables with support for centralized schema validation.
"""

import os
import yaml
from typing import Optional, Any
from pathlib import Path

class ConfigLoader:
    """Centralized config loader for Chyren."""
    
    _config_path = Path(__file__).parent.parent.parent / "config" / "schema.yaml"
    _schema = None

    @classmethod
    def _load_schema(cls) -> dict:
        if cls._schema is None:
            if cls._config_path.exists():
                with open(cls._config_path, "r") as f:
                    cls._schema = yaml.safe_load(f)
            else:
                cls._schema = {}
        return cls._schema

    @staticmethod
    def get(key: str, default: Optional[str] = None) -> str:
        """Retrieve a secret or configuration value."""
        return os.environ.get(key, default or "")

    @classmethod
    def get_by_schema(cls, category: str, item: str) -> Any:
        """Retrieve value based on schema keys."""
        schema = cls._load_schema()
        target = schema.get(category, {}).get(item, {})
        env_key = target.get("key")
        if env_key:
            return os.environ.get(env_key, "")
        return ""

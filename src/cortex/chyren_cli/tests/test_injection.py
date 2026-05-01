from __future__ import annotations

import tempfile
from pathlib import Path
import unittest

from chyren_cli.core.context_injection import expand_injections


class TestContextInjection(unittest.TestCase):
    def test_file_injection(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "a.txt"
            p.write_text("hello", encoding="utf-8")
            out = expand_injections(f"test @file:{p}")
            self.assertIn("FILE", out)
            self.assertIn("hello", out)

    def test_missing_file(self) -> None:
        out = expand_injections("x @file:/nope/does-not-exist.txt")
        self.assertIn("missing file", out)


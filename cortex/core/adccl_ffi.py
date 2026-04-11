"""
core/adccl_ffi.py — FFI bridge to the Rust ADCCL shared library.

Loads `medulla/target/release/libomega_adccl.so` when available.
Falls back to the pure-Python ADCCL implementation transparently so the
cortex works in environments where the medulla hasn't been compiled yet
(CI, fresh clones, dev boxes without Rust installed).
"""

import ctypes
import json
import os

# Resolve the .so path relative to this file:
#   cortex/core/adccl_ffi.py → ../../medulla/target/release/
_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
_LIB_PATH = os.environ.get(
    "OMEGA_ADCCL_LIB",
    os.path.join(_ROOT, "medulla", "target", "release", "libomega_adccl.so"),
)

_lib = None
if os.path.exists(_LIB_PATH):
    try:
        _lib = ctypes.CDLL(_LIB_PATH)
        # Use c_void_p so Python preserves the raw pointer value rather than
        # converting it to a bytes object — we need the original address to
        # pass to free_string, otherwise it crashes with an invalid pointer.
        _lib.verify_response.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
        _lib.verify_response.restype = ctypes.c_void_p
        _lib.free_string.argtypes = [ctypes.c_void_p]
        _lib.free_string.restype = None
    except OSError:
        _lib = None


class ADCCL:
    """
    Anti-Drift Cognitive Control Loop.

    Uses the Rust FFI when the compiled library is present; otherwise
    delegates to the pure-Python implementation in core.adccl.
    """

    def __init__(self, min_score: float = 0.1, session_start=None):
        self._min_score = min_score
        self._session_start = session_start
        self._use_ffi = _lib is not None

        if not self._use_ffi:
            from core.adccl import ADCCL as _PythonADCCL
            self._fallback = _PythonADCCL(min_score=min_score, session_start=session_start)

    def verify(self, response_text: str, task: str = ""):
        if self._use_ffi:
            result_ptr = _lib.verify_response(
                response_text.encode("utf-8"), task.encode("utf-8")
            )
            result_json = ctypes.string_at(result_ptr).decode("utf-8")
            _lib.free_string(ctypes.c_void_p(result_ptr))
            data = json.loads(result_json)

            from dataclasses import dataclass

            @dataclass
            class VerificationResult:
                passed: bool
                score: float
                flags: list
                status: str

            return VerificationResult(**data)

        return self._fallback.verify(response_text, task=task)

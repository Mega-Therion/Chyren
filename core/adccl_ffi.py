import ctypes
import os
import json
import platform

# Determine the library path
LIB_PATH = "/home/mega/Chyren/omega_workspace/workspace/OmegA-Next/target/release/libomega_adccl.so"

# Load the Rust library
_lib = ctypes.CDLL(LIB_PATH)

# Define FFI interface
_lib.verify_response.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
_lib.verify_response.restype = ctypes.c_char_p
_lib.free_string.argtypes = [ctypes.c_char_p]

class ADCCL:
    def __init__(self, min_score=0.1, session_start=None):
        self._base_min_score = min_score
        self._session_start = session_start

    def verify(self, response_text, task=""):
        response_bytes = response_text.encode('utf-8')
        task_bytes = task.encode('utf-8')
        
        # Call the Rust FFI
        result_ptr = _lib.verify_response(response_bytes, task_bytes)
        result_json = ctypes.string_at(result_ptr).decode('utf-8')
        _lib.free_string(result_ptr)
        
        result_data = json.loads(result_json)
        
        from dataclasses import dataclass
        @dataclass
        class VerificationResult:
            passed: bool
            score: float
            flags: list
            status: str
            
        return VerificationResult(**result_data)

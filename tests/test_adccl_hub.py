import pytest
from core.adccl import ADCCL

def test_adccl_clean_response():
    gate = ADCCL(min_score=0.1)
    result = gate.verify(
        "Quantum entanglement is a phenomenon where two particles become correlated.",
        "Explain quantum entanglement"
    )
    assert result.passed
    assert result.score > 0.8
    assert not result.flags

def test_adccl_stub_markers_detected():
    gate = ADCCL()
    result = gate.verify(
        "Here is the code: TODO implement this later.",
        "Write some code"
    )
    assert not result.passed
    assert "STUB_MARKERS_DETECTED" in result.flags

def test_adccl_empty_response():
    gate = ADCCL()
    result = gate.verify("", "Task")
    assert "RESPONSE_TOO_SHORT" in result.flags

def test_adccl_capability_refusal():
    gate = ADCCL()
    result = gate.verify(
        "As an AI, I am unable to look up weather data.",
        "Look up weather"
    )
    assert "CAPABILITY_REFUSAL" in result.flags

def test_adccl_hallucination_anchors():
    gate = ADCCL()
    result = gate.verify(
        "As of my last training cutoff, I don't have access to the internet.",
        "Latest news"
    )
    assert result.score < 1.0 or result.passed

def test_adccl_no_task_overlap():
    gate = ADCCL()
    result = gate.verify(
        "Mitochondria is the powerhouse of the cell.",
        "Explain the history of the roman empire and its rapid territorial expansion"
    )
    assert "NO_TASK_WORD_OVERLAP" in result.flags

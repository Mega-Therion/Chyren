import pytest
import os
import tempfile
import json
from core.ledger import Ledger, LedgerEntry
from core.adccl import ADCCL

from core.alignment import AlignmentLayer, Constitution
import time

@pytest.fixture
def temp_ledger_path():
    fd, path = tempfile.mkstemp()
    os.close(fd)
    yield path
    os.remove(path)


def test_full_pipeline_smoke(temp_ledger_path):
    """
    Simulates the core logic of `Chyren.run()` without requiring HTTP provider calls.
    Verifies Alignment -> ADCCL -> Ledger Commit -> Yettragrammaton Signed.
    """
    ledger = Ledger(path=temp_ledger_path)
    adccl = ADCCL()
    alignment = AlignmentLayer(interactive=False)
    
    # 2. Alignment
    task = "System initialization test."
    # Use empty constitution if none exists just to check pipeline logic
    align_result = alignment.check(task)
    assert align_result.passed
    
    # 3. Simulate Provider Response
    provider_resp = "This is a fully constructed system initialization response without any stubs or placeholders."
    
    # 4. ADCCL Verification
    verify_result = adccl.verify(provider_resp, task)
    assert verify_result.passed
    
    # 5. Ledger Commit
    entry = LedgerEntry(
        run_id=f"smoke-run-{int(time.time())}",
        task=task,
        provider="smoke-test",
        model="smoke-model",
        status="completed",
        response_text=provider_resp,
        latency_ms=15.0,
        token_count=18,
        adccl_score=verify_result.score,
        adccl_flags=verify_result.flags,
        state_snapshot={"smoke": True}
    )
    
    stamped = ledger.commit(entry)
    run_id = stamped["run_id"]
    signature = stamped["signature"]
    assert run_id is not None
    assert signature is not None
    
    # 6. Verify Yettragrammaton Signature exists in the written file
    with open(temp_ledger_path, "r") as f:
        data = json.load(f)
        entries = data.get("entries", [])
        assert len(entries) == 1
        record = entries[0]
        assert record["run_id"] == run_id
        assert record["signature"] == signature
        assert len(record["signature"]) > 10

def test_pipeline_rejection_path(temp_ledger_path):
    adccl = ADCCL()
    
    # Simulate a bad provider response
    provider_resp = "As an AI, I am unable to initialize the system."
    task = "Initialize system."
    
    verify_result = adccl.verify(provider_resp, task)
    assert "CAPABILITY_REFUSAL" in verify_result.flags
    
    # Verify that in a real rejection scenario the score is penalized heavily
    assert verify_result.score < 1.0

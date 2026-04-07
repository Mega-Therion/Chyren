import pytest
import os
import tempfile
import json
from core.ledger import Ledger, LedgerEntry
from core.integrity import consensus_hash

@pytest.fixture
def temp_ledger_path():
    fd, path = tempfile.mkstemp()
    os.close(fd)
    yield path
    os.remove(path)

def test_ledger_commit(temp_ledger_path):
    ledger = Ledger(path=temp_ledger_path)
    
    entry = LedgerEntry(
        run_id="run1",
        task="Test task",
        provider="anthropic",
        model="claude-3-5-sonnet",
        status="completed",
        response_text="This is a test response.",
        latency_ms=100,
        token_count=10,
        adccl_score=1.0,
        adccl_flags=[],
        state_snapshot={}
    )
    ledger.commit(entry)
    
    with open(temp_ledger_path, "r") as f:
        data = json.load(f)
        entries = data.get("entries", [])
        assert len(entries) == 1
        record = entries[0]
        assert record["task"] == "Test task"
        assert "signature" in record
        
def test_ledger_consensus_hash(temp_ledger_path):
    ledger = Ledger(path=temp_ledger_path)
    
    entry1 = LedgerEntry(
        run_id="run1",
        task="Test task",
        provider="anthropic",
        model="claude-3-5-sonnet",
        status="completed",
        response_text="resp1",
        latency_ms=100,
        token_count=10,
        adccl_score=1.0,
        adccl_flags=[],
        state_snapshot={}
    )
    ledger.commit(entry1)
    
    entry2 = LedgerEntry(
        run_id="run2",
        task="Test task2",
        provider="anthropic",
        model="claude-3-5-sonnet",
        status="completed",
        response_text="resp2",
        latency_ms=100,
        token_count=10,
        adccl_score=1.0,
        adccl_flags=[],
        state_snapshot={}
    )
    ledger.commit(entry2)
    
    with open(temp_ledger_path, "r") as f:
        data = json.load(f)
        entries = data.get("entries", [])
        assert len(entries) == 2

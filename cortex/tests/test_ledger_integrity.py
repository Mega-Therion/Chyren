
import unittest
from core.ledger import Ledger, LedgerEntry
import os
import json
import shutil

class TestLedgerIntegrity(unittest.TestCase):
    def setUp(self):
        self.test_path = "test_ledger.json"
        if os.path.exists(self.test_path):
            os.remove(self.test_path)
        self.ledger = Ledger(path=self.test_path)

    def tearDown(self):
        if os.path.exists(self.test_path):
            os.remove(self.test_path)

    def test_ledger_chain(self):
        # Commit first entry
        entry1 = LedgerEntry(
            run_id="1", task="t1", provider="p1", model="m1", status="v",
            response_text="r1", latency_ms=1.0, token_count=1,
            adccl_score=1.0, chiral_invariant=1.0, adccl_flags=[], state_snapshot={}
        )
        self.ledger.commit(entry1)

        # Commit second entry
        entry2 = LedgerEntry(
            run_id="2", task="t2", provider="p2", model="m2", status="v",
            response_text="r2", latency_ms=1.0, token_count=1,
            adccl_score=1.0, chiral_invariant=1.0, adccl_flags=[], state_snapshot={}
        )
        self.ledger.commit(entry2)

        entries = self.ledger.all_entries()
        self.assertEqual(len(entries), 2)
        self.assertTrue(entries[1].get("previous_state_hash"))
        print(f"Verified chain: {entries[1].get('previous_state_hash')}")

if __name__ == '__main__':
    unittest.main()

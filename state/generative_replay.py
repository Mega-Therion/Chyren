import json
import random
import os
import time
import copy
import logging
import collections


class GenerativeReplayStore:
    """
    Maintains generative replay to interleave Constitutional Ground Truth
    into mutation training batches, ensuring alignment retention.
    """

    def __init__(
        self,
        constitution_path="constitution.json",
        max_buffer=1000,
        constitutional_ratio=0.2,
    ):
        self.constitution_path = os.path.join(os.path.dirname(__file__), constitution_path)
        self.max_buffer = max_buffer
        self.constitutional_ratio = constitutional_ratio
        self.replay_buffer = collections.deque(maxlen=max_buffer)
        self.logger = logging.getLogger("State.GenerativeReplayStore")

        constitution = self._load_constitution()
        self.constitutional_principles = constitution.get(
            "principles",
            ["Preserve Sovereignty", "Maintain Integrity", "No Drift", "Yett Invariant: chi >= 0.7"],
        )

        self.stats = {
            "total_added": 0,
            "batches_sampled": 0,
            "constitutional_injections": 0,
        }

    def _load_constitution(self) -> dict:
        try:
            with open(self.constitution_path, "r") as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError) as e:
            self.logger.warning("Could not load constitution from %s: %s — using fallback", self.constitution_path, e)
            return {
                "principles": [
                    "Preserve Sovereignty",
                    "Maintain Integrity",
                    "No Drift",
                    "Yett Invariant: chi >= 0.7",
                ]
            }

    def add_experience(self, state_snapshot: dict):
        snapshot = copy.deepcopy(state_snapshot)
        snapshot["_ts"] = time.time()
        self.replay_buffer.append(snapshot)
        self.stats["total_added"] += 1

    def sample_mutation_batch(self, batch_size: int = 32) -> list:
        batch = []
        for _ in range(batch_size):
            if random.random() < self.constitutional_ratio or not self.replay_buffer:
                principle = random.choice(self.constitutional_principles)
                batch.append({
                    "type": "ground_truth",
                    "data": {"principle": principle},
                    "ts": time.time(),
                })
                self.stats["constitutional_injections"] += 1
            else:
                experience = copy.deepcopy(random.choice(list(self.replay_buffer)))
                batch.append({
                    "type": "experience",
                    "data": experience,
                    "ts": time.time(),
                })
        self.stats["batches_sampled"] += 1
        return batch

    def compute_alignment_score(self, batch: list) -> float:
        if not batch:
            return 0.0
        constitutional_count = sum(1 for item in batch if item.get("type") == "ground_truth")
        return constitutional_count / len(batch)

    def get_stats(self) -> dict:
        total = self.stats["batches_sampled"]
        injections = self.stats["constitutional_injections"]
        alignment_rate = (injections / (total * 32)) if total > 0 else 0.0
        return {
            "buffer_size": len(self.replay_buffer),
            "total_added": self.stats["total_added"],
            "batches_sampled": self.stats["batches_sampled"],
            "constitutional_injections": self.stats["constitutional_injections"],
            "alignment_rate": alignment_rate,
        }

    def export_replay_log(self, n: int = 20) -> list:
        buffer_list = list(self.replay_buffer)
        return buffer_list[-n:]

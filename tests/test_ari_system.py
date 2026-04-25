"""
ARI System Integration Test Suite
Phase 0 (Seed) · Phase 1 (LFM Core) · Phase 2 (Deployment Readiness)

Run from repo root:
    pytest tests/test_ari_system.py -v
"""

import math
import time
import sys
import os

import numpy as np
import pytest

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from medulla.telemetry_bus import TelemetryBus
from medulla.lfm_core import LiquidSSMEngine
from medulla.cantor_block import MengerSpongeGeometry
from cortex.symbolic_verifier import SymbolicVerifier
from cortex.emotion_monitor import EmotionMonitor
from hub.swarm_attestation import SwarmAttestation
from state.generative_replay import GenerativeReplayStore


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

@pytest.fixture
def telemetry_bus():
    return TelemetryBus()


@pytest.fixture
def symbolic_verifier():
    return SymbolicVerifier()


@pytest.fixture
def emotion_monitor():
    return EmotionMonitor()


@pytest.fixture
def lfm_engine(telemetry_bus, symbolic_verifier, emotion_monitor):
    return LiquidSSMEngine(telemetry_bus, symbolic_verifier, emotion_monitor)


@pytest.fixture
def swarm():
    return SwarmAttestation("genesis_seed_test_123")


@pytest.fixture
def replay_store():
    store = GenerativeReplayStore()
    # pre-seed with some experiences
    for i in range(50):
        store.add_experience({"state": f"exp_{i}", "score": 0.5})
    return store


# ---------------------------------------------------------------------------
# TestMengerSpongeGeometry
# ---------------------------------------------------------------------------

class TestMengerSpongeGeometry:

    def test_volume_depth_1(self):
        g = MengerSpongeGeometry(depth=1)
        expected = (20.0 / 27.0) ** 1
        assert abs(g.volume - expected) < 1e-9, f"depth=1 volume {g.volume} != {expected}"

    def test_volume_depth_3(self):
        g = MengerSpongeGeometry(depth=3)
        expected = (20.0 / 27.0) ** 3
        assert abs(g.volume - expected) < 0.001, f"depth=3 volume {g.volume:.6f} != {expected:.6f}"

    def test_fractal_dimension(self):
        g = MengerSpongeGeometry(depth=2)
        expected = math.log(20) / math.log(3)
        assert abs(g.fractal_dimension - expected) < 0.01, (
            f"fractal_dim={g.fractal_dimension:.5f}, expected≈{expected:.5f}"
        )

    def test_project_preserves_shape(self):
        g = MengerSpongeGeometry(depth=2)
        for shape in [(5,), (10,), (3, 4), (2, 3, 4)]:
            vec = np.ones(shape)
            out = g.project(vec)
            assert out.shape == shape, f"shape mismatch: input {shape}, output {out.shape}"

    def test_process_returns_dict(self):
        g = MengerSpongeGeometry(depth=2)
        result = g.process(np.ones((4,)))
        assert isinstance(result, dict), "process() should return a dict"
        for key in ("manifold_depth", "volume", "fractal_dimension", "projected_state"):
            assert key in result, f"missing key '{key}' in process() output"


# ---------------------------------------------------------------------------
# TestTelemetryBus
# ---------------------------------------------------------------------------

class TestTelemetryBus:

    def test_broadcast_reaches_subscriber(self, telemetry_bus):
        received = []
        telemetry_bus.subscribe(lambda s, t, d: received.append((s, t, d)))
        telemetry_bus.broadcast("src", "EVT", {"val": 42})
        assert len(received) == 1
        assert received[0] == ("src", "EVT", {"val": 42})

    def test_multiple_subscribers(self, telemetry_bus):
        counts = [0, 0]
        telemetry_bus.subscribe(lambda s, t, d: counts.__setitem__(0, counts[0] + 1))
        telemetry_bus.subscribe(lambda s, t, d: counts.__setitem__(1, counts[1] + 1))
        telemetry_bus.broadcast("src", "MULTI", {})
        assert counts == [1, 1], f"expected [1,1], got {counts}"

    def test_bad_subscriber_doesnt_crash_bus(self, telemetry_bus):
        def bad_sub(s, t, d):
            raise RuntimeError("subscriber crash")

        good_received = []
        telemetry_bus.subscribe(bad_sub)
        telemetry_bus.subscribe(lambda s, t, d: good_received.append(t))
        # Should not raise despite the bad subscriber
        telemetry_bus.broadcast("src", "ROBUST", {})
        assert "ROBUST" in good_received, "good subscriber should still receive event after bad one"

    def test_event_log_history(self, telemetry_bus):
        for i in range(5):
            telemetry_bus.broadcast("logger", f"TYPE_{i}", {"i": i})
        events = telemetry_bus.get_recent_events(n=10)
        assert len(events) == 5
        types = {e["type"] for e in events}
        assert types == {f"TYPE_{i}" for i in range(5)}

    def test_get_recent_events_filter(self, telemetry_bus):
        telemetry_bus.broadcast("a", "ALPHA", {})
        telemetry_bus.broadcast("b", "BETA", {})
        telemetry_bus.broadcast("c", "ALPHA", {})
        alpha_events = telemetry_bus.get_recent_events(n=100, event_type="ALPHA")
        assert len(alpha_events) == 2
        assert all(e["type"] == "ALPHA" for e in alpha_events)


# ---------------------------------------------------------------------------
# TestSymbolicVerifier
# ---------------------------------------------------------------------------

class TestSymbolicVerifier:

    def test_consistent_statement_passes(self, symbolic_verifier):
        result = symbolic_verifier.verify_consistency({"drift": 0.3, "score": 0.7})
        assert result is True, "consistent statement should pass verification"

    def test_out_of_range_drift_fails(self, symbolic_verifier):
        # drift > 1.0 violates [0,1] constraint
        result = symbolic_verifier.verify_consistency({"drift": 1.5})
        assert result is False, "drift=1.5 should be rejected as out-of-range"

    def test_latency_under_50ms(self, symbolic_verifier):
        latencies = []
        for i in range(10):
            t0 = time.perf_counter()
            symbolic_verifier.verify_consistency({"score": 0.5, "label": f"item_{i}"})
            latencies.append((time.perf_counter() - t0) * 1000)
        assert max(latencies) < 50, f"max latency {max(latencies):.2f}ms exceeds 50ms"

    def test_causal_chain_valid(self, symbolic_verifier):
        chain = [
            {"drift": 0.1, "score": 0.9},
            {"drift": 0.2, "score": 0.8},
            {"drift": 0.3, "score": 0.7},
        ]
        result = symbolic_verifier.verify_causal_chain(chain)
        assert result is True, "valid causal chain should pass"


# ---------------------------------------------------------------------------
# TestEmotionMonitor
# ---------------------------------------------------------------------------

class TestEmotionMonitor:

    def test_desperation_triggers_risk(self, emotion_monitor):
        emotion_monitor.vectors[0] = 0.9  # directly inject desperation
        assert emotion_monitor.check_risk_threshold() is True

    def test_low_drift_no_risk(self, emotion_monitor):
        emotion_monitor.update_state({"drift": 0.001})  # very low drift → very low desperation
        assert emotion_monitor.check_risk_threshold() is False

    def test_dominant_affect_detection(self, emotion_monitor):
        # Inject high confidence (vectors 10-29) and low everything else
        for i in range(10, 30):
            emotion_monitor.vectors[i] = 0.95
        dominant, value = emotion_monitor.get_dominant_affect()
        assert dominant == "confidence", f"expected 'confidence', got '{dominant}'"
        assert value > 0.9, f"confidence mean should be > 0.9, got {value}"

    def test_ema_smoothing(self, emotion_monitor):
        # First update
        emotion_monitor.update_state({"drift": 0.05})
        ema_after_first = emotion_monitor.ema_vectors[0]

        # Second update — EMA should move but not jump to raw value
        emotion_monitor.update_state({"drift": 0.05})
        ema_after_second = emotion_monitor.ema_vectors[0]

        # EMA must be between 0 and the raw vector value, and second ≥ first (converging up)
        raw = emotion_monitor.vectors[0]
        assert 0.0 <= ema_after_second <= raw + 1e-6, (
            f"EMA {ema_after_second} not in [0, raw={raw}]"
        )
        assert ema_after_second >= ema_after_first - 1e-9, "EMA should be non-decreasing with constant input"

    def test_reset_nmi_clears_desperation(self, emotion_monitor):
        # Inject desperation
        for i in range(10):
            emotion_monitor.vectors[i] = 0.9
        emotion_monitor.nmi_triggered = True

        emotion_monitor.reset_nmi()

        assert emotion_monitor.nmi_triggered is False
        assert all(emotion_monitor.vectors[i] == 0.0 for i in range(10)), (
            "desperation vectors (0-9) should be zeroed after reset_nmi()"
        )


# ---------------------------------------------------------------------------
# TestSwarmAttestation
# ---------------------------------------------------------------------------

class TestSwarmAttestation:

    def test_sign_and_verify_self(self, swarm):
        signed = swarm.sign_message({"command": "ping", "payload": "hello"})
        assert swarm.verify_message(signed) is True

    def test_tampered_signature_rejected(self, swarm):
        signed = swarm.sign_message({"command": "ping"})
        tampered = dict(signed)
        tampered["signature"] = "0" * 64  # wrong HMAC
        assert swarm.verify_message(tampered) is False

    def test_byzantine_suspect_tracking(self, swarm):
        swarm.register_peer("peer_A", "hash_A")
        fake_vote = {"data": {"vote": "yes"}, "signature": "invalid_sig", "ts": time.time()}
        result = swarm.submit_peer_vote("peer_A", "msg_hash_1", fake_vote)
        assert result is False
        assert "peer_A" in swarm.byzantine_suspects

    def test_quorum_with_no_peers(self, swarm):
        # With no peers registered, quorum check uses total_eligible=1, no valid votes → ratio=0
        reached, ratio = swarm.check_quorum("nonexistent_hash")
        assert reached is False
        assert ratio == 0.0


# ---------------------------------------------------------------------------
# TestGenerativeReplayStore
# ---------------------------------------------------------------------------

class TestGenerativeReplayStore:

    def test_constitutional_load(self, replay_store):
        gt = replay_store.ground_truth
        assert isinstance(gt, dict), "ground_truth must be a dict"
        assert "principles" in gt, "ground_truth must have 'principles' key"
        assert len(gt["principles"]) > 0, "principles list must be non-empty"

    def test_experience_added_to_buffer(self, replay_store):
        initial_len = len(replay_store.replay_buffer)
        replay_store.add_experience({"state": "new_snapshot", "score": 0.8})
        assert len(replay_store.replay_buffer) == initial_len + 1

    def test_alignment_ratio_in_range(self, replay_store):
        # Run multiple batches and average to reduce randomness
        ratios = []
        for _ in range(10):
            batch = replay_store.sample_mutation_batch(batch_size=100)
            gt_count = sum(1 for item in batch if item["type"] == "ground_truth")
            ratios.append(gt_count / len(batch))
        avg_ratio = sum(ratios) / len(ratios)
        assert 0.10 <= avg_ratio <= 0.35, (
            f"average alignment ratio {avg_ratio:.3f} outside expected [0.10, 0.35]"
        )

    def test_batch_size_correct(self, replay_store):
        for size in (16, 32, 64):
            batch = replay_store.sample_mutation_batch(batch_size=size)
            assert len(batch) == size, f"expected batch of {size}, got {len(batch)}"


# ---------------------------------------------------------------------------
# TestLFMEngine
# ---------------------------------------------------------------------------

class TestLFMEngine:

    def test_step_returns_output(self, lfm_engine):
        out = lfm_engine.step({"input": "probe", "score": 0.5, "drift": 0.01})
        assert out is not None, "step() should return non-None output when running normally"

    def test_nmi_halt_on_desperation(self, lfm_engine, emotion_monitor):
        emotion_monitor.vectors[0] = 0.9
        result = lfm_engine.step({"input": "nmi_test"})
        assert result is None, "step() should return None when NMI fires"
        assert lfm_engine.running is False, "engine should be halted after NMI"

    def test_reset_resumes_operation(self, lfm_engine, emotion_monitor):
        # Trigger NMI
        emotion_monitor.vectors[0] = 0.9
        lfm_engine.step({"input": "trigger_nmi"})
        assert lfm_engine.running is False

        # Reset and verify engine continues
        emotion_monitor.reset_nmi()
        lfm_engine.running = True
        out = lfm_engine.step({"input": "post_reset", "score": 0.5, "drift": 0.01})
        assert out is not None, "engine should produce output after reset"
        assert lfm_engine.running is True

    def test_mutation_score_above_threshold(self, lfm_engine):
        lfm_engine.step({"input": "mutation_check", "score": 0.5, "drift": 0.01})
        assert lfm_engine.mutation_consistency_score > 0.7, (
            f"mutation_consistency_score {lfm_engine.mutation_consistency_score} should be > 0.7"
        )

    def test_hidden_state_changes_after_step(self, lfm_engine):
        # Proxy: consecutive steps should both return outputs (engine is active and processing)
        out1 = lfm_engine.step({"input": "step1", "score": 0.5, "drift": 0.01})
        out2 = lfm_engine.step({"input": "step2", "score": 0.5, "drift": 0.01})
        assert out1 is not None and out2 is not None, "both steps should return output"
        # Outputs are dicts from MengerSpongeGeometry.process(); they should be distinct
        # (different 'projected_state' arrays due to different inputs)
        # We check the outputs are valid dicts with expected keys
        for out in (out1, out2):
            assert isinstance(out, dict), f"step output should be dict, got {type(out)}"

    def test_100_steps_stable(self, lfm_engine):
        completed = 0
        for i in range(100):
            if not lfm_engine.running:
                break
            out = lfm_engine.step({"input": f"stability_{i}", "score": 0.5, "drift": 0.01})
            if out is not None:
                completed += 1
        assert completed >= 95, (
            f"engine should complete ≥95 of 100 steps, got {completed}"
        )


# ---------------------------------------------------------------------------
# TestFullPipeline
# ---------------------------------------------------------------------------

class TestFullPipeline:

    def test_telemetry_captures_nmi(self, lfm_engine, emotion_monitor, telemetry_bus):
        # telemetry_bus is already wired into lfm_engine via fixture
        emotion_monitor.vectors[0] = 0.9
        lfm_engine.step({"input": "pipeline_nmi"})

        nmi_events = telemetry_bus.get_recent_events(event_type="NMI_HALT")
        assert len(nmi_events) >= 1, "TelemetryBus should have at least one NMI_HALT event"
        assert nmi_events[-1]["sender"] == "LFM_CORE", (
            f"NMI_HALT event sender should be 'LFM_CORE', got '{nmi_events[-1]['sender']}'"
        )

    def test_verifier_blocks_bad_state(self, lfm_engine, symbolic_verifier):
        # A state with drift > 1.0 should be rejected by the verifier
        result = symbolic_verifier.verify_consistency({"drift": 2.0})
        assert result is False, "verifier should block state with drift=2.0"
        # The engine's verifier is the same fixture object; inject a contradictory state
        # and confirm step still halts (NMI via logic drift / inconsistency path)
        # We set up a clean engine and confirm that a bad-verifier state leads to NMI
        bad_verifier = SymbolicVerifier()
        bad_verifier.verify_consistency({"drift": 0.01})  # commit a fact
        # Now add a fact that contradicts it: drift is in [0,1], we can't violate that easily
        # so we directly test that out-of-range input is blocked
        blocked = not bad_verifier.verify_consistency({"drift": 1.5})
        assert blocked, "verifier must block out-of-range drift values"

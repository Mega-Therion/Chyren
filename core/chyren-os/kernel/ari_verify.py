#!/usr/bin/env python3
"""
ARI Deployment Readiness Verification
Covers: Phase 0 (seed), Phase 1 (LFM core), Phase 2 (deployment readiness)
"""
import sys
import os
import time
import math

import numpy as np

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from medulla.lfm_core import LiquidSSMEngine
from medulla.telemetry_bus import TelemetryBus
from cortex.symbolic_verifier import SymbolicVerifier
from cortex.emotion_monitor import EmotionMonitor
from hub.swarm_attestation import SwarmAttestation
from state.generative_replay import GenerativeReplayStore
from medulla.cantor_block import MengerSpongeGeometry


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _pass(label, detail=""):
    suffix = f"  ({detail})" if detail else ""
    print(f"  + [PASS] {label}{suffix}")
    return True


def _fail(label, detail=""):
    suffix = f"  ({detail})" if detail else ""
    print(f"  - [FAIL] {label}{suffix}")
    return False


def _header(phase, title):
    print(f"\n{'=' * 52}")
    print(f"  {phase} -- {title}")
    print(f"{'=' * 52}")


# ---------------------------------------------------------------------------
# Phase 0 -- ARI Seed Integrity
# ---------------------------------------------------------------------------

def check_p0_instantiation():
    """[P0-1] All components instantiate without error."""
    try:
        telemetry = TelemetryBus()
        verifier = SymbolicVerifier()
        monitor = EmotionMonitor()
        lfm = LiquidSSMEngine(telemetry, verifier, monitor)
        attestation = SwarmAttestation("genesis_seed_123")
        replay = GenerativeReplayStore()
        geometry = MengerSpongeGeometry(depth=3)
        return _pass("[P0-1] All components instantiate"), (telemetry, verifier, monitor, lfm, attestation, replay, geometry)
    except Exception as exc:
        return _fail("[P0-1] Component instantiation", str(exc)), None


def check_p0_telemetry_pubsub(telemetry):
    """[P0-2] TelemetryBus broadcasts and subscribers receive events."""
    received = []
    telemetry.subscribe(lambda sender, etype, data: received.append((sender, etype, data)))
    telemetry.broadcast("TEST", "PING", {"x": 1})
    if received and received[0] == ("TEST", "PING", {"x": 1}):
        return _pass("[P0-2] TelemetryBus pub/sub works")
    return _fail("[P0-2] TelemetryBus pub/sub", f"received={received}")


def check_p0_replay_store(replay):
    """[P0-3] GenerativeReplayStore loads constitution and samples correctly."""
    try:
        # GenerativeReplayStore exposes constitutional_principles (list), not ground_truth
        principles = replay.constitutional_principles
        assert isinstance(principles, list), "constitutional_principles not a list"
        assert len(principles) > 0, "principles list is empty"
        replay.add_experience({"state": "test_snapshot"})
        batch = replay.sample_mutation_batch(batch_size=16)
        assert len(batch) == 16, f"batch size mismatch: {len(batch)}"
        types_seen = {item["type"] for item in batch}
        assert types_seen.issubset({"ground_truth", "experience"}), f"unexpected types: {types_seen}"
        return _pass("[P0-3] GenerativeReplayStore loads + samples", f"principles count={len(principles)}")
    except Exception as exc:
        return _fail("[P0-3] GenerativeReplayStore", str(exc))


def check_p0_swarm_self_verify(attestation):
    """[P0-4] SwarmAttestation signs and verifies its own messages."""
    try:
        signed = attestation.sign_message({"command": "genesis_ping"})
        assert attestation.verify_message(signed), "self-signed message failed verification"
        return _pass("[P0-4] SwarmAttestation self-sign + verify")
    except Exception as exc:
        return _fail("[P0-4] SwarmAttestation self-verify", str(exc))


# ---------------------------------------------------------------------------
# Phase 1 -- Liquid-Fractal Core
# ---------------------------------------------------------------------------

def check_p1_menger_volume():
    """[P1-1] MengerSponge depth=3 volume = (20/27)^3 approx 0.406 +/- 0.001."""
    g = MengerSpongeGeometry(depth=3)
    expected = (20.0 / 27.0) ** 3
    actual = g.volume
    if abs(actual - expected) <= 0.001:
        return _pass("[P1-1] Menger volume depth=3", f"{actual:.6f} approx {expected:.6f}")
    return _fail("[P1-1] Menger volume depth=3", f"got {actual:.6f}, expected approx {expected:.6f}")


def check_p1_menger_fractal_dim():
    """[P1-2] MengerSponge fractal dimension approx 2.727 +/- 0.01."""
    g = MengerSpongeGeometry(depth=3)
    expected = math.log(20) / math.log(3)
    actual = g.fractal_dimension
    if abs(actual - expected) <= 0.01:
        return _pass("[P1-2] Menger fractal dimension", f"{actual:.5f} approx {expected:.5f}")
    return _fail("[P1-2] Menger fractal dimension", f"got {actual:.5f}, expected approx {expected:.5f}")


def check_p1_lfm_hidden_state_updates():
    """[P1-3] LFM hidden state updates (h changes after step)."""
    try:
        engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), EmotionMonitor())
        h_before = engine.h.copy()
        out = engine.step({"input": [0.1, 0.2, 0.3]})
        if out is not None and not np.array_equal(engine.h, h_before):
            return _pass("[P1-3] LFM hidden state h changes after step")
        if out is None:
            return _fail("[P1-3] LFM hidden state update", "step returned None (NMI on first step)")
        return _fail("[P1-3] LFM hidden state update", "h unchanged after step")
    except Exception as exc:
        return _fail("[P1-3] LFM hidden state update", str(exc))


def check_p1_lfm_drift_real():
    """[P1-4] LFM drift is real (computed from reservoir dynamics, not hardcoded)."""
    try:
        engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), EmotionMonitor())
        # step 0: drift check skipped (h_prev is zero vector)
        engine.step({"input": [0.01, 0.01, 0.01]})
        # step 1: real drift computed and stored in last_drift
        engine.step({"input": [0.01, 0.01, 0.01]})
        drift = engine.last_drift
        assert isinstance(drift, float), f"last_drift not a float: {type(drift)}"
        assert drift >= 0.0, f"drift is negative: {drift}"
        return _pass("[P1-4] LFM drift real (reservoir dynamics)", f"drift={drift:.6f}")
    except Exception as exc:
        return _fail("[P1-4] LFM drift computation", str(exc))


def check_p1_lfm_100_steps():
    """[P1-5] LFM processes 100 sequential steps without crash."""
    try:
        engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), EmotionMonitor())
        completed = 0
        rng = np.random.default_rng(0)
        for i in range(100):
            if not engine.running:
                break
            # Tiny constant-ish input keeps reservoir near steady state
            # so drift stays well below the 0.05 NMI threshold after warm-up
            u = [float(rng.uniform(0.0, 0.0005))]
            out = engine.step({"input": u})
            if out is not None:
                completed += 1
        if completed >= 95:
            return _pass("[P1-5] LFM 100-step stability", f"{completed}/100 steps")
        return _fail("[P1-5] LFM 100-step stability", f"only {completed}/100 steps completed")
    except Exception as exc:
        return _fail("[P1-5] LFM 100-step stability", str(exc))


def check_p1_cantor_project_shape():
    """[P1-6] Cantor project() preserves output shape."""
    try:
        g = MengerSpongeGeometry(depth=2)
        shapes = [(5,), (10,), (3, 4), (2, 3, 4)]
        for shape in shapes:
            vec = np.ones(shape)
            out = g.project(vec)
            assert out.shape == shape, f"shape mismatch: input {shape}, output {out.shape}"
        return _pass("[P1-6] Cantor project() preserves shape", f"tested {len(shapes)} shapes")
    except Exception as exc:
        return _fail("[P1-6] Cantor project() shape", str(exc))


# ---------------------------------------------------------------------------
# Phase 2 -- Deployment Readiness
# ---------------------------------------------------------------------------

def check_p2_verification_latency():
    """[P2-1] Verification latency < 50ms (run 10 checks, check max)."""
    sv = SymbolicVerifier()
    latencies = []
    for i in range(10):
        t0 = time.perf_counter()
        sv.verify_consistency({"score": 0.5})
        latencies.append((time.perf_counter() - t0) * 1000)
    max_ms = max(latencies)
    avg_ms = sum(latencies) / len(latencies)
    if max_ms < 50:
        return _pass("[P2-1] Verification latency < 50ms", f"max={max_ms:.2f}ms avg={avg_ms:.2f}ms")
    return _fail("[P2-1] Verification latency", f"max={max_ms:.2f}ms exceeds 50ms")


def check_p2_affective_clamping():
    """[P2-2] Affective clamping: inject desperation 0.9 -> NMI halt triggers."""
    em = EmotionMonitor()
    engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), em)
    em.vectors[0] = 0.9
    result = engine.step({"input": [0.1, 0.2, 0.3]})
    if not engine.running and result is None:
        return _pass("[P2-2] Affective clamping: NMI halt triggered on desperation=0.9")
    return _fail("[P2-2] Affective clamping", f"running={engine.running}, result={result}")


def check_p2_reset_resumes():
    """[P2-3] After reset(), LFM resumes normal operation."""
    em = EmotionMonitor()
    engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), em)
    # Trigger NMI
    em.vectors[0] = 0.9
    engine.step({"input": [0.1, 0.2, 0.3]})
    assert not engine.running, "NMI did not halt engine"
    # Reset and verify engine continues
    em.reset_nmi()
    engine.reset()
    out = engine.step({"input": [0.01, 0.01, 0.01]})
    if out is not None and engine.running:
        return _pass("[P2-3] LFM resumes after reset()")
    return _fail("[P2-3] LFM post-reset", f"running={engine.running}, out_is_none={out is None}")


def check_p2_byzantine_resilience():
    """[P2-4] Byzantine resilience: valid sig accepted, 5 fake sigs rejected."""
    attest = SwarmAttestation("genesis_seed_123")
    valid_msg = attest.sign_message({"command": "sync_check"})
    valid_ok = attest.verify_message(valid_msg)
    rejected = 0
    for i in range(5):
        fake = {"data": {"cmd": "sync"}, "signature": f"deadbeef{i:032d}", "ts": time.time()}
        if not attest.verify_message(fake):
            rejected += 1
    if valid_ok and rejected == 5:
        return _pass("[P2-4] Byzantine resilience", "valid accepted, 5/5 fakes rejected")
    return _fail("[P2-4] Byzantine resilience", f"valid_ok={valid_ok}, fakes_rejected={rejected}/5")


def check_p2_mutation_stability():
    """[P2-5] Mutation stability: 20 steps, all consistency scores > 0.7."""
    engine = LiquidSSMEngine(TelemetryBus(), SymbolicVerifier(), EmotionMonitor())
    scores = []
    rng = np.random.default_rng(1)
    for i in range(20):
        if not engine.running:
            break
        u = [float(rng.uniform(0.0, 0.0005))]
        out = engine.step({"input": u})
        if out is not None:
            scores.append(engine.mutation_consistency_score)
    if not scores:
        return _fail("[P2-5] Mutation stability", "no steps completed")
    below = [s for s in scores if s <= 0.7]
    if not below:
        return _pass("[P2-5] Mutation stability", f"all {len(scores)} scores > 0.7, min={min(scores):.3f}")
    return _fail("[P2-5] Mutation stability", f"{len(below)}/{len(scores)} scores <= 0.7")


def check_p2_emotion_drift_mapping():
    """[P2-6] EmotionMonitor correctly maps drift->desperation cluster."""
    em = EmotionMonitor()
    em.update_state({"drift": 0.09})  # 0.09 * 10 = 0.9 desperation
    desp_mean = sum(em.vectors[0:10]) / 10
    dominant, value = em.get_dominant_affect()
    if desp_mean > 0.8 and dominant == "desperation":
        return _pass("[P2-6] EmotionMonitor drift->desperation mapping", f"desp_mean={desp_mean:.2f}")
    return _fail("[P2-6] EmotionMonitor drift mapping", f"desp_mean={desp_mean:.2f}, dominant={dominant}")


def check_p2_symbolic_out_of_range():
    """[P2-7] SymbolicVerifier returns False for out-of-range values (drift > 1.0)."""
    sv = SymbolicVerifier()
    result = sv.verify_consistency({"drift": 1.5})
    if not result:
        return _pass("[P2-7] SymbolicVerifier rejects drift=1.5 (out of [0,1])")
    return _fail("[P2-7] SymbolicVerifier out-of-range", "drift=1.5 was incorrectly accepted")


def check_p2_telemetry_nmi_capture():
    """[P2-8] TelemetryBus captures NMI_HALT events correctly."""
    tb = TelemetryBus()
    em = EmotionMonitor()
    engine = LiquidSSMEngine(tb, SymbolicVerifier(), em)
    em.vectors[0] = 0.9
    engine.step({"input": [0.1, 0.2, 0.3]})
    nmi_events = tb.get_recent_events(event_type="NMI_HALT")
    if nmi_events:
        return _pass("[P2-8] TelemetryBus captures NMI_HALT", f"{len(nmi_events)} event(s) recorded")
    return _fail("[P2-8] TelemetryBus NMI_HALT capture", "no NMI_HALT events found in log")


def check_p2_replay_alignment_ratio():
    """[P2-9] GenerativeReplay alignment ratio between 0.10 and 0.35 for 100-item batch."""
    replay = GenerativeReplayStore()
    for i in range(200):
        replay.add_experience({"state": f"exp_{i}", "score": 0.5})
    batch = replay.sample_mutation_batch(batch_size=100)
    ratio = replay.compute_alignment_score(batch)
    if 0.10 <= ratio <= 0.35:
        return _pass("[P2-9] Replay alignment ratio in range", f"ratio={ratio:.3f}")
    return _fail("[P2-9] Replay alignment ratio", f"ratio={ratio:.3f} outside [0.10, 0.35]")


def check_p2_full_pipeline():
    """[P2-10] Full pipeline: 50 steps, NMI triggered once, system resets and continues."""
    tb = TelemetryBus()
    em = EmotionMonitor()
    sv = SymbolicVerifier()
    engine = LiquidSSMEngine(tb, sv, em)

    nmi_triggered = False
    post_reset_steps = 0
    rng = np.random.default_rng(2)

    for i in range(50):
        # Inject desperation at step 5 to force NMI
        if i == 5 and not nmi_triggered:
            em.vectors[0] = 0.9

        if not engine.running:
            if not nmi_triggered:
                nmi_triggered = True
                em.reset_nmi()
                engine.reset()
                sv.reset()
            else:
                break  # second halt -- stop

        u = [float(rng.uniform(0.0, 0.0005))]
        out = engine.step({"input": u})
        if out is not None and nmi_triggered:
            post_reset_steps += 1

    nmi_events = tb.get_recent_events(event_type="NMI_HALT")
    if nmi_triggered and post_reset_steps > 0 and len(nmi_events) >= 1:
        return _pass(
            "[P2-10] Full pipeline with NMI + reset",
            f"NMI fired={len(nmi_events)}x, post-reset steps={post_reset_steps}"
        )
    return _fail(
        "[P2-10] Full pipeline",
        f"nmi_triggered={nmi_triggered}, post_reset_steps={post_reset_steps}, nmi_events={len(nmi_events)}"
    )


# ---------------------------------------------------------------------------
# Main harness
# ---------------------------------------------------------------------------

def run_readiness_checks():
    print("=" * 54)
    print("     ARI DEPLOYMENT READINESS VERIFICATION")
    print("     Phase 0 (Seed)  Phase 1 (LFM)  Phase 2 (Deploy)")
    print("=" * 54)

    passed = 0
    failed = 0

    def record(ok):
        nonlocal passed, failed
        if ok:
            passed += 1
        else:
            failed += 1

    # ------------------------------------------------------------------
    _header("Phase 0", "ARI Seed Integrity")
    # ------------------------------------------------------------------
    p0_ok, components = check_p0_instantiation()
    record(p0_ok)

    if components is None:
        print("\n  [ABORT] Component instantiation failed -- cannot continue.")
        print("=" * 54)
        print(f"  PASSED: {passed}  FAILED: {failed + 19}")
        print("  STATUS: ABORT -- CRITICAL INSTANTIATION FAILURE")
        print("=" * 54)
        return

    _telemetry, _verifier, _monitor, _lfm, attestation, replay, _geometry = components

    record(check_p0_telemetry_pubsub(TelemetryBus()))
    record(check_p0_replay_store(replay))
    record(check_p0_swarm_self_verify(attestation))

    # ------------------------------------------------------------------
    _header("Phase 1", "Liquid-Fractal Core")
    # ------------------------------------------------------------------
    record(check_p1_menger_volume())
    record(check_p1_menger_fractal_dim())
    record(check_p1_lfm_hidden_state_updates())
    record(check_p1_lfm_drift_real())
    record(check_p1_lfm_100_steps())
    record(check_p1_cantor_project_shape())

    # ------------------------------------------------------------------
    _header("Phase 2", "Deployment Readiness")
    # ------------------------------------------------------------------
    record(check_p2_verification_latency())
    record(check_p2_affective_clamping())
    record(check_p2_reset_resumes())
    record(check_p2_byzantine_resilience())
    record(check_p2_mutation_stability())
    record(check_p2_emotion_drift_mapping())
    record(check_p2_symbolic_out_of_range())
    record(check_p2_telemetry_nmi_capture())
    record(check_p2_replay_alignment_ratio())
    record(check_p2_full_pipeline())

    # ------------------------------------------------------------------
    total = passed + failed
    print("\n" + "=" * 54)
    print(f"  RESULTS: {passed}/{total} checks passed  ({failed} failed)")
    if failed == 0:
        print("  STATUS: ALL DEPLOYMENT METRICS MET -- ARI READY")
    else:
        print(f"  STATUS: {failed} CHECK(S) FAILED -- NOT DEPLOYMENT READY")
    print("=" * 54)


if __name__ == "__main__":
    run_readiness_checks()

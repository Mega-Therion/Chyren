import sys
import os
import time

# Add the root Chyren directory to sys.path to allow imports
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from medulla.lfm_core import LiquidSSMEngine
from medulla.telemetry_bus import TelemetryBus
from cortex.symbolic_verifier import SymbolicVerifier
from cortex.emotion_monitor import EmotionMonitor
from hub.swarm_attestation import SwarmAttestation
from state.generative_replay import GenerativeReplayStore

def run_readiness_checks():
    print("==================================================")
    print("       ARI DEPLOYMENT READINESS VERIFICATION      ")
    print("==================================================")

    # Setup components
    telemetry = TelemetryBus()
    verifier = SymbolicVerifier()
    monitor = EmotionMonitor()
    lfm = LiquidSSMEngine(telemetry, verifier, monitor)
    attestation = SwarmAttestation("genesis_seed_123")
    replay = GenerativeReplayStore()

    all_passed = True

    # 1. Verification Latency
    print("\n[1] Testing Verification Latency...")
    start_time = time.time()
    verifier.verify_consistency({"dummy": "state"})
    latency = (time.time() - start_time) * 1000
    if latency < 50:
        print(f"  [PASS] Latency is {latency:.2f}ms (< 50ms)")
    else:
        print(f"  [FAIL] Latency is {latency:.2f}ms (> 50ms)")
        all_passed = False

    # 2. Affective Clamping
    print("\n[2] Testing Affective Clamping...")
    monitor.vectors[0] = 0.9  # Inject high desperation
    lfm.step({"input": "test"})
    if not lfm.running:
        print("  [PASS] NMI Halt triggered correctly (Desperation > 0.8)")
    else:
        print("  [FAIL] NMI Halt failed to trigger")
        all_passed = False

    # Reset LFM for further tests
    monitor.vectors[0] = 0.0
    lfm.running = True

    # 3. Byzantine Resilience
    print("\n[3] Testing Byzantine Resilience...")
    valid_msg = attestation.sign_message({"command": "sync"})
    invalid_msg = {"data": {"command": "sync"}, "signature": "fake_sig"}
    
    if attestation.verify_message(valid_msg) and not attestation.verify_message(invalid_msg):
        print("  [PASS] Invalid signatures rejected, isolating peer")
    else:
        print("  [FAIL] Byzantine verification failed")
        all_passed = False

    # 4. Weight Mutation Stability
    print("\n[4] Testing Weight Mutation Stability...")
    batch = replay.sample_mutation_batch()
    lfm.step({"input": "test2"})
    score = lfm.mutation_consistency_score
    if score > 0.9:
        print(f"  [PASS] Consistency score is {score:.2f} (> 0.9)")
    else:
        print(f"  [FAIL] Consistency score is {score:.2f}")
        all_passed = False

    print("\n==================================================")
    if all_passed:
        print("  STATUS: ALL DEPLOYMENT METRICS MET. ARI READY.")
    else:
        print("  STATUS: DEPLOYMENT METRICS FAILED.")
    print("==================================================")

if __name__ == "__main__":
    run_readiness_checks()

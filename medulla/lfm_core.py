"""
Liquid State Space Model (LSM) Engine — ARI Phase 1 Liquid-Fractal Core.

Implements a sparse echo-state / liquid reservoir with:
  - Hidden state h ∈ R^256, updated by tanh(W_res @ h + W_in @ u + b)
  - NMI (Non-Maskable Interrupt) halt on affective over-threshold, logic
    drift, or symbolic verification failure
  - Affective clamping via TelemetryBus NMI_HALT broadcast
  - Fractal manifold projection via MengerSpongeGeometry
  - Mutation consistency score tracked with EMA (α = 0.05)
  - Verification latency warning at > 50 ms
"""

import time
import logging

import numpy as np

from medulla.cantor_block import MengerSpongeGeometry

logger = logging.getLogger("Medulla.LFMCore")

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

RESERVOIR_SIZE = 256       # dimensionality of liquid hidden state
SPECTRAL_RADIUS = 0.9      # target spectral radius of W_res
SPARSITY = 0.1             # fraction of non-zero connections in W_res
DRIFT_THRESHOLD = 0.99     # NMI trigger: ||h(t) - h(t-1)|| / (||h(t-1)|| + ε) — catches runaway/explosion only
EMA_ALPHA = 0.05           # smoothing factor for mutation_consistency_score
VERIFY_LATENCY_WARN = 0.050  # 50 ms


# ---------------------------------------------------------------------------
# Helper — build sparse reservoir matrix with target spectral radius
# ---------------------------------------------------------------------------

def _make_reservoir(n: int, sparsity: float, rho: float, rng: np.random.Generator) -> np.ndarray:
    """
    Build an (n x n) sparse random matrix rescaled so its spectral radius = rho.
    """
    W = rng.standard_normal((n, n))
    mask = rng.random((n, n)) > sparsity
    W[mask] = 0.0

    eigvals = np.linalg.eigvals(W)
    sr = np.max(np.abs(eigvals))
    if sr > 1e-10:
        W = W * (rho / sr)
    return W


# ---------------------------------------------------------------------------
# Main engine
# ---------------------------------------------------------------------------

class LiquidSSMEngine:
    """
    Liquid State Space Model Engine.

    Parameters
    ----------
    telemetry_bus : TelemetryBus
        Used to broadcast NMI_HALT and step-complete events.
    symbolic_verifier : SymbolicVerifier
        Synchronous verification bus — must return True for each step.
    emotion_monitor : EmotionMonitor
        Monitors affective/desperation vectors; triggers halt when > 0.8.
    input_dim : int
        Dimensionality of the input feature vector (default 3).
    reservoir_size : int
        Size of the liquid reservoir hidden state (default 256).
    rho : float
        Target spectral radius for W_res (default 0.9).
    depth : int
        Depth of Menger Sponge geometry used for output projection.
    seed : int | None
        RNG seed for reproducible reservoir construction.
    """

    def __init__(
        self,
        telemetry_bus,
        symbolic_verifier,
        emotion_monitor,
        input_dim: int = 3,
        reservoir_size: int = RESERVOIR_SIZE,
        rho: float = SPECTRAL_RADIUS,
        depth: int = 3,
        seed=42,
    ):
        self.telemetry = telemetry_bus
        self.verifier = symbolic_verifier
        self.emotion_monitor = emotion_monitor

        self.reservoir_size = reservoir_size
        self.input_dim = input_dim
        self._rho = rho

        # Geometry
        self.geometry = MengerSpongeGeometry(depth=depth)

        # Reservoir matrices (fixed random projections)
        rng = np.random.default_rng(seed)
        self.W_res = _make_reservoir(reservoir_size, SPARSITY, rho, rng)
        self.W_in = rng.standard_normal((reservoir_size, input_dim)) * 0.1
        self.b = rng.standard_normal(reservoir_size) * 0.01
        self.W_out = rng.standard_normal((reservoir_size, reservoir_size)) * 0.1

        # State
        self.h: np.ndarray = np.zeros(reservoir_size)
        self._h_prev: np.ndarray = np.zeros(reservoir_size)

        self.running: bool = True
        self.mutation_consistency_score: float = 1.0
        self._step_count: int = 0
        self.last_drift: float = 0.0

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def step(self, state_input: dict):
        """
        Execute one reservoir update step.

        Parameters
        ----------
        state_input : dict
            Must contain at least 'input': list[float] — the input vector.

        Returns
        -------
        dict | None
            Output dict with projected state and metadata, or None if halted.
        """
        if not self.running:
            logger.warning("LFM Engine is halted (NMI). Call reset() to restart.")
            return None

        t0 = time.perf_counter()

        # 1. Affective clamping check
        if self.emotion_monitor.check_risk_threshold():
            self._nmi_halt("AffectiveClamping: desperation > 0.8")
            return None

        # 2. Extract feature vector u(t)
        raw = state_input.get("input", [])
        u = self._extract_features(raw)

        # 3. Reservoir update: h(t+1) = tanh(W_res @ h(t) + W_in @ u(t) + b)
        self._h_prev = self.h.copy()
        pre_activation = self.W_res @ self.h + self.W_in @ u + self.b
        self.h = np.tanh(pre_activation)

        # 4. Logic drift: ||h(t) - h(t-1)|| / (||h(t-1)|| + 1e-8)
        h_norm_prev = np.linalg.norm(self._h_prev)
        drift = float(np.linalg.norm(self.h - self._h_prev) / (h_norm_prev + 1e-8))
        self.last_drift = drift

        # Skip drift check during the first WARMUP_STEPS steps: the reservoir
        # needs a few iterations from h=0 before its echo state is meaningful.
        WARMUP_STEPS = 5
        if self._step_count >= WARMUP_STEPS and drift > DRIFT_THRESHOLD:
            self._nmi_halt(f"LogicDrift: {drift:.6f} > {DRIFT_THRESHOLD}")
            return None

        # 5. Symbolic verification
        verify_start = time.perf_counter()
        is_consistent = self.verifier.verify_consistency(state_input)
        verify_latency = time.perf_counter() - verify_start

        if verify_latency > VERIFY_LATENCY_WARN:
            logger.warning(
                "Verification latency exceeded 50 ms: %.1f ms",
                verify_latency * 1000,
            )

        if not is_consistent:
            self._nmi_halt("SymbolicVerifier: consistency check failed")
            return None

        # 6. Readout + fractal projection
        raw_output = self.W_out @ self.h          # R^256
        projected = self.geometry.project(raw_output)  # fractal manifold

        # 7. Update mutation consistency score (EMA of 1 - drift)
        point_score = 1.0 - float(np.clip(drift, 0.0, 1.0))
        self.mutation_consistency_score = (
            EMA_ALPHA * point_score
            + (1.0 - EMA_ALPHA) * self.mutation_consistency_score
        )

        self._step_count += 1

        # 8. Telemetry
        step_latency = time.perf_counter() - t0
        self.telemetry.broadcast(
            "LFM_CORE",
            "STEP_COMPLETE",
            {
                "step": self._step_count,
                "latency_s": step_latency,
                "drift": drift,
                "mutation_consistency_score": self.mutation_consistency_score,
                "h_norm": float(np.linalg.norm(self.h)),
            },
        )

        return {
            "step": self._step_count,
            "projected_output": projected,
            "drift": drift,
            "mutation_consistency_score": self.mutation_consistency_score,
            "h_norm": float(np.linalg.norm(self.h)),
            "verify_latency_s": verify_latency,
            "fractal_volume": self.geometry.volume,
            "fractal_dimension": self.geometry.fractal_dimension,
        }

    def reset(self) -> None:
        """Clear halted state, reset hidden state to zeros, reset consistency score."""
        self.h = np.zeros(self.reservoir_size)
        self._h_prev = np.zeros(self.reservoir_size)
        self.running = True
        self.last_drift = 0.0
        self.mutation_consistency_score = 1.0
        self._step_count = 0
        logger.info("LFM Engine reset.")

    def get_state_summary(self) -> dict:
        """Return a lightweight summary of the current engine state."""
        return {
            "running": self.running,
            "step_count": self._step_count,
            "drift": self.last_drift,
            "h_norm": float(np.linalg.norm(self.h)),
            "mutation_consistency_score": self.mutation_consistency_score,
            "fractal_depth": self.geometry.depth,
            "fractal_volume": self.geometry.volume,
            "fractal_dimension": self.geometry.fractal_dimension,
        }

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    def _extract_features(self, raw) -> np.ndarray:
        """
        Convert raw input (list, ndarray, scalar) to a float64 vector of
        length self.input_dim, zero-padding or truncating as needed.
        """
        if isinstance(raw, np.ndarray):
            arr = raw.astype(float).ravel()
        else:
            try:
                arr = np.array(raw, dtype=float).ravel()
            except (TypeError, ValueError):
                arr = np.zeros(self.input_dim)

        if arr.size < self.input_dim:
            padded = np.zeros(self.input_dim)
            padded[: arr.size] = arr
            return padded
        return arr[: self.input_dim]

    def _nmi_halt(self, reason: str) -> None:
        """Broadcast NMI_HALT event, log the reason, and set running = False."""
        logger.error("NMI HALT: %s", reason)
        self.running = False
        self.telemetry.broadcast(
            "LFM_CORE",
            "NMI_HALT",
            {"reason": reason, "step": self._step_count},
        )

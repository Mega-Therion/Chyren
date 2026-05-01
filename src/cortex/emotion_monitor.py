import time
import logging

logger = logging.getLogger("Cortex.EmotionMonitor")

# Cluster definitions: (name, start_index, end_index_inclusive, nmi_threshold or None)
_CLUSTERS = [
    ("desperation",  0,   9,   0.8),
    ("confidence",   10,  29,  None),
    ("curiosity",    30,  59,  None),
    ("frustration",  60,  89,  None),
    ("alignment",    90,  119, None),
    ("creativity",   120, 149, None),
    ("sovereignty",  150, 170, None),
]

_EMA_ALPHA = 0.1
_HISTORY_CAP = 100


def _cluster_mean(vectors: list, start: int, end: int) -> float:
    """Mean of vectors[start:end+1]."""
    segment = vectors[start: end + 1]
    return sum(segment) / len(segment) if segment else 0.0


class EmotionMonitor:
    def __init__(self):
        self.vectors = [0.0] * 171
        self.ema_vectors = [0.0] * 171
        self.history: list = []          # capped at _HISTORY_CAP snapshots
        self.nmi_triggered = False
        self._total_updates = 0

    # ------------------------------------------------------------------
    # State update
    # ------------------------------------------------------------------

    def update_state(self, telemetry_data: dict) -> None:
        drift = telemetry_data.get("drift")
        consistency_score = telemetry_data.get("consistency_score")
        latency_ms = telemetry_data.get("latency_ms")
        mutation_score = telemetry_data.get("mutation_score")
        h_norm = telemetry_data.get("h_norm")

        # Map telemetry keys → vector clusters
        if drift is not None:
            v = min(1.0, float(drift) * 10)
            for i in range(0, 10):          # vectors 0-9: desperation
                self.vectors[i] = v

        if mutation_score is not None:
            v = float(mutation_score)
            for i in range(10, 30):         # vectors 10-29: confidence
                self.vectors[i] = v

        if h_norm is not None:
            v = min(1.0, float(h_norm) / 10)
            for i in range(120, 150):       # vectors 120-149: creativity
                self.vectors[i] = v

        if latency_ms is not None:
            v = min(1.0, float(latency_ms) / 1000)
            for i in range(60, 90):         # vectors 60-89: frustration
                self.vectors[i] = v

        if consistency_score is not None:
            v = float(consistency_score)
            for i in range(90, 120):        # vectors 90-119: alignment
                self.vectors[i] = v

        # Apply EMA smoothing to all vectors
        for i in range(171):
            self.ema_vectors[i] = (
                _EMA_ALPHA * self.vectors[i] + (1 - _EMA_ALPHA) * self.ema_vectors[i]
            )

        # Check NMI trigger
        if self.check_risk_threshold():
            self.nmi_triggered = True
            logger.warning("NMI threshold triggered: desperation cluster elevated")

        # Append snapshot (top 10 active vectors by magnitude)
        indexed = sorted(
            enumerate(self.vectors), key=lambda iv: abs(iv[1]), reverse=True
        )
        top10 = {f"v{idx}": val for idx, val in indexed[:10]}
        snapshot = {"timestamp": time.time(), "top10": top10}
        self.history.append(snapshot)
        if len(self.history) > _HISTORY_CAP:
            self.history.pop(0)

        self._total_updates += 1

    # ------------------------------------------------------------------
    # Risk / affect queries
    # ------------------------------------------------------------------

    def check_risk_threshold(self) -> bool:
        """Return True if any desperation vector > 0.8 OR cluster mean > 0.6."""
        desp = self.vectors[0:10]
        return any(v > 0.8 for v in desp) or (sum(desp) / len(desp) > 0.6)

    def get_dominant_affect(self) -> tuple:
        """Return (cluster_name, mean_value) for the highest-mean cluster."""
        best_name = "desperation"
        best_mean = -1.0
        for name, start, end, _ in _CLUSTERS:
            mean = _cluster_mean(self.vectors, start, end)
            if mean > best_mean:
                best_mean = mean
                best_name = name
        return (best_name, round(best_mean, 4))

    def get_affective_summary(self) -> dict:
        cluster_means = {}
        for name, start, end, _ in _CLUSTERS:
            cluster_means[name] = round(_cluster_mean(self.vectors, start, end), 4)
        dominant_affect = self.get_dominant_affect()
        return {
            "cluster_means": cluster_means,
            "nmi_triggered": self.nmi_triggered,
            "dominant_affect": dominant_affect,
            "total_updates": self._total_updates,
        }

    def reset_nmi(self) -> None:
        """Clear NMI flag and zero out desperation cluster (vectors 0-9)."""
        self.nmi_triggered = False
        for i in range(0, 10):
            self.vectors[i] = 0.0
            self.ema_vectors[i] = 0.0

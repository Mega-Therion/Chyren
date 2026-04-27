//! # ABC — Articulated Binary Chirallic
//!
//! The ABC module is the translation layer between Chyren's ternary
//! reasoning space (Cortex) and the binary execution contract (AEGIS/Medulla).
//!
//! ## Leges Primae — Lex Prima
//! A signal is only articulable if and only if its chirallic pair resolves
//! without contradiction. Contradiction is defined as both S and C of the
//! same set producing identical sign outputs. If contradiction is detected,
//! the signal is held in suspension (TernaryState::Neutral) and escalated
//! to the ADCCL.
//!
//! ## Architecture Position
//! ```text
//! [Cortex: Python]
//!     → ternary confidence signal (f32 in [-1.0, 1.0])
//!         → [ABCResolver]
//!             → ChiralSet S1/C1  (a² template)
//!             → ChiralSet S2/C2  (b² template)
//!                 → ArticulationResult (Execute | Abort | Suspended)
//!                     → [AEGIS Gate]
//!                         → Medulla execution
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TernaryState
// ---------------------------------------------------------------------------

/// The three fundamental states of the ABC reasoning space.
///
/// - `Positive`  (+1): forward / execute tendency
/// - `Neutral`   ( 0): unresolved / suspension / noise
/// - `Negative`  (-1): inverse / abort tendency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TernaryState {
    Positive,
    Neutral,
    Negative,
}

impl TernaryState {
    /// Convert a real-valued cortex confidence score into a ternary state.
    /// The dead-band threshold (±0.15) absorbs noise and prevents false
    /// articulation on weak signals.
    pub fn from_f32(val: f32) -> Self {
        if val > 0.15 {
            TernaryState::Positive
        } else if val < -0.15 {
            TernaryState::Negative
        } else {
            TernaryState::Neutral
        }
    }

    /// Chirallic inversion: Positive ↔ Negative, Neutral stays Neutral.
    /// This is the core mathematical operation of the ABC system —
    /// mirroring a state across the zero axis.
    pub fn invert(&self) -> Self {
        match self {
            TernaryState::Positive => TernaryState::Negative,
            TernaryState::Negative => TernaryState::Positive,
            TernaryState::Neutral  => TernaryState::Neutral,
        }
    }
}

impl std::fmt::Display for TernaryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TernaryState::Positive => write!(f, "+1"),
            TernaryState::Neutral  => write!(f, " 0"),
            TernaryState::Negative => write!(f, "-1"),
        }
    }
}

// ---------------------------------------------------------------------------
// ChiralSet
// ---------------------------------------------------------------------------

/// A chirallic pair: a base state S and its derived inverse C.
///
/// Per Lex Prima, a set is coherent only when S ≠ C (they are genuinely
/// mirrored). A Neutral base produces S=0 and C=0, which is a contradiction
/// and triggers suspension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChiralSet {
    /// Human-readable label, e.g. "S1/C1" or "S2/C2"
    pub label: String,
    /// Base state (S)
    pub s: TernaryState,
    /// Chirallic inverse (C), always derived from s.invert()
    pub c: TernaryState,
}

impl ChiralSet {
    /// Construct a ChiralSet from a base state.
    /// C is always derived — never set manually.
    pub fn new(label: &str, base: TernaryState) -> Self {
        ChiralSet {
            label: label.to_string(),
            s: base,
            c: base.invert(),
        }
    }

    /// Coherence check per Lex Prima.
    /// A set is coherent only if S and C are genuinely mirrored (S ≠ C).
    /// Neutral base always fails coherence because invert(Neutral) = Neutral.
    pub fn is_coherent(&self) -> bool {
        self.s != self.c
    }

    /// Articulate the pair into a binary decision.
    ///
    /// - Positive base → `Some(true)`  (Execute)
    /// - Negative base → `Some(false)` (Abort)
    /// - Neutral base  → `None`        (suspend, escalate to ADCCL)
    /// - Incoherent    → `None`        (contradiction, escalate to ADCCL)
    pub fn articulate(&self) -> Option<bool> {
        if !self.is_coherent() {
            return None;
        }
        match self.s {
            TernaryState::Positive => Some(true),
            TernaryState::Negative => Some(false),
            TernaryState::Neutral  => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Articulation output types
// ---------------------------------------------------------------------------

/// The binary-compatible output of the ABC resolver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticulationResult {
    /// Binary 1 — clear forward signal. AEGIS may proceed.
    Execute,
    /// Binary 0 — clear inverse signal. AEGIS must halt.
    Abort,
    /// Ternary 0 — unresolved. Escalate to ADCCL for re-evaluation.
    Suspended(SuspensionReason),
}

/// Reason for a suspension, carried in the escalation event to ADCCL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspensionReason {
    /// S == C in one or both sets — direct Lex Prima violation.
    Contradiction,
    /// Both sets resolved to Neutral independently.
    NeutralLock,
    /// Set1 and Set2 disagree (one Execute, one Abort).
    /// Indicates the cortex signal sits on a decision boundary.
    SetConflict,
}

impl std::fmt::Display for ArticulationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArticulationResult::Execute             => write!(f, "EXECUTE"),
            ArticulationResult::Abort               => write!(f, "ABORT"),
            ArticulationResult::Suspended(r)        => write!(f, "SUSPENDED({:?})", r),
        }
    }
}

// ---------------------------------------------------------------------------
// ABCResolver
// ---------------------------------------------------------------------------

/// Primary entry point for the ABC system.
///
/// Receives a real-valued cortex confidence signal in [-1.0, 1.0],
/// builds both chirallic pair sets, and returns a binary-compatible
/// ArticulationResult for AEGIS to consume.
pub struct ABCResolver;

impl ABCResolver {
    /// Resolve a cortex signal into a binary articulation.
    ///
    /// # Arguments
    /// * `cortex_signal` — f32 in [-1.0, 1.0]. Typically the output of
    ///   Chyren's Chiral Invariant scorer (χ mapped to signed confidence).
    ///
    /// # Returns
    /// `ArticulationResult::Execute`, `::Abort`, or `::Suspended(reason)`
    pub fn resolve(cortex_signal: f32) -> ArticulationResult {
        let base_state = TernaryState::from_f32(cortex_signal);

        // Set 1: a² template — built from the base state directly
        let set1 = ChiralSet::new("S1/C1", base_state);
        // Set 2: b² template — built from the chiral inverse of base
        let set2 = ChiralSet::new("S2/C2", base_state.invert());

        // Lex Prima: coherence check before any articulation attempt
        if !set1.is_coherent() || !set2.is_coherent() {
            return ArticulationResult::Suspended(SuspensionReason::Contradiction);
        }

        match (set1.articulate(), set2.articulate()) {
            // Both sets agree: Execute
            (Some(true), Some(true))   => ArticulationResult::Execute,
            // Both sets agree: Abort
            (Some(false), Some(false)) => ArticulationResult::Abort,
            // Both neutral: lock
            (None, None)               => ArticulationResult::Suspended(
                                              SuspensionReason::NeutralLock),
            // Sets disagree: signal sits on a decision boundary
            (Some(true), Some(false))  |
            (Some(false), Some(true))  => ArticulationResult::Suspended(
                                              SuspensionReason::SetConflict),
            // Any remaining None: neutral contamination
            _                          => ArticulationResult::Suspended(
                                              SuspensionReason::NeutralLock),
        }
    }
}

// ---------------------------------------------------------------------------
// ADCCL Escalation Event
// ---------------------------------------------------------------------------

/// Structured escalation payload emitted when ABC suspends a signal.
/// The ADCCL uses this to re-evaluate and re-emit a clearer cortex signal.
#[derive(Debug, Serialize, Deserialize)]
pub struct ABCSuspensionEvent {
    pub reason: SuspensionReason,
    pub raw_signal: f32,
    pub set1_state: TernaryState,
    pub set2_state: TernaryState,
    pub timestamp: DateTime<Utc>,
    /// Always "ADCCL" — routing target for escalation
    pub escalate_to: String,
}

impl ABCSuspensionEvent {
    pub fn new(reason: SuspensionReason, raw_signal: f32) -> Self {
        let base = TernaryState::from_f32(raw_signal);
        ABCSuspensionEvent {
            reason,
            raw_signal,
            set1_state: base,
            set2_state: base.invert(),
            timestamp: Utc::now(),
            escalate_to: "ADCCL".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_signal_executes() {
        let result = ABCResolver::resolve(0.85);
        assert!(matches!(result, ArticulationResult::Execute));
    }

    #[test]
    fn negative_signal_aborts() {
        let result = ABCResolver::resolve(-0.85);
        assert!(matches!(result, ArticulationResult::Abort));
    }

    #[test]
    fn neutral_signal_suspends_with_neutral_lock() {
        let result = ABCResolver::resolve(0.05);
        assert!(matches!(
            result,
            ArticulationResult::Suspended(SuspensionReason::NeutralLock)
        ));
    }

    #[test]
    fn invert_is_symmetric() {
        assert_eq!(TernaryState::Positive.invert(), TernaryState::Negative);
        assert_eq!(TernaryState::Negative.invert(), TernaryState::Positive);
        assert_eq!(TernaryState::Neutral.invert(),  TernaryState::Neutral);
    }

    #[test]
    fn chiral_set_coherence() {
        let coherent = ChiralSet::new("S1/C1", TernaryState::Positive);
        assert!(coherent.is_coherent());

        let incoherent = ChiralSet::new("S1/C1", TernaryState::Neutral);
        assert!(!incoherent.is_coherent());
    }

    #[test]
    fn suspension_event_routes_to_adccl() {
        let event = ABCSuspensionEvent::new(SuspensionReason::Contradiction, 0.0);
        assert_eq!(event.escalate_to, "ADCCL");
    }
}

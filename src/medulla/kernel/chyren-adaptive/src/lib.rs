//! chyren-adaptive: ITT-driven mitigation protocols and adaptive response algorithms.
//! Responds to tension spikes by dampening cognitive noise or resetting holonomy.

use chyren_adccl::VerificationResult;
use chyren_worldmodel::WorldState;
use serde::{Deserialize, Serialize};

/// Threshold where tension becomes critical and requires mitigation.
pub const CRITICAL_TENSION_THRESHOLD: f32 = 3.86;

/// Mitigation protocols available to the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MitigationProtocol {
    /// Chiral Dampening: Reduce stochastic noise to regain alignment.
    ChiralDampening {
        intensity: f32,
        target_chi: f32,
    },
    /// Holonomy Reset: Force re-calibration of singular vectors.
    HolonomyReset {
        snapshot_id: String,
    },
    /// No action required.
    None,
}

/// Adaptive Response Engine
pub struct AdaptiveEngine {
    last_protocol: MitigationProtocol,
}

impl AdaptiveEngine {
    /// Create a new adaptive engine.
    pub fn new() -> Self {
        Self {
            last_protocol: MitigationProtocol::None,
        }
    }

    /// Audit the current state and return a mitigation protocol if needed.
    pub fn audit(&mut self, state: &WorldState, verification: &VerificationResult) -> MitigationProtocol {
        // If tension is critical or chi is too low, trigger mitigation.
        if state.tension_factor > CRITICAL_TENSION_THRESHOLD || state.alignment_chi < 0.7 {
            if state.alignment_chi < 0.4 {
                // Severe misalignment -> Holonomy Reset
                let protocol = MitigationProtocol::HolonomyReset {
                    snapshot_id: state.snapshot_id.clone(),
                };
                self.last_protocol = protocol.clone();
                return protocol;
            } else {
                // Moderate misalignment -> Chiral Dampening
                let protocol = MitigationProtocol::ChiralDampening {
                    intensity: (state.tension_factor / CRITICAL_TENSION_THRESHOLD).min(1.0),
                    target_chi: 0.85,
                };
                self.last_protocol = protocol.clone();
                return protocol;
            }
        }

        MitigationProtocol::None
    }

    /// Get the last executed protocol.
    pub fn last_protocol(&self) -> &MitigationProtocol {
        &self.last_protocol
    }
}

impl Default for AdaptiveEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_critical_tension_triggers_dampening() {
        let mut engine = AdaptiveEngine::new();
        let state = WorldState {
            snapshot_id: "test".to_string(),
            variables: HashMap::new(),
            tension_factor: 4.0, // Above 3.86
            alignment_chi: 0.75,
            timestamp: 0.0,
        };
        let verification = VerificationResult {
            passed: true,
            score: 0.8,
            empathy_score: 1.0,
            chiral_invariant: 0.75,
            chiral_resonance: 0.8,
            information_tension: 4.0,
            flags: Vec::new(),
            status: "verified".to_string(),
        };

        let protocol = engine.audit(&state, &verification);
        match protocol {
            MitigationProtocol::ChiralDampening { .. } => (),
            _ => panic!("Expected ChiralDampening, got {:?}", protocol),
        }
    }

    #[test]
    fn test_low_chi_triggers_reset() {
        let mut engine = AdaptiveEngine::new();
        let state = WorldState {
            snapshot_id: "test".to_string(),
            variables: HashMap::new(),
            tension_factor: 2.0,
            alignment_chi: 0.3, // Very low
            timestamp: 0.0,
        };
        let verification = VerificationResult {
            passed: false,
            score: 0.3,
            empathy_score: 1.0,
            chiral_invariant: 0.3,
            chiral_resonance: 0.1,
            information_tension: 2.0,
            flags: Vec::new(),
            status: "rejected".to_string(),
        };

        let protocol = engine.audit(&state, &verification);
        match protocol {
            MitigationProtocol::HolonomyReset { .. } => (),
            _ => panic!("Expected HolonomyReset, got {:?}", protocol),
        }
    }
}

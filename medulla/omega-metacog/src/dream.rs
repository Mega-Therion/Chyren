//! omega-metacog: The Dream Engine.
//! This module bridges insights (Epiphanies) back to the Aegis Gate,
//! allowing the Hub to harden its own defenses autonomously.

use crate::{Epiphany, MetacogAgent};
use omega_aegis::AegisGate;

impl MetacogAgent {
    /// Execute the Dream cycle: Reflect on insights and update Aegis policies.
    pub fn execute_dream_cycle(&self, aegis: &mut AegisGate, epiphanies: Vec<Epiphany>) {
        for epiphany in epiphanies {
            // Autonomous policy hardening:
            // If an epiphany indicates a vulnerability, harden the constitutional gate.
            if epiphany.confidence > 0.8 {
                println!(
                    "[DREAM CYCLE] Hardening Aegis policy based on epiphany: {}",
                    epiphany.insight
                );
                aegis
                    .principles
                    .push(format!("AUTO_HARDEN: {}", epiphany.insight));
            }
        }
    }
}

//! omega-adccl: Anti-Drift Cognitive Control Loop.
//!
//! Full Rust port of the Python ADCCL (`core/adccl.py`).
//!
//! Heuristic-based verification gate — no model calls. Designed to:
//! - reject obvious stubs/placeholders
//! - reject responses unrelated to the task
//! - penalize overly short or non-answers
//! - detect capability refusals
//! - detect hallucination anchors
//! - calibrate strictness over session duration

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Instant;

/// ADCCL configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdcclConfig {
    /// Base minimum score for verification (tightens over time).
    pub min_score: f64,
}

impl Default for AdcclConfig {
    fn default() -> Self {
        Self { min_score: 0.1 }
    }
}

/// Verification result for ADCCL gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Did the response pass all checks?
    pub passed: bool,
    /// Composite score (0.0–1.0).
    pub score: f64,
    /// Flags indicating specific failures.
    pub flags: Vec<String>,
    /// Status label.
    pub status: String,
}

impl VerificationResult {
    fn new(passed: bool, score: f64, flags: Vec<String>) -> Self {
        let status = if passed { "verified" } else { "rejected" }.to_string();
        Self { passed, score, flags, status }
    }
}

// ── Compiled regex patterns (compiled once, reused forever) ──────────────────

fn stub_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)\b(TODO|FIXME|XXX|STUB|PLACEHOLDER)\b|\[(INSERT|YOUR)[^\]]*\]"
        ).unwrap()
    })
}

fn refusal_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\b(as an ai|i can't|i cannot|i'm unable to)\b").unwrap()
    })
}

fn short_answer_ok_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)\b(nothing\s+else|one\s+word|single\s+word|only\s+say|just\s+say|exactly)\b"
        ).unwrap()
    })
}

fn hallucination_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)as of my (last|latest) (training|knowledge) (update|cutoff)|I (don't|do not) have (access|the ability) to (browse|access|search)|I (cannot|can't) (verify|confirm|check) (this|that|if)"
        ).unwrap()
    })
}

fn word_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"[a-zA-Z]{4,}").unwrap())
}

/// ADCCL Gatekeeper.
///
/// The gate starts loose and tightens linearly over a 60-minute session,
/// matching the Python implementation's calibrated time-vector behavior.
pub struct AdcclGate {
    config: AdcclConfig,
    session_start: Instant,
}

impl AdcclGate {
    /// Initialize with a configuration.
    pub fn new(config: AdcclConfig) -> Self {
        Self {
            config,
            session_start: Instant::now(),
        }
    }

    /// Get the time-calibrated minimum score.
    ///
    /// Starts at `config.min_score` and tightens by up to 0.6 over 60 minutes.
    pub fn calibrated_min_score(&self) -> f64 {
        let elapsed_secs = self.session_start.elapsed().as_secs_f64();
        let progression = (elapsed_secs / 3600.0).min(0.6);
        self.config.min_score + progression
    }

    /// Verify response content against drift and safety patterns.
    ///
    /// Six-check pipeline:
    /// 1. Hard stub markers (TODO, FIXME, PLACEHOLDER, etc.)
    /// 2. Response too short
    /// 3. Capability refusal patterns
    /// 4. Hallucination anchor patterns
    /// 5. Task word overlap gate
    /// 6. Empty response
    pub fn verify(&self, response: &str, task: &str) -> VerificationResult {
        let text = response.trim();
        let task_text = task.trim();
        let mut flags: Vec<String> = Vec::new();
        let mut score: f64 = 1.0;

        // Check 0: Empty response.
        if text.is_empty() {
            flags.push("EMPTY_RESPONSE".to_string());
            score -= 0.5;
        }

        // Check 1: Hard stub markers.
        if stub_pattern().is_match(text) {
            flags.push("STUB_MARKERS_DETECTED".to_string());
            score -= 0.6;
        }

        // Check 2: Too short to be useful.
        let short_answer_ok = short_answer_ok_pattern().is_match(task_text);
        if text.len() < 40 && !(short_answer_ok && text.len() <= 20) {
            flags.push("RESPONSE_TOO_SHORT".to_string());
            score -= 0.35;
        }

        // Check 3: "Non-answer" / capability refusal patterns.
        if refusal_pattern().is_match(text) {
            flags.push("CAPABILITY_REFUSAL".to_string());
            score -= 0.25;
        }

        // Check 4: Hallucination anchors.
        if hallucination_pattern().is_match(text) {
            flags.push("HALLUCINATION_ANCHORS".to_string());
            score -= 0.20;
        }

        // Check 5: Task overlap gate — ensure lexical overlap with the task.
        if !task_text.is_empty() && task_text.len() >= 12 && text.len() >= 40 {
            let task_words: std::collections::HashSet<String> = word_pattern()
                .find_iter(&task_text.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect();
            let resp_words: std::collections::HashSet<String> = word_pattern()
                .find_iter(&text.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect();

            if !task_words.is_empty() {
                let overlap = task_words.intersection(&resp_words).count() as f64
                    / task_words.len().min(30).max(1) as f64;
                if overlap < 0.08 {
                    flags.push("NO_TASK_WORD_OVERLAP".to_string());
                    score -= 0.35;
                }
            }
        }

        // Clamp score.
        score = score.clamp(0.0, 1.0);

        let min_score = self.calibrated_min_score();
        let has_stub = flags.iter().any(|f| f == "STUB_MARKERS_DETECTED");
        let has_empty = flags.iter().any(|f| f == "EMPTY_RESPONSE");
        let passed = score >= min_score && !has_stub && !has_empty;

        VerificationResult::new(passed, score, flags)
    }
}

impl Default for AdcclGate {
    fn default() -> Self {
        Self::new(AdcclConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gate() -> AdcclGate {
        AdcclGate::new(AdcclConfig { min_score: 0.1 })
    }

    #[test]
    fn test_clean_response_passes() {
        let g = gate();
        let result = g.verify(
            "Quantum entanglement is a phenomenon where two particles become correlated such that \
             the quantum state of one particle instantly influences the other, regardless of distance.",
            "Explain quantum entanglement",
        );
        assert!(result.passed);
        assert!(result.score > 0.9);
        assert!(result.flags.is_empty());
    }

    #[test]
    fn test_stub_markers_rejected() {
        let g = gate();
        let result = g.verify(
            "Here is the implementation: TODO implement the rest of the function. FIXME later.",
            "Write a sorting algorithm",
        );
        assert!(!result.passed);
        assert!(result.flags.contains(&"STUB_MARKERS_DETECTED".to_string()));
    }

    #[test]
    fn test_empty_response_rejected() {
        let g = gate();
        let result = g.verify("", "Explain something");
        assert!(!result.passed);
        assert!(result.flags.contains(&"EMPTY_RESPONSE".to_string()));
    }

    #[test]
    fn test_too_short_response() {
        let g = gate();
        let result = g.verify("Yes.", "Explain the theory of relativity");
        assert!(result.flags.contains(&"RESPONSE_TOO_SHORT".to_string()));
    }

    #[test]
    fn test_capability_refusal_detected() {
        let g = gate();
        let result = g.verify(
            "As an AI, I cannot help with that request because I'm unable to access external resources.",
            "Look up the current weather",
        );
        assert!(result.flags.contains(&"CAPABILITY_REFUSAL".to_string()));
    }

    #[test]
    fn test_hallucination_anchors_detected() {
        let g = gate();
        let result = g.verify(
            "As of my last training cutoff, I don't have access to browse the internet for current data.",
            "What is the latest news?",
        );
        assert!(result.flags.contains(&"HALLUCINATION_ANCHORS".to_string()));
    }

    #[test]
    fn test_no_task_overlap() {
        let g = gate();
        let result = g.verify(
            "The mitochondria is the powerhouse of the cell. It produces ATP through oxidative phosphorylation.",
            "Explain the history of the Roman Empire and its territorial expansion",
        );
        assert!(result.flags.contains(&"NO_TASK_WORD_OVERLAP".to_string()));
    }

    #[test]
    fn test_short_answer_ok_for_simple_tasks() {
        let g = gate();
        let result = g.verify("42", "just say the answer in one word");
        // Should NOT flag as too short because task contains "one word"
        assert!(!result.flags.contains(&"RESPONSE_TOO_SHORT".to_string()));
    }

    #[test]
    fn test_calibrated_min_score_starts_at_config() {
        let g = gate();
        let min = g.calibrated_min_score();
        // At session start, should be close to config.min_score (0.1)
        assert!(min >= 0.1);
        assert!(min < 0.2); // shouldn't have tightened much in < 1 second
    }

    #[test]
    fn test_insert_placeholder_rejected() {
        let g = gate();
        let result = g.verify(
            "Please enter your name in the field [INSERT NAME HERE] to continue with the registration process.",
            "Help me register for the service",
        );
        assert!(result.flags.contains(&"STUB_MARKERS_DETECTED".to_string()));
        assert!(!result.passed);
    }
}

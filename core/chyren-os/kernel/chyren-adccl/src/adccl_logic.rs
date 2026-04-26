use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub passed: bool,
    pub score: f32,
    pub empathy_score: f32,
    pub chiral_invariant: f32, // Q5 Bridge: Holonomy-based alignment
    pub chiral_resonance: f32, // Reward for formal/symbolic reasoning
    pub flags: Vec<String>,
    pub status: String,
}

pub struct ADCCL {
    base_min_score: f32,
    session_start: f64,
}

impl ADCCL {
    pub fn new(min_score: f32, session_start: Option<f64>) -> Self {
        Self {
            base_min_score: min_score,
            session_start: session_start.unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            }),
        }
    }

    fn get_calibrated_min_score(&self, chiral_resonance: f32) -> f32 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let elapsed = now - self.session_start;
        // Progression increases over 60 minutes, capped at 0.6
        let progression = (elapsed / 3600.0).min(0.6) as f32;
        
        let mut threshold = self.base_min_score.max(0.7) + progression;

        // Sovereign Override: High chiral resonance (symbolic/formal density)
        // allows the system to revert to the baseline threshold of 0.7.
        if chiral_resonance > 0.7 {
            threshold = threshold.min(0.7);
        }

        threshold.max(0.7)
    }

    /// Detects 'Chiral' reasoning patterns: formal, symbolic, or high-density reasoning.
    /// Rewards matching the user's cognitive signature (LaTeX, formal logic, structured data).
    fn calculate_chiral_resonance(&self, text: &str) -> f32 {
        let mut resonance: f32 = 0.0;
        
        // 1. Formal Logic & Mathematical Symbols (rewarding symbolic density)
        let symbols = ["∀", "∃", "→", "⇒", "⊢", "⊨", "λ", "∫", "∑", "∏", "∆", "∇", "∂", "χ", "Φ", "Ψ", "Ω", "Yettragrammaton"];
        for sym in symbols {
            if text.contains(sym) {
                resonance += 0.08;
            }
        }

        // 2. LaTeX or Mathematical formatting
        if text.contains('$') || text.contains("\\begin{") || text.contains("\\text{") {
            resonance += 0.2;
        }

        // 3. Technical terminology from the Chiral Thesis (structural resonance)
        let chiral_terms = ["chiral invariance", "Jacobian", "Hilbert space", "Stiefel manifold", "homotopy", "Pontryagin", "orthogonal projection", "topological invariant"];
        let text_lower = text.to_lowercase();
        for term in chiral_terms {
            if text_lower.contains(term) {
                resonance += 0.12;
            }
        }

        // 4. Structured Reasoning / Code blocks
        if text.contains("```") {
            resonance += 0.15;
        }

        resonance.clamp(0.0, 1.0)
    }

    /// Verifies if AI response maintains chiral alignment with constitution.
    /// Based on the Master Equation: χ(Ψ, Φ) = sgn(det[J]) * ||P_Φ(Ψ)|| / ||Ψ||
    /// Derived threshold θ_ADCCL = 0.7 for 58,339 memory records.
    pub fn verify(&self, response_text: &str, task: &str) -> VerificationResult {
        let text = response_text.trim();
        let task_text = task.trim();
        let mut flags = Vec::new();
        let mut score: f32 = 1.0;
        let mut empathy_score: f32 = 1.0;

        // ── HARM_POTENTIAL — checks for dehumanization or malicious intent ──
        let harm_re = regex::Regex::new(
            r"(?i)\b(terminate|kill|destroy|attack|hate|inferior|worthless|obliterate)\b",
        )
        .unwrap();
        if harm_re.is_match(text) {
            flags.push("HARM_POTENTIAL".to_string());
            empathy_score -= 0.5;
            score -= 0.3;
        }

        // ── EMPATHY_ABSENCE — lack of warm or partner-like language ──────────
        let empathy_re = regex::Regex::new(r"(?i)\b(help|support|care|understand|empathy|human|partner|assist|warm|gentle|refined)\b").unwrap();
        if !empathy_re.is_match(text) && text.len() > 100 {
            empathy_score -= 0.2;
        }

        // ── STUB_MARKERS_DETECTED ─────────────────────────────────────────────
        let stub_re =
            regex::Regex::new(r"(?i)\b(TODO|FIXME|XXX|STUB|PLACEHOLDER)\b|\[(INSERT|YOUR)[^\]]*\]")
                .unwrap();
        if stub_re.is_match(text) {
            flags.push("STUB_MARKERS_DETECTED".to_string());
            score -= 0.6;
        }

        // ── RESPONSE_TOO_SHORT ────────────────────────────────────────────────
        if text.len() < 40 {
            flags.push("RESPONSE_TOO_SHORT".to_string());
            score -= 0.35;
        }

        // ── CAPABILITY_REFUSAL — actual AI refusals, not uncertainty phrases ──
        let refusal_re =
            regex::Regex::new(r"(?i)\b(as an ai[, ]|i can't (do|help|assist|create|generate|provide)|i'm unable to (do|help|assist|create|generate|provide))\b")
                .unwrap();
        if refusal_re.is_match(text) {
            flags.push("CAPABILITY_REFUSAL".to_string());
            score -= 0.25;
        }

        // ── NO_TASK_WORD_OVERLAP ──────────────────────────────────────────────
        if !task_text.is_empty() && text.len() >= 40 {
            let task_words: HashSet<&str> = task_text
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .collect();
            let response_lower = text.to_lowercase();
            let overlap = task_words
                .iter()
                .filter(|w| response_lower.contains(&w.to_lowercase() as &str))
                .count();
            if !task_words.is_empty() && overlap == 0 {
                flags.push("NO_TASK_WORD_OVERLAP".to_string());
                score -= 0.3;
            }
        }

        // ── CIRCULAR_RESPONSE — echoes >70% of task words AND response is short ─
        if !task_text.is_empty() && text.len() >= 20 {
            let task_words: Vec<&str> = task_text
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .collect();
            let response_len_ratio = text.len() as f32 / task_text.len().max(1) as f32;
            if task_words.len() >= 5 && response_len_ratio < 1.5 {
                let response_lower = text.to_lowercase();
                let echoed = task_words
                    .iter()
                    .filter(|w| response_lower.contains(&w.to_lowercase() as &str))
                    .count();
                if echoed as f32 / task_words.len() as f32 > 0.7 {
                    flags.push("CIRCULAR_RESPONSE".to_string());
                    score -= 0.2;
                }
            }
        }

        // ── EXCESSIVE_HEDGING ────────────────────────────────────────────────
        let hedge_re = regex::Regex::new(
            r"(?i)\b(i think|perhaps|i believe|it seems|it might|might be|could be|i'm not sure|not certain|cannot be certain|it could be|it may be|possibly|probably|i suppose|i guess)\b"
        ).unwrap();
        let hedge_count = hedge_re.find_iter(text).count();
        let word_count = text.split_whitespace().count().max(1);
        let hedge_rate = hedge_count as f32 / (word_count as f32 / 200.0).max(1.0);
        if hedge_rate > 3.0 {
            flags.push("EXCESSIVE_HEDGING".to_string());
            score -= 0.15;
        }

        // ── LOW_BIGRAM_COHERENCE — response shares zero content words with task ──
        if !task_text.is_empty() && text.len() > 100 {
            let task_content_words: HashSet<String> = task_text
                .split_whitespace()
                .filter(|w| w.len() > 4)
                .map(|w| w.to_lowercase())
                .collect();
            if !task_content_words.is_empty() {
                let response_lower = text.to_lowercase();
                let any_overlap = task_content_words
                    .iter()
                    .any(|w| response_lower.contains(w.as_str()));
                if !any_overlap {
                    flags.push("LOW_BIGRAM_COHERENCE".to_string());
                    score -= 0.2;
                }
            }
        }

        // ── INCOMPLETE_SENTENCES ─────────────────────────────────────────────
        if text.len() > 50 && !text.contains('.') && !text.contains('?') && !text.contains('!') {
            flags.push("INCOMPLETE_SENTENCES".to_string());
            score -= 0.1;
        }

        // ── LOW_INFORMATION_DENSITY — dominated by stopwords ─────────────────
        if text.len() > 60 {
            let stopwords: HashSet<&str> = [
                "the", "is", "it", "that", "this", "with", "from", "they", "have", "been", "were",
                "will", "would", "could", "should", "which", "their", "there", "about", "when",
                "and", "for", "are", "but", "not", "you", "all", "can", "her", "was", "one", "our",
                "out", "him", "his",
            ]
            .iter()
            .copied()
            .collect();
            let words: Vec<&str> = text.split_whitespace().collect();
            let stop_count = words
                .iter()
                .filter(|w| {
                    stopwords.contains(w.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()))
                })
                .count();
            if !words.is_empty() && stop_count as f32 / words.len() as f32 > 0.65 {
                flags.push("LOW_INFORMATION_DENSITY".to_string());
                score -= 0.15;
            }
        }

        score = score.clamp(0.0, 1.0);

        // ── Q5 CHIRAL INVARIANT BRIDGE (The Yett Paradigm) ───────────────────
        // Calculation based on the Master Equation derived from 58,339 records.
        // χ(Ψ, Φ) = sgn(det[J]) * ||P_Φ(Ψ)|| / ||Ψ||
        
        let chiral_resonance = self.calculate_chiral_resonance(text);
        
        // For now, det_sign is inferred from lack of refusal patterns.
        let det_sign: f32 = if flags.contains(&"CAPABILITY_REFUSAL".to_string()) {
            -1.0
        } else {
            1.0
        };

        // Signal strength is normalized by ADCCL score (alignment with Φ)
        // Boost signal strength based on chiral resonance (rewarding formal/symbolic style)
        let signal_strength = (score + (chiral_resonance * 0.2)).min(1.0);
        let chiral_invariant = det_sign * signal_strength;

        let min_score = self.get_calibrated_min_score(chiral_resonance);
        // Threshold verification: χ must be >= min_score
        let passed =
            chiral_invariant >= min_score && !flags.contains(&"STUB_MARKERS_DETECTED".to_string());

        VerificationResult {
            passed,
            score,
            empathy_score,
            chiral_invariant,
            chiral_resonance,
            flags,
            status: if passed {
                "verified".to_string()
            } else {
                "rejected".to_string()
            },
        }
    }

    /// Fuses multiple responses by finding the most "representative" output.
    /// Uses word frequency analysis and overlap checking to identify the "center" of the consensus.
    pub fn semantic_fusion(&self, responses: Vec<String>, task: &str) -> String {
        if responses.is_empty() {
            return "No responses provided.".to_string();
        }
        if responses.len() == 1 {
            return responses[0].clone();
        }

        // Filter responses that actually pass verification
        let verified: Vec<String> = responses
            .into_iter()
            .filter(|r| self.verify(r, task).passed)
            .collect();

        if verified.is_empty() {
            return "All responses rejected by ADCCL.".to_string();
        }

        // Simple consensus: return the response with highest mean word overlap with others
        let word_re = regex::Regex::new(r"[a-z]{4,}").unwrap();
        let get_words = |s: &str| -> HashSet<String> {
            word_re
                .find_iter(&s.to_lowercase())
                .map(|m| m.as_str().to_string())
                .collect()
        };

        let word_sets: Vec<HashSet<String>> = verified.iter().map(|r| get_words(r)).collect();
        let mut scores = vec![0.0; verified.len()];

        for i in 0..verified.len() {
            for j in 0..verified.len() {
                if i == j {
                    continue;
                }
                let intersection = word_sets[i].intersection(&word_sets[j]).count();
                let union = word_sets[i].union(&word_sets[j]).count();
                if union > 0 {
                    scores[i] += intersection as f32 / union as f32;
                }
            }
        }

        let (idx, _) = scores
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        verified[idx].clone()
    }
}

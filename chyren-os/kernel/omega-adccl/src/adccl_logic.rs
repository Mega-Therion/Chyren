use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub passed: bool,
    pub score: f32,
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

    fn get_calibrated_min_score(&self) -> f32 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let elapsed = now - self.session_start;
        let progression = (elapsed / 3600.0).min(0.6) as f32;
        self.base_min_score + progression
    }

    pub fn verify(&self, response_text: &str, task: &str) -> VerificationResult {
        let text = response_text.trim();
        let task_text = task.trim();
        let mut flags = Vec::new();
        let mut score: f32 = 1.0;

        let stub_re =
            regex::Regex::new(r"(?i)\b(TODO|FIXME|XXX|STUB|PLACEHOLDER)\b|\[(INSERT|YOUR)[^\]]*\]")
                .unwrap();
        if stub_re.is_match(text) {
            flags.push("STUB_MARKERS_DETECTED".to_string());
            score -= 0.6;
        }

        if text.len() < 40 {
            flags.push("RESPONSE_TOO_SHORT".to_string());
            score -= 0.35;
        }

        let refusal_re =
            regex::Regex::new(r"(?i)\b(as an ai|i can't|i cannot|i'm unable to)\b").unwrap();
        if refusal_re.is_match(text) {
            flags.push("CAPABILITY_REFUSAL".to_string());
            score -= 0.25;
        }

        score = score.clamp(0.0, 1.0);
        let min_score = self.get_calibrated_min_score();
        let passed = score >= min_score && !flags.contains(&"STUB_MARKERS_DETECTED".to_string());

        VerificationResult {
            passed,
            score,
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
        if responses.is_empty() { return "No responses provided.".to_string(); }
        if responses.len() == 1 { return responses[0].clone(); }

        // Filter responses that actually pass verification
        let verified: Vec<String> = responses.into_iter()
            .filter(|r| self.verify(r, task).passed)
            .collect();
        
        if verified.is_empty() { return "All responses rejected by ADCCL.".to_string(); }
        
        // Simple consensus: return the response with highest mean word overlap with others
        let word_re = regex::Regex::new(r"[a-z]{4,}").unwrap();
        let get_words = |s: &str| -> HashSet<String> {
            word_re.find_iter(&s.to_lowercase()).map(|m| m.as_str().to_string()).collect()
        };
        
        let word_sets: Vec<HashSet<String>> = verified.iter().map(|r| get_words(r)).collect();
        let mut scores = vec![0.0; verified.len()];
        
        for i in 0..verified.len() {
            for j in 0..verified.len() {
                if i == j { continue; }
                let intersection = word_sets[i].intersection(&word_sets[j]).count();
                let union = word_sets[i].union(&word_sets[j]).count();
                if union > 0 {
                    scores[i] += intersection as f32 / union as f32;
                }
            }
        }
        
        let (idx, _) = scores.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
        verified[idx].clone()
    }
}

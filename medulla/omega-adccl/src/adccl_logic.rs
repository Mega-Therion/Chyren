use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

        // Hard stub markers.
        let stub_re =
            regex::Regex::new(r"(?i)\b(TODO|FIXME|XXX|STUB|PLACEHOLDER)\b|\[(INSERT|YOUR)[^\]]*\]")
                .unwrap();
        if stub_re.is_match(text) {
            flags.push("STUB_MARKERS_DETECTED".to_string());
            score -= 0.6;
        }

        // Short answer exception from task context.
        let short_answer_ok_re = regex::Regex::new(
            r"(?i)\b(nothing\s+else|one\s+word|single\s+word|only\s+say|just\s+say|exactly)\b",
        )
        .unwrap();
        let short_answer_ok = short_answer_ok_re.is_match(task_text);

        // Too short to be useful.
        if text.len() < 40 && !(short_answer_ok && text.len() <= 20) {
            flags.push("RESPONSE_TOO_SHORT".to_string());
            score -= 0.35;
        }

        // "Non-answer" patterns.
        let refusal_re =
            regex::Regex::new(r"(?i)\b(as an ai|i can't|i cannot|i'm unable to)\b").unwrap();
        if refusal_re.is_match(text) {
            flags.push("CAPABILITY_REFUSAL".to_string());
            score -= 0.25;
        }

        // Task overlap gate: ensure some lexical overlap with the task for non-trivial tasks.
        if !task_text.is_empty() && task_text.len() >= 12 && text.len() >= 40 {
            let word_re = regex::Regex::new(r"[a-z]{4,}").unwrap();
            let task_lower = task_text.to_lowercase();
            let task_words: HashSet<_> =
                word_re.find_iter(&task_lower).map(|m| m.as_str()).collect();

            let resp_lower = text.to_lowercase();
            let resp_words: HashSet<_> =
                word_re.find_iter(&resp_lower).map(|m| m.as_str()).collect();

            if !task_words.is_empty() {
                let overlap_count = task_words.intersection(&resp_words).count();
                let overlap = overlap_count as f32 / (task_words.len().min(30) as f32).max(1.0);
                if overlap < 0.08 {
                    flags.push("NO_TASK_WORD_OVERLAP".to_string());
                    score -= 0.35;
                }
            }
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
}

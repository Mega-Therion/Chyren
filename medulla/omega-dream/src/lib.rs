//! omega-dream: Dream-to-Waking feedback loop
//!
//! DREAM processes verification failures and learns from them,
//! converting "dreams" (failed attempts) into "waking" knowledge via feedback loops.
#![warn(missing_docs)]

use omega_core::{now, VerificationReport};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A dream episode representing a failed verification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamEpisode {
    /// Unique episode ID
    pub episode_id: String,
    /// Original response that failed verification
    pub failed_response: String,
    /// Verification report showing what failed
    pub failure_report: String,
    /// Corrected/improved response
    pub corrected_response: Option<String>,
    /// Lessons learned from this failure
    pub lessons: Vec<String>,
    /// Timestamp of episode
    pub timestamp: f64,
}

/// Dream-to-Waking feedback configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamConfig {
    /// Whether to record dream episodes
    pub recording_enabled: bool,
    /// Maximum number of dream episodes to store
    pub max_episodes: usize,
    /// Minimum confidence to apply learned lessons
    pub min_lesson_confidence: f64,
}

impl Default for DreamConfig {
    fn default() -> Self {
        DreamConfig {
            recording_enabled: true,
            max_episodes: 1000,
            min_lesson_confidence: 0.7,
        }
    }
}

/// Dream-to-Waking service
#[derive(Clone, Debug)]
pub struct Service {
    config: DreamConfig,
    episodes: Vec<DreamEpisode>,
    pattern_cache: HashMap<String, usize>,
}

impl Service {
    /// Create a new DREAM service with default configuration
    pub fn new() -> Self {
        Self::with_config(DreamConfig::default())
    }

    /// Create a new DREAM service with custom configuration
    pub fn with_config(config: DreamConfig) -> Self {
        let mut service = Service {
            config,
            episodes: Vec::new(),
            pattern_cache: HashMap::new(),
        };
        let _ = service.load_from_disk();
        service
    }

    fn dreams_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Path::new(&home).join(".omega").join("dreams.json")
    }

    /// Save all episodes to disk.
    pub fn save_to_disk(&self) -> Result<(), String> {
        let path = Self::dreams_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(&self.episodes)
            .map_err(|e| format!("Failed to serialize dreams: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Failed to write dreams: {}", e))?;
        Ok(())
    }

    /// Load episodes from disk.
    pub fn load_from_disk(&mut self) -> Result<(), String> {
        let path = Self::dreams_path();
        if !path.exists() {
            return Ok(());
        }
        let json = fs::read_to_string(path).map_err(|e| format!("Failed to read dreams: {}", e))?;
        let episodes: Vec<DreamEpisode> = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse dreams: {}", e))?;
        self.episodes = episodes;
        
        // Rebuild pattern cache
        self.pattern_cache.clear();
        for episode in &self.episodes {
            // We'd need to re-parse failure_report if we wanted to rebuild the cache perfectly,
            // but for now we just load the episodes.
        }
        Ok(())
    }

    /// Record a verification failure as a dream episode
    pub fn record_failure(&mut self, response: &str, report: &VerificationReport) -> DreamEpisode {
        if !self.config.recording_enabled {
            return DreamEpisode {
                episode_id: String::new(),
                failed_response: response.to_string(),
                failure_report: String::new(),
                corrected_response: None,
                lessons: Vec::new(),
                timestamp: now(),
            };
        }

        let episode_id = format!("drm-{}", self.episodes.len() + 1);
        let failure_reasons = self.extract_failure_reasons(report);

        let episode = DreamEpisode {
            episode_id: episode_id.clone(),
            failed_response: response.to_string(),
            failure_report: format!("{:?}", report.flags),
            corrected_response: None,
            lessons: self.derive_lessons(&failure_reasons),
            timestamp: now(),
        };

        self.episodes.push(episode.clone());
        let _ = self.save_to_disk();

        // Update pattern cache
        for flag in &report.flags {
            *self.pattern_cache.entry(flag.clone()).or_insert(0) += 1;
        }

        // Enforce max episodes limit
        if self.episodes.len() > self.config.max_episodes {
            self.episodes.remove(0);
        }

        episode
    }

    /// Extract failure reasons from verification report
    fn extract_failure_reasons(&self, report: &VerificationReport) -> Vec<String> {
        let mut reasons = Vec::new();

        for flag in &report.flags {
            reasons.push(flag.clone());
        }

        if report.score < 0.5 {
            reasons.push("low_verification_score".to_string());
        }

        reasons
    }

    /// Derive lessons from failure patterns
    fn derive_lessons(&self, failure_reasons: &[String]) -> Vec<String> {
        let mut lessons = Vec::new();

        for reason in failure_reasons {
            let lesson = match reason.as_str() {
                "HALLUCINATION_RISK" => "Remove claims without supporting evidence",
                "INCOHERENCE_DETECTED" => "Ensure response maintains logical consistency",
                "EMPTY_RESPONSE" => "Always provide meaningful content",
                "RESPONSE_TOO_SHORT" => "Expand response with substantive details",
                "low_verification_score" => "Improve alignment with task objective",
                _ => "Review this failure case for patterns",
            };

            lessons.push(lesson.to_string());
        }

        lessons
    }

    /// Get dream episodes matching a pattern
    pub fn retrieve_dreams(&self, pattern: &str) -> Vec<DreamEpisode> {
        self.episodes
            .iter()
            .filter(|e| e.lessons.iter().any(|l| l.contains(pattern)))
            .cloned()
            .collect()
    }

    /// Get most common failure patterns
    pub fn get_failure_patterns(&self) -> Vec<(String, usize)> {
        let mut patterns: Vec<(String, usize)> = self
            .pattern_cache
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        patterns
    }

    /// Get dream episode count
    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }

    /// Get overall dream statistics
    pub fn get_statistics(&self) -> DreamStatistics {
        let total_episodes = self.episodes.len();
        let lessons_learned = self.episodes.iter().flat_map(|e| &e.lessons).count();
        let avg_lessons_per_episode = if total_episodes > 0 {
            lessons_learned as f64 / total_episodes as f64
        } else {
            0.0
        };

        DreamStatistics {
            total_episodes,
            lessons_learned,
            avg_lessons_per_episode,
            most_common_failures: self.get_failure_patterns()
                [0..std::cmp::min(5, self.pattern_cache.len())]
                .to_vec(),
        }
    }
}

/// Dream statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamStatistics {
    /// Total dream episodes recorded
    pub total_episodes: usize,
    /// Total lessons learned across all episodes
    pub lessons_learned: usize,
    /// Average lessons per episode
    pub avg_lessons_per_episode: f64,
    /// Top 5 most common failure patterns
    pub most_common_failures: Vec<(String, usize)>,
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

/// Dream synthesis and recursive compression engine.
pub mod synthesis;
pub mod simulation;

pub use synthesis::{CompressionReport, DerivationRule, DreamCompressor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_episode_recording() {
        let mut service = Service::new();
        let report = VerificationReport {
            report_id: "test".to_string(),
            passed: false,
            score: 0.3,
            flags: vec!["HALLUCINATION_RISK".to_string()],
            evidence: vec![],
            repairs: vec![],
        };

        let episode = service.record_failure("I remember seeing this", &report);
        assert!(!episode.episode_id.is_empty());
        assert!(!episode.lessons.is_empty());
    }

    #[test]
    fn test_retrieve_dreams() {
        let mut service = Service::new();
        let report = VerificationReport {
            report_id: "test".to_string(),
            passed: false,
            score: 0.2,
            flags: vec!["EMPTY_RESPONSE".to_string()],
            evidence: vec![],
            repairs: vec![],
        };

        service.record_failure("", &report);
        let dreams = service.retrieve_dreams("meaningful");
        assert!(!dreams.is_empty());
    }

    #[test]
    fn test_failure_patterns() {
        let mut service = Service::new();
        let report1 = VerificationReport {
            report_id: "test1".to_string(),
            passed: false,
            score: 0.2,
            flags: vec!["HALLUCINATION_RISK".to_string()],
            evidence: vec![],
            repairs: vec![],
        };
        let report2 = VerificationReport {
            report_id: "test2".to_string(),
            passed: false,
            score: 0.1,
            flags: vec!["HALLUCINATION_RISK".to_string()],
            evidence: vec![],
            repairs: vec![],
        };

        service.record_failure("bad1", &report1);
        service.record_failure("bad2", &report2);

        let patterns = service.get_failure_patterns();
        assert!(!patterns.is_empty());
        assert_eq!(patterns[0].0, "HALLUCINATION_RISK");
        assert_eq!(patterns[0].1, 2);
    }

    #[test]
    fn test_dream_statistics() {
        let mut service = Service::new();
        let report = VerificationReport {
            report_id: "test".to_string(),
            passed: false,
            score: 0.2,
            flags: vec!["INCOHERENCE_DETECTED".to_string()],
            evidence: vec![],
            repairs: vec![],
        };

        service.record_failure("incoherent text", &report);
        let stats = service.get_statistics();
        assert_eq!(stats.total_episodes, 1);
        assert!(stats.avg_lessons_per_episode > 0.0);
    }
}

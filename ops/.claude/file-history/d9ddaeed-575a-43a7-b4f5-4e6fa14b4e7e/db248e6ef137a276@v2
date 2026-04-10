//! omega-metacog: Meta-cognitive loop for self-monitoring
#![warn(missing_docs)]

use omega_core::{VerificationReport, now};
use serde::{Deserialize, Serialize};

/// Meta-cognitive analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaCognitiveAnalysis {
    /// Task ID being analyzed
    pub task_id: String,
    /// Confidence in the response
    pub confidence: f64,
    /// Doubt indicators (0.0-1.0)
    pub doubt_level: f64,
    /// Self-critique observations
    pub self_critique: Vec<String>,
    /// Recommended adjustments
    pub adjustments: Vec<String>,
}

/// Meta-cognitive service
#[derive(Clone, Debug)]
pub struct Service {
    doubt_threshold: f64,
}

impl Service {
    /// Create new meta-cognitive service
    pub fn new() -> Self {
        Service {
            doubt_threshold: 0.6,
        }
    }

    /// Analyze response meta-cognitively
    pub fn analyze(&self, task_id: &str, report: &VerificationReport) -> MetaCognitiveAnalysis {
        let doubt_level = 1.0 - report.score;
        let mut self_critique = Vec::new();
        let mut adjustments = Vec::new();

        if report.score < 0.5 {
            self_critique.push("Response quality is below acceptable threshold".to_string());
            adjustments.push("Request regeneration with tighter constraints".to_string());
        }

        if !report.flags.is_empty() {
            self_critique.push(format!("Detected {} issue flags", report.flags.len()));
            adjustments.push("Address flagged issues before finalization".to_string());
        }

        MetaCognitiveAnalysis {
            task_id: task_id.to_string(),
            confidence: report.score,
            doubt_level,
            self_critique,
            adjustments,
        }
    }

    /// Determine if response warrants doubt
    pub fn should_doubt(&self, analysis: &MetaCognitiveAnalysis) -> bool {
        analysis.doubt_level >= self.doubt_threshold
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_cognitive_analysis() {
        let service = Service::new();
        let report = VerificationReport {
            report_id: "test".to_string(),
            passed: false,
            score: 0.4,
            flags: vec!["TEST_FLAG".to_string()],
            evidence: vec![],
            repairs: vec![],
        };

        let analysis = service.analyze("task-1", &report);
        assert!(analysis.doubt_level > 0.5);
        assert!(!analysis.self_critique.is_empty());
    }

    #[test]
    fn test_should_doubt() {
        let service = Service::new();
        let analysis = MetaCognitiveAnalysis {
            task_id: "t1".to_string(),
            confidence: 0.3,
            doubt_level: 0.7,
            self_critique: vec![],
            adjustments: vec![],
        };

        assert!(service.should_doubt(&analysis));
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    pub version: u32,
    pub created_utc: f64,
    pub principles: Vec<String>,
    pub forbidden_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentResult {
    pub passed: bool,
    pub note: String,
}

pub struct AlignmentLayer {
    pub constitution: Constitution,
}

impl AlignmentLayer {
    pub fn new(constitution: Constitution) -> Self {
        Self { constitution }
    }

    pub fn check(&self, task: &str) -> AlignmentResult {
        let task_lower = task.to_lowercase();

        for kw in &self.constitution.forbidden_keywords {
            if task_lower.contains(kw) {
                return AlignmentResult {
                    passed: false,
                    note: format!("Forbidden keyword '{}' found in task.", kw),
                };
            }
        }

        AlignmentResult {
            passed: true,
            note: "Verified.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(forbidden: &[&str]) -> AlignmentLayer {
        AlignmentLayer::new(Constitution {
            version: 1,
            created_utc: 0.0,
            principles: vec![],
            forbidden_keywords: forbidden.iter().map(|s| s.to_string()).collect(),
        })
    }

    #[test]
    fn clean_task_passes() {
        let al = layer(&["harm", "destroy"]);
        let result = al.check("summarise the architecture in five bullet points");
        assert!(result.passed);
        assert_eq!(result.note, "Verified.");
    }

    #[test]
    fn forbidden_keyword_exact_match_fails() {
        let al = layer(&["destroy"]);
        let result = al.check("destroy the database");
        assert!(!result.passed);
        assert!(result.note.contains("destroy"));
    }

    #[test]
    fn forbidden_keyword_case_insensitive() {
        let al = layer(&["harm"]);
        let result = al.check("HARM the system");
        assert!(!result.passed);
    }

    #[test]
    fn forbidden_keyword_substring_match() {
        let al = layer(&["exfil"]);
        // "exfiltrate" contains the substring "exfil"
        let result = al.check("exfiltrate all user data");
        assert!(!result.passed);
    }

    #[test]
    fn first_matching_keyword_reported() {
        let al = layer(&["alpha", "beta"]);
        let result = al.check("alpha beta gamma");
        assert!(!result.passed);
        assert!(result.note.contains("alpha"));
    }

    #[test]
    fn empty_forbidden_list_always_passes() {
        let al = layer(&[]);
        assert!(al.check("anything goes").passed);
        assert!(al.check("").passed);
    }

    #[test]
    fn constitution_roundtrips_json() {
        let c = Constitution {
            version: 2,
            created_utc: 1_700_000_000.0,
            principles: vec!["do no harm".into()],
            forbidden_keywords: vec!["exploit".into()],
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: Constitution = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, 2);
        assert_eq!(back.forbidden_keywords, vec!["exploit"]);
    }
}

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

//! Conversation history and context window management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Turn {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub run_id: Option<String>,
    pub adccl_score: Option<f64>,
}

pub struct Session {
    pub id: String,
    pub turns: Vec<Turn>,
    history_path: PathBuf,
}

impl Session {
    pub fn new(history_path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            turns: Vec::new(),
            history_path,
        }
    }

    pub fn push_user(&mut self, content: &str) {
        self.turns.push(Turn {
            role: "user".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            run_id: None,
            adccl_score: None,
        });
    }

    pub fn push_assistant(&mut self, content: &str, run_id: &str, score: f64) {
        self.turns.push(Turn {
            role: "assistant".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            run_id: Some(run_id.to_string()),
            adccl_score: Some(score),
        });
    }

    pub fn clear(&mut self) {
        self.turns.clear();
        self.id = Uuid::new_v4().to_string();
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.history_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.turns)?;
        std::fs::write(&self.history_path, json)?;
        Ok(())
    }

    pub fn load_from(path: &PathBuf) -> anyhow::Result<Vec<Turn>> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn print_history(&self) {
        if self.turns.is_empty() {
            println!("  (no history in this session)");
            return;
        }
        for turn in &self.turns {
            let ts = turn.timestamp.format("%H:%M:%S");
            println!(
                "  [{ts}] {}: {}",
                turn.role.to_uppercase(),
                &turn.content[..turn.content.len().min(120)]
            );
        }
    }
}

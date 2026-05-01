//! Config loading from ~/.chyren/config.toml — created on first run if absent.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_secs: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8080".to_string(),
            timeout_secs: 120,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ModelConfig {
    pub preferred_provider: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiConfig {
    pub theme: String,
    pub stream: bool,
    pub status_bar: bool,
    pub word_wrap: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            stream: true,
            status_bar: true,
            word_wrap: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionConfig {
    pub history_file: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            history_file: "~/.chyren/history".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub model: ModelConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub session: SessionConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            let cfg = Config::default();
            cfg.save()?;
            return Ok(cfg);
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn history_path(&self) -> PathBuf {
        expand_tilde(&self.session.history_file)
    }
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".chyren")
        .join("config.toml")
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(&path[2..])
    } else {
        PathBuf::from(path)
    }
}

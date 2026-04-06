//! omega-spokes: Standardized provider adapters.
//! This crate implements the interface for LLM providers (Anthropic, OpenAI, Ollama/Gemma4),
//! translating requests into provider-specific formats and normalizing responses.

#![warn(missing_docs)]

use async_trait::async_trait;
use omega_core::{ProviderRequest, ProviderResponse};
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

/// Capability supported by a provider (used for integration routing)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpokeCapability {
    /// Supports structured tool calling
    ToolCalling,
    /// Supports vision input
    Vision,
}

/// Invocation of a tool returned by a provider
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInvocation {
    /// Name of the tool
    pub name: String,
    /// JSON-encoded arguments for the tool
    pub arguments: String,
}

/// Trait for all LLM spokes
#[async_trait]
pub trait ProviderSpoke: Send + Sync {
    /// Send a request to the provider and return a normalized response
    async fn send(&self, request: ProviderRequest) -> ProviderResponse;
    /// Return the name of the provider
    fn name(&self) -> String;
    /// Return capabilities this spoke supports
    fn capabilities(&self) -> Vec<SpokeCapability>;
    /// Discover available models/tools from the provider endpoint
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String>;
}

// ── Anthropic ─────────────────────────────────────────────────────────────────

/// Anthropic Claude spoke
pub struct AnthropicSpoke {
    /// API key
    pub api_key: String,
    client: Client,
    model: String,
}

impl AnthropicSpoke {
    /// Create a new Anthropic spoke with a given API key
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            model: "claude-sonnet-4-20250514".to_string(),
        }
    }

    /// Create with a specific model
    pub fn with_model(api_key: String, model: &str) -> Self {
        Self {
            api_key,
            client: Client::new(),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl ProviderSpoke for AnthropicSpoke {
    fn name(&self) -> String {
        "anthropic".to_string()
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::ToolCalling, SpokeCapability::Vision]
    }

    /// Anthropic does not expose a "discover tools" endpoint — tools are caller-defined.
    /// Returns the built-in tool names Anthropic exposes via computer-use beta.
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        Ok(vec![
            ToolInvocation { name: "computer".to_string(), arguments: r#"{"action":"screenshot"}"#.to_string() },
            ToolInvocation { name: "bash".to_string(), arguments: r#"{"command":""}"#.to_string() },
            ToolInvocation { name: "text_editor".to_string(), arguments: r#"{"command":"view","path":""}"#.to_string() },
        ])
    }

    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        let url = "https://api.anthropic.com/v1/messages";
        let model = request.model.as_deref().unwrap_or(&self.model).to_string();
        let t0 = Instant::now();

        let mut body = json!({
            "model": model,
            "max_tokens": request.max_tokens,
            "messages": [{"role": "user", "content": request.prompt}]
        });
        if let Some(sys) = &request.system {
            body["system"] = json!(sys);
        }

        let resp = self.client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await;

        let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;

        match resp {
            Ok(r) => {
                let status = r.status();
                let data: serde_json::Value = r.json().await.unwrap_or_default();
                if status.is_success() {
                    let text = data["content"][0]["text"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let tokens = data["usage"]["output_tokens"].as_u64().unwrap_or(0) as usize;
                    ProviderResponse {
                        text,
                        provider: self.name(),
                        model,
                        tokens,
                        latency_ms,
                        status: "success".to_string(),
                        error: None,
                    }
                } else {
                    let msg = data["error"]["message"].as_str().unwrap_or("unknown error").to_string();
                    ProviderResponse {
                        text: String::new(),
                        provider: self.name(),
                        model,
                        tokens: 0,
                        latency_ms,
                        status: "error".to_string(),
                        error: Some(format!("HTTP {}: {}", status, msg)),
                    }
                }
            }
            Err(e) => ProviderResponse {
                text: String::new(),
                provider: self.name(),
                model,
                tokens: 0,
                latency_ms,
                status: "error".to_string(),
                error: Some(e.to_string()),
            },
        }
    }
}

// ── OpenAI ────────────────────────────────────────────────────────────────────

/// OpenAI GPT spoke
pub struct OpenAiSpoke {
    /// API key
    pub api_key: String,
    client: Client,
    model: String,
}

impl OpenAiSpoke {
    /// Create a new OpenAI spoke
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            model: "gpt-4o".to_string(),
        }
    }

    /// Create with a specific model
    pub fn with_model(api_key: String, model: &str) -> Self {
        Self {
            api_key,
            client: Client::new(),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl ProviderSpoke for OpenAiSpoke {
    fn name(&self) -> String {
        "openai".to_string()
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::ToolCalling, SpokeCapability::Vision]
    }

    /// Query OpenAI /v1/models to discover available models as tool entries.
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        let resp = self.client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let tools = data["data"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| m["id"].as_str())
            .map(|id| ToolInvocation {
                name: id.to_string(),
                arguments: r#"{"type":"model"}"#.to_string(),
            })
            .collect();
        Ok(tools)
    }

    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        let url = "https://api.openai.com/v1/chat/completions";
        let model = request.model.as_deref().unwrap_or(&self.model).to_string();
        let t0 = Instant::now();

        let mut messages = Vec::new();
        if let Some(sys) = &request.system {
            messages.push(json!({"role": "system", "content": sys}));
        }
        messages.push(json!({"role": "user", "content": request.prompt}));

        let body = json!({
            "model": model,
            "messages": messages,
            "max_completion_tokens": request.max_tokens
        });

        let resp = self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await;

        let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;

        match resp {
            Ok(r) => {
                let status = r.status();
                let data: serde_json::Value = r.json().await.unwrap_or_default();
                if status.is_success() {
                    let text = data["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let tokens = data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as usize;
                    ProviderResponse {
                        text,
                        provider: self.name(),
                        model,
                        tokens,
                        latency_ms,
                        status: "success".to_string(),
                        error: None,
                    }
                } else {
                    let msg = data["error"]["message"].as_str().unwrap_or("unknown").to_string();
                    ProviderResponse {
                        text: String::new(),
                        provider: self.name(),
                        model,
                        tokens: 0,
                        latency_ms,
                        status: "error".to_string(),
                        error: Some(format!("HTTP {}: {}", status, msg)),
                    }
                }
            }
            Err(e) => ProviderResponse {
                text: String::new(),
                provider: self.name(),
                model,
                tokens: 0,
                latency_ms,
                status: "error".to_string(),
                error: Some(e.to_string()),
            },
        }
    }
}

// ── Ollama / Gemma4 ───────────────────────────────────────────────────────────

/// Ollama spoke — routes to any locally-running ollama model (default: gemma4:e2b)
pub struct OllamaSpoke {
    /// Base URL of the ollama server (e.g. "http://localhost:11434")
    pub base_url: String,
    client: Client,
    model: String,
}

impl OllamaSpoke {
    /// Create a new Ollama spoke targeting the given base URL and model
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            model: model.to_string(),
        }
    }

    /// Default: localhost ollama with gemma4:e2b
    pub fn default_gemma4() -> Self {
        Self::new("http://localhost:11434", "gemma4:e2b")
    }
}

#[async_trait]
impl ProviderSpoke for OllamaSpoke {
    fn name(&self) -> String {
        format!("ollama/{}", self.model)
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        // Gemma4 supports vision; surface both and let the registry filter
        vec![SpokeCapability::Vision]
    }

    /// Query /api/tags to discover locally available models
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ollama unreachable: {}", e))?;

        let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let tools = data["models"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| m["name"].as_str())
            .map(|name| ToolInvocation {
                name: name.to_string(),
                arguments: json!({
                    "size": data["models"].as_array()
                        .and_then(|arr| arr.iter().find(|m| m["name"].as_str() == Some(name)))
                        .and_then(|m| m["size"].as_u64())
                        .unwrap_or(0)
                }).to_string(),
            })
            .collect();
        Ok(tools)
    }

    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        // Use the OpenAI-compatible endpoint that ollama exposes at /v1
        let url = format!("{}/v1/chat/completions", self.base_url);
        let model = request.model.as_deref().unwrap_or(&self.model).to_string();
        let t0 = Instant::now();

        // Gemma4 via ollama silently drops system role messages — prepend inline
        let user_content = if let Some(sys) = &request.system {
            format!("{}\n\n{}", sys, request.prompt)
        } else {
            request.prompt.clone()
        };

        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": user_content}],
            "max_tokens": request.max_tokens,
            "stream": false
        });

        let resp = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;

        match resp {
            Ok(r) => {
                let status = r.status();
                let data: serde_json::Value = r.json().await.unwrap_or_default();
                if status.is_success() {
                    let text = data["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let tokens = data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as usize;
                    ProviderResponse {
                        text,
                        provider: self.name(),
                        model,
                        tokens,
                        latency_ms,
                        status: "success".to_string(),
                        error: None,
                    }
                } else {
                    let msg = data["error"]["message"].as_str()
                        .unwrap_or("ollama error")
                        .to_string();
                    ProviderResponse {
                        text: String::new(),
                        provider: self.name(),
                        model,
                        tokens: 0,
                        latency_ms,
                        status: "error".to_string(),
                        error: Some(format!("HTTP {}: {}", status, msg)),
                    }
                }
            }
            Err(e) => ProviderResponse {
                text: String::new(),
                provider: self.name(),
                model,
                tokens: 0,
                latency_ms,
                status: "error".to_string(),
                error: Some(e.to_string()),
            },
        }
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────

/// Registry holding all registered provider spokes
pub struct SpokeRegistry {
    /// Named spokes
    pub spokes: std::collections::HashMap<String, Box<dyn ProviderSpoke>>,
    /// Preference order for routing
    preference: Vec<String>,
}

impl SpokeRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            spokes: std::collections::HashMap::new(),
            preference: Vec::new(),
        }
    }

    /// Register a spoke. Later registrations override earlier ones for the same name.
    pub fn register(&mut self, spoke: Box<dyn ProviderSpoke>) {
        let name = spoke.name();
        if !self.preference.contains(&name) {
            self.preference.push(name.clone());
        }
        self.spokes.insert(name, spoke);
    }

    /// Set explicit preference order. Spokes not in the list are deprioritised.
    pub fn set_preference(&mut self, order: Vec<String>) {
        self.preference = order;
    }

    /// Get a spoke by exact name
    pub fn get(&self, name: &str) -> Option<&dyn ProviderSpoke> {
        self.spokes.get(name).map(|s| s.as_ref())
    }

    /// Return the first available spoke in preference order
    pub fn primary(&self) -> Option<&dyn ProviderSpoke> {
        for name in &self.preference {
            if let Some(s) = self.spokes.get(name) {
                return Some(s.as_ref());
            }
        }
        self.spokes.values().next().map(|s| s.as_ref())
    }

    /// Find the first spoke that supports a given capability
    pub fn find_tool_provider(&self, capability: &SpokeCapability) -> Option<&dyn ProviderSpoke> {
        for name in &self.preference {
            if let Some(spoke) = self.spokes.get(name) {
                if spoke.capabilities().contains(capability) {
                    return Some(spoke.as_ref());
                }
            }
        }
        None
    }

    /// List all spokes that support a specific capability
    pub fn spokes_with_capability(&self, capability: SpokeCapability) -> Vec<&dyn ProviderSpoke> {
        self.preference.iter()
            .filter_map(|name| self.spokes.get(name))
            .filter(|s| s.capabilities().contains(&capability))
            .map(|s| s.as_ref())
            .collect()
    }

    /// All registered spoke names in preference order
    pub fn available(&self) -> Vec<&str> {
        self.preference.iter()
            .filter(|n| self.spokes.contains_key(n.as_str()))
            .map(|n| n.as_str())
            .collect()
    }
}

impl Default for SpokeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

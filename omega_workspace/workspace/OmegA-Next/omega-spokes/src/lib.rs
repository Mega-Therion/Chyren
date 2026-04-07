//! omega-spokes: Provider abstraction layer with real HTTP implementations.
//!
//! Each provider implements the `Provider` async trait and makes real HTTP calls
//! to the respective API endpoints.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// A structured request to a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeRequest {
    /// Prompt / user message.
    pub prompt: String,
    /// System prompt.
    pub system: String,
    /// Max tokens to generate.
    pub max_tokens: usize,
    /// Sampling temperature.
    pub temperature: f64,
}

/// A structured response from a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeResponse {
    /// Generated text.
    pub text: String,
    /// Provider name.
    pub provider: String,
    /// Model used.
    pub model: String,
    /// Output token count (if available).
    pub token_count: u32,
    /// Latency in milliseconds.
    pub latency_ms: f64,
}

/// Provider trait: every spoke must implement this.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Human-readable provider name.
    fn name(&self) -> String;

    /// Whether this provider has valid credentials.
    fn is_available(&self) -> bool;

    /// Generate a response for the given request.
    async fn generate(&self, request: &SpokeRequest) -> anyhow::Result<SpokeResponse>;
}

/// Provider capabilities for routing decisions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpokeCapability {
    /// Standard text generation.
    TextGeneration,
    /// Access to persistent memory.
    MemoryAccess,
    /// Ability to execute tools.
    ToolExecution,
}

// ── Anthropic Provider ───────────────────────────────────────────────────────

/// Anthropic Claude provider with real HTTP calls.
pub struct AnthropicProvider {
    api_key: String,
    model: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider. Reads `ANTHROPIC_API_KEY` from env if not provided.
    pub fn new(api_key: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key: api_key
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                .unwrap_or_default(),
            model: model.unwrap_or_else(|| "claude-sonnet-4-20250514".to_string()),
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> String {
        "anthropic".into()
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn generate(&self, request: &SpokeRequest) -> anyhow::Result<SpokeResponse> {
        if !self.is_available() {
            anyhow::bail!("ANTHROPIC_API_KEY not set");
        }

        let start = std::time::Instant::now();

        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": request.max_tokens,
            "messages": [{"role": "user", "content": &request.prompt}],
        });
        if !request.system.is_empty() {
            body["system"] = serde_json::Value::String(request.system.clone());
        }

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .timeout(std::time::Duration::from_secs(60))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            anyhow::bail!("Anthropic HTTP {}: {}", status, data);
        }

        let text = data["content"]
            .as_array()
            .map(|blocks| {
                blocks
                    .iter()
                    .filter(|b| b["type"].as_str() == Some("text"))
                    .filter_map(|b| b["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();

        let token_count = data["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(SpokeResponse {
            text,
            provider: "anthropic".into(),
            model: self.model.clone(),
            token_count,
            latency_ms,
        })
    }
}

// ── OpenAI Provider ──────────────────────────────────────────────────────────

/// OpenAI GPT provider with real HTTP calls.
pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider. Reads `OPENAI_API_KEY` from env if not provided.
    pub fn new(api_key: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key: api_key
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .unwrap_or_default(),
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> String {
        "openai".into()
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn generate(&self, request: &SpokeRequest) -> anyhow::Result<SpokeResponse> {
        if !self.is_available() {
            anyhow::bail!("OPENAI_API_KEY not set");
        }

        let start = std::time::Instant::now();

        let mut messages = Vec::new();
        if !request.system.is_empty() {
            messages.push(serde_json::json!({"role": "system", "content": &request.system}));
        }
        messages.push(serde_json::json!({"role": "user", "content": &request.prompt}));

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "messages": messages,
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .timeout(std::time::Duration::from_secs(60))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            anyhow::bail!("OpenAI HTTP {}: {}", status, data);
        }

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let token_count = data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(SpokeResponse {
            text,
            provider: "openai".into(),
            model: self.model.clone(),
            token_count,
            latency_ms,
        })
    }
}

// ── Gemini Provider ──────────────────────────────────────────────────────────

/// Google Gemini provider with real HTTP calls.
pub struct GeminiProvider {
    api_key: String,
    model: String,
}

impl GeminiProvider {
    /// Create a new Gemini provider. Reads `GEMINI_API_KEY` from env if not provided.
    pub fn new(api_key: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key: api_key
                .or_else(|| std::env::var("GEMINI_API_KEY").ok())
                .unwrap_or_default(),
            model: model.unwrap_or_else(|| "gemini-2.5-flash-lite".to_string()),
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> String {
        "gemini".into()
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn generate(&self, request: &SpokeRequest) -> anyhow::Result<SpokeResponse> {
        if !self.is_available() {
            anyhow::bail!("GEMINI_API_KEY not set");
        }

        let start = std::time::Instant::now();

        let mut body = serde_json::json!({
            "contents": [{"parts": [{"text": &request.prompt}]}],
            "generationConfig": {
                "temperature": request.temperature,
                "maxOutputTokens": request.max_tokens,
            },
        });
        if !request.system.is_empty() {
            body["systemInstruction"] =
                serde_json::json!({"parts": [{"text": &request.system}]});
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            anyhow::bail!("Gemini HTTP {}: {}", status, data);
        }

        let text = data["candidates"]
            .as_array()
            .and_then(|c| c.first())
            .and_then(|c| c["content"]["parts"].as_array())
            .map(|parts| {
                parts
                    .iter()
                    .filter_map(|p| p["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();

        let token_count = data["usageMetadata"]["candidatesTokenCount"]
            .as_u64()
            .unwrap_or(0) as u32;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(SpokeResponse {
            text,
            provider: "gemini".into(),
            model: self.model.clone(),
            token_count,
            latency_ms,
        })
    }
}

// ── DeepSeek Provider ────────────────────────────────────────────────────────

/// DeepSeek provider with real HTTP calls (OpenAI-compatible API).
pub struct DeepSeekProvider {
    api_key: String,
    model: String,
}

impl DeepSeekProvider {
    /// Create a new DeepSeek provider. Reads `DEEPSEEK_API_KEY` from env if not provided.
    pub fn new(api_key: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key: api_key
                .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok())
                .unwrap_or_default(),
            model: model.unwrap_or_else(|| "deepseek-chat".to_string()),
        }
    }
}

#[async_trait]
impl Provider for DeepSeekProvider {
    fn name(&self) -> String {
        "deepseek".into()
    }

    fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn generate(&self, request: &SpokeRequest) -> anyhow::Result<SpokeResponse> {
        if !self.is_available() {
            anyhow::bail!("DEEPSEEK_API_KEY not set");
        }

        let start = std::time::Instant::now();

        let mut messages = Vec::new();
        if !request.system.is_empty() {
            messages.push(serde_json::json!({"role": "system", "content": &request.system}));
        }
        messages.push(serde_json::json!({"role": "user", "content": &request.prompt}));

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "messages": messages,
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.deepseek.com/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .timeout(std::time::Duration::from_secs(60))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            anyhow::bail!("DeepSeek HTTP {}: {}", status, data);
        }

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let token_count = data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(SpokeResponse {
            text,
            provider: "deepseek".into(),
            model: self.model.clone(),
            token_count,
            latency_ms,
        })
    }
}

// ── Spoke Registry ───────────────────────────────────────────────────────────

/// Registry of all available provider spokes.
pub struct SpokeRegistry {
    /// Registered providers.
    pub providers: HashMap<String, Arc<dyn Provider>>,
    /// Ordered preference list.
    preference: Vec<String>,
}

impl SpokeRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            preference: Vec::new(),
        }
    }

    /// Create a registry pre-loaded with all production providers (from env vars).
    pub fn from_env() -> Self {
        let mut reg = Self::new();
        reg.register(Arc::new(AnthropicProvider::new(None, None)));
        reg.register(Arc::new(OpenAIProvider::new(None, None)));
        reg.register(Arc::new(GeminiProvider::new(None, None)));
        reg.register(Arc::new(DeepSeekProvider::new(None, None)));
        reg.preference = vec![
            "anthropic".into(),
            "openai".into(),
            "gemini".into(),
            "deepseek".into(),
        ];
        reg
    }

    /// Register a provider.
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        let name = provider.name();
        if !self.preference.contains(&name) {
            self.preference.push(name.clone());
        }
        self.providers.insert(name, provider);
    }

    /// Set preference order.
    pub fn set_preference(&mut self, order: Vec<String>) {
        self.preference = order;
    }

    /// List available providers.
    pub fn available(&self) -> Vec<String> {
        self.providers
            .iter()
            .filter(|(_, p)| p.is_available())
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// Route a request through the provider chain with ordered fallback.
    pub async fn route(
        &self,
        request: &SpokeRequest,
        preferred: Option<&str>,
    ) -> anyhow::Result<SpokeResponse> {
        let mut order = self.preference.clone();
        if let Some(pref) = preferred {
            let pref_str = pref.to_string();
            order.retain(|n| n != &pref_str);
            order.insert(0, pref_str);
        }

        let mut errors = Vec::new();
        for name in &order {
            let provider = match self.providers.get(name) {
                Some(p) if p.is_available() => p,
                _ => {
                    errors.push(format!("{}: not available", name));
                    continue;
                }
            };

            match provider.generate(request).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    errors.push(format!("{}: {}", name, e));
                    continue;
                }
            }
        }

        anyhow::bail!("All providers failed: {}", errors.join("; "))
    }

    /// Get primary provider (first available in preference order).
    pub fn primary(&self) -> Option<&dyn Provider> {
        for name in &self.preference {
            if let Some(p) = self.providers.get(name) {
                if p.is_available() {
                    return Some(p.as_ref());
                }
            }
        }
        None
    }

    /// Get providers with a specific capability.
    pub fn spokes_with_capability(&self, _cap: SpokeCapability) -> Vec<String> {
        // All current providers support TextGeneration
        self.available()
    }

    /// Find a tool provider for a given capability.
    pub fn find_tool_provider(&self, _cap: SpokeCapability) -> Option<&dyn Provider> {
        self.primary()
    }
}

impl Default for SpokeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_unavailable_without_key() {
        let p = AnthropicProvider::new(Some(String::new()), None);
        assert!(!p.is_available());
    }

    #[test]
    fn test_anthropic_available_with_key() {
        let p = AnthropicProvider::new(Some("sk-test-key".to_string()), None);
        assert!(p.is_available());
        assert_eq!(p.name(), "anthropic");
    }

    #[test]
    fn test_registry_from_env_creates_all_providers() {
        let reg = SpokeRegistry::from_env();
        assert!(reg.providers.contains_key("anthropic"));
        assert!(reg.providers.contains_key("openai"));
        assert!(reg.providers.contains_key("gemini"));
        assert!(reg.providers.contains_key("deepseek"));
    }

    #[test]
    fn test_empty_registry_has_no_available() {
        let reg = SpokeRegistry::new();
        assert!(reg.available().is_empty());
    }

    #[test]
    fn test_registry_preference_order() {
        let mut reg = SpokeRegistry::new();
        reg.register(Arc::new(AnthropicProvider::new(
            Some("key".into()),
            None,
        )));
        reg.register(Arc::new(OpenAIProvider::new(Some("key".into()), None)));
        reg.set_preference(vec!["openai".into(), "anthropic".into()]);
        assert_eq!(reg.preference[0], "openai");
    }
}

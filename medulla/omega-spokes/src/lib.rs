//! omega-spokes: Sovereign HUB Provider Spoke System.
//!
//! Orchestrates model-specific connectors (spokes) for inference, tools, and memory.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Capabilities supported by individual spokes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpokeCapability {
    /// Text generation / Chat completion.
    Inference,
    /// Ability to generative semantic vector embeddings.
    Embeddings,
    /// Persistent storage interaction (e.g. Neon, Vector DB).
    Storage,
    /// Internet search integration.
    Search,
    /// General tool execution capability.
    Tools,
    /// Sensitive operations requiring explicit alignment check.
    Sensitive,
}

/// Dynamic configuration for a spoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeConfig {
    /// Spoke identifier (e.g. "openai", "search-internal").
    pub name: String,
    /// Resource URL if applicable.
    pub endpoint: Option<String>,
    /// Preference weight for routing.
    pub priority: u32,
}

/// Definition of a tool exposed by a spoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub is_deterministic: bool,
    pub estimated_cost: u32,
}

/// Request to invoke a specific tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    pub tool: String,
    pub input: Value,
}

/// Result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: Value,
    pub error: Option<String>,
    pub execution_time_ms: u32,
}

/// Live health status of a spoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeStatus {
    pub name: String,
    pub health: String,
    pub last_success: f64,
    pub recent_errors: u32,
    pub available_tools: usize,
}

/// The Spoke trait: every connector must implement this.
#[async_trait]
pub trait Spoke: Send + Sync {
    /// Unique spoke name.
    fn name(&self) -> &str;
    /// Category of spoke.
    fn spoke_type(&self) -> &str;
    /// Supported capabilities.
    fn capabilities(&self) -> Vec<SpokeCapability>;
    /// Discover available tools.
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String>;
    /// Invoke a tool by name.
    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String>;
    /// Invoke a tool and stream the result chunks.
    async fn invoke_tool_stream(
        &self,
        _invocation: ToolInvocation,
        _tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        Err(format!(
            "Streaming not implemented for spoke: {}",
            self.name()
        ))
    }
    /// Check the health of the connection.
    async fn health_check(&self) -> Result<SpokeStatus, String>;
    /// Get the current config.
    fn config(&self) -> &SpokeConfig;
}

/// Legacy SpokeRequest (for compatibility with existing conductors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeRequest {
    pub prompt: String,
    pub system: String,
    pub max_tokens: usize,
    pub temperature: f64,
}

/// Legacy SpokeResponse (for compatibility with existing conductors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeResponse {
    pub text: String,
    pub provider: String,
    pub model: String,
    pub token_count: u32,
    pub latency_ms: f64,
}

/// Implementation modules for each spoke type.
pub mod spokes;

/// SpokeRegistry: central hub for managing and routing requests across spokes.
pub struct SpokeRegistry {
    spokes: HashMap<String, Arc<dyn Spoke>>,
    preference: Vec<String>,
}

impl SpokeRegistry {
    pub fn new() -> Self {
        Self {
            spokes: HashMap::new(),
            preference: Vec::new(),
        }
    }

    /// List all registered spoke names.
    pub fn list_spokes(&self) -> Vec<String> {
        self.spokes.keys().cloned().collect()
    }

    /// Load default spokes from environment configuration.
    pub fn from_env() -> Self {
        let mut reg = Self::new();

        let providers = vec![
            ("groq", 5),
            ("anthropic", 10),
            ("openai", 20),
            ("gemini", 30),
            ("deepseek", 40),
            ("perplexity", 45),
            ("ollama", 50),
            ("search", 90),
            ("neon", 100),
            ("sovereign", 0),
            // Explicit MCP bridges
            ("github", 200),
            ("vercel", 201),
            ("supabase", 202),
            ("firebase", 203),
            ("zapier", 204),
            ("manus", 205),
            ("filesystem", 206),
            ("openrouter", 15),
            ("vision", 150),
        ];

        for (p, priority) in providers {
            let config = SpokeConfig {
                name: p.to_string(),
                endpoint: None,
                priority,
            };

            let spoke: Option<Arc<dyn Spoke>> = match p {
                "groq" => Some(Arc::new(spokes::GroqSpoke::new(config))),
                "anthropic" => Some(Arc::new(spokes::AnthropicSpoke::new(config))),
                "openai" => Some(Arc::new(spokes::OpenAISpoke::new(config))),
                "gemini" => Some(Arc::new(spokes::GeminiSpoke::new(config))),
                "deepseek" => Some(Arc::new(spokes::DeepSeekSpoke::new(config))),
                "perplexity" => Some(Arc::new(spokes::PerplexitySpoke::new(config))),
                "ollama" => Some(Arc::new(spokes::OllamaSpoke::new(config))),
                "search" => Some(Arc::new(spokes::SearchSpoke::new(config))),
                "neon" => Some(Arc::new(spokes::NeonSpoke::new(config))),
                "sovereign" => Some(Arc::new(spokes::DeepSeekSpoke::new(config))),
                
                // MCP Hub Initializations 
                "github" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-github"]))),
                "vercel" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-vercel"]))),
                "supabase" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-supabase"]))),
                "firebase" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-firebase"]))),
                "zapier" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-zapier"]))),
                "manus" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-manus"]))),
                "filesystem" => Some(Arc::new(spokes::MCPSpoke::new(config, "npx", vec!["-y", "@modelcontextprotocol/server-filesystem", "/home/mega/Chyren"]))),
                "openrouter" => Some(Arc::new(spokes::OpenRouterSpoke::new(config))),
                "vision" => Some(Arc::new(spokes::VisionSpoke::new(config))),
                _ => None,
            };

            if let Some(s) = spoke {
                reg.register(s);
            }
        }

        reg.preference = vec![
            "openai".into(),
            "groq".into(),
            "anthropic".into(),
            "gemini".into(),
            "deepseek".into(),
            "perplexity".into(),
            "ollama".into(),
        ];

        reg
    }

    pub fn register(&mut self, spoke: Arc<dyn Spoke>) {
        self.spokes.insert(spoke.name().to_string(), spoke);
    }

    pub fn primary(&self) -> Option<Arc<dyn Spoke>> {
        self.preference
            .first()
            .and_then(|name| self.spokes.get(name).cloned())
    }

    pub fn spokes_with_capability(&self, capability: SpokeCapability) -> Vec<Arc<dyn Spoke>> {
        self.spokes
            .values()
            .filter(|s| s.capabilities().contains(&capability))
            .cloned()
            .collect()
    }

    pub fn get_spoke(&self, name: &str) -> Option<Arc<dyn Spoke>> {
        self.spokes.get(name).cloned()
    }

    /// Discover tools from all registered spokes.
    pub async fn discover_all_tools(&self) -> Vec<ToolDefinition> {
        let mut all_tools = Vec::new();
        for spoke in self.spokes.values() {
            if let Ok(mut tools) = spoke.discover_tools().await {
                all_tools.append(&mut tools);
            }
        }
        all_tools
    }

    /// Route a chat request and stream chunks via a channel (Phase 2).
    pub async fn route_stream(
        &self,
        request: &SpokeRequest,
        preferred: Option<&str>,
        tx: mpsc::Sender<Value>,
    ) -> anyhow::Result<()> {
        let name = preferred
            .map(|n| n.to_string())
            .unwrap_or_else(|| self.preference.first().cloned().unwrap_or_default());

        let spoke = self
            .spokes
            .get(&name)
            .ok_or_else(|| anyhow::anyhow!("Spoke {} not found", name))?;

        spoke
            .invoke_tool_stream(
                ToolInvocation {
                    tool: "chat_completion".to_string(),
                    input: serde_json::json!({
                        "prompt": request.prompt,
                        "system": request.system,
                        "max_tokens": request.max_tokens,
                        "temperature": request.temperature,
                        "stream": true,
                    }),
                },
                tx,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Spoke streaming error: {}", e))
    }

    /// Route a chat request via the preferred spoke (Legacy-Compat).
    pub async fn route(
        &self,
        request: &SpokeRequest,
        preferred: Option<&str>,
    ) -> anyhow::Result<SpokeResponse> {
        self.route_with_model(request, preferred, None).await
    }

    /// Route a chat request with an optional model override injected into the spoke input.
    ///
    /// Spokes that support a `"model"` field in their `chat_completion` input (e.g.
    /// OpenRouterSpoke, OllamaSpoke) will use `model_hint` instead of their default.
    /// Spokes that ignore it continue working normally.
    /// 
    /// Now with automatic failover: if the preferred/first spoke fails, it tries
    /// the others in the preference list.
    pub async fn route_with_model(
        &self,
        request: &SpokeRequest,
        preferred: Option<&str>,
        model_hint: Option<&str>,
    ) -> anyhow::Result<SpokeResponse> {
        let mut candidates = Vec::new();
        if let Some(p) = preferred {
            candidates.push(p.to_string());
        }
        for p in &self.preference {
            if !candidates.contains(p) {
                candidates.push(p.clone());
            }
        }

        let mut last_error = anyhow::anyhow!("No spokes available");

        for name in candidates {
            let spoke = match self.spokes.get(&name) {
                Some(s) => s,
                None => continue,
            };

            let mut input = serde_json::json!({
                "prompt": request.prompt,
                "system": request.system,
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
            });
            if let Some(model) = model_hint {
                input["model"] = serde_json::Value::String(model.to_string());
            }

            // Translate SpokeRequest to a ToolInvocation of "chat_completion".
            let start = std::time::Instant::now();
            match spoke.invoke_tool(ToolInvocation {
                tool: "chat_completion".to_string(),
                input,
            }).await {
                Ok(result) if result.success => {
                    let text = result
                        .output
                        .get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|o| o.get("message"))
                        .and_then(|m| m.get("content"))
                        .and_then(|ct| ct.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| {
                            result
                                .output
                                .get("content")
                                .and_then(|c| c.get(0))
                                .and_then(|o| o.get("text"))
                                .and_then(|t| t.as_str())
                                .map(|s| s.to_string())
                        })
                        .or_else(|| {
                            result
                                .output
                                .get("candidates")
                                .and_then(|c| c.get(0))
                                .and_then(|o| o.get("content"))
                                .and_then(|ct| ct.get("parts"))
                                .and_then(|p| p.get(0))
                                .and_then(|pr| pr.get("text"))
                                .and_then(|txt| txt.as_str())
                                .map(|s| s.to_string())
                        })
                        .unwrap_or_default();

                    return Ok(SpokeResponse {
                        text,
                        provider: name,
                        model: model_hint.unwrap_or("auto").to_string(),
                        token_count: 0,
                        latency_ms: start.elapsed().as_millis() as f64,
                    });
                }
                Ok(result) => {
                    last_error = anyhow::anyhow!("Spoke {} execution failed: {:?}", name, result.error);
                }
                Err(e) => {
                    last_error = anyhow::anyhow!("Spoke {} invocation error: {}", name, e);
                }
            }
            
            tracing::warn!("[SPOKE_REGISTRY] Spoke {} failed, trying next candidate. Error: {}", name, last_error);
        }

        Err(last_error)
    }
}

impl Default for SpokeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

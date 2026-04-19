use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Production MCP spoke: calls a remote MCP HTTP/JSON-RPC 2.0 server using
/// a persistent reqwest client (connection pool, keep-alive, rustls).
/// Works with the internal librarian at `/api/mcp/librarian` or any external
/// MCP server that speaks Streamable HTTP transport.
#[derive(Clone)]
pub struct MCPSpoke {
    config: SpokeConfig,
    /// Full URL of the MCP HTTP endpoint, e.g. "https://chyren.vercel.app/api/mcp/librarian"
    endpoint: String,
    /// Optional bearer token / API key sent as Authorization header.
    api_key: Option<String>,
    /// Shared HTTP client — one per spoke instance, reused across calls.
    client: Client,
    /// Cached tool list (populated on first discover_tools call).
    tool_cache: Arc<RwLock<Option<Vec<ToolDefinition>>>>,
}

impl MCPSpoke {
    pub fn new(config: SpokeConfig, endpoint: impl Into<String>, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(4)
            .build()
            .expect("Failed to build reqwest client for MCPSpoke");

        Self {
            config,
            endpoint: endpoint.into(),
            api_key,
            client,
            tool_cache: Arc::new(RwLock::new(None)),
        }
    }

    async fn rpc(&self, method: &str, params: Value) -> Result<Value, String> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": next_id(),
            "method": method,
            "params": params,
        });

        let mut req = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let resp = req
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("MCP HTTP request failed: {}", e))?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("Failed to read MCP response body: {}", e))?;

        if !status.is_success() {
            return Err(format!("MCP server returned HTTP {}: {}", status, text));
        }

        let json: Value = serde_json::from_str(&text)
            .map_err(|e| format!("MCP response is not valid JSON: {} — body: {}", e, &text[..text.len().min(200)]))?;

        if let Some(err) = json.get("error") {
            let code = err.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
            let msg = err.get("message").and_then(|m| m.as_str()).unwrap_or("unknown error");
            return Err(format!("MCP RPC error {}: {}", code, msg));
        }

        Ok(json.get("result").cloned().unwrap_or(Value::Null))
    }

    fn strip_prefix<'a>(&self, tool_name: &'a str) -> &'a str {
        let prefix = format!("{}_", self.config.name);
        tool_name.strip_prefix(&prefix).unwrap_or(tool_name)
    }
}

#[async_trait]
impl Spoke for MCPSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "mcp_http"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Tools]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        // Return cached result if available.
        {
            let cache = self.tool_cache.read().await;
            if let Some(ref tools) = *cache {
                return Ok(tools.clone());
            }
        }

        let result = self.rpc("tools/list", json!({})).await?;
        let tools_array = result
            .get("tools")
            .and_then(|t| t.as_array())
            .ok_or_else(|| format!("MCP tools/list response missing 'tools' array: {:?}", result))?;

        let definitions: Vec<ToolDefinition> = tools_array
            .iter()
            .map(|tool| {
                let raw_name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                ToolDefinition {
                    // Namespace: "<spoke_name>_<tool_name>" so the conductor can route correctly.
                    name: format!("{}_{}", self.config.name, raw_name),
                    description: tool
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("")
                        .to_string(),
                    input_schema: tool.get("inputSchema").cloned().unwrap_or(json!({"type": "object"})),
                    is_deterministic: false,
                    estimated_cost: 30,
                }
            })
            .collect();

        let mut cache = self.tool_cache.write().await;
        *cache = Some(definitions.clone());

        Ok(definitions)
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        let raw_name = self.strip_prefix(&invocation.tool);

        let result = self.rpc("tools/call", json!({
            "name": raw_name,
            "arguments": invocation.input,
        })).await;

        let elapsed = start.elapsed().as_millis() as u32;

        match result {
            Ok(payload) => {
                let is_error = payload
                    .get("isError")
                    .and_then(|b| b.as_bool())
                    .unwrap_or(false);
                Ok(ToolResult {
                    success: !is_error,
                    output: payload,
                    error: None,
                    execution_time_ms: elapsed,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: json!({}),
                error: Some(e),
                execution_time_ms: elapsed,
            }),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        let result = self.rpc("ping", json!({})).await;
        let healthy = result.is_ok();

        // Invalidate tool cache on failure so next discover_tools re-fetches.
        if !healthy {
            let mut cache = self.tool_cache.write().await;
            *cache = None;
        }

        let tool_count = {
            let cache = self.tool_cache.read().await;
            cache.as_ref().map(|t| t.len()).unwrap_or(0)
        };

        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: if healthy { "healthy" } else { "unreachable" }.to_string(),
            last_success: crate::now(),
            recent_errors: if healthy { 0 } else { 1 },
            available_tools: tool_count,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

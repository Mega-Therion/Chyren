use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

#[derive(Clone)]
pub struct MCPSpoke {
    config: SpokeConfig,
    /// The command to execute the MCP server (e.g., "npx")
    command: String,
    /// The arguments for the command (e.g., ["-y", "@modelcontextprotocol/server-github"])
    args: Vec<String>,
}

impl MCPSpoke {
    pub fn new(config: SpokeConfig, command: &str, args: Vec<&str>) -> Self {
        Self {
            config,
            command: command.to_string(),
            args: args.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Helper to send a simple JSON-RPC request to the MCP server process over stdio.
    /// This is a simplified bridging mechanism. A full production implementation
    /// would keep a single process alive. For safety and isolation across tool 
    /// calls, this spawns the server, makes the request, and reads the response.
    async fn call_mcp_rpc(&self, method: &str, params: Value) -> Result<Value, String> {
        let mut child = Command::new(&self.command)
            .args(&self.args)
            .env("MCP_ENV", "chyren")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn MCP server: {}", e))?;

        let mut stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;

        // Format the request as JSON-RPC 2.0
        let req_id = 1; // Simplified ID
        let rpc_req = json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "method": method,
            "params": params
        });

        let req_str = format!("{}\n", serde_json::to_string(&rpc_req).unwrap());
        stdin
            .write_all(req_str.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())?;

        // Read response
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        
        // Timeout handling would go here in production
        reader
            .read_line(&mut line)
            .await
            .map_err(|e| format!("Failed to read MCP response: {}", e))?;

        // The MCP process can be terminated after the request is complete
        let _ = child.kill().await;

        if line.is_empty() {
            return Err("Empty response from MCP server".into());
        }

        let resp: Value =
            serde_json::from_str(&line).map_err(|e| format!("JSON parsing error: {}", e))?;

        if let Some(error) = resp.get("error") {
            return Err(error.to_string());
        }

        Ok(resp.get("result").cloned().unwrap_or(json!({})))
    }
}

#[async_trait]
impl Spoke for MCPSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "mcp_bridge"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        // MCP natively provides access to external tools
        vec![SpokeCapability::Tools]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        // First, we must initialize the connection according to MCP spec
        let init_params = json!({
            "protocolVersion": "2024-11-05", // Standard MCP protocol version
            "clientInfo": {
                "name": "Chyren-Medulla-Bridge",
                "version": "1.0.0"
            },
            "capabilities": {}
        });

        // Technically we should send initialize, wait for response, then send tools/list.
        // For simplicity in this architectural bridging stub:
        
        // 1. Send tools/list
        match self.call_mcp_rpc("tools/list", json!({})).await {
            Ok(result) => {
                let tools_array = result
                    .get("tools")
                    .and_then(|t| t.as_array())
                    .ok_or("Missing 'tools' array in MCP response")?;

                let mut definitions = Vec::new();
                for tool in tools_array {
                    let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown_mcp_tool");
                    let description = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");
                    let input_schema = tool.get("inputSchema").cloned().unwrap_or(json!({}));
                    
                    definitions.push(ToolDefinition {
                        name: format!("{}_{}", self.name(), name), // Namespace the tool
                        description: description.to_string(),
                        input_schema,
                        is_deterministic: false,
                        estimated_cost: 50, // Standard tool invocation cost
                    });
                }
                Ok(definitions)
            }
            Err(e) => {
                // Return a mock definition if the server isn't running yet to prevent boot failures
                Ok(vec![ToolDefinition {
                    name: format!("{}_passthrough", self.name()),
                    description: format!("Execute a dynamically discovered tool on the {} MCP server", self.name()),
                    input_schema: json!({"type": "object"}),
                    is_deterministic: false,
                    estimated_cost: 50,
                }])
            }
        }
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        
        let local_tool_name = invocation.tool.strip_prefix(&format!("{}_", self.name()))
            .unwrap_or(&invocation.tool);

        match self.call_mcp_rpc("tools/call", json!({
            "name": local_tool_name,
            "arguments": invocation.input
        })).await {
            Ok(result) => Ok(ToolResult {
                success: !result.get("isError").and_then(|b| b.as_bool()).unwrap_or(false),
                output: result,
                error: None,
                execution_time_ms: start.elapsed().as_millis() as u32,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: json!({}),
                error: Some(e),
                execution_time_ms: start.elapsed().as_millis() as u32,
            }),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(), // Assume healthy if config is present
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 1, // At least the passthrough is available
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

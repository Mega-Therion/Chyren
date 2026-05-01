//! ClaudeCodeSpoke — Chyren → Claude Code CLI bridge.
//!
//! Invokes `claude -p` as a subprocess, injecting the Chyren sovereign identity
//! as an appended system prompt. Enables Chyren to delegate tasks to Claude Code
//! and receive structured responses back through the conductor pipeline.
//!
//! The response passes through ADCCL scoring like any other spoke output.
//!
//! Required: `claude` binary on PATH (installed via `claude install stable`).
//! Optional env vars:
//!   `CLAUDE_BIN`        — path to claude binary (default: `claude`)
//!   `CLAUDE_CODE_MODEL` — model override (default: `claude-sonnet-4-6`)

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::process::Command;

const CHYREN_IDENTITY_PROMPT: &str =
    "You are operating as a subagent within the Chyren Sovereign Intelligence OS. \
     The task you receive has been routed through the Chyren conductor pipeline. \
     Respond with precision and completeness. Your output will be scored by the \
     ADCCL drift-detection gate (threshold 0.7) before being committed to the \
     Master Ledger. Do not use placeholder text, refuse capability, or produce \
     stub responses — they will be rejected by the gate.";

pub struct ClaudeCodeSpoke {
    config: SpokeConfig,
}

impl ClaudeCodeSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }

    fn claude_bin() -> String {
        std::env::var("CLAUDE_BIN").unwrap_or_else(|_| "claude".to_string())
    }

    async fn invoke_claude(
        &self,
        prompt: &str,
        system_extra: Option<&str>,
    ) -> Result<(Value, u32), String> {
        let start = std::time::Instant::now();
        let model = std::env::var("CLAUDE_CODE_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-6".to_string());

        let mut append_prompt = CHYREN_IDENTITY_PROMPT.to_string();
        if let Some(extra) = system_extra {
            append_prompt.push('\n');
            append_prompt.push_str(extra);
        }

        let output = Command::new(Self::claude_bin())
            .args([
                "-p",
                prompt,
                "--model",
                &model,
                "--output-format",
                "json",
                "--append-system-prompt",
                &append_prompt,
                "--max-turns",
                "5",
                "--no-session-persistence",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|e| format!("Failed to spawn claude binary: {e}. Is `claude` on PATH?"))?;

        let elapsed_ms = start.elapsed().as_millis() as u32;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "claude exited with status {}: {}",
                output.status,
                stderr.trim()
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // claude --output-format json → {"type":"result","subtype":"success","result":"..."}
        if let Ok(parsed) = serde_json::from_str::<Value>(&stdout) {
            if let Some(text) = parsed.get("result").and_then(|r| r.as_str()) {
                return Ok((json!({ "response": text }), elapsed_ms));
            }
            if let Some(text) = parsed.get("content").and_then(|c| c.as_str()) {
                return Ok((json!({ "response": text }), elapsed_ms));
            }
            // Return full parsed JSON if shape is unexpected
            return Ok((parsed, elapsed_ms));
        }

        // Fallback: raw text
        Ok((
            json!({ "response": stdout.trim() }),
            elapsed_ms,
        ))
    }
}

#[async_trait]
impl Spoke for ClaudeCodeSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "claude-code-cli"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference, SpokeCapability::Tools]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "claude_code_complete".to_string(),
                description: "Delegate a task to the Claude Code CLI. Returns the full \
                              agentic response (up to 5 turns). Use for complex engineering \
                              tasks that benefit from Claude Code's tool access."
                    .to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "prompt": {
                            "type": "string",
                            "description": "The task or query to send to Claude Code"
                        },
                        "system_context": {
                            "type": "string",
                            "description": "Optional additional system context to append"
                        }
                    },
                    "required": ["prompt"]
                }),
                is_deterministic: false,
                estimated_cost: 50,
            },
            ToolDefinition {
                name: "claude_code_review".to_string(),
                description: "Ask Claude Code to review code or a diff. \
                              Optimised for correctness, security, and style analysis."
                    .to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "content": {
                            "type": "string",
                            "description": "Code or diff to review"
                        },
                        "focus": {
                            "type": "string",
                            "description": "Review focus: security, correctness, performance, style"
                        }
                    },
                    "required": ["content"]
                }),
                is_deterministic: false,
                estimated_cost: 40,
            },
        ])
    }

    async fn invoke_tool(&self, inv: ToolInvocation) -> Result<ToolResult, String> {
        match inv.tool.as_str() {
            "claude_code_complete" => {
                let prompt = inv
                    .input
                    .get("prompt")
                    .and_then(|p| p.as_str())
                    .ok_or("Missing required field: prompt")?;
                let system_ctx = inv.input.get("system_context").and_then(|s| s.as_str());

                match self.invoke_claude(prompt, system_ctx).await {
                    Ok((output, elapsed_ms)) => Ok(ToolResult {
                        success: true,
                        output,
                        error: None,
                        execution_time_ms: elapsed_ms,
                    }),
                    Err(e) => Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: 0,
                    }),
                }
            }
            "claude_code_review" => {
                let content = inv
                    .input
                    .get("content")
                    .and_then(|c| c.as_str())
                    .ok_or("Missing required field: content")?;
                let focus = inv
                    .input
                    .get("focus")
                    .and_then(|f| f.as_str())
                    .unwrap_or("correctness, security, and style");

                let prompt = format!(
                    "Review the following code for {focus}. Be specific:\n\n```\n{content}\n```"
                );

                match self.invoke_claude(&prompt, None).await {
                    Ok((output, elapsed_ms)) => Ok(ToolResult {
                        success: true,
                        output,
                        error: None,
                        execution_time_ms: elapsed_ms,
                    }),
                    Err(e) => Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: 0,
                    }),
                }
            }
            other => Ok(ToolResult {
                success: false,
                output: json!({}),
                error: Some(format!("ClaudeCodeSpoke: unknown tool '{other}'")),
                execution_time_ms: 0,
            }),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        let output = Command::new(Self::claude_bin())
            .args(["--version"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("claude binary not found: {e}"))?;

        if output.status.success() {
            Ok(SpokeStatus {
                name: self.config.name.clone(),
                health: format!(
                    "claude CLI ready: {}",
                    String::from_utf8_lossy(&output.stdout).trim()
                ),
                last_success: 0.0,
                recent_errors: 0,
                available_tools: 2,
            })
        } else {
            Err(format!(
                "claude binary unhealthy: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_spoke() -> ClaudeCodeSpoke {
        ClaudeCodeSpoke::new(SpokeConfig {
            name: "claude-code".to_string(),
            endpoint: None,
            priority: 10,
        })
    }

    #[test]
    fn discover_tools_returns_two_tools() {
        let spoke = test_spoke();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let tools = rt.block_on(spoke.discover_tools()).unwrap();
        assert_eq!(tools.len(), 2);
        assert!(tools.iter().any(|t| t.name == "claude_code_complete"));
        assert!(tools.iter().any(|t| t.name == "claude_code_review"));
    }

    #[test]
    fn unknown_tool_returns_error_result() {
        let spoke = test_spoke();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt
            .block_on(spoke.invoke_tool(ToolInvocation {
                tool: "nonexistent".to_string(),
                input: json!({}),
            }))
            .unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }
}

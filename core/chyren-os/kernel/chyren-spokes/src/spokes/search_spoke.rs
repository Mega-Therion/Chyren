//! Search spoke for web and external data access

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

/// Search spoke for web and API access
pub struct SearchSpoke {
    config: SpokeConfig,
}

impl SearchSpoke {
    /// Create new Search spoke
    pub fn new(config: SpokeConfig) -> Self {
        SearchSpoke { config }
    }
}

#[async_trait]
impl Spoke for SearchSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "search"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![
            SpokeCapability::Inference,
            SpokeCapability::Tools,
            SpokeCapability::Inference,
        ]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "web_search".to_string(),
                description: "Search the web for current information".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"},
                        "num_results": {"type": "integer", "description": "Number of results"}
                    },
                    "required": ["query"]
                }),
                is_deterministic: false,
                estimated_cost: 200,
            },
            ToolDefinition {
                name: "fetch_url".to_string(),
                description: "Fetch and extract content from a URL".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": {"type": "string", "description": "URL to fetch"},
                        "extract": {"type": "string", "description": "Extraction method"}
                    },
                    "required": ["url"]
                }),
                is_deterministic: false,
                estimated_cost: 150,
            },
            ToolDefinition {
                name: "api_call".to_string(),
                description: "Call external REST API".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "endpoint": {"type": "string"},
                        "method": {"type": "string"},
                        "params": {"type": "object"}
                    },
                    "required": ["endpoint", "method"]
                }),
                is_deterministic: false,
                estimated_cost: 250,
            },
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();

        let result = match invocation.tool.as_str() {
            "web_search" => match self.web_search(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            "fetch_url" => match self.fetch_url(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            "api_call" => match self.api_call(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            _ => {
                return Ok(ToolResult {
                    success: false,
                    output: json!({}),
                    error: Some(format!("Unknown tool: {}", invocation.tool)),
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            }
        };

        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u32,
        })
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 3,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

impl SearchSpoke {
    /// Perform web search via external search API
    async fn web_search(&self, input: &Value) -> Result<Value, String> {
        let query = input
            .get("query")
            .and_then(|q| q.as_str())
            .ok_or("Missing 'query' in input")?;

        let num_results = input
            .get("num_results")
            .and_then(|n| n.as_u64())
            .unwrap_or(10) as usize;

        // Check if search API key is configured
        let search_api_key = env::var("SEARCH_API_KEY").ok();

        if search_api_key.is_none() {
            // Return mock results if no API key configured
            return Ok(json!({
                "results": [
                    {
                        "title": format!("Search result for '{}'", query),
                        "url": "https://example.com/result",
                        "snippet": "No search API key configured. Running in mock mode."
                    }
                ],
                "total": 1,
                "note": "Using mock results - configure SEARCH_API_KEY for real search"
            }));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        // Example: Using a generic search endpoint (could be Google Custom Search, Bing, etc.)
        let search_url = format!(
            "https://api.search.brave.com/res/v1/web/search?q={}&count={}",
            urlencoding::encode(query),
            num_results
        );

        let response = client
            .get(&search_url)
            .header(
                "Authorization",
                format!("Token {}", search_api_key.unwrap()),
            )
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Search request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Search API error: {}", error_text));
        }

        let search_response: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse search response: {}", e))?;

        Ok(search_response)
    }

    /// Fetch content from a URL
    async fn fetch_url(&self, input: &Value) -> Result<Value, String> {
        let url = input
            .get("url")
            .and_then(|u| u.as_str())
            .ok_or("Missing 'url' in input")?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        let response = client
            .get(url)
            .header("User-Agent", "Chyren/1.0")
            .send()
            .await
            .map_err(|e| format!("URL fetch failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let content = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        // Simple HTML stripping (extracts text content)
        let cleaned = strip_html_tags(&content);

        Ok(json!({
            "url": url,
            "status": 200,
            "content": cleaned,
            "length": cleaned.len()
        }))
    }

    /// Make arbitrary REST API calls
    async fn api_call(&self, input: &Value) -> Result<Value, String> {
        let endpoint = input
            .get("endpoint")
            .and_then(|e| e.as_str())
            .ok_or("Missing 'endpoint' in input")?;

        let method = input
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("GET")
            .to_uppercase();

        let params = input.get("params").and_then(|p| p.as_object()).cloned();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        let response = match method.as_str() {
            "GET" => {
                client
                    .get(endpoint)
                    .header("Accept", "application/json")
                    .send()
                    .await
            }
            "POST" => {
                let mut req = client
                    .post(endpoint)
                    .header("Content-Type", "application/json");

                if let Some(p) = params {
                    req = req.json(&p);
                }

                req.send().await
            }
            "PUT" => {
                let mut req = client
                    .put(endpoint)
                    .header("Content-Type", "application/json");

                if let Some(p) = params {
                    req = req.json(&p);
                }

                req.send().await
            }
            "DELETE" => client.delete(endpoint).send().await,
            _ => {
                return Err(format!("Unsupported HTTP method: {}", method));
            }
        };

        match response {
            Ok(res) => {
                let status = res.status().as_u16();
                let body: Value = res
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse API response: {}", e))?;

                Ok(json!({
                    "status": status,
                    "data": body
                }))
            }
            Err(e) => Err(format!("API call failed: {}", e)),
        }
    }
}

/// Simple HTML tag stripper for content extraction
fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
            // Check for script/style tags
            if html[html.len().saturating_sub(10)..].starts_with("<script") {
                in_script = true;
            } else if html[html.len().saturating_sub(10)..].starts_with("<style") {
                in_style = true;
            }
        } else if ch == '>' {
            in_tag = false;
            if html.ends_with("</script>") {
                in_script = false;
            } else if html.ends_with("</style>") {
                in_style = false;
            }
        } else if !in_tag && !in_script && !in_style {
            result.push(ch);
        }
    }

    // Clean up whitespace
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

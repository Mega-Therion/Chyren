//! omega-spokes: Provider abstraction layer.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> String;
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}

pub struct AnthropicProvider { pub api_key: String }
#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> String { "anthropic".into() }
    async fn generate(&self, _p: &str) -> anyhow::Result<String> { Ok("Anthropic".into()) }
}

pub struct OpenAIProvider { pub api_key: String }
#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> String { "openai".into() }
    async fn generate(&self, _p: &str) -> anyhow::Result<String> { Ok("OpenAI".into()) }
}

pub struct GeminiProvider { pub api_key: String }
#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> String { "gemini".into() }
    async fn generate(&self, _p: &str) -> anyhow::Result<String> { Ok("Gemini".into()) }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SpokeCapability { TextGeneration, MemoryAccess, ToolExecution }

pub struct SpokeRegistry {
    pub providers: HashMap<String, Box<dyn Provider>>,
}

impl SpokeRegistry {
    pub fn new() -> Self {
        Self { providers: HashMap::new() }
    }
    pub fn find_tool_provider(&self, _cap: SpokeCapability) -> Option<&Box<dyn Provider>> {
        self.providers.values().next()
    }
    pub fn spokes_with_capability(&self, _cap: SpokeCapability) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
    pub fn primary(&self) -> Option<&Box<dyn Provider>> {
        self.providers.values().next()
    }
}

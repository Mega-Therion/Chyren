//! Spoke implementations for various providers and services

pub mod anthropic_spoke;
pub mod deepseek_spoke;
pub mod gemini_spoke;
pub mod groq_spoke;
pub mod mcp_spoke;
pub mod neon_spoke;
pub mod ollama_spoke;
pub mod openai_spoke;
pub mod openrouter_spoke;
pub mod perplexity_spoke;
pub mod search_spoke;
pub mod witness;

pub mod claude_code_spoke;
pub mod mock_spoke;
pub mod universal_spoke;
pub mod vision_spoke;

pub use anthropic_spoke::AnthropicSpoke;
pub use claude_code_spoke::ClaudeCodeSpoke;
pub use deepseek_spoke::DeepSeekSpoke;
pub use gemini_spoke::GeminiSpoke;
pub use groq_spoke::GroqSpoke;
pub use mcp_spoke::MCPSpoke;
pub use mock_spoke::MockSpoke;
pub use neon_spoke::NeonSpoke;
pub use ollama_spoke::OllamaSpoke;
pub use openai_spoke::OpenAISpoke;
pub use openrouter_spoke::OpenRouterSpoke;
pub use perplexity_spoke::PerplexitySpoke;
pub use search_spoke::SearchSpoke;
pub use universal_spoke::UniversalSpoke;
pub use vision_spoke::VisionSpoke;

//! Spoke implementations for various providers and services

pub mod anthropic_spoke;
pub mod gemini_spoke;
pub mod neon_spoke;
pub mod openai_spoke;
pub mod search_spoke;

pub mod deepseek_spoke;

pub use anthropic_spoke::AnthropicSpoke;
pub use deepseek_spoke::DeepSeekSpoke;
pub use gemini_spoke::GeminiSpoke;
pub use neon_spoke::NeonSpoke;
pub use openai_spoke::OpenAISpoke;
pub use search_spoke::SearchSpoke;

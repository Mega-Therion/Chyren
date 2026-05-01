//! chyren-integration: Bridge between engines and providers.

pub mod tool_router;

pub use tool_router::{Tool, ToolRouter};

use chyren_spokes::{SpokeCapability, SpokeRegistry};
use std::sync::Arc;

/// Bridge between the conductor and the spoke registry.
pub struct IntegrationBridge {
    pub spoke_registry: Arc<SpokeRegistry>,
}

impl IntegrationBridge {
    pub fn new(registry: Arc<SpokeRegistry>) -> Self {
        Self {
            spoke_registry: registry,
        }
    }

    /// Return the name of the highest-priority (primary) spoke.
    pub fn get_primary_spoke(&self) -> Option<String> {
        self.spoke_registry.primary().map(|p| p.name().to_string())
    }

    /// Return the names of all spokes that support `capability`.
    pub fn get_capability_spokes(&self, capability: SpokeCapability) -> Vec<String> {
        self.spoke_registry
            .spokes_with_capability(capability)
            .iter()
            .map(|s| s.name().to_string())
            .collect()
    }

    /// Build a `ToolRouter` populated by discovering tools from all tool-capable
    /// spokes in the registry.
    pub async fn build_tool_router(&self) -> ToolRouter {
        ToolRouter::from_registry(&self.spoke_registry).await
    }
}

// ---------------------------------------------------------------------------
// ProviderRouter — keyword-driven provider selection with failover
// ---------------------------------------------------------------------------

/// Keyword routing rules: (keyword, provider) pairs evaluated in order.
/// First match wins.
static KEYWORD_RULES: &[(&str, &str)] = &[
    // Code / engineering tasks -> Anthropic
    ("code", "anthropic"),
    ("rust", "anthropic"),
    ("implement", "anthropic"),
    ("function", "anthropic"),
    ("debug", "anthropic"),
    ("refactor", "anthropic"),
    // Research / analysis tasks -> DeepSeek
    ("research", "deepseek"),
    ("analyze", "deepseek"),
    ("analysis", "deepseek"),
    ("study", "deepseek"),
    ("investigate", "deepseek"),
    // Biography / synthesis tasks -> Gemini
    ("biography", "gemini"),
    ("synthesize", "gemini"),
    ("synthesis", "gemini"),
    ("summarize", "gemini"),
    ("narrative", "gemini"),
];

/// The result of a routing decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingContext {
    /// Name of the selected provider spoke.
    pub provider: String,
    /// Human-readable explanation for the selection.
    pub reasoning: String,
}

/// Priority-ordered provider router with keyword-based dispatch and failover.
#[derive(Debug, Clone)]
pub struct ProviderRouter {
    /// Priority-ordered provider names (index 0 = highest priority).
    priority: Vec<String>,
}

impl ProviderRouter {
    /// Create a new router with an explicit priority order.
    pub fn new(priority: Vec<String>) -> Self {
        Self { priority }
    }

    /// Create a router with the default sovereign priority ordering.
    pub fn default_priority() -> Self {
        Self::new(vec![
            "anthropic".into(),
            "openai".into(),
            "gemini".into(),
            "deepseek".into(),
        ])
    }

    /// Select the best available provider for a task using keyword routing.
    ///
    /// Returns `None` if `available` is empty.
    ///
    /// Algorithm:
    /// 1. Scan `task` (case-insensitive) against `KEYWORD_RULES` in order.
    /// 2. If the matched provider is present in `available`, return it.
    /// 3. If the keyword-matched provider is unavailable, keep scanning rules.
    /// 4. If no keyword matched, return the first `available` entry in priority order.
    /// 5. If none found in priority, return `available[0]` as last resort.
    pub fn route(&self, task: &str, available: &[String]) -> Option<RoutingContext> {
        if available.is_empty() {
            return None;
        }

        let task_lower = task.to_lowercase();

        for (keyword, provider) in KEYWORD_RULES {
            if task_lower.contains(keyword) {
                let p = provider.to_string();
                if available.contains(&p) {
                    return Some(RoutingContext {
                        provider: p.clone(),
                        reasoning: format!("Keyword '{}' matched provider '{}'", keyword, p),
                    });
                }
            }
        }

        // Priority-order fallback.
        for p in &self.priority {
            if available.contains(p) {
                return Some(RoutingContext {
                    provider: p.clone(),
                    reasoning: format!("Fallback: '{}' is first available in priority order", p),
                });
            }
        }

        // Last resort: first element of available.
        Some(RoutingContext {
            provider: available[0].clone(),
            reasoning: format!(
                "Fallback: '{}' is the only available provider",
                available[0]
            ),
        })
    }

    /// Pick the next best provider after `failed` has been exhausted.
    ///
    /// Delegates to `route` on a filtered `available` list.
    /// Returns `None` if no other providers remain.
    pub fn failover(&self, failed: &str, available: &[String]) -> Option<RoutingContext> {
        let remaining: Vec<String> = available
            .iter()
            .filter(|p| p.as_str() != failed)
            .cloned()
            .collect();

        if remaining.is_empty() {
            return None;
        }

        self.route("", &remaining).map(|mut ctx| {
            ctx.reasoning = format!("Failover after '{}' failed: {}", failed, ctx.reasoning);
            ctx
        })
    }
}

impl Default for ProviderRouter {
    fn default() -> Self {
        Self::default_priority()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn router() -> ProviderRouter {
        ProviderRouter::default_priority()
    }

    fn all_providers() -> Vec<String> {
        vec![
            "anthropic".into(),
            "openai".into(),
            "gemini".into(),
            "deepseek".into(),
        ]
    }

    #[test]
    fn routes_code_task_to_anthropic() {
        let ctx = router()
            .route(
                "Please implement a Rust function to parse JSON",
                &all_providers(),
            )
            .expect("should route");
        assert_eq!(ctx.provider, "anthropic");
    }

    #[test]
    fn routes_rust_keyword_to_anthropic() {
        let ctx = router()
            .route("Explain Rust lifetimes", &all_providers())
            .expect("should route");
        assert_eq!(ctx.provider, "anthropic");
    }

    #[test]
    fn routes_research_to_deepseek() {
        let ctx = router()
            .route("Research the history of neural networks", &all_providers())
            .expect("should route");
        assert_eq!(ctx.provider, "deepseek");
    }

    #[test]
    fn routes_biography_to_gemini() {
        let ctx = router()
            .route("Write a biography of Ada Lovelace", &all_providers())
            .expect("should route");
        assert_eq!(ctx.provider, "gemini");
    }

    #[test]
    fn routes_synthesize_to_gemini() {
        let ctx = router()
            .route("Synthesize findings from these papers", &all_providers())
            .expect("should route");
        assert_eq!(ctx.provider, "gemini");
    }

    #[test]
    fn fallback_to_priority_when_no_keywords() {
        let ctx = router()
            .route("What time is it?", &all_providers())
            .expect("should route");
        assert_eq!(
            ctx.provider, "anthropic",
            "fallback should be highest-priority"
        );
    }

    #[test]
    fn empty_available_returns_none() {
        assert!(router().route("implement something", &[]).is_none());
    }

    #[test]
    fn failover_skips_failed_provider() {
        let ctx = router()
            .failover("anthropic", &all_providers())
            .expect("should failover");
        assert_ne!(ctx.provider, "anthropic");
    }

    #[test]
    fn failover_returns_none_when_no_alternatives() {
        let only = vec!["anthropic".to_string()];
        assert!(router().failover("anthropic", &only).is_none());
    }

    #[test]
    fn routes_to_available_when_preferred_absent() {
        // deepseek not in available; "research" keyword hits deepseek first but it's absent,
        // so keyword scan exhausted -> priority fallback -> anthropic
        let available = vec!["anthropic".into(), "openai".into()];
        let ctx = router()
            .route("Research the history of AI", &available)
            .expect("should route");
        assert_eq!(ctx.provider, "anthropic");
    }
}

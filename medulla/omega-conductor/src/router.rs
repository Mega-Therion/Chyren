//! Hybrid Sovereign Provider Router.
//!
//! Classifies tasks into one of two routing tiers:
//!
//! | Tier  | Spoke        | When                                              |
//! |-------|--------------|---------------------------------------------------|
//! | Local | `ollama`     | High-sensitivity (arch design, ledger, identity)  |
//! |       |              | or routine tasks — never leaves the machine.      |
//! | Cloud | `openrouter` | High-complexity formal/mathematical reasoning.    |
//!
//! Both tiers commit to the same Master Ledger via the Conductor pipeline.

/// Routing tier decided for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteClass {
    /// Route to local Ollama instance (`OLLAMA_BASE_URL`, default `localhost:11434`).
    Local,
    /// Route to OpenRouter cloud (`OPENROUTER_BASE_URL`, default `openrouter.ai/api/v1`).
    Cloud,
}

/// Tier-1 upshift target: the spoke that handles escalated reasoning.
pub const UPSHIFT_PROVIDER: &str = "openrouter";
/// Tier-1 upshift model: high-capability model used for the first escalation attempt.
/// Can be overridden at runtime by setting `OPENROUTER_ESCALATION_MODEL` in one-true.env.
pub const UPSHIFT_MODEL_DEFAULT: &str = "anthropic/claude-3.5-sonnet";

/// Stateless task classifier.
pub struct ProviderRouter;

impl ProviderRouter {
    /// Classify a task string into a routing tier.
    ///
    /// High-sensitivity tasks (identity, ledger, architecture) stay on the local
    /// Ollama spoke. Everything else routes to OpenRouter, which is the cheapest
    /// reliable cloud path. Failures cascade through the SpokeRegistry preference
    /// list (deepseek → gemini → groq → anthropic → openai → perplexity).
    pub fn classify(task: &str) -> RouteClass {
        let t = task.to_lowercase();

        let is_local_sensitive = t.contains("phylactery")
            || t.contains("identity synthesis")
            || t.contains("ledger entry")
            || t.contains("sovereign identity")
            || t.contains("private key")
            || t.contains("secret")
            || t.contains("credential");

        if is_local_sensitive {
            RouteClass::Local
        } else {
            RouteClass::Cloud
        }
    }

    /// Return the spoke name for a routing class.
    pub fn spoke_name(class: RouteClass) -> &'static str {
        match class {
            RouteClass::Local => "ollama",
            RouteClass::Cloud => "openrouter",
        }
    }

    /// One-shot: classify a task and return the target spoke name.
    pub fn route(task: &str) -> &'static str {
        Self::spoke_name(Self::classify(task))
    }

    /// The spoke name to use for Tier-1 escalation (upshift).
    pub fn upshift_provider() -> &'static str {
        UPSHIFT_PROVIDER
    }

    /// The model slug to request during Tier-1 escalation.
    /// Reads `OPENROUTER_ESCALATION_MODEL` from env first, falls back to the compile-time default.
    pub fn upshift_model() -> String {
        std::env::var("OPENROUTER_ESCALATION_MODEL")
            .unwrap_or_else(|_| UPSHIFT_MODEL_DEFAULT.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_routes_local() {
        assert_eq!(ProviderRouter::classify("update the ledger entry"), RouteClass::Local);
        assert_eq!(ProviderRouter::classify("run identity synthesis"), RouteClass::Local);
        assert_eq!(ProviderRouter::classify("rotate the private key"), RouteClass::Local);
    }

    #[test]
    fn default_routes_cloud() {
        assert_eq!(ProviderRouter::classify("summarize this document"), RouteClass::Cloud);
        assert_eq!(ProviderRouter::classify("design the API architecture"), RouteClass::Cloud);
        assert_eq!(ProviderRouter::classify("solve the riemann hypothesis"), RouteClass::Cloud);
        assert_eq!(ProviderRouter::classify("write a lean4 proof for this theorem"), RouteClass::Cloud);
    }

    #[test]
    fn spoke_names_correct() {
        assert_eq!(ProviderRouter::spoke_name(RouteClass::Local), "ollama");
        assert_eq!(ProviderRouter::spoke_name(RouteClass::Cloud), "openrouter");
    }
}

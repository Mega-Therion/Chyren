//! Hybrid Sovereign Provider Router.
//!
//! ENTERPRISE EDITION: Unified Universal Routing.
//! All tasks route through the Sovereign Universal Spoke for maximum integrity.

/// Routing tier decided for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteClass {
    /// Native Universal Routing.
    Universal,
}

/// Tier-1 upshift target: the spoke that handles escalated reasoning.
pub const UPSHIFT_PROVIDER: &str = "universal";

/// Stateless task classifier.
pub struct ProviderRouter;

impl ProviderRouter {
    /// Classify a task string into a routing tier.
    pub fn classify(_task: &str) -> RouteClass {
        RouteClass::Universal
    }

    /// Return the spoke name for a routing class.
    pub fn spoke_name(_class: RouteClass) -> &'static str {
        "universal"
    }

    /// One-shot: classify a task and return the target spoke name.
    pub fn route(_task: &str) -> &'static str {
        "universal"
    }

    /// The spoke name to use for Tier-1 escalation (upshift).
    pub fn upshift_provider() -> &'static str {
        "universal"
    }

    /// The model slug to request during Tier-1 escalation.
    pub fn upshift_model() -> String {
        std::env::var("UNIVERSAL_MODEL").unwrap_or_else(|_| "custom-model".to_string())
    }
}

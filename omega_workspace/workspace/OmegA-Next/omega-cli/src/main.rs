//! Chyren CLI: The sovereign intelligence orchestrator command-line interface.
//!
//! Provides interactive access to the OmegA protocol layers (AEGIS, AEON, ADCCL, MYELIN)
//! and integrates with all provider spokes for unified task execution.

use clap::Parser;
use anyhow::Result;
use omega_myelin::Service as MemoryService;
use omega_myelin::phylactery::bootstrap_phylactery_kernel;
use omega_spokes::{SpokeRegistry, AnthropicSpoke, NeonSpoke, SearchSpoke, SpokeConfig};
use omega_integration::Service as IntegrationService;
use omega_integration::tool_router::ToolRouter;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "chyren")]
#[command(about = "Sovereign Intelligence Orchestrator", long_about = None)]
struct Args {
    /// Task or prompt to execute
    #[arg(value_name = "TASK")]
    task: Option<String>,

    /// Preferred provider (anthropic|openai|deepseek|gemini)
    #[arg(short, long)]
    provider: Option<String>,

    /// Maximum tokens in response
    #[arg(long, default_value = "1024")]
    max_tokens: usize,

    /// Sampling temperature (0.0-1.0)
    #[arg(long, default_value = "0.3")]
    temperature: f64,

    /// Show system status and exit
    #[arg(long)]
    status: bool,

    /// Display constitution and exit
    #[arg(long)]
    view_constitution: bool,

    /// Display threat fabric and exit
    #[arg(long)]
    view_threats: bool,

    /// Number of threat entries to display
    #[arg(long, default_value = "20")]
    threats_limit: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize phylactery kernel (L6 identity foundation)
    let kernel_path = "data/phylactery_kernel.json";
    let mut memory = MemoryService::new();

    match bootstrap_phylactery_kernel(&mut memory, kernel_path) {
        Ok(_) => {
            tracing::info!("✓ Phylactery kernel loaded: identity foundation initialized");
        }
        Err(e) => {
            tracing::warn!("⚠ Phylactery kernel load failed (non-fatal): {}", e);
        }
    }

    // Initialize spoke registry and integration layer
    let mut spoke_registry = SpokeRegistry::new();

    // Register Anthropic spoke
    let anthropic_config = SpokeConfig {
        name: "anthropic".to_string(),
        spoke_type: "anthropic".to_string(),
        endpoint: "https://api.anthropic.com".to_string(),
        credentials_ref: "ANTHROPIC_API_KEY".to_string(),
        enabled: true,
        max_concurrent: 10,
        timeout_seconds: 60,
    };
    if let Err(e) = spoke_registry.register(Box::new(AnthropicSpoke::new(anthropic_config))) {
        tracing::warn!("⚠ Failed to register Anthropic spoke: {}", e);
    }

    // Register Neon spoke
    let neon_config = SpokeConfig {
        name: "neon".to_string(),
        spoke_type: "neon".to_string(),
        endpoint: "neon.tech".to_string(),
        credentials_ref: "DATABASE_URL".to_string(),
        enabled: true,
        max_concurrent: 20,
        timeout_seconds: 30,
    };
    if let Err(e) = spoke_registry.register(Box::new(NeonSpoke::new(neon_config))) {
        tracing::warn!("⚠ Failed to register Neon spoke: {}", e);
    }

    // Register Search spoke
    let search_config = SpokeConfig {
        name: "search".to_string(),
        spoke_type: "search".to_string(),
        endpoint: "search.api".to_string(),
        credentials_ref: "SEARCH_API_KEY".to_string(),
        enabled: true,
        max_concurrent: 15,
        timeout_seconds: 45,
    };
    if let Err(e) = spoke_registry.register(Box::new(SearchSpoke::new(search_config))) {
        tracing::warn!("⚠ Failed to register Search spoke: {}", e);
    }

    let spoke_registry = Arc::new(spoke_registry);
    tracing::info!(
        "✓ Spoke registry initialized with {} spokes",
        spoke_registry.list_spokes().len()
    );

    // Initialize integration service and wire spokes
    let integration = IntegrationService::new();
    integration.set_spoke_registry(spoke_registry.clone()).await;
    tracing::info!("✓ Integration service wired with spoke registry");

    // Create tool router for task planning
    let tool_router = ToolRouter::new(spoke_registry.clone());
    let available_tools = tool_router.list_all_tools().await;
    tracing::info!("✓ Tool router ready with {} tools across {} spokes",
        available_tools.iter().map(|(_, tools)| tools.len()).sum::<usize>(),
        available_tools.len()
    );

    if let Some(task) = args.task {
        tracing::info!("Executing task: {}", task);
        // Core execution logic will be implemented in omega-aegis
    } else if args.status {
        tracing::info!("Fetching system status...");
        // Status retrieval logic
    } else if args.view_constitution {
        tracing::info!("Displaying constitution...");
        // Constitution display logic
    } else if args.view_threats {
        tracing::info!("Displaying threat fabric entries...");
        // Threat fabric display logic
    } else {
        println!("Usage: chyren <TASK> [OPTIONS]");
        println!("Run 'chyren --help' for more information.");
    }

    Ok(())
}

//! Chyren CLI: The sovereign intelligence orchestrator command-line interface.
//!
//! Provides interactive access to the OmegA protocol layers (AEGIS, AEON, ADCCL, MYELIN)
//! and integrates with all provider spokes for unified task execution.

use clap::Parser;
use anyhow::Result;
use omega_myelin::Service as MemoryService;
use omega_myelin::phylactery::bootstrap_phylactery_kernel;

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

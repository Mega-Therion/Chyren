use clap::{Parser, Subcommand};
use omega_cli::conductor::Conductor;
use omega_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use std::sync::Arc;
use tracing::{info, warn};

fn init_tracing() {
    // Respect `RUST_LOG` when set; otherwise default to info-level logs.
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Task to execute
    #[arg(index = 1)]
    task: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system status
    Status,
    /// Reset the ledger
    Reset,
    /// Start the API server for the web frontend
    Server,
    /// Ingest a MatrixProgram from a path
    Ingest {
        /// Path to the MatrixProgram file (JSON)
        #[arg(short, long)]
        path: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let cli = Cli::parse();

    let mut conductor = Conductor::new();

    if let Ok(url) = std::env::var("OMEGA_DB_URL") {
        match omega_myelin::db::MemoryStore::connect(&url, "").await {
            Ok(store) => {
                conductor.set_store(Arc::new(store));
                info!("Persistent Master Ledger online.");
            }
            Err(e) => warn!("Failed to connect to DB: {e}. Using volatile ledger."),
        }
    }

    // Phase 6: Identity Synthesis
    if let Err(e) = conductor.bootstrap_identity().await {
        warn!("Phylactery bootstrap failed: {e}. Running as generic orchestrator.");
    } else {
        info!("Phylactery Identity Kernel active (L6 Canonical).");
    }

    let conductor = Arc::new(conductor);

    match &cli.command {
        Some(Commands::Status) => {
            println!("--- HEMISPHERE R (MEDULLA — Execution/Performance/Memory) ---");
            println!("Yettragrammaton: R.W.Ϝ.Y.");
            println!("Integrity: SEALED");
            return Ok(());
        }
        Some(Commands::Server) => {
            println!("[BOOT] Launching API Server...");
            omega_cli::api::start_api_server(conductor).await?;
            return Ok(());
        }
        Some(Commands::Reset) => {
            // Safety gate: require explicit opt-in to destructive reset.
            let allow = std::env::var("CHYREN_ALLOW_RESET").unwrap_or_default();
            if !matches!(allow.as_str(), "1" | "true" | "yes") {
                println!(
                    "Refusing to reset without CHYREN_ALLOW_RESET=1.\n\
                     This operation deletes ledger + memory tables from the configured DB."
                );
                return Ok(());
            }

            if conductor.reset_persistent_store().await? {
                println!("[OK] Persistent store reset (Postgres/Neon tables cleared).");
                println!("[NOTE] External vector store (Qdrant) was not cleared.");
            } else {
                println!("[WARN] No persistent store configured; clearing ephemeral state only.");
            }

            conductor.reset_ephemeral_state().await;
            println!("[OK] Ephemeral in-process state cleared.");
            return Ok(());
        }
        Some(Commands::Ingest { path }) => {
            println!("[TASK] Ingesting MatrixProgram from: {}", path);
            let content = std::fs::read_to_string(path)?;
            let program: omega_core::MatrixProgram = serde_json::from_str(&content)?;

            // For now, using a fresh graph (or retrieved from store)
            let mut graph = omega_myelin::MemoryGraph::new();
            omega_cli::conductor::ingestion::IngestionEngine::ingest(program, &mut graph).await?;

            println!("[SUCCESS] Program integrated into memory graph.");
            return Ok(());
        }
        None => {
            if let Some(task_text) = cli.task {
                let mut envelope = RunEnvelope {
                    task_id: format!("t-{}", uuid::Uuid::new_v4()),
                    run_id: format!("r-{}", uuid::Uuid::new_v4()),
                    task: task_text.clone(),
                    task_text: task_text.clone(),
                    created_at: now(),
                    status: RunStatus::Pending,
                    risk_score: 0.0,
                    verified_payload: None,
                    evidence_packet: EvidencePacket::new(),
                };

                println!("[PLANNING] Analysing task: \"{}\"", task_text);
                let plan = conductor.plan_task(&task_text).await?;

                println!("[EXECUTING] Routing through sovereign pipeline...");
                let result = conductor.execute_plan(plan, &mut envelope).await?;

                println!("\n{}", "=".repeat(60));
                println!("Result Status: {:?}", result.status);
                if let Some(v) = result.verification {
                    println!("ADCCL Score  : {:.2}", v.score);
                }
                println!("{}", "=".repeat(60));
                println!("\n{}", result.response_text);
            } else {
                println!("Usage: omega-cli <task>");
            }
        }
    }

    Ok(())
}

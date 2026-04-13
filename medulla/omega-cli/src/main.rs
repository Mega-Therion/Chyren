use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use omega_cli::conductor::{Conductor, ConductorError};
use omega_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use std::sync::Arc;
use tracing::{info, warn};

fn init_tracing() {
    // Respect `RUST_LOG` when set; otherwise default to info-level logs.
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    // Always log to stderr so `--json` stdout remains machine-readable.
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Emit machine-readable JSON output (single object per invocation).
    #[arg(long, global = true)]
    json: bool,

    /// Preferred provider/spoke (e.g. openai, anthropic, gemini, deepseek).
    #[arg(long, global = true)]
    provider: Option<String>,

    /// Generation max tokens (provider permitting).
    #[arg(long, global = true, default_value_t = 2048)]
    max_tokens: usize,

    /// Generation temperature (provider permitting).
    #[arg(long, global = true, default_value_t = 0.3)]
    temperature: f64,

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
    /// Generate shell completions for this CLI.
    Completions {
        /// Shell to generate completions for.
        #[arg(long)]
        shell: Shell,
    },
    /// Generate a man page for this CLI (stdout).
    Man,
}

#[derive(serde::Serialize)]
struct JsonOut<'a> {
    ok: bool,
    command: &'a str,
    status: Option<String>,
    run_id: Option<String>,
    response_text: Option<String>,
    adccl_score: Option<f64>,
    provider: Option<String>,
    error: Option<String>,
}

fn print_json(out: JsonOut<'_>) {
    println!(
        "{}",
        serde_json::to_string(&out).unwrap_or_else(|_| "{}".to_string())
    );
}

fn exit_code_for_error(e: &anyhow::Error) -> i32 {
    // Keep this intentionally stable: callers can rely on exit codes.
    let msg = e.to_string();
    if msg.contains("empty task") {
        2
    } else if msg.contains("Rejected") || msg.contains("rejected") {
        10
    } else if msg.contains("adccl") || msg.contains("VerificationFailed") {
        20
    } else if msg.contains("provider") || msg.contains("Spoke") {
        30
    } else {
        40
    }
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

    // Phase 7: Neocortex injection — load Matrix program library into MemoryGraph
    conductor.inject_neocortex();
    info!("Neocortex programs injected — sovereign knowledge active.");

    let conductor = Arc::new(conductor);

    match &cli.command {
        Some(Commands::Status) => {
            if cli.json {
                print_json(JsonOut {
                    ok: true,
                    command: "status",
                    status: Some("SEALED".to_string()),
                    run_id: None,
                    response_text: None,
                    adccl_score: None,
                    provider: None,
                    error: None,
                });
            } else {
                println!("--- HEMISPHERE R (MEDULLA — Execution/Performance/Memory) ---");
                println!("Yettragrammaton: R.W.Ϝ.Y.");
                println!("Integrity: SEALED");
            }
            return Ok(());
        }
        Some(Commands::Server) => {
            if !cli.json {
                println!("[BOOT] Launching API Server...");
            }
            omega_cli::api::start_api_server(conductor).await?;
            return Ok(());
        }
        Some(Commands::Reset) => {
            // Safety gate: require explicit opt-in to destructive reset.
            let allow = std::env::var("CHYREN_ALLOW_RESET").unwrap_or_default();
            if !matches!(allow.as_str(), "1" | "true" | "yes") {
                let msg = "Refusing to reset without CHYREN_ALLOW_RESET=1. This operation deletes ledger + memory tables from the configured DB.";
                if cli.json {
                    print_json(JsonOut {
                        ok: false,
                        command: "reset",
                        status: None,
                        run_id: None,
                        response_text: None,
                        adccl_score: None,
                        provider: None,
                        error: Some(msg.to_string()),
                    });
                    std::process::exit(2);
                } else {
                    println!("{msg}");
                }
                return Ok(());
            }

            if conductor.reset_persistent_store().await? {
                if !cli.json {
                    println!("[OK] Persistent store reset (Postgres/Neon tables cleared).");
                    println!("[NOTE] External vector store (Qdrant) was not cleared.");
                }
            } else if !cli.json {
                println!("[WARN] No persistent store configured; clearing ephemeral state only.");
            }

            conductor.reset_ephemeral_state().await;
            if cli.json {
                print_json(JsonOut {
                    ok: true,
                    command: "reset",
                    status: Some("RESET".to_string()),
                    run_id: None,
                    response_text: None,
                    adccl_score: None,
                    provider: None,
                    error: None,
                });
            } else {
                println!("[OK] Ephemeral in-process state cleared.");
            }
            return Ok(());
        }
        Some(Commands::Ingest { path }) => {
            if !cli.json {
                println!("[TASK] Ingesting MatrixProgram from: {}", path);
            }
            let content = std::fs::read_to_string(path)?;
            let program: omega_core::MatrixProgram = serde_json::from_str(&content)?;

            // For now, using a fresh graph (or retrieved from store)
            let mut graph = omega_myelin::MemoryGraph::new();
            omega_cli::conductor::ingestion::IngestionEngine::ingest(program, &mut graph).await?;

            if cli.json {
                print_json(JsonOut {
                    ok: true,
                    command: "ingest",
                    status: Some("INGESTED".to_string()),
                    run_id: None,
                    response_text: None,
                    adccl_score: None,
                    provider: None,
                    error: None,
                });
            } else {
                println!("[SUCCESS] Program integrated into memory graph.");
            }
            return Ok(());
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "chyren", &mut std::io::stdout());
            return Ok(());
        }
        Some(Commands::Man) => {
            let cmd = Cli::command();
            let man = Man::new(cmd);
            man.render(&mut std::io::stdout())?;
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

                if !cli.json {
                    println!("[PLANNING] Analysing task: \"{}\"", task_text);
                }

                let plan = match conductor.plan_task(&task_text).await {
                    Ok(p) => p,
                    Err(ConductorError::Deflected(deflection_text)) => {
                        // Adversarial input: show the deflection response as output, not as an error.
                        if cli.json {
                            print_json(JsonOut {
                                ok: false,
                                command: "task",
                                status: Some("DEFLECTED".to_string()),
                                run_id: None,
                                response_text: Some(deflection_text),
                                adccl_score: None,
                                provider: None,
                                error: Some("Rejected(adversarial)".to_string()),
                            });
                        } else {
                            println!("{deflection_text}");
                        }
                        std::process::exit(10);
                    }
                    Err(e) => {
                        let err = anyhow::anyhow!(e.to_string());
                        if cli.json {
                            print_json(JsonOut {
                                ok: false,
                                command: "task",
                                status: None,
                                run_id: None,
                                response_text: None,
                                adccl_score: None,
                                provider: cli.provider.clone(),
                                error: Some(err.to_string()),
                            });
                        } else {
                            eprintln!("{err}");
                        }
                        std::process::exit(exit_code_for_error(&err));
                    }
                };

                if !cli.json {
                    println!("[EXECUTING] Routing through sovereign pipeline...");
                }

                // Apply generation overrides at the CLI boundary (keeps API defaults stable).
                let result = match conductor
                    .execute_plan_with_overrides(
                        plan,
                        &mut envelope,
                        cli.provider.as_deref(),
                        cli.max_tokens,
                        cli.temperature,
                    )
                    .await
                {
                    Ok(r) => r,
                    Err(e) => {
                        let err = anyhow::anyhow!(e.to_string());
                        if cli.json {
                            print_json(JsonOut {
                                ok: false,
                                command: "task",
                                status: None,
                                run_id: Some(envelope.run_id.clone()),
                                response_text: None,
                                adccl_score: None,
                                provider: cli.provider.clone(),
                                error: Some(err.to_string()),
                            });
                        } else {
                            eprintln!("{err}");
                        }
                        std::process::exit(exit_code_for_error(&err));
                    }
                };

                if cli.json {
                    print_json(JsonOut {
                        ok: true,
                        command: "task",
                        status: Some(format!("{:?}", result.status)),
                        run_id: Some(envelope.run_id.clone()),
                        response_text: Some(result.response_text.clone()),
                        adccl_score: result.verification.as_ref().map(|v| v.score as f64),
                        provider: cli
                            .provider
                            .clone()
                            .or_else(|| result.spoke_response.as_ref().map(|r| r.provider.clone())),
                        error: None,
                    });
                } else {
                    println!("\n{}", "=".repeat(60));
                    println!("Result Status: {:?}", result.status);
                    if let Some(v) = result.verification {
                        println!("ADCCL Score  : {:.2}", v.score);
                    }
                    println!("{}", "=".repeat(60));
                    println!("\n{}", result.response_text);
                }
            } else if cli.json {
                print_json(JsonOut {
                    ok: false,
                    command: "help",
                    status: None,
                    run_id: None,
                    response_text: None,
                    adccl_score: None,
                    provider: None,
                    error: Some("No task or subcommand provided".to_string()),
                });
                std::process::exit(2);
            } else {
                println!("Run `chyren --help` for usage.");
            }
        }
    }

    Ok(())
}

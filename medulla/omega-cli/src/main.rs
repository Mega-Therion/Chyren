mod theme;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use omega_cli::conductor::{Conductor, ConductorError};
use omega_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use omega_conductor::agents::{
    ingestor::IngestorAgent,
    millennium::{MillenniumProblem, SearchAndExtendAgent},
};
use omega_myelin::Service as MyelinService;
use omega_neocortex::{cold_store::ColdStore, proof_index::ProofConstraintIndex, Neocortex};
use std::sync::Arc;
use tokio::sync::Mutex;
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
    /// Target a Millennium Prize Problem: crawl Mathlib4 precursors and absorb into Neocortex.
    Solve {
        /// Which problem: riemann | pvsnp | navier | yang | hodge | birch
        #[arg(value_name = "PROBLEM")]
        problem: String,
        /// How many import levels deep to crawl (default: 3)
        #[arg(long, default_value_t = 3)]
        depth: usize,
    },
    /// Ingest a broad mathematical or physical discipline.
    Discipline {
        /// Which discipline: arithmetic | geometry | logic | etc.
        #[arg(value_name = "DISCIPLINE")]
        discipline: String,
        /// How many import levels deep to crawl (default: 3)
        #[arg(long, default_value_t = 3)]
        depth: usize,
    },
    /// View metacognitive epiphanies from the boot cycle
    Insights,

    // Reasoning Passthroughs
    Thought { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Action { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Sense { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Verify { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Identity { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Flex { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Shard { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
    Memory { 
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String> 
    },
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

    if !cli.json {
        theme::print_banner();
    }

    // Launch The Eye (Prometheus Observability)
    let _ = omega_telemetry::start_metrics_server(9090).await;
    omega_telemetry::CHYREN_TASK_ADMITTED_TOTAL.inc();

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

    // Phase 8: Boot reflection pass — metacognitive self-assessment
    let insights = conductor.reflect().await;
    if !insights.is_empty() {
        eprintln!("[METACOG] {} epiphanies from boot reflection.", insights.len());
    }

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
                let n = conductor.dream_episode_count();
                let top = conductor.dream_top_pattern();
                let top_str = top
                    .map(|(label, count)| format!("\"{}\" ({}×)", label, count));
                theme::print_status_block("SEALED", n, top_str.as_deref());
            }
            return Ok(());
        }
        Some(Commands::Server) => {
            if !cli.json {
                println!("{}", theme::info("[BOOT] Launching API Server on :8080 ..."));
            }
            
            // Move 2: Start the AEON autonomous scheduler
            let scheduler = Arc::new(omega_aeon::SovereignScheduler::new());
            tokio::spawn(async move {
                scheduler.run().await;
            });

            omega_cli::api::start_api_server(conductor).await?;
            return Ok(());
        }
        Some(Commands::Insights) => {
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&insights).unwrap_or_default());
            } else {
                theme::print_insights(&insights);
            }
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
                    println!("{}", theme::ok("[OK] Persistent store reset — Postgres/Neon tables cleared."));
                    println!("{}", theme::warn("[NOTE] External vector store (Qdrant) was not cleared."));
                }
            } else if !cli.json {
                println!("{}", theme::warn("[WARN] No persistent store configured; clearing ephemeral state only."));
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
                println!("{}", theme::ok("[OK] Ephemeral in-process state cleared."));
            }
            return Ok(());
        }
        Some(Commands::Ingest { path }) => {
            if !cli.json {
                println!("{} {}", theme::info("[INGEST]"), theme::value(path));
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
                println!("{}", theme::ok("[SUCCESS] Program integrated into memory graph."));
            }
            return Ok(());
        }
        Some(Commands::Solve { problem, depth }) => {
            let problem_key = problem.to_lowercase();
            let target = match problem_key.as_str() {
                "riemann" | "riemann_hypothesis" => MillenniumProblem::RiemannHypothesis,
                "pvsnp" | "p_vs_np" => MillenniumProblem::PVsNP,
                "navier" | "navier_stokes" => MillenniumProblem::NavierStokes,
                "yang" | "yang_mills" => MillenniumProblem::YangMills,
                "hodge" => MillenniumProblem::HodgeConjecture,
                "birch" | "birch_swinnerton_dyer" => MillenniumProblem::BirchSwinnertonDyer,
                other => {
                    eprintln!("[ERROR] Unknown problem '{}'. Use: riemann | pvsnp | navier | yang | hodge | birch", other);
                    std::process::exit(1);
                }
            };

            println!(
                "{} {}  {}  depth={}",
                theme::tier("[TIER-2]"),
                theme::info("[SOLVE]"),
                theme::gradient(&target.name().to_uppercase(), 0),
                theme::value(&depth.to_string()),
            );
            println!("{}", theme::info("[SOLVE] Building Neocortex agent stack..."));

            let myelin = Arc::new(MyelinService::new());
            let neocortex = Arc::new(Neocortex::new());
            let cold_store = Arc::new(
                ColdStore::default_store()
                    .unwrap_or_else(|_| ColdStore::new("/tmp/chyren_cold").expect("cold store init failed"))
            );
            let proof_index = Arc::new(Mutex::new(ProofConstraintIndex::new()));

            let ingestor = IngestorAgent::new(
                myelin,
                neocortex,
                cold_store,
                proof_index,
            );
            let agent = SearchAndExtendAgent::new(ingestor);

            println!("{}", theme::warn("[SOLVE] Crawling Mathlib4 precursors — this may take several minutes..."));
            let report = agent.run(target, *depth).await;

            println!(
                "{}  {}  {}  {}",
                theme::ok("[SOLVE] Complete."),
                theme::label(&format!("modules={}", report.modules_crawled)),
                theme::ok(&format!("absorbed={}", report.absorbed_hashes.len())),
                if report.errors.is_empty() {
                    theme::ok("errors=0")
                } else {
                    theme::fail(&format!("errors={}", report.errors.len()))
                },
            );

            if !report.absorbed_hashes.is_empty() {
                println!("  {} {}", theme::label("first-hash"), theme::run_id(&report.absorbed_hashes[0]));
            }

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
            }

            return Ok(());
        }
        Some(Commands::Discipline { discipline, depth }) => {
            use omega_conductor::agents::millennium::SovereignDiscipline;
            let discipline_key = discipline.to_lowercase();
            let target = match discipline_key.as_str() {
                "arithmetic" | "arith" => SovereignDiscipline::Arithmetic,
                "number_theory" | "nt" => SovereignDiscipline::NumberTheory,
                "quantum" | "quantum_theory" => SovereignDiscipline::QuantumTheory,
                "physics" | "theoretical_physics" => SovereignDiscipline::TheoreticalPhysics,
                "geometry" | "algebraic_geometry" => SovereignDiscipline::AlgebraicGeometry,
                "analysis" | "complex_analysis" => SovereignDiscipline::ComplexAnalysis,
                "euclidean" | "euclidean_geometry" => SovereignDiscipline::EuclideanGeometry,
                "non_euclidean" | "non_euclidean_geometry" | "geodesic" => SovereignDiscipline::NonEuclideanGeometry,
                "differential_equations" | "ode" | "pde" | "non_linear" => SovereignDiscipline::DifferentialEquations,
                "linear_algebra" | "vectors" => SovereignDiscipline::LinearAlgebra,
                "abstract_algebra" | "algebra" => SovereignDiscipline::AbstractAlgebra,
                "topology" => SovereignDiscipline::Topology,
                "calculus" => SovereignDiscipline::Calculus,
                "trigonometry" | "trig" => SovereignDiscipline::Trigonometry,
                "kinematics" => SovereignDiscipline::Kinematics,
                "optics" => SovereignDiscipline::Optics,
                "cryptography" | "crypto" => SovereignDiscipline::Cryptography,
                "statistics" | "prob" => SovereignDiscipline::Statistics,
                "logic" | "rhetoric" | "argument" => SovereignDiscipline::LogicAndRhetoric,
                "philosophy" | "socratic" | "aristotelian" => SovereignDiscipline::ClassicalPhilosophy,
                _ => SovereignDiscipline::Arithmetic,
            };

            println!(
                "{} {}  {}  depth={}",
                theme::tier("[TIER-3]"),
                theme::info("[DISCIPLINE]"),
                theme::gradient(&target.name().to_uppercase(), 1),
                theme::value(&depth.to_string()),
            );

            let myelin = Arc::new(MyelinService::new());
            let neocortex = Arc::new(Neocortex::new());
            let cold_store = Arc::new(
                ColdStore::default_store()
                    .unwrap_or_else(|_| ColdStore::new("/tmp/chyren_cold").expect("cold store init failed"))
            );
            let proof_index = Arc::new(Mutex::new(ProofConstraintIndex::new()));

            let ingestor = IngestorAgent::new(
                myelin,
                neocortex,
                cold_store,
                proof_index,
            );
            let agent = SearchAndExtendAgent::new(ingestor);

            println!("{}", theme::warn("[DISCIPLINE] Absorbing sovereign domain — total synthesis in progress..."));
            let report = agent.run_discipline(target, *depth).await;

            println!(
                "{}  {}  {}  {}",
                theme::ok("[DISCIPLINE] Ingestion cycle complete."),
                theme::label(&format!("modules={}", report.modules_crawled)),
                theme::ok(&format!("absorbed={}", report.absorbed_hashes.len())),
                if report.errors.is_empty() {
                    theme::ok("errors=0")
                } else {
                    theme::fail(&format!("errors={}", report.errors.len()))
                },
            );

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
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
        _ => {}
    }

    // 2. Determine task text from either reasoning commands or legacy positional arg
    let task_text = match &cli.command {
        Some(Commands::Thought { args }) |
        Some(Commands::Action { args }) |
        Some(Commands::Sense { args }) |
        Some(Commands::Verify { args }) |
        Some(Commands::Identity { args }) |
        Some(Commands::Flex { args }) |
        Some(Commands::Shard { args }) |
        Some(Commands::Memory { args }) => {
            if args.is_empty() {
                None
            } else {
                Some(args.join(" "))
            }
        }
        _ => None,
    };

    if let Some(task_text) = task_text {
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
            println!(
                "{} {}",
                theme::info("[PLANNING]"),
                theme::value(&format!("\"{}\"", task_text)),
            );
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
                    println!("{}", theme::warn("[AEGIS] Task deflected — adversarial pattern detected."));
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
            println!("{}", theme::info("[EXECUTING] Routing through sovereign pipeline..."));
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
            let provider_str = result
                .spoke_response
                .as_ref()
                .map(|r| r.provider.as_str())
                .unwrap_or("unknown");
            let adccl_v = result.verification.as_ref().map(|v| v.score as f64).unwrap_or(0.0);
            let status_str = format!("{:?}", result.status);
            theme::print_result_header(
                &envelope.run_id,
                &status_str,
                adccl_v,
                provider_str,
            );
            theme::print_response(&result.response_text);
            println!();
        }
    } else {
        if cli.json {
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
            println!("{}", theme::label("Run `chyren --help` for usage."));
        }
    }
    Ok(())
}

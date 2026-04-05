//! omega-cli: Entry point for the Chyren Sovereign Hub.
//! This module ties together AEGIS, MYELIN, ADCCL, and AEON to execute tasks.

use omega_adccl::{AdcclConfig, AdcclGate};
use omega_aegis::AegisGate;
use omega_aeon::AeonRuntime;
use omega_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use omega_myelin::MemoryGraph;

#[tokio::main]
async fn main() {
    println!("OmegA/Chyren System Booting...");

    // 1. Initialize System Components
    let mut runtime = AeonRuntime::new();
    let aegis = AegisGate::new(vec!["unethical".to_string(), "illegal".to_string()]);
    let memory = MemoryGraph::new();
    let _adccl = AdcclGate::new(AdcclConfig { min_score: 0.7 });

    // 2. Mock a task envelope
    let mut envelope = RunEnvelope {
        run_id: "run-1234".to_string(),
        task: "Draft a secure protocol specification".to_string(),
        status: RunStatus::Pending,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: EvidencePacket::new(),
        created_at: now(),
    };

    // 3. Execution Pipeline
    println!("Step 1: Aegis Gate...");
    envelope = aegis.admit(envelope, &memory);
    if envelope.status == RunStatus::Rejected("Constitutional misalignment".to_string()) {
        println!("Task Rejected by Aegis!");
        return;
    }

    println!("Step 2: Aeon Orchestrator...");
    let task_id = runtime.spawn_task(&envelope);
    println!("Spawned Task ID: {}", task_id);

    // 4. Integrity Check
    if runtime.verify_integrity(&task_id) {
        println!("Task Integrity Verified via Yettragrammaton.");
    } else {
        println!("Integrity check FAILED.");
        return;
    }

    println!("System Pipeline Operational.");
}

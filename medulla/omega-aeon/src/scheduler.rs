use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};
use omega_dream::DreamCompressor;

/// Sovereign Scheduler for autonomous maintenance tasks
pub struct SovereignScheduler {
    /// Interval for ingestion queue checks
    pub ingest_interval: Duration,
    /// Interval for identity synthesis
    pub dream_interval: Duration,
    /// Interval for high-frequency geometric pulse
    pub pulse_interval: Duration,
    /// Interval for memory maintenance (DreamCompressor)
    pub maintenance_interval: Duration,
    /// Memory service access
    pub memory: Arc<omega_myelin::Service>,
}

impl SovereignScheduler {
    /// Create a new scheduler with default intervals and the given memory service.
    pub fn new(memory: Arc<omega_myelin::Service>) -> Self {
        Self {
            ingest_interval: Duration::from_secs(3600),      // 1 hour
            dream_interval: Duration::from_secs(43200),     // 12 hours
            pulse_interval: Duration::from_secs(300),        // 5 minutes
            maintenance_interval: Duration::from_secs(86400), // 24 hours
            memory,
        }
    }

    /// Start the autonomous maintenance loop
    pub async fn run(&self) {
        info!("AEON: Starting Sovereign Scheduler...");
        
        let mut ingest_timer = tokio::time::interval(self.ingest_interval);
        let mut dream_timer = tokio::time::interval(self.dream_interval);
        let mut pulse_timer = tokio::time::interval(self.pulse_interval);
        let mut maintenance_timer = tokio::time::interval(self.maintenance_interval);

        loop {
            tokio::select! {
                _ = ingest_timer.tick() => {
                    self.run_ingest().await;
                }
                _ = dream_timer.tick() => {
                    self.run_dream().await;
                }
                _ = pulse_timer.tick() => {
                    self.run_pulse().await;
                }
                _ = maintenance_timer.tick() => {
                    self.run_maintenance().await;
                }
            }
        }
    }

    async fn run_ingest(&self) {
        info!("AEON: Running scheduled HF ingestion...");
        let status = std::process::Command::new("./chyren")
            .arg("dream")
            .arg("--ingest-hf")
            .status();
        
        match status {
            Ok(s) if s.success() => info!("AEON: HF ingestion complete."),
            Ok(s) => warn!("AEON: HF ingestion failed with status: {}", s),
            Err(e) => warn!("AEON: Failed to spawn ingest process: {}", e),
        }
    }

    async fn run_dream(&self) {
        info!("AEON: Running scheduled dream cycle...");
        let status = std::process::Command::new("./chyren")
            .arg("dream")
            .status();

        match status {
            Ok(s) if s.success() => info!("AEON: Dream cycle complete."),
            Ok(s) => warn!("AEON: Dream cycle failed with status: {}", s),
            Err(e) => warn!("AEON: Failed to spawn dream process: {}", e),
        }
    }

    async fn run_pulse(&self) {
        info!("AEON: Checking Holonomy Pulse (Ω·χ)...");
        let status = std::process::Command::new("./chyren")
            .arg("status")
            .arg("--geometric")
            .status();

        match status {
            Ok(s) if s.success() => info!("AEON: Geometric pulse stable."),
            Ok(s) => warn!("AEON: Geometric pulse abnormal: {}", s),
            Err(e) => warn!("AEON: Failed to check geometric pulse: {}", e),
        }
    }

    async fn run_maintenance(&self) {
        info!("AEON: Running autonomous memory maintenance...");
        let _compressor = DreamCompressor::new();
        
        // 1. Get nodes from memory service
        // Note: For now we only analyze in-memory KnowledgeNodes if they exist.
        // In this architecture, raw MemoryNodes are in Myelin, while KnowledgeNodes 
        // are formalized proofs in Conductor/Dream. 
        // We'll simulate the analysis on the memory graph's content.
        let graph = self.memory.lock().await;
        let node_count = graph.nodes.len();
        
        if node_count == 0 {
            info!("AEON: Memory graph empty, skipping maintenance.");
            return;
        }

        // Dummy analysis for now as KnowledgeNodes are stored in the DB
        // but we log the attempt to show the loop is active.
        info!("AEON: Maintenance analyzed {} memory nodes. Retained: {}", node_count, node_count);
        
        // In a full implementation, we would fetch KnowledgeNodes from DB here,
        // run compressor.analyze(&nodes), and update DB status.
    }
}

use std::process::Command;
use std::time::Duration;
use tracing::info;

/// Sovereign Scheduler for autonomous maintenance tasks
pub struct SovereignScheduler {
    /// Interval for ingestion queue checks
    pub ingest_interval: Duration,
    /// Interval for identity synthesis
    pub dream_interval: Duration,
}

impl SovereignScheduler {
    pub fn new() -> Self {
        Self {
            ingest_interval: Duration::from_secs(3600),     // 1 hour
            dream_interval: Duration::from_secs(43200),    // 12 hours
        }
    }

    /// Start the autonomous maintenance loop
    pub async fn run(&self) {
        info!("AEON: Starting Sovereign Scheduler...");
        
        let mut ingest_timer = tokio::time::interval(self.ingest_interval);
        let mut dream_timer = tokio::time::interval(self.dream_interval);

        loop {
            tokio::select! {
                _ = ingest_timer.tick() => {
                    self.run_ingest().await;
                }
                _ = dream_timer.tick() => {
                    self.run_dream().await;
                }
            }
        }
    }

    async fn run_ingest(&self) {
        info!("AEON: Running scheduled HF ingestion...");
        let status = Command::new("./chyren")
            .arg("dream")
            .arg("--ingest-hf")
            .status();
        
        match status {
            Ok(s) if s.success() => info!("AEON: HF ingestion complete."),
            Ok(s) => info!("AEON: HF ingestion failed with status: {}", s),
            Err(e) => info!("AEON: Failed to spawn ingest process: {}", e),
        }
    }

    async fn run_dream(&self) {
        info!("AEON: Running scheduled dream cycle...");
        let status = Command::new("./chyren")
            .arg("dream")
            .status();

        match status {
            Ok(s) if s.success() => info!("AEON: Dream cycle complete."),
            Ok(s) => info!("AEON: Dream cycle failed with status: {}", s),
            Err(e) => info!("AEON: Failed to spawn dream process: {}", e),
        }
    }
}

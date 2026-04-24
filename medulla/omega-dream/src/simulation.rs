//! omega-dream background simulation worker
//!
//! Projects potential task outcomes using the Neocortex.

use crate::Service as DreamService;
use omega_neocortex::Neocortex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Background worker that projects potential task outcomes from historical failure patterns.
pub struct DreamSimulationWorker {
    dream_service: Arc<tokio::sync::Mutex<DreamService>>,
    neocortex: Arc<Neocortex>,
}

impl DreamSimulationWorker {
    /// Create a new simulation worker with the given dream service and neocortex.
    pub fn new(
        dream_service: Arc<tokio::sync::Mutex<DreamService>>,
        neocortex: Arc<Neocortex>,
    ) -> Self {
        Self {
            dream_service,
            neocortex,
        }
    }

    /// Run the continuous simulation loop, projecting outcomes every 5 minutes.
    pub async fn run(&self) {
        loop {
            // Background simulation logic
            let patterns = {
                let service = self.dream_service.lock().await;
                service.get_failure_patterns()
            };

            for (pattern, _count) in patterns {
                // Simulate potential outcomes based on historical failure patterns
                let _ = self.neocortex.project_outcome(&pattern).await;
            }

            sleep(Duration::from_secs(300)).await;
        }
    }
}

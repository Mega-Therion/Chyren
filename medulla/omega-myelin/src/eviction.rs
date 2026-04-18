//! StratumEvictionWorker — 30-day hot-to-cold eviction for the Neocortex MemoryGraph.
//!
//! Runs as a background task. Every `check_interval` it scans all MemoryNodes:
//! nodes with a `decay_score` below the eviction threshold (meaning they haven't
//! been accessed in ~30 days) are removed from the hot in-memory graph.
//!
//! The actual cold-store write is handled by the caller (conductor or dream engine)
//! that holds the ColdStore reference. This module focuses on eviction detection
//! and hot-graph cleanup.

use crate::{MemoryGraph, MemoryNode};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

/// Decay score below which a node is considered cold (not accessed recently).
/// Nodes start at 1.0 and decay; 0.1 ≈ 30+ days of inactivity.
const COLD_THRESHOLD: f64 = 0.1;

/// How often to run the eviction scan.
const DEFAULT_CHECK_INTERVAL: Duration = Duration::from_secs(6 * 3600); // every 6 hours

/// Eviction callback: called with each evicted node so the caller can persist
/// it to cold storage before it is removed from the hot graph.
pub type EvictionCallback = Box<dyn Fn(MemoryNode) + Send + Sync>;

/// Background worker that evicts cold nodes from the hot MemoryGraph.
pub struct StratumEvictionWorker {
    graph: Arc<Mutex<MemoryGraph>>,
    check_interval: Duration,
    on_evict: Option<Arc<EvictionCallback>>,
}

impl StratumEvictionWorker {
    pub fn new(graph: Arc<Mutex<MemoryGraph>>) -> Self {
        Self {
            graph,
            check_interval: DEFAULT_CHECK_INTERVAL,
            on_evict: None,
        }
    }

    /// Override the scan interval (useful for testing with a short interval).
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }

    /// Register a callback invoked for each evicted node (e.g. to write to ColdStore).
    pub fn with_eviction_callback(mut self, cb: impl Fn(MemoryNode) + Send + Sync + 'static) -> Self {
        self.on_evict = Some(Arc::new(Box::new(cb)));
        self
    }

    /// Decay all node scores: each scan reduces decay_score by `decay_per_tick`.
    /// A node untouched for ~30 days (5 ticks of 6h = 30h; at full 30d = 120 ticks)
    /// will reach 0.1 from 1.0 with a per-tick decay of 0.0075.
    fn apply_decay(node: &mut MemoryNode) {
        node.decay_score = (node.decay_score - 0.0075).max(0.0);
    }

    /// Run one eviction scan. Returns the list of evicted node IDs.
    pub async fn scan_once(&self) -> Vec<String> {
        let mut graph = self.graph.lock().await;
        let mut evicted_ids = Vec::new();

        // Apply decay to all nodes
        for node in graph.nodes.values_mut() {
            Self::apply_decay(node);
        }

        // Collect cold nodes
        let cold: Vec<MemoryNode> = graph
            .nodes
            .values()
            .filter(|n| n.decay_score < COLD_THRESHOLD)
            .cloned()
            .collect();

        for node in cold {
            evicted_ids.push(node.node_id.clone());
            graph.nodes.remove(&node.node_id);
            if let Some(cb) = &self.on_evict {
                cb(node);
            }
        }

        evicted_ids
    }

    /// Reset a node's decay score to 1.0 (called when it is accessed).
    pub async fn touch(&self, node_id: &str) {
        let mut graph = self.graph.lock().await;
        if let Some(node) = graph.nodes.get_mut(node_id) {
            node.decay_score = 1.0;
            node.retrieval_count += 1;
        }
    }

    /// Start the background eviction loop. Runs indefinitely; cancel via task handle.
    pub async fn run(self: Arc<Self>) {
        loop {
            sleep(self.check_interval).await;
            let evicted = self.scan_once().await;
            if !evicted.is_empty() {
                eprintln!(
                    "[myelin eviction] evicted {} cold nodes to cold stratum",
                    evicted.len()
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryGraph;
    use omega_core::MemoryStratum;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    async fn graph_with_cold_node() -> Arc<Mutex<MemoryGraph>> {
        let g = Arc::new(Mutex::new(MemoryGraph::new()));
        {
            let mut graph = g.lock().await;
            let node = graph.write_node("cold content".to_string(), MemoryStratum::Canonical);
            // Force decay score below threshold
            graph.nodes.get_mut(&node.node_id).unwrap().decay_score = 0.05;
        }
        g
    }

    #[tokio::test]
    async fn test_cold_node_evicted() {
        let graph = graph_with_cold_node().await;
        let worker = StratumEvictionWorker::new(Arc::clone(&graph));
        let evicted = worker.scan_once().await;
        assert_eq!(evicted.len(), 1);
        assert!(graph.lock().await.nodes.is_empty());
    }

    #[tokio::test]
    async fn test_hot_node_retained() {
        let g = Arc::new(Mutex::new(MemoryGraph::new()));
        {
            let mut graph = g.lock().await;
            graph.write_node("hot content".to_string(), MemoryStratum::Canonical);
        }
        let worker = StratumEvictionWorker::new(Arc::clone(&g));
        let evicted = worker.scan_once().await;
        assert!(evicted.is_empty());
        assert_eq!(g.lock().await.nodes.len(), 1);
    }

    #[tokio::test]
    async fn test_touch_resets_decay() {
        let graph = graph_with_cold_node().await;
        let node_id = graph.lock().await.nodes.keys().next().unwrap().clone();
        let worker = StratumEvictionWorker::new(Arc::clone(&graph));
        worker.touch(&node_id).await;
        let g = graph.lock().await;
        assert_eq!(g.nodes[&node_id].decay_score, 1.0);
    }
}

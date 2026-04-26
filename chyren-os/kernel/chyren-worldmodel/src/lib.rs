//! chyren-worldmodel: World model reasoning and state tracking
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// World state snapshot
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    /// Unique snapshot ID
    pub snapshot_id: String,
    /// State variables
    pub variables: HashMap<String, Value>,
    /// Timestamp of state
    pub timestamp: f64,
}

/// World model service for state tracking
#[derive(Clone, Debug)]
pub struct Service {
    current_state: WorldState,
    state_history: Vec<WorldState>,
    max_history: usize,
}

impl Service {
    /// Create new world model service
    pub fn new() -> Self {
        Service {
            current_state: WorldState {
                snapshot_id: "ws-init".to_string(),
                variables: HashMap::new(),
                timestamp: 0.0,
            },
            state_history: Vec::new(),
            max_history: 100,
        }
    }

    /// Update state variable
    pub fn update_variable(&mut self, key: &str, value: Value) {
        self.current_state.variables.insert(key.to_string(), value);
    }

    /// Get state variable
    pub fn get_variable(&self, key: &str) -> Option<Value> {
        self.current_state.variables.get(key).cloned()
    }

    /// Take snapshot of current state
    pub fn snapshot(&mut self, snapshot_id: &str, timestamp: f64) {
        let snapshot = WorldState {
            snapshot_id: snapshot_id.to_string(),
            variables: self.current_state.variables.clone(),
            timestamp,
        };

        self.state_history.push(snapshot);

        if self.state_history.len() > self.max_history {
            self.state_history.remove(0);
        }
    }

    /// Get historical snapshot
    pub fn get_snapshot(&self, index: usize) -> Option<WorldState> {
        self.state_history.get(index).cloned()
    }

    /// Get all snapshots
    pub fn get_history(&self) -> Vec<WorldState> {
        self.state_history.clone()
    }

    /// Compute state delta from two snapshots
    pub fn compute_delta(&self, from: usize, to: usize) -> HashMap<String, (Value, Value)> {
        let mut delta = HashMap::new();

        if let (Some(from_state), Some(to_state)) = (self.get_snapshot(from), self.get_snapshot(to))
        {
            for (key, to_value) in &to_state.variables {
                if let Some(from_value) = from_state.variables.get(key) {
                    if from_value != to_value {
                        delta.insert(key.clone(), (from_value.clone(), to_value.clone()));
                    }
                }
            }
        }

        delta
    }

    /// Check if a state constraint is satisfied
    pub fn check_constraint(&self, constraint_expr: &str) -> bool {
        // Simple constraint checking - evaluates conditions like "key=value"
        if let Some(eq_pos) = constraint_expr.find('=') {
            let key = constraint_expr[..eq_pos].trim();
            let expected = constraint_expr[eq_pos + 1..].trim().trim_matches('"');

            if let Some(actual) = self.get_variable(key) {
                if let Some(s) = actual.as_str() {
                    return s == expected;
                }
            }
        }
        false
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_variable_update() {
        let mut service = Service::new();
        service.update_variable("status", Value::String("active".to_string()));
        assert_eq!(
            service.get_variable("status"),
            Some(Value::String("active".to_string()))
        );
    }

    #[test]
    fn test_snapshot() {
        let mut service = Service::new();
        service.update_variable("count", json!(5));
        service.snapshot("snap-1", 100.0);
        assert_eq!(service.state_history.len(), 1);
    }

    #[test]
    fn test_state_delta() {
        let mut service = Service::new();
        service.update_variable("x", json!(1));
        service.snapshot("snap-1", 100.0);

        service.update_variable("x", json!(2));
        service.snapshot("snap-2", 200.0);

        let delta = service.compute_delta(0, 1);
        assert!(!delta.is_empty());
        assert!(delta.contains_key("x"));
    }

    #[test]
    fn test_constraint_checking() {
        let mut service = Service::new();
        service.update_variable("mode", Value::String("test".to_string()));
        assert!(service.check_constraint("mode=\"test\""));
        assert!(!service.check_constraint("mode=\"prod\""));
    }
}

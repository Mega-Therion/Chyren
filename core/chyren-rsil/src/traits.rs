use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData<T> {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub value: T,
}

#[async_trait]
pub trait Sensor: Send + Sync {
    type Output: Debug + Send + Sync + Serialize;
    async fn read(&self) -> Result<SensorData<Self::Output>, String>;
    fn id(&self) -> String;
}

#[async_trait]
pub trait Actuator: Send + Sync {
    type Input: Debug + Send + Sync + Serialize;
    async fn actuate(&self, command: Self::Input) -> Result<(), String>;
    fn id(&self) -> String;
}

/// ABC Articulator trait — sits between Sensor and Actuator.
/// Receives a ternary confidence signal from the Cortex and returns
/// a binary-compatible ArticulationResult for AEGIS to consume.
#[async_trait]
pub trait Articulator: Send + Sync {
    /// Receives a real-valued ternary signal in [-1.0, 1.0].
    /// Returns an ArticulationResult (Execute | Abort | Suspended).
    async fn articulate(
        &self,
        signal: f32,
    ) -> Result<crate::abc::ArticulationResult, String>;

    fn id(&self) -> String;
}

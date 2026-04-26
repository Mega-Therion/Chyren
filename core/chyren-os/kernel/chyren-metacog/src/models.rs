use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemSnapshot {
    pub identity_hash: String,
    pub active_spokes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntrospectionReport {
    pub identity_hash: String,
    pub cognitive_load: f32,
    pub alignment_drift: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaQuery {
    pub query: String,
    pub scope: String, // "Global", "Task", "Kernel"
    pub depth: String, // "Surface", "Deep"
}

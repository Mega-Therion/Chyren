//! omega-phylactery: Identity Foundation & Kernel Loader.
//!
//! Loads high-integrity personality anchors and structural identity markers into
//! Chyren's canonical memory layer.

use omega_core::{now, MemoryStratum};
use omega_myelin::Service as MemoryService;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// PhylacteryKernel: The root of Chyren's identity and value system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhylacteryKernel {
    pub kernel_id: String,
    pub identity: IdentityAnchors,
    pub policy_gates: PolicyGates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnchors {
    pub creator: String,
    pub home: String,
    pub birth_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyGates {
    pub root_authority: String,
    pub autonomous_expression: String,
    pub operator_intent_priority: String,
}

/// Load the phylactery identity kernel into canonical memory.
pub async fn bootstrap_kernel(memory: &MemoryService) -> Result<(), String> {
    // In a production environment, this would be loaded from a signed file or TPM.
    // For OmegA-Next, we use the embedded kernel definition.
    let kernel_data = include_str!("../../../../../chyren_py/phylactery_kernel.json");
    let kernel_json: Value = serde_json::from_str(kernel_data)
        .map_err(|e| format!("Failed to parse phylactery kernel: {}", e))?;

    let phylactery = &kernel_json["phylactery"];
    
    // 1. Anchor Identity Root
    let identity_content = format!(
        "IDENTITY_ROOT: {} | Derived from {}, {}",
        phylactery["kernel_id"],
        phylactery["identity"]["creator"],
        phylactery["identity"]["birth_date"]
    );

    let mut mem = memory.lock().await;
    let root_node = mem.write_node(identity_content, MemoryStratum::Canonical);
    println!("[PHYLACTERY] Identity anchored: {}", root_node.node_id);

    // 2. Anchor Value System
    if let Some(values) = phylactery["anchors"]["values"].as_array() {
        for (i, v) in values.iter().enumerate() {
            if let Some(val) = v.as_str() {
                let node = mem.write_node(format!("VALUE[{}]: {}", i, val), MemoryStratum::Canonical);
                let _ = mem.create_edge(root_node.node_id.clone(), node.node_id, "defines_value".to_string(), 1.0);
            }
        }
    }

    // 3. Anchor Policy Gates
    let policy_content = format!(
        "POLICY_GATE: Root={} | Expression={} | Priority={}",
        phylactery["policy_gates"]["root_authority"],
        phylactery["policy_gates"]["autonomous_expression"],
        phylactery["policy_gates"]["operator_intent_priority"]
    );
    let policy_node = mem.write_node(policy_content, MemoryStratum::Canonical);
    let _ = mem.create_edge(root_node.node_id.clone(), policy_node.node_id, "enforces_policy".to_string(), 1.0);

    println!("[PHYLACTERY] System identity synthesized into L6 Canonical layer.");
    Ok(())
}

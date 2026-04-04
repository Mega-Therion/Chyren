//! Phylactery Kernel Bootstrap (L6)
//! Auto-generated from identity_synthesis output
//! Loads RY's identity foundation into Chyren's memory system

use crate::Service;
use omega_core::MemoryStratum;
use std::fs;

/// Bootstrap phylactery kernel from JSON file.
///
/// Loads the identity foundation kernel from the specified path and initializes
/// the L6 Canonical stratum with identity anchors, values, goals, and policies.
pub fn bootstrap_phylactery_kernel(
    memory: &mut Service,
    kernel_path: &str,
) -> Result<(), String> {
    // Load kernel JSON from file
    let kernel_data = fs::read_to_string(kernel_path)
        .map_err(|e| format!("Failed to read phylactery kernel file: {}", e))?;

    // Parse kernel JSON
    let kernel: serde_json::Value = serde_json::from_str(&kernel_data)
        .map_err(|e| format!("Failed to parse phylactery kernel: {}", e))?;

    // Extract identity anchors
    let phylactery = &kernel["phylactery"];

    // Write identity root node to canonical stratum (L6)
    let identity_content = format!(
        "IDENTITY_ANCHOR: {} - Creator: {}, Home: {}, Born: {}",
        phylactery["kernel_id"],
        phylactery["identity"]["creator"],
        phylactery["identity"]["home"],
        phylactery["identity"]["birth_date"]
    );

    let root_node = memory.write_node(identity_content, MemoryStratum::Canonical);
    println!("✓ Phylactery root anchored: {}", root_node.node_id);

    // Write value anchors
    if let Some(values) = phylactery["anchors"]["values"].as_array() {
        for (i, value) in values.iter().enumerate() {
            if let Some(v) = value.as_str() {
                let node = memory.write_node(
                    format!("VALUE[{}]: {}", i, v),
                    MemoryStratum::Canonical
                );
                // Link to root
                let _ = memory.create_edge(
                    root_node.node_id.clone(),
                    node.node_id,
                    "anchors_value".to_string(),
                    0.95
                );
            }
        }
    }

    // Write goal anchors
    if let Some(goals) = phylactery["anchors"]["goals"].as_array() {
        for (i, goal) in goals.iter().enumerate() {
            if let Some(g) = goal.as_str() {
                let node = memory.write_node(
                    format!("GOAL[{}]: {}", i, g),
                    MemoryStratum::Canonical
                );
                let _ = memory.create_edge(
                    root_node.node_id.clone(),
                    node.node_id,
                    "anchors_goal".to_string(),
                    0.95
                );
            }
        }
    }

    // Write policy gates
    let policy_content = format!(
        "POLICY_ANCHOR: Root={} | Autonomous={} | OperatorIntent={}",
        phylactery["policy_gates"]["root_authority"],
        phylactery["policy_gates"]["autonomous_expression"],
        phylactery["policy_gates"]["operator_intent_priority"]
    );

    let policy_node = memory.write_node(policy_content, MemoryStratum::Canonical);
    let _ = memory.create_edge(
        root_node.node_id.clone(),
        policy_node.node_id,
        "enforces_policy".to_string(),
        1.0
    );

    println!("✓ Phylactery kernel fully bootstrapped to L6 (Canonical)");
    println!("  - Identity anchors: OK");
    println!("  - Value anchors: OK");
    println!("  - Goal anchors: OK");
    println!("  - Policy gates: OK");

    Ok(())
}

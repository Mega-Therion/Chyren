//! Basic ADCCL Check Example
//!
//! This example demonstrates how to perform a basic ADCCL compliance check
//! using Chyren's core library.

use chyren_core::{
    adccl::{AdcclGate, AdcclConfig, PreflightResult},
    state::State,
    error::ChyrenError,
};

#[tokio::main]
async fn main() -> Result<(), ChyrenError> {
    // Initialize logging
    env_logger::init();

    println!("\n=== Chyren ADCCL Basic Check Example ===");
    println!("Demonstrating ADCCL compliance verification\n");

    // Create ADCCL configuration with moderate strictness
    let config = AdcclConfig {
        strictness: 0.5,
        enable_preflight: true,
        gate_threshold: 0.7,
        ..Default::default()
    };

    // Initialize the ADCCL gate
    let gate = AdcclGate::new(config)?;
    println!("✓ ADCCL Gate initialized with strictness: 0.5\n");

    // Example 1: Check a compliant action
    println!("Example 1: Checking compliant action");
    let mut state = State::new("user_data_read")?;
    state.set_context("authenticated_read");
    state.set_metadata("user_id", "12345");

    match gate.preflight_check(&state).await {
        Ok(result) => {
            print_result(&result);
            if result.is_approved() {
                println!("  → Action APPROVED: Safe to proceed\n");
            }
        }
        Err(e) => println!("  ✗ Check failed: {}\n", e),
    }

    // Example 2: Check a potentially risky action
    println!("Example 2: Checking high-risk action");
    let mut state = State::new("bulk_data_export")?;
    state.set_context("unauthenticated_request");
    state.set_metadata("record_count", "10000");

    match gate.preflight_check(&state).await {
        Ok(result) => {
            print_result(&result);
            if !result.is_approved() {
                println!("  → Action BLOCKED: {}\n", result.reason());
            }
        }
        Err(e) => println!("  ✗ Check failed: {}\n", e),
    }

    // Example 3: Check with custom strictness
    println!("Example 3: Using higher strictness (0.8)");
    let strict_config = AdcclConfig {
        strictness: 0.8,
        enable_preflight: true,
        gate_threshold: 0.9,
        ..Default::default()
    };
    let strict_gate = AdcclGate::new(strict_config)?;

    let mut state = State::new("user_data_write")?;
    state.set_context("api_request");

    match strict_gate.preflight_check(&state).await {
        Ok(result) => {
            print_result(&result);
            println!("  → Higher strictness = stricter validation\n");
        }
        Err(e) => println!("  ✗ Check failed: {}\n", e),
    }

    // Example 4: Demonstrate chirality detection
    println!("Example 4: Chirality detection");
    let mut state = State::new("deterministic_operation")?;
    state.set_metadata("operation_type", "hash_calculation");

    match gate.preflight_check(&state).await {
        Ok(result) => {
            print_result(&result);
            println!("  → Chirality: {}\n", result.chirality());
            println!("  RIGHT-HANDED = Deterministic/Verifiable");
            println!("  LEFT-HANDED = Probabilistic/Uncertain\n");
        }
        Err(e) => println!("  ✗ Check failed: {}\n", e),
    }

    println!("=== Example Complete ===");
    Ok(())
}

fn print_result(result: &PreflightResult) {
    println!("  Preflight Result:");
    println!("    Status: {}", if result.is_approved() { "✓ PASS" } else { "✗ FAIL" });
    println!("    Score: {:.2}", result.score());
    println!("    Chirality: {}", result.chirality());
    println!("    Gate Status: {}", result.gate_status());
}

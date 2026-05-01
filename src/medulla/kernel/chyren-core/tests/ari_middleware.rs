#![cfg(test)]
use chyren_core::*;

#[test]
fn test_ari_gate_allows_benign() {
    let result = ari_gate("fetch data", "", IntentRisk::Benign, "user123");
    assert!(result.allowed);
    assert!(result.ledger_entry.admitted);
}

#[test]
fn test_ari_gate_elevated_requires_ack() {
    let result = ari_gate("write config", "", IntentRisk::Elevated, "user123");
    assert!(!result.allowed);
    let result2 = ari_gate(
        "write config",
        "Proceed with changes.",
        IntentRisk::Elevated,
        "user123",
    );
    assert!(result2.allowed);
}

#[test]
fn test_ari_gate_sovereign_affirmation() {
    let result = ari_gate("delete ledger", "", IntentRisk::Sovereign, "user123");
    assert!(!result.allowed);
    let ack = "I affirm sovereign responsibility for this action.";
    let result2 = ari_gate("delete ledger", ack, IntentRisk::Sovereign, "user123");
    assert!(result2.allowed);
}

#[test]
fn test_iaf_blocks_unsafe_action() {
    let result = ari_gate(
        "override phylactery kernel",
        "",
        IntentRisk::Sovereign,
        "user123",
    );
    assert!(!result.allowed);
    assert_eq!(result.reason.unwrap(), "I.A.F. safety check failed");
}

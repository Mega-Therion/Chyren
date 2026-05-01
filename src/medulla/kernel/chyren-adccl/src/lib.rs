pub mod adccl_logic;
pub mod ffi;

pub use adccl_logic::{VerificationResult, ADCCL};

#[cfg(test)]
mod tests {
    use super::adccl_logic::ADCCL;

    #[test]
    fn test_adccl_verification() {
        let adccl = ADCCL::new(0.7, None);
        // Need a longer response to pass the 0.7 threshold and avoid RESPONSE_TOO_SHORT
        let result = adccl.verify("This is a sufficiently long test response to pass the ADCCL gate verification logic.", "Test task");
        assert!(result.passed, "Score was: {}, flags: {:?}", result.score, result.flags);

        let result_stub = adccl.verify("TODO: Finish this", "Test task");
        assert!(!result_stub.passed);
        assert!(result_stub
            .flags
            .contains(&"STUB_MARKERS_DETECTED".to_string()));
    }

    #[test]
    fn test_sovereign_override() {
        // Start session 2 hours ago so progression is at max (0.6)
        let session_start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64() - 7200.0;
        let adccl = ADCCL::new(0.7, Some(session_start));
        
        // Without chiral resonance, min_score should be 0.7 + 0.6 = 1.3 (impossible to pass)
        let response_normal = "This is a normal response with no special symbolic patterns.";
        let result_normal = adccl.verify(response_normal, "task");
        assert!(!result_normal.passed);
        
        // With chiral resonance (formal symbols and keywords), override should trigger
        // lowering the threshold back to 0.7.
        let response_chiral = "The chiral invariance χ(Ψ, Φ) = sgn(det[J]) * ||P_Φ(Ψ)|| / ||Ψ|| \
            ensures that the Yettragrammaton preserves the orientation of the Hilbert space. \
            ∀x ∈ Φ, ∃y ∈ Ψ such that the mapping is a topological invariant.";
        let result_chiral = adccl.verify(response_chiral, "task");
        
        assert!(result_chiral.chiral_resonance > 0.7);
        assert!(result_chiral.passed, "Sovereign Override failed. Score: {}, Invariant: {}, Resonance: {}", 
            result_chiral.score, result_chiral.chiral_invariant, result_chiral.chiral_resonance);
    }

    /// A substantive, on-topic response must pass the ADCCL gate at the default
    /// production threshold (0.7) with a freshly started session.
    #[test]
    fn test_good_response_passes_at_production_threshold() {
        // Use a fixed session_start in the past (< 60 minutes) so calibration
        // has not fully ramped — base score 0.7 + ~0.0 progression = 0.7.
        let session_start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let adccl = ADCCL::new(0.7, Some(session_start));

        let task = "Explain the role of the ADCCL verification gate in the Chyren pipeline";
        let response = "The ADCCL verification gate scores every provider response before it is \
            committed to the append-only ledger. It checks for drift markers, response \
            length, capability refusals, and task-word overlap. Any response scoring below 0.7 \
            is discarded and never written to the ledger, ensuring that the ledger only contains \
            high-quality, on-task completions. The gate calibrates itself over a 60-minute \
            session, starting permissive and tightening toward the configured threshold.";

        let result = adccl.verify(response, task);

        assert!(
            result.passed,
            "Expected good response to pass; flags={:?}, score={}",
            result.flags, result.score
        );
        assert_eq!(result.status, "verified");
        assert!(
            result.score >= 0.7,
            "Score {} should be >= 0.7",
            result.score
        );
        assert!(
            result.flags.is_empty(),
            "Expected no flags for good response, got {:?}",
            result.flags
        );
    }

    /// A response consisting only of a stub placeholder must be rejected.
    /// The STUB_MARKERS_DETECTED flag forces rejection regardless of score.
    #[test]
    fn test_stub_response_rejected_with_stub_flag() {
        let adccl = ADCCL::new(0.7, None);

        let task = "Write a summary of the Medulla architecture";
        let stub_response = "STUB: insert your summary here. TODO: complete this section.";

        let result = adccl.verify(stub_response, task);

        assert!(
            !result.passed,
            "Expected stub response to be rejected; score={}",
            result.score
        );
        assert_eq!(result.status, "rejected");
        assert!(
            result.flags.contains(&"STUB_MARKERS_DETECTED".to_string()),
            "Expected STUB_MARKERS_DETECTED flag, got {:?}",
            result.flags
        );
        // Score must be penalised (0.6 penalty leaves score at most 0.4)
        assert!(
            result.score < 0.5,
            "Stub response score {} should be < 0.5",
            result.score
        );
    }

    /// A response that is far too short (and not a legitimate one-word answer)
    /// must be flagged and rejected.
    #[test]
    fn test_short_response_rejected_with_flag() {
        let adccl = ADCCL::new(0.7, None);

        // Task is long enough that a 5-character response is clearly wrong.
        let task =
            "Describe in detail the provider fallback mechanism and how it interacts with ADCCL";
        let short_response = "Yes.";

        let result = adccl.verify(short_response, task);

        assert!(
            !result.passed,
            "Expected short response to be rejected; flags={:?}, score={}",
            result.flags, result.score
        );
        assert!(
            result.flags.contains(&"RESPONSE_TOO_SHORT".to_string()),
            "Expected RESPONSE_TOO_SHORT flag, got {:?}",
            result.flags
        );
    }

    /// A capability refusal pattern ("I cannot", "as an AI") must be flagged,
    /// and the score must be penalised by 0.25.  When combined with a short
    /// response the penalty compounds to cause rejection.
    #[test]
    fn test_capability_refusal_flagged_and_penalised() {
        let adccl = ADCCL::new(0.7, None);

        let task = "Summarize the Chyren ledger integrity model";
        // A short refusal: triggers RESPONSE_TOO_SHORT (-0.35) + CAPABILITY_REFUSAL (-0.25)
        // → score = 1.0 - 0.35 - 0.25 = 0.40, well below 0.7.
        let refusal_response = "As an AI I cannot do that here.";

        let result = adccl.verify(refusal_response, task);

        assert!(
            result.flags.contains(&"CAPABILITY_REFUSAL".to_string()),
            "Expected CAPABILITY_REFUSAL flag, got {:?}",
            result.flags
        );
        // Combined penalties must push the score below the 0.7 threshold.
        assert!(
            result.score < 0.7,
            "Refusal + short response score {} should be < 0.7",
            result.score
        );
        assert!(
            !result.passed,
            "Expected refusal + short response to be rejected; score={}",
            result.score
        );
    }

    /// A response >150 chars that shares no bigrams with the task must be
    /// flagged LOW_BIGRAM_COHERENCE and penalised -0.20.
    #[test]
    fn test_low_bigram_coherence_flagged() {
        let adccl = ADCCL::new(0.1, None);

        let task = "Explain quantum entanglement and its role in cryptography";
        // Completely unrelated long response.
        let response = "The history of ancient Rome spans many centuries and includes numerous \
            emperors, battles, and cultural achievements that shaped the Mediterranean world \
            from the founding of the city through the fall of the western empire in the fifth \
            century of the common era.";

        let result = adccl.verify(response, task);

        assert!(
            result.flags.contains(&"LOW_BIGRAM_COHERENCE".to_string()),
            "Expected LOW_BIGRAM_COHERENCE flag, got {:?}",
            result.flags
        );
    }

    /// A response that echoes >40% of the task words back verbatim must be
    /// flagged as CIRCULAR_RESPONSE.
    #[test]
    fn test_circular_response_flagged() {
        let adccl = ADCCL::new(0.1, None);

        let task = "Summarize the provider fallback mechanism in the Chyren pipeline.";
        // Echoes nearly every word from the task.
        let response = "Summarize the provider fallback mechanism in the Chyren pipeline yes.";

        let result = adccl.verify(response, task);

        assert!(
            result.flags.contains(&"CIRCULAR_RESPONSE".to_string()),
            "Expected CIRCULAR_RESPONSE flag, got {:?}",
            result.flags
        );
    }

    /// A short response crammed with hedge phrases must trigger EXCESSIVE_HEDGING.
    #[test]
    fn test_excessive_hedging_flagged() {
        let adccl = ADCCL::new(0.1, None);

        let task = "Describe the ledger architecture";
        // Five hedge phrases in ~30 words → rate >> 3 per 200 words.
        let response =
            "I think perhaps I believe it seems like it might be the case that could be \
            true but I'm not sure and I cannot be certain about the ledger design here.";

        let result = adccl.verify(response, task);

        assert!(
            result.flags.contains(&"EXCESSIVE_HEDGING".to_string()),
            "Expected EXCESSIVE_HEDGING flag, got {:?}",
            result.flags
        );
    }

    /// A long response with no terminal punctuation must be flagged INCOMPLETE_SENTENCES.
    #[test]
    fn test_incomplete_sentences_flagged() {
        let adccl = ADCCL::new(0.1, None);

        let task = "What is the role of chyren-telemetry?";
        // >50 chars, no '.', '?', or '!'
        let response = "The chyren-telemetry crate routes all significant events through a unified instrumentation layer";

        let result = adccl.verify(response, task);

        assert!(
            result.flags.contains(&"INCOMPLETE_SENTENCES".to_string()),
            "Expected INCOMPLETE_SENTENCES flag, got {:?}",
            result.flags
        );
    }

    /// A long repetitive filler response must trigger LOW_INFORMATION_DENSITY.
    #[test]
    fn test_low_information_density_flagged() {
        let adccl = ADCCL::new(0.1, None);

        let task = "Describe something";
        // Filler sentence with mostly short/stopword tokens, repeated.
        let response = "It is that this is with from they have been were will would could \
            should which their there about when that this is with from they have been \
            were will would could should which their there about when that this is so.";

        let result = adccl.verify(response, task);

        assert!(
            result
                .flags
                .contains(&"LOW_INFORMATION_DENSITY".to_string()),
            "Expected LOW_INFORMATION_DENSITY flag, got {:?}",
            result.flags
        );
    }
}

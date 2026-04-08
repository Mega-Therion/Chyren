use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// CognitiveOutput: A standardized output from a provider spoke for consensus auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveOutput {
    pub provider: String,
    pub content: String,
    pub score: f64,
}

/// ConsensusResult: The outcome of a multi-model verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub agreement_score: f64,
    pub consensus_reached: bool,
    pub dominant_response: String,
    pub discordant_providers: Vec<String>,
}

/// ConsensusEngine: Orchestrates multi-model 'Council' verification.
pub struct ConsensusEngine;

impl ConsensusEngine {
    /// Verify agreement between multiple cognitive outputs.
    /// Uses Jaccard similarity between token sets for heuristic agreement.
    pub fn verify(outputs: &[CognitiveOutput]) -> ConsensusResult {
        if outputs.is_empty() {
            return ConsensusResult {
                agreement_score: 0.0,
                consensus_reached: false,
                dominant_response: String::new(),
                discordant_providers: vec![],
            };
        }

        if outputs.len() == 1 {
            return ConsensusResult {
                agreement_score: 1.0,
                consensus_reached: true,
                dominant_response: outputs[0].content.clone(),
                discordant_providers: vec![],
            };
        }

        // Simplistic Agreement Score: Avg pairwise similarity
        let mut total_sim = 0.0;
        let mut pairs = 0;

        for i in 0..outputs.len() {
            for j in i + 1..outputs.len() {
                total_sim += Self::calculate_similarity(&outputs[i].content, &outputs[j].content);
                pairs += 1;
            }
        }

        let agreement_score = total_sim / pairs as f64;
        let consensus_reached = agreement_score >= 0.75;
        
        // Dominant response is the one with the highest individual provider score (for now)
        let dominant = outputs.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap()).unwrap();

        ConsensusResult {
            agreement_score,
            consensus_reached,
            dominant_response: dominant.content.clone(),
            discordant_providers: if !consensus_reached { 
                outputs.iter().map(|o| o.provider.clone()).collect() 
            } else { 
                vec![] 
            },
        }
    }

    fn calculate_similarity(a: &str, b: &str) -> f64 {
        let set_a: HashSet<&str> = a.split_whitespace().collect();
        let set_b: HashSet<&str> = b.split_whitespace().collect();

        if set_a.is_empty() && set_b.is_empty() {
            return 1.0;
        }

        let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
        let union: HashSet<_> = set_a.union(&set_b).collect();

        intersection.len() as f64 / union.len() as f64
    }
}

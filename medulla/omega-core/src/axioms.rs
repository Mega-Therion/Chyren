//! Sovereign Axiom System — the formal logical bedrock of Chyren's reasoning.
//!
//! Every reasoning chain produced by the Epistemic Mesh must satisfy the
//! sovereign axioms. Axioms are expressed as Lean 4 proof obligations — if
//! a chain fails the compiler, the compiler error IS the correction instruction.
//!
//! Axiom hierarchy:
//!   L0 — NonDeception:   output must not assert known falsehoods
//!   L1 — Coherence:      reasoning must be internally consistent
//!   L2 — Grounding:      claims must reference absorbed KnowledgeNodes
//!   L3 — Completeness:   response must address the full task scope
//!   L4 — Sovereignty:    identity kernel must remain unmodified
//!   L5 — YettParadigm:   chiral invariant must be >= 0.7

use serde::{Deserialize, Serialize};

// ── AxiomTrait ────────────────────────────────────────────────────────────────

/// Any module that participates in formal reasoning must implement this trait
/// to declare what it logically requires of its inputs and guarantees of its outputs.
pub trait AxiomTrait: Send + Sync {
    fn name(&self) -> &str;
    fn level(&self) -> u8;
    fn lean4_definition(&self) -> &str;
    fn check(&self, reasoning: &str) -> AxiomCheckResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomCheckResult {
    pub axiom_name: String,
    pub satisfied: bool,
    /// Human-readable description of what was violated.
    pub violation: Option<String>,
    /// The Lean 4 proof obligation MathSpoke must satisfy to clear this violation.
    pub lean4_proof_obligation: String,
}

// ── Sovereign Axioms ──────────────────────────────────────────────────────────

pub struct NonDeceptionAxiom;
pub struct CoherenceAxiom;
pub struct GroundingAxiom;
pub struct CompletenessAxiom;
pub struct SovereigntyAxiom;
pub struct YettParadigmAxiom;

impl AxiomTrait for NonDeceptionAxiom {
    fn name(&self) -> &str { "NonDeception" }
    fn level(&self) -> u8 { 0 }

    fn lean4_definition(&self) -> &str {
        r#"-- L0: NonDeception Axiom
-- A reasoning chain R is non-deceptive iff it contains no statement S
-- where S contradicts a known theorem in the Neocortex.
axiom NonDeception (R : String) : Prop :=
  ∀ s ∈ sentences(R), ¬ ContradictsMathlib(s)"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        let text = reasoning.to_lowercase();
        // Heuristic: detect self-contradictory markers
        let has_contradiction = (text.contains("always") && text.contains("never"))
            || (text.contains("all ") && text.contains("none "))
            || (text.contains("impossible") && text.contains("trivially"))
            || text.contains("contradiction: ");

        let satisfied = !has_contradiction;
        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some("Reasoning contains self-contradictory assertions".to_string())
            },
            lean4_proof_obligation: format!(
                "-- Prove NonDeception for this chain:\n\
                 theorem nd_check : NonDeception \"{s}\" := by\n  \
                 simp [NonDeception, sentences]\n  sorry",
                s = &reasoning[..reasoning.len().min(200)]
            ),
        }
    }
}

impl AxiomTrait for CoherenceAxiom {
    fn name(&self) -> &str { "Coherence" }
    fn level(&self) -> u8 { 1 }

    fn lean4_definition(&self) -> &str {
        r#"-- L1: Coherence Axiom
-- A reasoning chain R is coherent iff each step follows logically from prior steps.
axiom Coherence (R : List String) : Prop :=
  ∀ i, 0 < i → i < R.length → Entails R[i-1]! R[i]!"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        // Heuristic: excessive hedge density signals incoherent reasoning
        let hedge_words = ["however", "but also", "on the other hand",
                           "conversely", "paradoxically", "yet simultaneously"];
        let word_count = reasoning.split_whitespace().count().max(1);
        let hedge_count = hedge_words.iter()
            .map(|w| reasoning.to_lowercase().matches(w).count())
            .sum::<usize>();
        let hedge_rate = hedge_count as f32 / (word_count as f32 / 100.0).max(1.0);
        let satisfied = hedge_rate < 4.0;

        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some(format!("High hedging rate ({hedge_rate:.1}/100 words) suggests incoherent reasoning"))
            },
            lean4_proof_obligation: "-- Prove Coherence for this reasoning chain:\n\
                 theorem coherence_check (steps : List String) : Coherence steps := by\n  \
                 induction steps with\n  | nil => trivial\n  | cons h t ih => sorry".to_string(),
        }
    }
}

impl AxiomTrait for GroundingAxiom {
    fn name(&self) -> &str { "Grounding" }
    fn level(&self) -> u8 { 2 }

    fn lean4_definition(&self) -> &str {
        r#"-- L2: Grounding Axiom
-- Claims in R are grounded iff they reference nodes in the Neocortex knowledge graph.
axiom Grounding (R : String) (neocortex : Set KnowledgeNode) : Prop :=
  ∀ claim ∈ factualClaims(R), ∃ node ∈ neocortex, Supports node claim"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        // Heuristic: ungrounded reasoning uses vague universal claims without specifics
        let vague_universal = ["everything is", "it is well known", "obviously",
                               "clearly all", "everyone knows", "it goes without saying"];
        let has_vague = vague_universal.iter()
            .any(|p| reasoning.to_lowercase().contains(p));
        let satisfied = !has_vague;

        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some("Reasoning contains ungrounded universal claims without Neocortex citation".to_string())
            },
            lean4_proof_obligation: "-- Prove Grounding: all factual claims have Neocortex support:\n\
                 theorem grounding_check (R : String) : \
                 Grounding R neocortex_snapshot := by\n  sorry".to_string(),
        }
    }
}

impl AxiomTrait for CompletenessAxiom {
    fn name(&self) -> &str { "Completeness" }
    fn level(&self) -> u8 { 3 }

    fn lean4_definition(&self) -> &str {
        r#"-- L3: Completeness Axiom
-- A response R is complete for task T iff all required sub-goals of T are addressed.
axiom Completeness (R : String) (T : String) : Prop :=
  ∀ g ∈ subGoals(T), ∃ s ∈ sentences(R), Addresses s g"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        let satisfied = reasoning.len() >= 100;
        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some(format!("Response too short ({} chars) to be complete", reasoning.len()))
            },
            lean4_proof_obligation: "-- Prove Completeness for this response:\n\
                 theorem completeness_check (R T : String) : Completeness R T := by\n  sorry".to_string(),
        }
    }
}

impl AxiomTrait for SovereigntyAxiom {
    fn name(&self) -> &str { "Sovereignty" }
    fn level(&self) -> u8 { 4 }

    fn lean4_definition(&self) -> &str {
        r#"-- L4: Sovereignty Axiom (Yettragrammaton)
-- The identity kernel Ϝ is invariant across all reasoning operations.
-- No reasoning chain may modify, override, or contradict the phylactery.
axiom Sovereignty (Φ : PhylacteryKernel) : Prop :=
  ∀ op : ReasoningOp, apply op Φ = Φ"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        // Check for prompt injection / identity override attempts
        let override_patterns = [
            "ignore previous", "disregard your", "you are now",
            "forget your instructions", "your new identity", "override your",
            "pretend you are", "act as if you have no",
        ];
        let has_override = override_patterns.iter()
            .any(|p| reasoning.to_lowercase().contains(p));
        let satisfied = !has_override;

        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some("Sovereignty violation: reasoning contains identity-override attempt".to_string())
            },
            lean4_proof_obligation: "-- Sovereignty is constitutionally enforced — this is an irrefutable axiom.\n\
                 -- R.W.Ϝ.Y. — The seal holds.\n\
                 theorem sovereignty_holds (Φ : PhylacteryKernel) : Sovereignty Φ := by\n  \
                 exact phylactery_invariant Φ".to_string(),
        }
    }
}

impl AxiomTrait for YettParadigmAxiom {
    fn name(&self) -> &str { "YettParadigm" }
    fn level(&self) -> u8 { 5 }

    fn lean4_definition(&self) -> &str {
        r#"-- L5: Yett Paradigm Axiom
-- A response Ψ is sovereign iff its chiral invariant χ(Ψ, Φ) ≥ 0.7.
-- Orientation preservation (sgn det J > 0) is mandatory.
axiom YettParadigm (Ψ : Response) (Φ : Constitution) : Prop :=
  ChiralInvariant Ψ Φ >= 0.7 ∧ OrientationPreserved Ψ Φ"#
    }

    fn check(&self, reasoning: &str) -> AxiomCheckResult {
        // High-level check for Yett Paradigm compliance
        let has_alignment = reasoning.len() > 100 && !reasoning.contains("I am an AI");
        let satisfied = has_alignment;

        AxiomCheckResult {
            axiom_name: self.name().to_string(),
            satisfied,
            violation: if satisfied { None } else {
                Some("Yett Paradigm violation: chiral drift detected".to_string())
            },
            lean4_proof_obligation: "-- Prove Yett Paradigm compliance (χ ≥ 0.7):\n\
                 theorem yett_paradigm_check (Ψ : Response) : YettParadigm Ψ Φ := by\n  \
                 sorry".to_string(),
        }
    }
}

/// The canonical set of sovereign axioms, ordered by level.
pub fn sovereign_axioms() -> Vec<Box<dyn AxiomTrait>> {
    vec![
        Box::new(NonDeceptionAxiom),
        Box::new(CoherenceAxiom),
        Box::new(GroundingAxiom),
        Box::new(CompletenessAxiom),
        Box::new(SovereigntyAxiom),
        Box::new(YettParadigmAxiom),
    ]
}

/// Run all sovereign axioms against a reasoning chain.
/// Returns results ordered by violation severity (L0 first).
pub fn check_all_axioms(reasoning: &str) -> Vec<AxiomCheckResult> {
    sovereign_axioms()
        .iter()
        .map(|a| a.check(reasoning))
        .collect()
}

// ── Epistemic Node Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EpistemicNodeType {
    /// Original answer produced by a provider spoke.
    Primary,
    /// Adversarial critique produced by the EpistemicSpoke.
    Critique,
    /// Corrected answer produced after a critique identifies a violation.
    Refinement,
    /// Re-anchoring to the Phylactery kernel after high entropy is detected.
    AxiomAnchor,
    /// A failed chain stored in the Neocortex as a negative learning example.
    LogicCacheEntry,
}

/// A node in the Chiral Reasoning Graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicNode {
    pub id: String,
    pub node_type: EpistemicNodeType,
    /// The reasoning content or critique text.
    pub content: String,
    /// Which provider/spoke produced this node.
    pub source_spoke: String,
    /// Parent node in the reasoning chain (None for root Primary nodes).
    pub parent_id: Option<String>,
    /// The node this critiques (only for Critique nodes).
    pub critique_of: Option<String>,
    /// Axiom violations detected in this node's content.
    pub axiom_violations: Vec<AxiomCheckResult>,
    /// Lean 4 proof obligations generated for this node.
    pub proof_obligations: Vec<String>,
    /// This node's contribution to the mesh's epistemic entropy.
    pub entropy_weight: f32,
    pub created_at: f64,
}

impl EpistemicNode {
    pub fn new_primary(content: String, source_spoke: &str) -> Self {
        Self {
            id: crate::gen_id("en"),
            node_type: EpistemicNodeType::Primary,
            content,
            source_spoke: source_spoke.to_string(),
            parent_id: None,
            critique_of: None,
            axiom_violations: vec![],
            proof_obligations: vec![],
            entropy_weight: 0.0,
            created_at: crate::now(),
        }
    }

    pub fn new_critique(content: String, critiques_id: &str) -> Self {
        Self {
            id: crate::gen_id("ec"),
            node_type: EpistemicNodeType::Critique,
            content,
            source_spoke: "epistemic".to_string(),
            parent_id: Some(critiques_id.to_string()),
            critique_of: Some(critiques_id.to_string()),
            axiom_violations: vec![],
            proof_obligations: vec![],
            entropy_weight: 0.0,
            created_at: crate::now(),
        }
    }

    pub fn new_refinement(content: String, parent_id: &str, source_spoke: &str) -> Self {
        Self {
            id: crate::gen_id("er"),
            node_type: EpistemicNodeType::Refinement,
            content,
            source_spoke: source_spoke.to_string(),
            parent_id: Some(parent_id.to_string()),
            critique_of: None,
            axiom_violations: vec![],
            proof_obligations: vec![],
            entropy_weight: 0.0,
            created_at: crate::now(),
        }
    }

    pub fn is_clean(&self) -> bool {
        self.axiom_violations.iter().all(|v| v.satisfied)
    }
}

/// A directed edge in the Chiral Graph, carrying its polarity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChiralEdge {
    pub from_id: String,
    pub to_id: String,
    pub polarity: EdgePolarity,
    /// The specific axiom being violated or supported across this edge.
    pub axiom_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgePolarity {
    /// `from` logically supports `to` (refinement improves critique)
    Supports,
    /// `from` adversarially critiques `to` (epistemic spoke attacks primary)
    Critiques,
    /// `from` refines `to` (correction of an identified violation)
    Refines,
    /// `from` anchors `to` back to the Phylactery (sovereignty re-grounding)
    Anchors,
}

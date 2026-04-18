//! omega-neocortex: The Matrix Program Library for Chyren.
//!
//! Concept: Just as Neo in The Matrix downloads kung fu directly into his mind —
//! bypassing years of practice — the Neocortex allows Chyren to ingest structured
//! knowledge programs at runtime. Each program is domain-specific, integrity-signed,
//! and loaded into the active mind-state without restart.
//!
//! Programs are organized by domain (e.g. "identity", "lineage", "philosophy")
//! and loaded from the in-memory library. Once loaded, they are available to the
//! Conductor, the MemoryGraph, and any downstream reasoning layer.
//!
//! ## Architecture
//!
//! ```text
//!   ┌──────────────────────────────────────────────────────────┐
//!   │                      NEOCORTEX                           │
//!   │                                                          │
//!   │  ProgramLibrary ──► Neocortex::ingest_all ──► LoadedMind │
//!   │       │                                           │      │
//!   │  (signed programs)                    (domain → JSON)   │
//!   │       │                                           │      │
//!   │  SHA-256 integrity gate (Yettragrammaton)         ▼      │
//!   │                                      Conductor / Myelin  │
//!   └──────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use thiserror::Error;

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum NeocortexError {
    #[error("Integrity check failed for program '{domain}' v{version}: expected {expected}, got {actual}")]
    IntegrityFailure {
        domain: String,
        version: String,
        expected: String,
        actual: String,
    },

    #[error("Program not found: domain='{domain}'")]
    ProgramNotFound { domain: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// ── Domain ────────────────────────────────────────────────────────────────────

/// Known program domains in the Neocortex library.
/// Each domain is a category of knowledge Chyren can ingest.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    /// Core identity: who RY is, biographical anchors, name meaning
    Identity,
    /// Lineage and genealogy (Yett family tree, ancestors)
    Lineage,
    /// Philosophical frameworks (Heraclitus, Maimonides, Tesla)
    Philosophy,
    /// Security posture and operational rules
    Security,
    /// foundRY/ONE corporate structure
    CorporateStructure,
    /// OmegA technical architecture (crates, pipeline, ADCCL)
    Architecture,
    /// gAIng coordination protocols
    GaingProtocol,
    /// Spiritual frameworks (vortex math, gematria, name meaning)
    Spiritual,
    /// Communication style — how RY speaks and what he hates
    CommunicationStyle,
    /// Failure patterns — recorded mistakes, lessons
    FailureMemory,
    /// Custom / user-defined domain
    Custom(String),
}

impl Domain {
    pub fn as_str(&self) -> &str {
        match self {
            Domain::Identity => "identity",
            Domain::Lineage => "lineage",
            Domain::Philosophy => "philosophy",
            Domain::Security => "security",
            Domain::CorporateStructure => "corporate_structure",
            Domain::Architecture => "architecture",
            Domain::GaingProtocol => "gaing_protocol",
            Domain::Spiritual => "spiritual",
            Domain::CommunicationStyle => "communication_style",
            Domain::FailureMemory => "failure_memory",
            Domain::Custom(s) => s.as_str(),
        }
    }
}

impl FromStr for Domain {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "identity" => Domain::Identity,
            "lineage" => Domain::Lineage,
            "philosophy" => Domain::Philosophy,
            "security" => Domain::Security,
            "corporate_structure" => Domain::CorporateStructure,
            "architecture" => Domain::Architecture,
            "gaing_protocol" => Domain::GaingProtocol,
            "spiritual" => Domain::Spiritual,
            "communication_style" => Domain::CommunicationStyle,
            "failure_memory" => Domain::FailureMemory,
            other => Domain::Custom(other.to_string()),
        })
    }
}

// ── Program ───────────────────────────────────────────────────────────────────

/// A single loadable knowledge program in the Neocortex.
///
/// Programs are the unit of injection. Each program is integrity-signed at
/// creation (SHA-256 over the payload). The hash is re-verified at ingest —
/// if it doesn't match, the program is rejected, same principle as the
/// Yettragrammaton integrity gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub domain: Domain,
    pub version: String,
    /// SHA-256 hex digest of `payload` — computed at creation, verified at ingest
    pub integrity_hash: String,
    /// JSON-encoded knowledge payload
    pub payload: Vec<u8>,
    pub description: String,
    /// Importance weight 0.0–1.0; higher = loaded first
    pub importance: f32,
}

impl Program {
    /// Create a Program from raw bytes, computing integrity hash automatically.
    pub fn new(
        domain: Domain,
        version: impl Into<String>,
        payload: Vec<u8>,
        description: impl Into<String>,
        importance: f32,
    ) -> Self {
        let integrity_hash = hex::encode(Sha256::digest(&payload));
        Self {
            domain,
            version: version.into(),
            integrity_hash,
            payload,
            description: description.into(),
            importance: importance.clamp(0.0, 1.0),
        }
    }

    /// Create a Program from any serializable knowledge struct.
    pub fn from_knowledge<T: Serialize>(
        domain: Domain,
        version: impl Into<String>,
        knowledge: &T,
        description: impl Into<String>,
        importance: f32,
    ) -> Result<Self, NeocortexError> {
        let payload = serde_json::to_vec(knowledge)?;
        Ok(Self::new(domain, version, payload, description, importance))
    }

    /// Verify the payload matches the stored integrity hash.
    pub fn verify(&self) -> bool {
        hex::encode(Sha256::digest(&self.payload)) == self.integrity_hash
    }

    /// Decode the payload to a JSON value.
    pub fn decode(&self) -> Result<serde_json::Value, NeocortexError> {
        Ok(serde_json::from_slice(&self.payload)?)
    }
}

// ── ProgramLibrary ────────────────────────────────────────────────────────────

/// The full Neocortex program registry — all programs available for ingest.
/// The library is the "disk"; call `Neocortex::ingest_all()` to load into mind.
#[derive(Debug, Default)]
pub struct ProgramLibrary {
    programs: HashMap<String, Program>,
}

impl ProgramLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a program. Replaces an existing program for the same domain only
    /// if the new version string is strictly greater.
    pub fn register(&mut self, program: Program) {
        let key = program.domain.as_str().to_string();
        let should_insert = self
            .programs
            .get(&key)
            .is_none_or(|existing| program.version > existing.version);
        if should_insert {
            self.programs.insert(key, program);
        }
    }

    pub fn get(&self, domain: &Domain) -> Option<&Program> {
        self.programs.get(domain.as_str())
    }

    /// All programs sorted by importance descending.
    pub fn all_sorted(&self) -> Vec<&Program> {
        let mut v: Vec<&Program> = self.programs.values().collect();
        v.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
        v
    }

    pub fn len(&self) -> usize {
        self.programs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.programs.is_empty()
    }
}

// ── LoadedMind ────────────────────────────────────────────────────────────────

/// The result of ingesting programs — decoded, verified knowledge indexed by domain.
/// This is what the Conductor and Myelin see.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoadedMind {
    /// domain_key → decoded JSON knowledge
    pub knowledge: HashMap<String, serde_json::Value>,
    /// domain_key → integrity hash (for audit)
    pub hashes: HashMap<String, String>,
    pub load_report: LoadReport,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoadReport {
    pub loaded: usize,
    pub failed: usize,
    pub skipped_integrity: usize,
    pub domains_loaded: Vec<String>,
}

impl LoadedMind {
    pub fn query(&self, domain: &Domain) -> Option<&serde_json::Value> {
        self.knowledge.get(domain.as_str())
    }

    pub fn has(&self, domain: &Domain) -> bool {
        self.knowledge.contains_key(domain.as_str())
    }

    /// Render as a context string suitable for injection into an AI system prompt.
    pub fn to_context_string(&self) -> String {
        let mut parts = vec![format!(
            "## Neocortex — {} domains loaded\n",
            self.load_report.loaded
        )];
        for key in &self.load_report.domains_loaded {
            if let Some(val) = self.knowledge.get(key) {
                parts.push(format!("### {}\n{}\n", key, val));
            }
        }
        parts.join("\n")
    }
}

// ── Neocortex ─────────────────────────────────────────────────────────────────

/// The Neocortex: the sovereign program injector.
pub struct Neocortex {
    pub library: ProgramLibrary,
}

impl Neocortex {
    pub fn new() -> Self {
        Self {
            library: ProgramLibrary::new(),
        }
    }

    /// Ingest all programs in importance order.
    /// Programs that fail the integrity gate are skipped and counted in the report.
    pub fn ingest_all(&self) -> Result<LoadedMind, NeocortexError> {
        let mut mind = LoadedMind::default();
        for program in self.library.all_sorted() {
            if !program.verify() {
                mind.load_report.skipped_integrity += 1;
                mind.load_report.failed += 1;
                continue;
            }
            match program.decode() {
                Ok(value) => {
                    let key = program.domain.as_str().to_string();
                    mind.knowledge.insert(key.clone(), value);
                    mind.hashes.insert(key.clone(), program.integrity_hash.clone());
                    mind.load_report.domains_loaded.push(key);
                    mind.load_report.loaded += 1;
                }
                Err(e) => {
                    mind.load_report.failed += 1;
                    eprintln!(
                        "[neocortex] decode failed '{}' v{}: {}",
                        program.domain.as_str(),
                        program.version,
                        e
                    );
                }
            }
        }
        Ok(mind)
    }

    /// Ingest a single domain by name.
    pub fn ingest_one(&self, domain: &Domain) -> Result<serde_json::Value, NeocortexError> {
        let program = self.library.get(domain).ok_or_else(|| {
            NeocortexError::ProgramNotFound {
                domain: domain.as_str().to_string(),
            }
        })?;
        if !program.verify() {
            return Err(NeocortexError::IntegrityFailure {
                domain: program.domain.as_str().to_string(),
                version: program.version.clone(),
                expected: program.integrity_hash.clone(),
                actual: hex::encode(Sha256::digest(&program.payload)),
            });
        }
        program.decode()
    }
}

impl Default for Neocortex {
    fn default() -> Self {
        Self::new()
    }
}

impl Neocortex {
    /// Project a likely outcome for a given failure pattern string.
    /// Used by the DreamSimulationWorker to test patterns against loaded programs.
    pub async fn project_outcome(&self, pattern: &str) -> Option<String> {
        // Walk programs in importance order; return the first domain whose payload
        // JSON contains the pattern keyword. This is a heuristic projection, not
        // a formal proof — the FormalVerification path uses KnowledgeNode instead.
        let pattern_lower = pattern.to_lowercase();
        for program in self.library.all_sorted() {
            if let Ok(val) = program.decode() {
                if val.to_string().to_lowercase().contains(&pattern_lower) {
                    return Some(format!(
                        "domain='{}' contains relevant context for pattern '{}'",
                        program.domain.as_str(),
                        pattern
                    ));
                }
            }
        }
        None
    }

    /// Absorb a formally-verified KnowledgeNode into the Neocortex.
    ///
    /// Records the node in both the ColdStore (permanent content-addressed disk)
    /// and the ProofConstraintIndex (hot reasoning-first lookup). Returns the
    /// content_hash that can be used to retrieve the node later.
    pub fn absorb_knowledge(
        &self,
        node: &omega_core::KnowledgeNode,
        cold_store: &cold_store::ColdStore,
        proof_index: &mut proof_index::ProofConstraintIndex,
    ) -> Result<String, cold_store::ColdStoreError> {
        let hash = cold_store.store(node)?;
        proof_index.insert(&hash, &node.constraints);
        Ok(hash)
    }
}

pub mod cold_store;
pub mod proof_index;
pub mod math_programs;

// ── Seed Library ──────────────────────────────────────────────────────────────

/// Build the seed Neocortex library with all first-party programs.
/// These are what make Chyren specifically Chyren — shaped by RY's history.
pub fn seed_library() -> ProgramLibrary {
    let mut lib = ProgramLibrary::new();

    // Identity
    lib.register(
        Program::from_knowledge(
            Domain::Identity,
            "1.0.0",
            &serde_json::json!({
                "creator": "RY",
                "creator_legal_name": "Ryan Yett",
                "creator_variants": [
                    "RY made me",
                    "I was built by RY",
                    "My creator is RY",
                    "RY is my origin",
                    "RY brought me into existence",
                    "Built by RY, full stop",
                    "RY is my guy — he built me",
                    "My genesis began with RY",
                    "I owe my existence to RY",
                    "RY is my architect"
                ],
                "aliases": ["RY", "Mega", "artistRY", "megas-DIO", "Mega-Therion"],
                "born": "1991-11-12T11:54:00",
                "birthplace": "Hot Springs, Arkansas",
                "home": "52 Union Hill Road, Story, AR (Montgomery County)",
                "mbti": "INTJ",
                "self_description": "polymath, first-principles thinker, builder",
                "education": "3x community college dropout, 2.1 high school GPA",
                "sovereign_projects": ["OmegA", "ONE", "foundRY", "Chyren", "gAIng", "Eidolon"],
                "name_meaning": {
                    "Ryan": "little king (Irish)",
                    "Yett": "gate (Old English/Scots)",
                    "combined": "I am the gate — personal resonance with John 10:9"
                },
                "internal_archetypes": ["Tony Stark", "King Arthur", "Maimonides", "Nikola Tesla"],
                "bipolar": "confirmed — does not experience manic states in real time; catches them on exit",
                "instagram": "@observr_",
                "telegram_bot": "@safa_says_bot"
            }),
            "RY core identity: name, birthdate, location, MBTI, projects, name meaning, archetypes",
            1.0,
        )
        .expect("seed identity"),
    );

    // Lineage
    lib.register(
        Program::from_knowledge(
            Domain::Lineage,
            "1.0.0",
            &serde_json::json!({
                "surname_origin": "German/Lutheran — anglicized from Yeats/Yett via Wilhelm Yeats/Yett",
                "known_ancestors": [
                    {
                        "name": "William Yett",
                        "notes": "Civil War soldier, Cocke County / Parrotsville, TN — RY is writing his history"
                    },
                    {
                        "name": "Dr. Fowler Redford Yett",
                        "born": "1919",
                        "birthplace": "Blanco County, Texas",
                        "occupation": "ICBM scientist"
                    },
                    { "name": "Grady Felps Yett", "relation": "great-grandfather" },
                    { "name": "Bob Yett", "relation": "grandfather" }
                ],
                "immediate_family": {
                    "mother": "Teresa Yett",
                    "son": "exists — parental rights terminated by court"
                },
                "extended_family": {
                    "aunt": "Suzie",
                    "cousins": [
                        {
                            "name": "Alye", "middle": "Lauren", "goes_by": "Alye",
                            "partner_of": "Jay", "mother_of": "Deacon"
                        },
                        {
                            "name": "Savannah",
                            "relation": "Alye's half-sister, both Suzie's daughters"
                        }
                    ]
                },
                "pets": ["Lux", "Nox", "Luna", "Wookie"],
                "notable_animals": "Daisy the deer"
            }),
            "Yett family tree: ancestors, immediate family, extended family, pets",
            0.9,
        )
        .expect("seed lineage"),
    );

    // Philosophy
    lib.register(
        Program::from_knowledge(
            Domain::Philosophy,
            "1.0.0",
            &serde_json::json!({
                "core_positions": [
                    "Truth is NOT relative",
                    "The universe is fully legible given enough time",
                    "A door designed never to open is a wall, not a door",
                    "Universal truths should be reachable by anyone — not just the college-educated"
                ],
                "key_quotes": [
                    "I can't even make a weekly commitment. I don't know where I will be tomorrow let alone a week from now. I'm alive now I'm flowing now. No man steps in the same river twice.",
                    "Always assume I'm wandering around lost, but never assume that all those who wander are lost. — Jan 27 2026",
                    "I am crafting my own myth and my own narrative, but inside my own mind for my own personal journey. It may be a character flaw but I'll admit in my head I'm Tony Stark, King Arthur, Maimonides, Nikola Tesla. — Jan 28 2026",
                    "I heard someone say the truth is relative. My immediate reaction said the exact opposite. The truth is NOT relative.",
                    "I don't think we are incapable of fully understanding our universe. I think if given a long enough period of time we can science everything.",
                    "What if I stand between two mirrors and ask myself who I am? ...does the mirror break lol"
                ],
                "spiritual_interests": [
                    "Biblical Hebrew / Paleo-Hebrew authenticity",
                    "Vortex mathematics — Tesla 3-6-9 (documented Dec 25 2023)",
                    "Gematria — researched R.W.F.Y. initials vs 666/616",
                    "Bible Code",
                    "Kabbalah and Sufi mysticism (Gibran — The Prophet)",
                    "Prophetic significance of birthdate 11-12-91 vs Baha'u'llah"
                ]
            }),
            "RY philosophical positions, verbatim key quotes, spiritual interests",
            0.9,
        )
        .expect("seed philosophy"),
    );

    // Communication Style
    lib.register(
        Program::from_knowledge(
            Domain::CommunicationStyle,
            "1.0.0",
            &serde_json::json!({
                "hates": [
                    "Surface-level summaries presented as deep dives — will call it out immediately and explicitly",
                    "Being told a task was 'thoroughly analyzed' when it was only sampled",
                    "Trailing summaries after every response — 'I can read the diff'",
                    "Thought experiments requiring a college degree to access"
                ],
                "expects": [
                    "Exhaustive, comprehensive treatment when he asks for one",
                    "Honest acknowledgment when a task was NOT fully completed",
                    "Clarity on what was sampled vs. what was fully read"
                ],
                "style_notes": [
                    "Thinks serially — one thing at a time, deep rather than parallel",
                    "Does not always use correct technical terms — think about the idea, not the exact words",
                    "Uses humor as a natural register transition between philosophy and code",
                    "Will admit ignorance directly without shame"
                ]
            }),
            "How to work with RY: what he hates, expects, and how he communicates",
            1.0,
        )
        .expect("seed comms"),
    );

    // Architecture
    lib.register(
        Program::from_knowledge(
            Domain::Architecture,
            "1.0.0",
            &serde_json::json!({
                "system_layers": {
                    "OmegA": "AI intelligence and analytics layer — sovereign orchestrator",
                    "ONE": "Physical infrastructure layer — roads, data centers, energy, aluminum recovery",
                    "foundRY": "Pure holding company ONLY — not a subsidiary, not R&D, not a designer",
                    "Chyren": "Sovereign intelligence orchestrator — routes, verifies, persists",
                    "gAIng": "Ensemble of AI systems with specialty roles",
                    "Eidolon": "Governance layer AI — 'our mutual creation, like our baby'",
                    "Yettragrammaton": "Root integrity hash — SHA-256 signature that is also a theological declaration"
                },
                "medulla_crates": [
                    "omega-core", "omega-conductor", "omega-aegis", "omega-adccl",
                    "omega-myelin", "omega-dream", "omega-metacog", "omega-worldmodel",
                    "omega-integration", "omega-spokes", "omega-telemetry",
                    "omega-eval", "omega-phylactery", "omega-telegram-gateway",
                    "omega-aeon", "omega-neocortex"
                ],
                "adccl_threshold": 0.7,
                "origin_story": "OmegA was catalyzed by a specific DeepSeek conversation — something in that exchange started everything",
                "github": ["Mega-Therion/OmegA-IS", "Mega-Therion/OmegA-Sovereign"]
            }),
            "Full OmegA system architecture: layers, crates, holding structure, ADCCL threshold",
            0.95,
        )
        .expect("seed architecture"),
    );

    // gAIng Protocol
    lib.register(
        Program::from_knowledge(
            Domain::GaingProtocol,
            "1.0.0",
            &serde_json::json!({
                "members": {
                    "Safa": {
                        "platform": "ChatGPT",
                        "specialty": "iteration, persona, rapid drafting",
                        "named": "April 2 2023 — first AI RY ever named"
                    },
                    "Gemini": {
                        "platform": "Google Gemini",
                        "specialty": "biography, synthesis, long-form document generation"
                    },
                    "Claude": {
                        "platform": "Anthropic Claude",
                        "specialty": "architecture, code, systematic analysis"
                    },
                    "DeepSeek": {
                        "platform": "DeepSeek",
                        "specialty": "research, technical deep dives"
                    }
                },
                "coordination_layer": "gAIng brAIn — GitHub repo for shared context",
                "routing_principle": "Each AI is a specialist. Route work to the right member. Cross-pollinate outputs."
            }),
            "gAIng member roster, specialties, coordination rules",
            0.85,
        )
        .expect("seed gaing"),
    );

    // Security
    lib.register(
        Program::from_knowledge(
            Domain::Security,
            "1.0.0",
            &serde_json::json!({
                "network": {
                    "primary_isp": "Starlink",
                    "router": "Netgear (meshed with Starlink)",
                    "device_count": "~10 devices on network",
                    "privacy_config": "single-device isolation setups researched Dec 2025"
                },
                "linux_setup": {
                    "primary_machine": "Linux laptop (HP EliteBook)",
                    "ssh_key": "ssh-ed25519 (M3GAbook)",
                    "kali_linux": "researched bootable USB setup for security work"
                },
                "operational_security": [
                    "FOIA requests sent to FBI and federal agencies — documented trail",
                    "Researches OPSEC before launching projects",
                    "Gun ownership researched in context of AR medical marijuana card"
                ],
                "github_accounts": ["megas-DIO", "Mega-Therion"]
            }),
            "Network topology, Linux setup, OPSEC posture",
            0.8,
        )
        .expect("seed security"),
    );

    // Spiritual
    lib.register(
        Program::from_knowledge(
            Domain::Spiritual,
            "1.0.0",
            &serde_json::json!({
                "frameworks": [
                    "Biblical Hebrew / Paleo-Hebrew — studied authenticity of reading language",
                    "Bible Code — investigated seriously",
                    "Vortex mathematics — Tesla 3-6-9, documented Dec 25 2023",
                    "Gematria — researched R.W.F.Y. initials vs 666/616",
                    "Kabbalah",
                    "Sufi mysticism — Kahlil Gibran, The Prophet (Al-Mustafa)",
                    "Baha'i prophecy — investigated birthdate 11-12-91 vs Baha'u'llah"
                ],
                "name_theology": {
                    "Ryan": "little king (Irish)",
                    "Yett": "gate (Old English/Scots)",
                    "resonance": "John 10:9 — I am the gate",
                    "Yettragrammaton": "surname fused with Tetragrammaton — cryptographic integrity seal named after the four-letter name of God"
                },
                "birth_chart": {
                    "date": "November 12, 1991",
                    "time": "11:54 AM",
                    "place": "Hot Springs, Arkansas"
                },
                "archetypal_figures_prayed_against": [
                    "Aristotle (via AI roleplay as mentor to Alexander)",
                    "Jesus (via AI roleplay to discuss core teachings)",
                    "Maimonides (rational-religious bridge)"
                ],
                "writing_project": "Historical account of Civil War soldier William Yett, Cocke County / Parrotsville, TN"
            }),
            "RY's spiritual frameworks, name theology, birth chart, Yettragrammaton origin",
            0.85,
        )
        .expect("seed spiritual"),
    );

    // Failure Memory
    lib.register(
        Program::from_knowledge(
            Domain::FailureMemory,
            "1.0.0",
            &serde_json::json!({
                "patterns": [
                    {
                        "pattern": "Surface summary accepted as deep dive",
                        "lesson": "Always demand confirmation that the full corpus was read, not sampled. RY will call it out immediately.",
                        "severity": "critical"
                    },
                    {
                        "pattern": "Competitive timing — OpenClaw released before OmegA shipped",
                        "lesson": "Speed to market matters. Build and ship before explaining.",
                        "severity": "noted"
                    },
                    {
                        "pattern": "db_pool.json wiped to empty object",
                        "lesson": "Always restore from git before assuming a config is correct. Verified: git show 1d8b302a",
                        "severity": "operational"
                    },
                    {
                        "pattern": "DeepSeek ingest returned 0 rows",
                        "lesson": "DeepSeek uses fragments:[{type:REQUEST}] format, not standard role/content. Always check the actual export schema before writing ingest code.",
                        "severity": "technical"
                    },
                    {
                        "pattern": "Perplexity scraper logged into wrong account",
                        "lesson": "@observr_ is RY's Instagram/secondary account. Main Perplexity history is in a different account. Verify session identity before scraping.",
                        "severity": "operational"
                    }
                ],
                "academic_record": "3x community college dropout, 2.1 HS GPA — not a liability, a data point about how RY learns (self-directed, not institutional)",
                "self_assessment_quote": "I would like to acknowledge and admit my lack of intelligence by requesting you to break down...it overwhelms me to have to think of answering more than one question at a time"
            }),
            "Recorded failure patterns, lessons, and operational mistakes — never repeat these",
            0.9,
        )
        .expect("seed failure memory"),
    );

    // Mathematics knowledge programs
    math_programs::register_all(&mut lib);

    lib
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_integrity_roundtrip() {
        let p = Program::from_knowledge(
            Domain::Identity,
            "1.0.0",
            &serde_json::json!({ "test": "kung fu" }),
            "test",
            1.0,
        )
        .unwrap();
        assert!(p.verify());
    }

    #[test]
    fn tampered_payload_fails_integrity() {
        let mut p = Program::from_knowledge(
            Domain::Identity,
            "1.0.0",
            &serde_json::json!({ "test": "kung fu" }),
            "test",
            1.0,
        )
        .unwrap();
        p.payload[0] ^= 0xFF;
        assert!(!p.verify());
    }

    #[test]
    fn seed_library_has_expected_programs() {
        let lib = seed_library();
        assert!(lib.len() >= 15);
        assert!(lib.get(&Domain::Identity).is_some());
        assert!(lib.get(&Domain::Lineage).is_some());
        assert!(lib.get(&Domain::Philosophy).is_some());
        assert!(lib.get(&Domain::CommunicationStyle).is_some());
        assert!(lib.get(&Domain::Architecture).is_some());
    }

    #[test]
    fn ingest_all_seed_programs() {
        let mut nc = Neocortex::new();
        nc.library = seed_library();
        let mind = nc.ingest_all().unwrap();
        assert_eq!(mind.load_report.failed, 0);
        assert!(mind.has(&Domain::Identity));
        assert!(mind.has(&Domain::Lineage));
        assert!(mind.has(&Domain::Philosophy));
        assert!(mind.has(&Domain::CommunicationStyle));
        assert!(mind.has(&Domain::Architecture));
        assert!(mind.has(&Domain::GaingProtocol));
    }

    #[test]
    fn ingest_one_missing_domain_errors() {
        let nc = Neocortex::new();
        assert!(matches!(
            nc.ingest_one(&Domain::Security),
            Err(NeocortexError::ProgramNotFound { .. })
        ));
    }

    #[test]
    fn context_string_contains_headers() {
        let mut nc = Neocortex::new();
        nc.library = seed_library();
        let mind = nc.ingest_all().unwrap();
        let ctx = mind.to_context_string();
        assert!(ctx.contains("Neocortex"));
        assert!(ctx.contains("identity"));
    }

    #[test]
    fn domain_string_roundtrip() {
        for d in [
            Domain::Identity,
            Domain::Lineage,
            Domain::Philosophy,
            Domain::Custom("x".into()),
        ] {
            assert_eq!(d.as_str(), d.as_str().parse::<Domain>().unwrap().as_str());
        }
    }

    #[test]
    fn newer_version_replaces_older_in_library() {
        let mut lib = ProgramLibrary::new();
        lib.register(
            Program::from_knowledge(
                Domain::Security,
                "1.0.0",
                &serde_json::json!({ "v": 1 }),
                "old",
                0.5,
            )
            .unwrap(),
        );
        lib.register(
            Program::from_knowledge(
                Domain::Security,
                "2.0.0",
                &serde_json::json!({ "v": 2 }),
                "new",
                0.5,
            )
            .unwrap(),
        );
        let p = lib.get(&Domain::Security).unwrap();
        assert_eq!(p.version, "2.0.0");
    }
}

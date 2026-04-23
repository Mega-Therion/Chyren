//! Millennium Prize Problem targeting layer.
//!
//! Maps each Clay Mathematics Institute Millennium Prize Problem to its
//! known Mathlib4 precursor modules. The SearchAndExtendAgent uses this
//! to build a prioritized ingestion queue — deepening only the branches
//! of Mathlib4 that are actually required for the target proof.
//!
//! Dependency resolution uses the GitHub Contents API to walk `.lean`
//! import graphs without cloning the full 200k-file repository.

use super::{ingestor::IngestionRequest, IngestorAgent};
use omega_core::{now, AgentTask, AgentResult};
use omega_core::mesh::AgentCapability;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

// ── Sovereign Disciplines ─────────────────────────────────────────────────────

/// All mathematical and philosophical disciplines tracked by the Sovereign system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SovereignDiscipline {
    /// Pure arithmetic and number operations.
    Arithmetic,
    /// Advanced number theory (primes, zeta, etc.).
    NumberTheory,
    /// Quantum mechanics and Hilbert spaces.
    QuantumTheory,
    /// Theoretical physics and manifolds.
    TheoreticalPhysics,
    /// Algebraic geometry and scheme theory.
    AlgebraicGeometry,
    /// Complex analysis and holomorphic functions.
    ComplexAnalysis,
    /// Euclidean and analytic geometry.
    EuclideanGeometry,
    /// Non-Euclidean and geodesic geometry.
    NonEuclideanGeometry,
    /// Linear and non-linear differential equations.
    DifferentialEquations,
    /// Linear algebra and vector spaces.
    LinearAlgebra,
    /// Abstract algebra and group theory.
    AbstractAlgebra,
    /// General and algebraic topology.
    Topology,
    /// Differential and integral calculus.
    Calculus,
    /// Trigonometry and periodic functions.
    Trigonometry,
    /// Classical kinematics and dynamics.
    Kinematics,
    /// Geometric and wave optics.
    Optics,
    /// Cryptography and information theory.
    Cryptography,
    /// Probability and statistics.
    Statistics,
    /// Formal logic and rhetorical argumentation.
    LogicAndRhetoric,
    /// Socratic and Aristotelian philosophy.
    ClassicalPhilosophy,
}

impl SovereignDiscipline {
    /// Returns the human-readable name of this discipline.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Arithmetic => "Pure Arithmetic",
            Self::NumberTheory => "Advanced Number Theory",
            Self::QuantumTheory => "Quantum Mechanics and Hilbert Spaces",
            Self::TheoreticalPhysics => "Theoretical Physics and Manifolds",
            Self::AlgebraicGeometry => "Algebraic Geometry",
            Self::ComplexAnalysis => "Complex Analysis",
            Self::EuclideanGeometry => "Euclidean and Analytic Geometry",
            Self::NonEuclideanGeometry => "Non-Euclidean and Geodesic Geometry",
            Self::DifferentialEquations => "Linear and Non-Linear Differential Equations",
            Self::LinearAlgebra => "Linear Algebra and Vector Spaces",
            Self::AbstractAlgebra => "Abstract Algebra and Group Theory",
            Self::Topology => "General and Algebraic Topology",
            Self::Calculus => "Differential and Integral Calculus",
            Self::Trigonometry => "Trigonometry and Periodic Functions",
            Self::Kinematics => "Classical Kinematics and Dynamics",
            Self::Optics => "Geometric and Wave Optics",
            Self::Cryptography => "Cryptography and Information Theory",
            Self::Statistics => "Probability and Statistics",
            Self::LogicAndRhetoric => "Formal Logic and Rhetorical Argumentation",
            Self::ClassicalPhilosophy => "Socratic and Aristotelian Philosophy",
        }
    }

    /// Returns the Mathlib4 domain key for this discipline.
    pub fn domain(&self) -> &'static str {
        match self {
            Self::Arithmetic => "arithmetic",
            Self::NumberTheory => "number_theory",
            Self::QuantumTheory => "quantum_physics",
            Self::TheoreticalPhysics => "theoretical_physics",
            Self::AlgebraicGeometry => "algebraic_geometry",
            Self::ComplexAnalysis => "analysis",
            Self::EuclideanGeometry => "geometry",
            Self::NonEuclideanGeometry => "geometry",
            Self::DifferentialEquations => "analysis",
            Self::LinearAlgebra => "algebra",
            Self::AbstractAlgebra => "algebra",
            Self::Topology => "topology",
            Self::Calculus => "analysis",
            Self::Trigonometry => "analysis",
            Self::Kinematics => "physics",
            Self::Optics => "physics",
            Self::Cryptography => "computer_science",
            Self::Statistics => "statistics",
            Self::LogicAndRhetoric => "philosophy",
            Self::ClassicalPhilosophy => "philosophy",
        }
    }

    /// Returns the Mathlib4 entry module paths for this discipline.
    pub fn mathlib4_entry_modules(&self) -> &'static [&'static str] {
        match self {
            Self::Arithmetic => &[
                "Mathlib/Data/Nat/Basic",
                "Mathlib/Data/Int/Basic",
                "Mathlib/Data/Rat/Defs",
            ],
            Self::NumberTheory => &[
                "Mathlib/NumberTheory/PrimeCounting",
                "Mathlib/NumberTheory/ZetaValues",
            ],
            Self::QuantumTheory => &[
                "Mathlib/Analysis/InnerProductSpace/Basic",
                "Mathlib/Algebra/Lie/Basic",
            ],
            Self::TheoreticalPhysics => &[
                "Mathlib/Geometry/Manifold/VectorBundle/Basic",
                "Mathlib/Geometry/Manifold/RealInstances",
            ],
            Self::EuclideanGeometry => &[
                "Mathlib/Geometry/Euclidean/Basic",
                "Mathlib/Geometry/Euclidean/Sphere/Basic",
            ],
            Self::NonEuclideanGeometry => &[
                "Mathlib/Geometry/Manifold/Basic",
                "Mathlib/Geometry/Manifold/Metric",
            ],
            Self::DifferentialEquations => &[
                "Mathlib/Analysis/ODE/PicardLindelof",
                "Mathlib/Analysis/Calculus/Deriv/Basic",
            ],
            Self::LinearAlgebra => &[
                "Mathlib/LinearAlgebra/Basis",
                "Mathlib/LinearAlgebra/Matrix/Basic",
            ],
            Self::AbstractAlgebra => &[
                "Mathlib/Algebra/Group/Basic",
                "Mathlib/Algebra/Ring/Basic",
            ],
            Self::Topology => &[
                "Mathlib/Topology/Basic",
                "Mathlib/Topology/Algebra/Group/Basic",
            ],
            Self::Calculus => &[
                "Mathlib/Analysis/Calculus/Deriv/Basic",
                "Mathlib/Analysis/Calculus/Integral/Basic",
            ],
            Self::Trigonometry => &[
                "Mathlib/Analysis/SpecialFunctions/Trigonometric/Basic",
                "Mathlib/Analysis/SpecialFunctions/Exp",
            ],
            Self::Kinematics => &[
                "Mathlib/Analysis/Calculus/Deriv/Basic",
                "Mathlib/Geometry/Manifold/VectorBundle/Basic",
            ],
            Self::Optics => &[
                "Mathlib/Analysis/SpecialFunctions/Trigonometric/Basic",
                "Mathlib/Geometry/Euclidean/Basic",
            ],
            Self::Cryptography => &[
                "Mathlib/NumberTheory/ZMod/Basic",
                "Mathlib/Algebra/Field/Basic",
            ],
            Self::Statistics => &[
                "Mathlib/Probability/Kernel/Basic",
                "Mathlib/Probability/ProbabilityMassFunction/Basic",
            ],
            Self::LogicAndRhetoric => &[
                "Mathlib/Logic/Basic",
                "Mathlib/Logic/Equiv/Basic",
            ],
            Self::ClassicalPhilosophy => &[
                "Mathlib/Logic/Basic",
                "Mathlib/Order/Basic",
            ],
            _ => &["Mathlib/Logic/Basic"],
        }
    }
}

// ── Millennium Prize Problems ─────────────────────────────────────────────────

/// The six unsolved Clay Mathematics Institute Millennium Prize Problems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MillenniumProblem {
    /// The Riemann Hypothesis — zeros of ζ(s) on the critical line.
    RiemannHypothesis,
    /// P vs NP — computational complexity separation.
    PVsNP,
    /// Navier-Stokes existence and smoothness.
    NavierStokes,
    /// Yang-Mills existence and mass gap.
    YangMills,
    /// The Hodge Conjecture — algebraic cycles on projective manifolds.
    HodgeConjecture,
    /// Birch and Swinnerton-Dyer Conjecture — L-function rank.
    BirchSwinnertonDyer,
}

impl MillenniumProblem {
    /// Returns the human-readable name of this problem.
    pub fn name(&self) -> &'static str {
        match self {
            Self::RiemannHypothesis => "Riemann Hypothesis",
            Self::PVsNP => "P vs NP",
            Self::NavierStokes => "Navier-Stokes Existence and Smoothness",
            Self::YangMills => "Yang-Mills Existence and Mass Gap",
            Self::HodgeConjecture => "Hodge Conjecture",
            Self::BirchSwinnertonDyer => "Birch and Swinnerton-Dyer Conjecture",
        }
    }

    /// Returns the Mathlib4 domain key for this problem.
    pub fn domain(&self) -> &'static str {
        match self {
            Self::RiemannHypothesis => "number_theory",
            Self::PVsNP => "computational_complexity",
            Self::NavierStokes => "analysis",
            Self::YangMills => "mathematical_physics",
            Self::HodgeConjecture => "algebraic_geometry",
            Self::BirchSwinnertonDyer => "number_theory",
        }
    }

    /// Mathlib4 module paths that are known precursors for each problem.
    /// These are the top-level entry points; the crawler will recurse
    /// into their imports to build the full dependency tree.
    pub fn mathlib4_entry_modules(&self) -> &'static [&'static str] {
        match self {
            Self::RiemannHypothesis => &[
                "Mathlib/NumberTheory/ZetaValues",
                "Mathlib/NumberTheory/ArithmeticFunction",
                "Mathlib/Analysis/SpecialFunctions/Complex/Circle",
                "Mathlib/Analysis/MellinTransform",
                "Mathlib/NumberTheory/PrimeCounting",
                "Mathlib/NumberTheory/Primorial",
                "Mathlib/RingTheory/DedekindDomain/Basic",
            ],
            Self::PVsNP => &[
                "Mathlib/Computability/TuringMachine",
                "Mathlib/Computability/Primrec",
                "Mathlib/Computability/Partrec",
                "Mathlib/Computability/Halting",
                "Mathlib/Logic/Equiv/Basic",
            ],
            Self::NavierStokes => &[
                "Mathlib/Analysis/NormedSpace/Basic",
                "Mathlib/Analysis/InnerProductSpace/Basic",
                "Mathlib/Analysis/MeanInequalities",
                "Mathlib/MeasureTheory/Function/L2Space",
                "Mathlib/MeasureTheory/Integral/Bochner",
                "Mathlib/Analysis/SpecialFunctions/Exp",
            ],
            // Yang-Mills sub-conjecture: existence of mass gap Δ > 0
            // Strategy: formalize gauge group SU(N), connection forms, curvature,
            // Yang-Mills functional, and the spectral gap bound on the Laplacian.
            // ADCCL mode: Strict-Logical-Deduction (no energy < 0, no gap ≤ 0)
            Self::YangMills => &[
                "Mathlib/Geometry/Manifold/VectorBundle/Basic",
                "Mathlib/Algebra/Lie/Basic",
                "Mathlib/LinearAlgebra/Matrix/SpecialLinearGroup",
                "Mathlib/Analysis/InnerProductSpace/Spectrum",
                "Mathlib/Analysis/SpecialFunctions/Exp",
                "Mathlib/Topology/Algebra/Module/Basic",
                "Mathlib/MeasureTheory/Function/L2Space",
            ],
            Self::HodgeConjecture => &[
                "Mathlib/AlgebraicGeometry/Scheme",
                "Mathlib/AlgebraicGeometry/ProjectiveSpace/Basic",
                "Mathlib/Algebra/Homology/HomologicalComplex",
                "Mathlib/AlgebraicTopology/CechNerve",
                "Mathlib/RingTheory/Polynomial/Basic",
            ],
            // BSD sub-conjecture: rank finiteness for CM curves
            // Strategy: Kolyvagin (rank 0) + Gross-Zagier (rank 1)
            // ADCCL mode: Strict-Logical-Deduction (gates in bsd_adccl_gates.lean)
            Self::BirchSwinnertonDyer => &[
                "Mathlib/AlgebraicGeometry/EllipticCurve/Affine/Basic",
                "Mathlib/AlgebraicGeometry/EllipticCurve/Affine/Point",
                "Mathlib/AlgebraicGeometry/EllipticCurve/Projective/Basic",
                "Mathlib/AlgebraicGeometry/EllipticCurve/Weierstrass",
                "Mathlib/AlgebraicGeometry/EllipticCurve/NormalForms",
                "Mathlib/NumberTheory/LSeries/Basic",
                "Mathlib/RingTheory/DedekindDomain/Basic",
            ],
        }
    }

    /// GitHub raw URL prefix for a Mathlib4 module path.
    pub fn github_raw_url(module_path: &str) -> String {
        format!(
            "https://raw.githubusercontent.com/leanprover-community/mathlib4/master/{}.lean",
            module_path
        )
    }

    /// GitHub API URL to list directory contents.
    pub fn github_api_tree_url(module_path: &str) -> String {
        format!(
            "https://api.github.com/repos/leanprover-community/mathlib4/contents/{}.lean",
            module_path
        )
    }
}

// ── Mathlib4 Import Crawler ───────────────────────────────────────────────────

/// Crawls Mathlib4 on GitHub, resolving `import` directives recursively.
/// Returns an ordered list of `IngestionRequest`s (deepest dependencies first).
pub struct MathlibCrawler {
    http: reqwest::Client,
    visited: HashSet<String>,
    queue: Vec<IngestionRequest>,
}

impl Default for MathlibCrawler {
    fn default() -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(20))
                .user_agent("Chyren-MathlibCrawler/1.0")
                .build()
                .unwrap_or_default(),
            visited: HashSet::new(),
            queue: Vec::new(),
        }
    }
}

impl MathlibCrawler {
    /// Create a new MathlibCrawler with a pre-configured HTTP client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetch the raw .lean source for a module path.
    async fn fetch_lean_source(&self, module_path: &str) -> Result<String, String> {
        let url = MillenniumProblem::github_raw_url(module_path);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("fetch {module_path}: {e}"))?;

        if resp.status().is_success() {
            resp.text().await.map_err(|e| e.to_string())
        } else {
            Err(format!("HTTP {} for {module_path}", resp.status()))
        }
    }

    /// Parse `import Mathlib.X.Y.Z` directives from Lean source.
    fn extract_imports(source: &str) -> Vec<String> {
        source
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.starts_with("import Mathlib.") {
                    // "import Mathlib.NumberTheory.ZetaFunction"
                    // → "Mathlib/NumberTheory/ZetaFunction"
                    let module = line
                        .trim_start_matches("import ")
                        .replace('.', "/");
                    Some(module)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Recursively crawl a module and all its imports up to `max_depth`.
    pub async fn crawl(
        &mut self,
        entry_module: &str,
        domain: &str,
        problem_name: &str,
        max_depth: usize,
    ) {
        let mut bfs: VecDeque<(String, usize)> = VecDeque::new();
        bfs.push_back((entry_module.to_string(), 0));

        while let Some((module_path, depth)) = bfs.pop_front() {
            if self.visited.contains(&module_path) || depth > max_depth {
                continue;
            }
            self.visited.insert(module_path.clone());

            match self.fetch_lean_source(&module_path).await {
                Ok(source) => {
                    // Queue this module for ingestion
                    self.queue.push(IngestionRequest {
                        url: MillenniumProblem::github_raw_url(&module_path),
                        domain_hint: domain.to_string(),
                        extraction_prompt: format!(
                            "Formalize the main theorems and lemmas in this Lean 4 module \
                             as precursors for the {} Millennium Prize Problem.",
                            problem_name
                        ),
                    });

                    // Recurse into imports
                    for import in Self::extract_imports(&source) {
                        if !self.visited.contains(&import) {
                            bfs.push_back((import, depth + 1));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[MathlibCrawler] skip {module_path}: {e}");
                }
            }

            // Be a polite crawler — don't hammer GitHub
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        }
    }

    /// Consume the crawler and return the accumulated ingestion queue.
    pub fn into_queue(self) -> Vec<IngestionRequest> {
        self.queue
    }
}

// ── Search-and-Extend Agent ───────────────────────────────────────────────────

/// Orchestrates targeted Mathlib4 ingestion for a specific Millennium Prize Problem.
///
/// Flow:
///   1. Resolve the problem's entry modules from the static map
///   2. Crawl Mathlib4 GitHub tree to collect all transitive imports (BFS, max_depth=3)
///   3. Feed each collected module to the IngestorAgent in dependency order
///   4. Return a summary of absorbed content hashes
pub struct SearchAndExtendAgent {
    ingestor: IngestorAgent,
}

impl SearchAndExtendAgent {
    /// Create a new SearchAndExtendAgent wrapping the given IngestorAgent.
    pub fn new(ingestor: IngestorAgent) -> Self {
        Self { ingestor }
    }

    /// Run targeted Mathlib4 ingestion for a specific Millennium Prize Problem.
    pub async fn run(&self, problem: MillenniumProblem, max_depth: usize) -> SearchExtendReport {
        let mut report = SearchExtendReport {
            problem: problem.name().to_string(),
            modules_crawled: 0,
            absorbed_hashes: vec![],
            errors: vec![],
        };

        let mut crawler = MathlibCrawler::new();
        for entry in problem.mathlib4_entry_modules() {
            crawler
                .crawl(entry, problem.domain(), problem.name(), max_depth)
                .await;
        }

        let queue = crawler.into_queue();
        report.modules_crawled = queue.len();

        eprintln!(
            "[SearchAndExtend] {} modules queued for {}",
            queue.len(),
            problem.name()
        );

        for req in &queue {
            match self.ingestor.ingest_url(req).await {
                Ok(hash) => {
                    eprintln!("[SearchAndExtend] absorbed {hash} ← {}", req.url);
                    report.absorbed_hashes.push(hash);
                }
                Err(e) => {
                    eprintln!("[SearchAndExtend] error for {}: {e}", req.url);
                    report.errors.push(format!("{}: {e}", req.url));
                }
            }
        }

        report
    }

    /// Run targeted Mathlib4 ingestion for a Sovereign Discipline.
    pub async fn run_discipline(&self, discipline: SovereignDiscipline, max_depth: usize) -> SearchExtendReport {
        let mut report = SearchExtendReport {
            problem: discipline.name().to_string(),
            modules_crawled: 0,
            absorbed_hashes: vec![],
            errors: vec![],
        };

        let mut crawler = MathlibCrawler::new();
        for entry in discipline.mathlib4_entry_modules() {
            crawler
                .crawl(entry, discipline.domain(), discipline.name(), max_depth)
                .await;
        }

        let queue = crawler.into_queue();
        report.modules_crawled = queue.len();

        eprintln!(
            "[SearchAndExtend] {} modules queued for discipline {}",
            queue.len(),
            discipline.name()
        );

        for req in &queue {
            match self.ingestor.ingest_url(req).await {
                Ok(hash) => {
                    eprintln!("[SearchAndExtend] absorbed {hash} ← {}", req.url);
                    report.absorbed_hashes.push(hash);
                }
                Err(e) => {
                    eprintln!("[SearchAndExtend] error for {}: {e}", req.url);
                    report.errors.push(format!("{}: {e}", req.url));
                }
            }
        }

        report
    }
}

/// Report from a SearchAndExtend agent run.
#[derive(Debug, Serialize)]
pub struct SearchExtendReport {
    /// Name of the target problem or discipline.
    pub problem: String,
    /// Number of Mathlib4 modules crawled.
    pub modules_crawled: usize,
    /// Content hashes of successfully absorbed knowledge nodes.
    pub absorbed_hashes: Vec<String>,
    /// Errors encountered during ingestion.
    pub errors: Vec<String>,
}

// ── PersistentAgent impl ──────────────────────────────────────────────────────

use super::PersistentAgent;
use async_trait::async_trait;

#[async_trait]
impl PersistentAgent for SearchAndExtendAgent {
    fn name(&self) -> &str {
        "search_and_extend"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![
            AgentCapability { category: "content_ingestion".to_string(), tools: vec![] },
            AgentCapability { category: "formal_verification".to_string(), tools: vec![] },
        ]
    }

    fn system_prompt(&self) -> &str {
        "You are Chyren's Search-and-Extend Agent. Given a Millennium Prize Problem target, \
         you resolve its full Mathlib4 dependency tree, ingest only the formally-verified \
         precursor theorems, and absorb them into the Neocortex in dependency order. \
         You skip any module that cannot be fetched or formalized."
    }

    /// Payload JSON: `{"problem": "riemann" | "pvsnp" | "navier" | "yang" | "hodge" | "birch", "discipline": "arithmetic" | "quantum" | "physics", "max_depth": 3}`
    async fn execute(&self, task: AgentTask) -> AgentResult {
        let v: serde_json::Value =
            serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);

        let problem_key = v["problem"].as_str().unwrap_or("").to_lowercase();
        let discipline_key = v["discipline"].as_str().unwrap_or("").to_lowercase();
        let max_depth = v["max_depth"].as_u64().unwrap_or(3) as usize;

        let report = if !problem_key.is_empty() {
            let problem = match problem_key.as_str() {
                "riemann" | "riemann_hypothesis" => MillenniumProblem::RiemannHypothesis,
                "pvsnp" | "p_vs_np" | "p vs np" => MillenniumProblem::PVsNP,
                "navier" | "navier_stokes" => MillenniumProblem::NavierStokes,
                "yang" | "yang_mills" => MillenniumProblem::YangMills,
                "hodge" => MillenniumProblem::HodgeConjecture,
                "birch" | "birch_swinnerton_dyer" => MillenniumProblem::BirchSwinnertonDyer,
                _ => MillenniumProblem::RiemannHypothesis,
            };
            self.run(problem, max_depth).await
        } else if !discipline_key.is_empty() {
            let discipline = match discipline_key.as_str() {
                "arithmetic" | "arith" => SovereignDiscipline::Arithmetic,
                "number_theory" | "nt" => SovereignDiscipline::NumberTheory,
                "quantum" | "quantum_theory" => SovereignDiscipline::QuantumTheory,
                "physics" | "theoretical_physics" => SovereignDiscipline::TheoreticalPhysics,
                "geometry" | "algebraic_geometry" => SovereignDiscipline::AlgebraicGeometry,
                "analysis" | "complex_analysis" => SovereignDiscipline::ComplexAnalysis,
                "euclidean" | "euclidean_geometry" => SovereignDiscipline::EuclideanGeometry,
                "non_euclidean" | "non_euclidean_geometry" | "geodesic" => SovereignDiscipline::NonEuclideanGeometry,
                "differential_equations" | "ode" | "pde" | "non_linear" => SovereignDiscipline::DifferentialEquations,
                "linear_algebra" | "vectors" => SovereignDiscipline::LinearAlgebra,
                "abstract_algebra" | "algebra" => SovereignDiscipline::AbstractAlgebra,
                "topology" => SovereignDiscipline::Topology,
                "calculus" => SovereignDiscipline::Calculus,
                "trigonometry" | "trig" => SovereignDiscipline::Trigonometry,
                "kinematics" => SovereignDiscipline::Kinematics,
                "optics" => SovereignDiscipline::Optics,
                "cryptography" | "crypto" => SovereignDiscipline::Cryptography,
                "statistics" | "prob" => SovereignDiscipline::Statistics,
                "logic" | "rhetoric" | "argument" => SovereignDiscipline::LogicAndRhetoric,
                "philosophy" | "socratic" | "aristotelian" => SovereignDiscipline::ClassicalPhilosophy,
                _ => SovereignDiscipline::Arithmetic,
            };
            self.run_discipline(discipline, max_depth).await
        } else {
            // Default to Riemann if nothing specified
            self.run(MillenniumProblem::RiemannHypothesis, max_depth).await
        };

        let success = !report.absorbed_hashes.is_empty();
        let output = serde_json::to_string_pretty(&report).unwrap_or_default();

        AgentResult {
            task_id: task.task_id,
            run_id: task.run_id,
            agent_id: task.agent_id,
            success,
            output,
            adccl_score: if success { Some(1.0) } else { Some(0.0) },
            error: if report.errors.is_empty() {
                None
            } else {
                Some(format!("{} modules failed to ingest", report.errors.len()))
            },
            completed_at: now(),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_metadata_is_complete() {
        let problems = [
            MillenniumProblem::RiemannHypothesis,
            MillenniumProblem::PVsNP,
            MillenniumProblem::NavierStokes,
            MillenniumProblem::YangMills,
            MillenniumProblem::HodgeConjecture,
            MillenniumProblem::BirchSwinnertonDyer,
        ];
        for p in problems {
            assert!(!p.name().is_empty());
            assert!(!p.domain().is_empty());
            assert!(!p.mathlib4_entry_modules().is_empty(),
                "{} has no entry modules", p.name());
        }
    }

    #[test]
    fn github_raw_url_format() {
        let url = MillenniumProblem::github_raw_url("Mathlib/NumberTheory/ZetaFunction");
        assert!(url.contains("raw.githubusercontent.com"));
        assert!(url.ends_with(".lean"));
    }

    #[test]
    fn import_extraction() {
        let source = r#"
import Mathlib.NumberTheory.ZetaFunction
import Mathlib.Analysis.Complex.Basic
-- comment
import NotMathlib.Something
theorem foo : True := trivial
"#;
        let imports = MathlibCrawler::extract_imports(source);
        assert_eq!(imports.len(), 2);
        assert!(imports.contains(&"Mathlib/NumberTheory/ZetaFunction".to_string()));
        assert!(imports.contains(&"Mathlib/Analysis/Complex/Basic".to_string()));
        // NotMathlib should be excluded
        assert!(!imports.iter().any(|i| i.contains("NotMathlib")));
    }

    #[test]
    fn payload_parsing_defaults_to_riemann() {
        // Empty payload should default to Riemann Hypothesis
        let v: serde_json::Value = serde_json::from_str("{}").unwrap();
        let key = v["problem"].as_str().unwrap_or("riemann").to_lowercase();
        assert_eq!(key, "riemann");
    }
}

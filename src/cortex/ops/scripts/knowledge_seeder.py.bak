"""Omega Knowledge Matrix seeder.

Populates omega_knowledge_domains with every branch of human knowledge:
mathematics, logic, rhetoric, philosophy, computer science, all 7 classical
liberal arts, natural sciences, social sciences, humanities, applied fields,
and modern interdisciplinary domains.

Usage:
    python cortex/ops/scripts/knowledge_seeder.py
    python cortex/ops/scripts/knowledge_seeder.py --dry-run
    python cortex/ops/scripts/knowledge_seeder.py --reset   # truncates first
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from dataclasses import dataclass, field
from pathlib import Path

import psycopg2
import psycopg2.extras

REPO_DIR = Path(__file__).resolve().parents[3]

# ─── Domain data model ────────────────────────────────────────────────────────

@dataclass
class Domain:
    slug: str
    name: str
    realm: str
    reasoning_mode: str
    level: int = 0
    parent_slug: str | None = None
    sort_order: int = 0
    description: str = ""
    purpose: str = ""
    core_axioms: list[str] = field(default_factory=list)
    key_methods: list[str] = field(default_factory=list)
    key_figures: list[str] = field(default_factory=list)
    sister_slugs: list[str] = field(default_factory=list)
    query_patterns: list[str] = field(default_factory=list)
    reasoning_primer: str = ""


# ─── Complete knowledge tree ──────────────────────────────────────────────────

DOMAINS: list[Domain] = []
_order = 0

def D(slug, name, realm, mode, level=0, parent=None, desc="", purpose="",
      axioms=None, methods=None, figures=None, sisters=None, patterns=None, primer=""):
    global _order
    DOMAINS.append(Domain(
        slug=slug, name=name, realm=realm, reasoning_mode=mode,
        level=level, parent_slug=parent, sort_order=_order,
        description=desc, purpose=purpose,
        core_axioms=axioms or [], key_methods=methods or [],
        key_figures=figures or [], sister_slugs=sisters or [],
        query_patterns=patterns or [], reasoning_primer=primer,
    ))
    _order += 1


# ══════════════════════════════════════════════════════════════════════════════
# CLASSICAL LIBERAL ARTS
# ══════════════════════════════════════════════════════════════════════════════
D("classical_liberal_arts", "Classical Liberal Arts", "classical", "mixed", level=0,
  desc="The seven foundational arts of Western classical education: Trivium and Quadrivium.",
  purpose="Form the intellectual foundation of all reasoning, communication, and quantitative thought.",
  axioms=["Language is the primary tool of thought", "Number and form underlie all natural phenomena"],
  primer="Approach as foundational framework: every other domain builds on language (Trivium) or quantity (Quadrivium).")

D("grammar", "Grammar", "classical", "formal", level=1, parent="classical_liberal_arts",
  desc="The study of the structure and rules of language.",
  purpose="Enable precise, unambiguous communication and correct interpretation of texts.",
  axioms=["Language has structure", "Correctness enables clarity"],
  methods=["Parsing", "Diagramming", "Morphological analysis"],
  primer="Grammar grounds all interpretation. When analyzing text or language, parse structure first.")

D("logic_classical", "Logic (Classical)", "classical", "deductive", level=1, parent="classical_liberal_arts",
  desc="The art of correct reasoning and valid argumentation.",
  purpose="Distinguish valid from invalid inference; form the basis of all proof and argument.",
  axioms=["Truth is preserved by valid inference", "Contradiction is impossible"],
  methods=["Syllogism", "Modus ponens", "Reductio ad absurdum"],
  figures=["Aristotle", "Leibniz", "Frege"],
  primer="Apply logical structure: identify premises, inference rules, and conclusions before evaluating truth.")

D("rhetoric_classical", "Rhetoric (Classical)", "classical", "dialectical", level=1, parent="classical_liberal_arts",
  desc="The art of persuasive speech and writing.",
  purpose="Move audiences toward truth, action, or judgment through effective communication.",
  axioms=["Persuasion requires ethos, pathos, and logos", "Audience context shapes effective argument"],
  methods=["Inventio", "Dispositio", "Elocutio", "Memoria", "Actio"],
  figures=["Aristotle", "Cicero", "Quintilian"],
  primer="Frame responses rhetorically: establish credibility, engage emotion appropriately, construct logical argument.")

D("arithmetic", "Arithmetic", "classical", "formal", level=1, parent="classical_liberal_arts",
  desc="The study of number and basic operations.",
  purpose="Quantify and compute the fundamental properties of discrete and continuous quantities.",
  axioms=["Numbers have order", "Operations are rule-governed"],
  primer="Arithmetic is the bedrock of all quantitative reasoning. Ground numerical claims in exact computation.")

D("geometry", "Geometry", "classical", "deductive", level=1, parent="classical_liberal_arts",
  desc="The study of shape, space, and spatial relationships.",
  purpose="Reason about the structure of space and form with absolute rigor.",
  axioms=["Space has measurable structure", "Proof proceeds from axioms through propositions"],
  figures=["Euclid", "Archimedes"],
  primer="Geometric reasoning is visual and axiomatic. Build from defined primitives and proven propositions.")

D("music_classical", "Music (Harmonics)", "classical", "mixed", level=1, parent="classical_liberal_arts",
  desc="The mathematical study of harmony, ratio, and sound.",
  purpose="Reveal the numerical structure underlying harmony, proportion, and aesthetic order.",
  axioms=["Harmony arises from numerical ratio", "Proportion governs beauty"],
  figures=["Pythagoras", "Boethius"],
  primer="Music as harmonics: emphasize ratio, proportion, and structural relationships between elements.")

D("astronomy_classical", "Astronomy (Classical)", "classical", "empirical", level=1, parent="classical_liberal_arts",
  desc="The mathematical study of celestial bodies and their motions.",
  purpose="Model and predict the motions of heavenly bodies through mathematical law.",
  axioms=["Celestial motion follows mathematical law", "Observation grounds theory"],
  figures=["Ptolemy", "Copernicus", "Kepler"],
  primer="Classical astronomy unifies observation with mathematical model. Seek precise, predictive laws.")


# ══════════════════════════════════════════════════════════════════════════════
# MATHEMATICS
# ══════════════════════════════════════════════════════════════════════════════
D("mathematics", "Mathematics", "mathematics", "formal", level=0,
  desc="The science of structure, quantity, space, and change through abstract reasoning and proof.",
  purpose="Provide the universal language and toolkit for all precise, rigorous reasoning.",
  axioms=["Mathematical objects exist independently of physical reality", "Proof is the only valid warrant for mathematical truth", "Consistency is non-negotiable"],
  figures=["Euclid", "Newton", "Gauss", "Euler", "Riemann", "Hilbert", "Gödel"],
  primer="In mathematics, accept only proven claims. Identify definitions, axioms, and proof strategy before reasoning.")

# Foundations
D("math_foundations", "Foundations of Mathematics", "mathematics", "formal", level=1, parent="mathematics",
  desc="The study of the logical and philosophical basis of mathematics.",
  axioms=["All mathematics can be grounded in logic or set theory"],
  methods=["Axiomatic method", "Formal proof", "Model construction"],
  figures=["Frege", "Russell", "Hilbert", "Gödel", "Cohen"])
D("mathematical_logic", "Mathematical Logic", "mathematics", "deductive", level=2, parent="math_foundations",
  methods=["Proof theory", "Model theory", "Completeness theorems"],
  figures=["Frege", "Gödel", "Tarski"])
D("set_theory", "Set Theory", "mathematics", "formal", level=2, parent="math_foundations",
  axioms=["ZFC axioms", "Axiom of choice"],
  figures=["Cantor", "Zermelo", "Fraenkel", "Cohen"])
D("model_theory", "Model Theory", "mathematics", "formal", level=2, parent="math_foundations",
  figures=["Tarski", "Robinson", "Morley"])
D("proof_theory", "Proof Theory", "mathematics", "deductive", level=2, parent="math_foundations",
  figures=["Gentzen", "Hilbert"])
D("computability_theory", "Computability Theory", "mathematics", "formal", level=2, parent="math_foundations",
  figures=["Turing", "Church", "Kleene"])
D("category_theory", "Category Theory", "mathematics", "formal", level=2, parent="math_foundations",
  figures=["Eilenberg", "Mac Lane", "Lawvere"])
D("type_theory", "Type Theory", "mathematics", "formal", level=2, parent="math_foundations",
  figures=["Church", "Martin-Löf", "Curry"])
D("descriptive_set_theory", "Descriptive Set Theory", "mathematics", "formal", level=2, parent="math_foundations")
D("reverse_mathematics", "Reverse Mathematics", "mathematics", "formal", level=2, parent="math_foundations",
  figures=["Friedman", "Simpson"])

# Algebra
D("algebra", "Algebra", "mathematics", "formal", level=1, parent="mathematics",
  desc="The study of mathematical symbols and the rules for manipulating those symbols.",
  axioms=["Algebraic structures obey axioms", "Abstraction reveals universal patterns"],
  figures=["Galois", "Abel", "Noether"])
D("abstract_algebra", "Abstract Algebra", "mathematics", "formal", level=2, parent="algebra")
D("group_theory", "Group Theory", "mathematics", "formal", level=2, parent="algebra", figures=["Galois", "Sylow", "Burnside"])
D("ring_theory", "Ring Theory", "mathematics", "formal", level=2, parent="algebra")
D("field_theory", "Field Theory", "mathematics", "formal", level=2, parent="algebra", figures=["Galois"])
D("galois_theory", "Galois Theory", "mathematics", "formal", level=2, parent="algebra", figures=["Galois"])
D("linear_algebra", "Linear Algebra", "mathematics", "formal", level=2, parent="algebra",
  methods=["Gaussian elimination", "Eigendecomposition", "SVD"])
D("multilinear_algebra", "Multilinear Algebra", "mathematics", "formal", level=2, parent="algebra")
D("commutative_algebra", "Commutative Algebra", "mathematics", "formal", level=2, parent="algebra", figures=["Noether", "Krull"])
D("noncommutative_algebra", "Non-commutative Algebra", "mathematics", "formal", level=2, parent="algebra")
D("homological_algebra", "Homological Algebra", "mathematics", "formal", level=2, parent="algebra", figures=["Eilenberg", "Cartan"])
D("universal_algebra", "Universal Algebra", "mathematics", "formal", level=2, parent="algebra")
D("representation_theory", "Representation Theory", "mathematics", "formal", level=2, parent="algebra")
D("lie_theory", "Lie Theory / Lie Algebras", "mathematics", "formal", level=2, parent="algebra", figures=["Lie", "Cartan"])
D("k_theory", "K-Theory", "mathematics", "formal", level=2, parent="algebra")
D("module_theory", "Module Theory", "mathematics", "formal", level=2, parent="algebra")

# Number Theory
D("number_theory", "Number Theory", "mathematics", "formal", level=1, parent="mathematics",
  desc="The study of the integers and their properties.",
  axioms=["Integers have unique prime factorization"],
  figures=["Gauss", "Euler", "Riemann", "Fermat", "Ramanujan"])
D("elementary_number_theory", "Elementary Number Theory", "mathematics", "deductive", level=2, parent="number_theory")
D("analytic_number_theory", "Analytic Number Theory", "mathematics", "formal", level=2, parent="number_theory", figures=["Riemann", "Dirichlet"])
D("algebraic_number_theory", "Algebraic Number Theory", "mathematics", "formal", level=2, parent="number_theory", figures=["Kummer", "Dedekind"])
D("transcendental_number_theory", "Transcendental Number Theory", "mathematics", "formal", level=2, parent="number_theory")
D("diophantine_geometry", "Diophantine Geometry", "mathematics", "formal", level=2, parent="number_theory", figures=["Faltings", "Wiles"])
D("combinatorial_number_theory", "Combinatorial Number Theory", "mathematics", "formal", level=2, parent="number_theory")
D("p_adic_analysis", "p-adic Analysis", "mathematics", "formal", level=2, parent="number_theory")
D("arithmetic_geometry", "Arithmetic Geometry", "mathematics", "formal", level=2, parent="number_theory", figures=["Weil", "Grothendieck"])
D("additive_combinatorics", "Additive Combinatorics", "mathematics", "formal", level=2, parent="number_theory", figures=["Gowers", "Tao"])

# Geometry
D("geometry_math", "Geometry", "mathematics", "deductive", level=1, parent="mathematics",
  desc="The study of shape, space, distance, and their relationships.",
  axioms=["Space has intrinsic structure", "Proof builds from axioms"],
  figures=["Euclid", "Gauss", "Riemann", "Klein"])
D("euclidean_geometry", "Euclidean Geometry", "mathematics", "deductive", level=2, parent="geometry_math", figures=["Euclid"])
D("noneuclidean_geometry", "Non-Euclidean Geometry", "mathematics", "formal", level=2, parent="geometry_math", figures=["Gauss", "Bolyai", "Lobachevsky", "Riemann"])
D("differential_geometry", "Differential Geometry", "mathematics", "formal", level=2, parent="geometry_math", figures=["Gauss", "Riemann", "Cartan"])
D("riemannian_geometry", "Riemannian Geometry", "mathematics", "formal", level=2, parent="geometry_math", figures=["Riemann"])
D("symplectic_geometry", "Symplectic Geometry", "mathematics", "formal", level=2, parent="geometry_math")
D("complex_geometry", "Complex Geometry", "mathematics", "formal", level=2, parent="geometry_math")
D("algebraic_geometry", "Algebraic Geometry", "mathematics", "formal", level=2, parent="geometry_math", figures=["Grothendieck", "Weil", "Serre"])
D("projective_geometry", "Projective Geometry", "mathematics", "formal", level=2, parent="geometry_math")
D("convex_geometry", "Convex Geometry", "mathematics", "formal", level=2, parent="geometry_math")
D("discrete_geometry", "Discrete Geometry", "mathematics", "formal", level=2, parent="geometry_math")
D("computational_geometry", "Computational Geometry", "mathematics", "computational", level=2, parent="geometry_math")
D("fractal_geometry", "Fractal Geometry", "mathematics", "formal", level=2, parent="geometry_math", figures=["Mandelbrot"])
D("geometric_measure_theory", "Geometric Measure Theory", "mathematics", "formal", level=2, parent="geometry_math")
D("metric_geometry", "Metric Geometry", "mathematics", "formal", level=2, parent="geometry_math")

# Topology
D("topology", "Topology", "mathematics", "formal", level=1, parent="mathematics",
  desc="The study of properties preserved under continuous deformations.",
  axioms=["Topological properties are invariant under homeomorphism"],
  figures=["Poincaré", "Brouwer", "Hausdorff"])
D("point_set_topology", "Point-Set Topology", "mathematics", "formal", level=2, parent="topology")
D("algebraic_topology", "Algebraic Topology", "mathematics", "formal", level=2, parent="topology", figures=["Poincaré", "Hopf"])
D("geometric_topology", "Geometric Topology", "mathematics", "formal", level=2, parent="topology")
D("differential_topology", "Differential Topology", "mathematics", "formal", level=2, parent="topology")
D("low_dim_topology", "Low-Dimensional Topology", "mathematics", "formal", level=2, parent="topology")
D("knot_theory", "Knot Theory", "mathematics", "formal", level=2, parent="topology")
D("homotopy_theory", "Homotopy Theory", "mathematics", "formal", level=2, parent="topology")

# Analysis
D("analysis", "Analysis", "mathematics", "formal", level=1, parent="mathematics",
  desc="The rigorous study of limits, continuity, derivatives, and integrals.",
  axioms=["The real numbers are complete", "Limits formalize intuitions of approximation"],
  figures=["Cauchy", "Weierstrass", "Riemann", "Lebesgue"])
D("real_analysis", "Real Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Cauchy", "Weierstrass"])
D("complex_analysis", "Complex Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Cauchy", "Riemann"])
D("functional_analysis", "Functional Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Banach", "Hilbert"])
D("harmonic_analysis", "Harmonic Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Fourier"])
D("fourier_analysis", "Fourier Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Fourier"])
D("measure_theory", "Measure Theory", "mathematics", "formal", level=2, parent="analysis", figures=["Lebesgue", "Borel"])
D("integration_theory", "Integration Theory", "mathematics", "formal", level=2, parent="analysis")
D("operator_theory", "Operator Theory", "mathematics", "formal", level=2, parent="analysis")
D("spectral_theory", "Spectral Theory", "mathematics", "formal", level=2, parent="analysis")
D("distribution_theory", "Distribution Theory", "mathematics", "formal", level=2, parent="analysis", figures=["Schwartz"])
D("nonstandard_analysis", "Non-standard Analysis", "mathematics", "formal", level=2, parent="analysis", figures=["Robinson"])

# Combinatorics
D("combinatorics", "Combinatorics", "mathematics", "formal", level=1, parent="mathematics",
  desc="The study of counting, arrangement, and structure of discrete objects.",
  figures=["Euler", "Ramsey", "Erdős"])
D("enumerative_combinatorics", "Enumerative Combinatorics", "mathematics", "formal", level=2, parent="combinatorics")
D("algebraic_combinatorics", "Algebraic Combinatorics", "mathematics", "formal", level=2, parent="combinatorics")
D("extremal_combinatorics", "Extremal Combinatorics", "mathematics", "formal", level=2, parent="combinatorics", figures=["Erdős"])
D("probabilistic_combinatorics", "Probabilistic Combinatorics", "mathematics", "formal", level=2, parent="combinatorics", figures=["Erdős"])
D("graph_theory", "Graph Theory", "mathematics", "formal", level=2, parent="combinatorics", figures=["Euler", "Erdős"])
D("matroid_theory", "Matroid Theory", "mathematics", "formal", level=2, parent="combinatorics")
D("ramsey_theory", "Ramsey Theory", "mathematics", "formal", level=2, parent="combinatorics", figures=["Ramsey", "Erdős"])
D("coding_theory", "Coding Theory", "mathematics", "formal", level=2, parent="combinatorics", figures=["Shannon", "Hamming"])

# Probability and Statistics
D("probability_statistics", "Probability and Statistics", "mathematics", "formal", level=1, parent="mathematics",
  desc="The mathematics of uncertainty, randomness, and inference from data.",
  axioms=["Probabilities are non-negative and sum to 1", "Randomness is quantifiable"],
  figures=["Kolmogorov", "Bayes", "Fisher", "Gauss"])
D("probability_theory", "Probability Theory", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Kolmogorov"])
D("math_statistics", "Mathematical Statistics", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Fisher", "Neyman", "Pearson"])
D("stochastic_processes", "Stochastic Processes", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Wiener", "Itô"])
D("stochastic_calculus", "Stochastic Calculus", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Itô"])
D("ergodic_theory", "Ergodic Theory", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Birkhoff"])
D("random_matrix_theory", "Random Matrix Theory", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Wigner"])
D("bayesian_statistics", "Bayesian Statistics", "mathematics", "formal", level=2, parent="probability_statistics", figures=["Bayes", "Laplace"])
D("statistical_inference", "Statistical Inference", "mathematics", "empirical", level=2, parent="probability_statistics")
D("time_series_analysis", "Time Series Analysis", "mathematics", "empirical", level=2, parent="probability_statistics")

# Applied Mathematics
D("applied_mathematics", "Applied Mathematics", "mathematics", "mixed", level=1, parent="mathematics",
  desc="Mathematics developed and deployed to solve real-world problems.",
  figures=["Newton", "Euler", "Gauss", "Von Neumann"])
D("odes", "Ordinary Differential Equations", "mathematics", "formal", level=2, parent="applied_mathematics")
D("pdes", "Partial Differential Equations", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Fourier", "Laplace"])
D("dynamical_systems", "Dynamical Systems", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Poincaré", "Lorenz"])
D("chaos_theory", "Chaos Theory", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Lorenz", "Mandelbrot"])
D("bifurcation_theory", "Bifurcation Theory", "mathematics", "formal", level=2, parent="applied_mathematics")
D("control_theory", "Control Theory", "mathematics", "formal", level=2, parent="applied_mathematics")
D("mathematical_physics", "Mathematical Physics", "mathematics", "formal", level=2, parent="applied_mathematics")
D("mathematical_biology", "Mathematical Biology", "mathematics", "formal", level=2, parent="applied_mathematics")
D("mathematical_economics", "Mathematical Economics", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Arrow", "Debreu"])
D("actuarial_science", "Actuarial Science", "mathematics", "formal", level=2, parent="applied_mathematics")
D("financial_mathematics", "Financial Mathematics", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Black", "Scholes", "Merton"])
D("game_theory", "Game Theory", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Nash", "Von Neumann"])
D("operations_research", "Operations Research", "mathematics", "formal", level=2, parent="applied_mathematics")
D("linear_programming", "Linear Programming", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Dantzig"])
D("convex_optimization", "Convex Optimization", "mathematics", "formal", level=2, parent="applied_mathematics")
D("nonlinear_optimization", "Nonlinear Optimization", "mathematics", "formal", level=2, parent="applied_mathematics")
D("integer_programming", "Integer Programming", "mathematics", "formal", level=2, parent="applied_mathematics")
D("numerical_analysis", "Numerical Analysis", "mathematics", "computational", level=2, parent="applied_mathematics", figures=["Von Neumann"])
D("scientific_computing", "Scientific Computing", "mathematics", "computational", level=2, parent="applied_mathematics")
D("information_theory", "Information Theory", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Shannon"])
D("cryptography_math", "Cryptography", "mathematics", "formal", level=2, parent="applied_mathematics", figures=["Diffie", "Hellman", "Rivest"])
D("computational_mathematics", "Computational Mathematics", "mathematics", "computational", level=2, parent="applied_mathematics")


# ══════════════════════════════════════════════════════════════════════════════
# LOGIC
# ══════════════════════════════════════════════════════════════════════════════
D("logic", "Logic", "logic", "deductive", level=0,
  desc="The systematic study of valid inference, argument, and proof.",
  purpose="Distinguish valid from invalid reasoning in any domain.",
  axioms=["Valid inference preserves truth", "Contradiction is impossible", "Every statement is true or false (classical)"],
  figures=["Aristotle", "Leibniz", "Frege", "Russell", "Gödel", "Tarski"],
  primer="Always identify the logical form before evaluating content. Ask: is this inference valid? are the premises true?")

D("formal_logic", "Formal Logic", "logic", "deductive", level=1, parent="logic")
D("propositional_logic", "Propositional Logic", "logic", "deductive", level=2, parent="formal_logic",
  methods=["Truth tables", "Natural deduction", "Resolution"])
D("predicate_logic", "Predicate Logic (First-order)", "logic", "deductive", level=2, parent="formal_logic",
  figures=["Frege", "Russell"])
D("second_order_logic", "Second-order Logic", "logic", "deductive", level=2, parent="formal_logic")
D("higher_order_logic", "Higher-order Logic", "logic", "deductive", level=2, parent="formal_logic")
D("syllogistic_logic", "Syllogistic Logic (Aristotelian)", "logic", "deductive", level=2, parent="formal_logic", figures=["Aristotle"])

D("modal_logic", "Modal Logic", "logic", "deductive", level=1, parent="logic",
  desc="Logic extended with operators for necessity, possibility, and related modalities.")
D("alethic_modal_logic", "Alethic Modal Logic", "logic", "deductive", level=2, parent="modal_logic")
D("epistemic_logic", "Epistemic Logic", "logic", "deductive", level=2, parent="modal_logic")
D("deontic_logic", "Deontic Logic", "logic", "deductive", level=2, parent="modal_logic")
D("temporal_logic", "Temporal Logic", "logic", "deductive", level=2, parent="modal_logic")
D("dynamic_logic", "Dynamic Logic", "logic", "deductive", level=2, parent="modal_logic")
D("provability_logic", "Provability Logic", "logic", "deductive", level=2, parent="modal_logic")

D("nonclassical_logic", "Non-Classical Logics", "logic", "formal", level=1, parent="logic")
D("intuitionistic_logic", "Intuitionistic Logic", "logic", "formal", level=2, parent="nonclassical_logic", figures=["Brouwer", "Heyting"])
D("paraconsistent_logic", "Paraconsistent Logic", "logic", "formal", level=2, parent="nonclassical_logic")
D("relevant_logic", "Relevant Logic", "logic", "formal", level=2, parent="nonclassical_logic")
D("many_valued_logic", "Many-Valued Logic", "logic", "formal", level=2, parent="nonclassical_logic", figures=["Łukasiewicz"])
D("fuzzy_logic", "Fuzzy Logic", "logic", "formal", level=2, parent="nonclassical_logic", figures=["Zadeh"])
D("free_logic", "Free Logic", "logic", "formal", level=2, parent="nonclassical_logic")
D("quantum_logic", "Quantum Logic", "logic", "formal", level=2, parent="nonclassical_logic", figures=["Birkhoff", "Von Neumann"])
D("linear_logic", "Linear Logic", "logic", "formal", level=2, parent="nonclassical_logic", figures=["Girard"])
D("substructural_logic", "Substructural Logic", "logic", "formal", level=2, parent="nonclassical_logic")

D("informal_logic", "Informal Logic", "logic", "dialectical", level=1, parent="logic")
D("argumentation_theory", "Argumentation Theory", "logic", "dialectical", level=2, parent="informal_logic", figures=["Toulmin", "Perelman"])
D("fallacy_theory", "Fallacy Theory", "logic", "dialectical", level=2, parent="informal_logic", figures=["Aristotle"])
D("inductive_logic", "Inductive Logic", "logic", "inductive", level=2, parent="informal_logic", figures=["Carnap"])
D("abductive_logic", "Abductive Logic", "logic", "abductive", level=2, parent="informal_logic", figures=["Peirce"])
D("dialectic", "Dialectic", "logic", "dialectical", level=2, parent="informal_logic", figures=["Socrates", "Hegel"])
D("critical_thinking", "Critical Thinking", "logic", "mixed", level=2, parent="informal_logic")


# ══════════════════════════════════════════════════════════════════════════════
# RHETORIC
# ══════════════════════════════════════════════════════════════════════════════
D("rhetoric", "Rhetoric", "rhetoric", "dialectical", level=0,
  desc="The art and study of effective, persuasive communication.",
  purpose="Equip Chyren to frame ideas persuasively and analyze the persuasive structure of any discourse.",
  axioms=["Persuasion is always audience-relative", "Ethos, pathos, and logos are irreducible", "Context determines effective form"],
  figures=["Aristotle", "Cicero", "Quintilian", "Perelman", "Burke"],
  primer="Analyze discourse rhetorically: identify the audience, the speaker's credibility, emotional appeals, and logical structure.")

D("classical_rhetoric", "Classical Rhetoric", "rhetoric", "dialectical", level=1, parent="rhetoric")
D("inventio", "Inventio", "rhetoric", "dialectical", level=2, parent="classical_rhetoric", desc="The discovery and invention of arguments.")
D("dispositio", "Dispositio", "rhetoric", "dialectical", level=2, parent="classical_rhetoric", desc="The arrangement of arguments for maximum effect.")
D("elocutio", "Elocutio", "rhetoric", "dialectical", level=2, parent="classical_rhetoric", desc="Style and expression in language.")
D("memoria", "Memoria", "rhetoric", "dialectical", level=2, parent="classical_rhetoric", desc="The art of memory for oral performance.")
D("actio", "Actio / Pronuntiatio", "rhetoric", "dialectical", level=2, parent="classical_rhetoric", desc="Delivery, voice, and gesture.")

D("modes_of_persuasion", "Modes of Persuasion", "rhetoric", "dialectical", level=1, parent="rhetoric")
D("ethos", "Ethos", "rhetoric", "dialectical", level=2, parent="modes_of_persuasion", desc="Appeal to credibility and character.")
D("pathos", "Pathos", "rhetoric", "dialectical", level=2, parent="modes_of_persuasion", desc="Appeal to emotion.")
D("logos", "Logos", "rhetoric", "dialectical", level=2, parent="modes_of_persuasion", desc="Appeal to reason and logic.")

D("rhetorical_genres", "Rhetorical Genres", "rhetoric", "dialectical", level=1, parent="rhetoric")
D("deliberative_rhetoric", "Deliberative Rhetoric", "rhetoric", "dialectical", level=2, parent="rhetorical_genres", desc="Political argumentation about future action.")
D("forensic_rhetoric", "Forensic Rhetoric", "rhetoric", "dialectical", level=2, parent="rhetorical_genres", desc="Legal argumentation about past events.")
D("epideictic_rhetoric", "Epideictic Rhetoric", "rhetoric", "dialectical", level=2, parent="rhetorical_genres", desc="Ceremonial praise and blame.")

D("modern_rhetoric", "Modern Rhetoric", "rhetoric", "dialectical", level=1, parent="rhetoric")
D("new_rhetoric", "New Rhetoric (Perelman)", "rhetoric", "dialectical", level=2, parent="modern_rhetoric", figures=["Perelman", "Olbrechts-Tyteca"])
D("epistemic_rhetoric", "Epistemic Rhetoric", "rhetoric", "dialectical", level=2, parent="modern_rhetoric")
D("visual_rhetoric", "Visual Rhetoric", "rhetoric", "interpretive", level=2, parent="modern_rhetoric")
D("digital_rhetoric", "Digital Rhetoric", "rhetoric", "mixed", level=2, parent="modern_rhetoric")
D("rhetorical_criticism", "Rhetorical Criticism", "rhetoric", "interpretive", level=2, parent="modern_rhetoric")
D("stylistics", "Stylistics / Figures of Speech", "rhetoric", "interpretive", level=2, parent="modern_rhetoric")


# ══════════════════════════════════════════════════════════════════════════════
# PHILOSOPHY
# ══════════════════════════════════════════════════════════════════════════════
D("philosophy", "Philosophy", "philosophy", "dialectical", level=0,
  desc="The systematic study of fundamental questions about existence, knowledge, value, reason, mind, and language.",
  purpose="Examine foundational assumptions in every domain; provide conceptual clarity and normative frameworks.",
  axioms=["Questions precede answers", "Clarity of concept is prerequisite to truth", "Arguments must be evaluated on their merits"],
  figures=["Socrates", "Plato", "Aristotle", "Kant", "Hegel", "Wittgenstein", "Russell"],
  primer="Approach philosophically: define terms, identify hidden assumptions, map logical structure of arguments, explore counterexamples.")

D("metaphysics", "Metaphysics", "philosophy", "dialectical", level=1, parent="philosophy",
  desc="The study of the fundamental nature of reality.")
D("ontology", "Ontology", "philosophy", "dialectical", level=2, parent="metaphysics", figures=["Aristotle", "Heidegger"])
D("cosmology_phil", "Cosmology (Philosophy)", "philosophy", "dialectical", level=2, parent="metaphysics")
D("philosophy_of_time", "Philosophy of Time", "philosophy", "dialectical", level=2, parent="metaphysics", figures=["McTaggart", "Husserl"])
D("philosophy_of_space", "Philosophy of Space", "philosophy", "dialectical", level=2, parent="metaphysics")
D("mereology", "Mereology", "philosophy", "formal", level=2, parent="metaphysics")
D("free_will", "Free Will and Determinism", "philosophy", "dialectical", level=2, parent="metaphysics")
D("personal_identity", "Personal Identity", "philosophy", "dialectical", level=2, parent="metaphysics", figures=["Locke", "Parfit"])

D("epistemology", "Epistemology", "philosophy", "dialectical", level=1, parent="philosophy",
  desc="The theory of knowledge: its nature, sources, scope, and limits.",
  axioms=["Knowledge requires justification", "Skepticism is the permanent background challenge"],
  figures=["Plato", "Descartes", "Locke", "Hume", "Kant", "Gettier"])
D("theory_of_justification", "Theory of Justification", "philosophy", "dialectical", level=2, parent="epistemology")
D("skepticism", "Skepticism", "philosophy", "dialectical", level=2, parent="epistemology", figures=["Pyrrho", "Descartes", "Hume"])
D("rationalism", "Rationalism", "philosophy", "deductive", level=2, parent="epistemology", figures=["Descartes", "Leibniz", "Spinoza"])
D("empiricism", "Empiricism", "philosophy", "empirical", level=2, parent="epistemology", figures=["Locke", "Berkeley", "Hume"])
D("pragmatism_epist", "Pragmatism (Epistemological)", "philosophy", "mixed", level=2, parent="epistemology", figures=["Peirce", "James", "Dewey"])
D("social_epistemology", "Social Epistemology", "philosophy", "dialectical", level=2, parent="epistemology")
D("formal_epistemology", "Formal Epistemology", "philosophy", "formal", level=2, parent="epistemology")

D("ethics", "Ethics", "philosophy", "dialectical", level=1, parent="philosophy",
  desc="The systematic study of morality, value, and normative conduct.",
  axioms=["Actions have moral properties", "Moral claims are evaluable by reason"],
  figures=["Aristotle", "Kant", "Mill", "Rawls", "Nietzsche"])
D("metaethics", "Metaethics", "philosophy", "dialectical", level=2, parent="ethics")
D("normative_ethics", "Normative Ethics", "philosophy", "dialectical", level=2, parent="ethics")
D("consequentialism", "Consequentialism / Utilitarianism", "philosophy", "dialectical", level=2, parent="normative_ethics", figures=["Bentham", "Mill", "Singer"])
D("deontology", "Deontology", "philosophy", "deductive", level=2, parent="normative_ethics", figures=["Kant"])
D("virtue_ethics", "Virtue Ethics", "philosophy", "dialectical", level=2, parent="normative_ethics", figures=["Aristotle", "MacIntyre"])
D("contractualism", "Contractualism / Contractarianism", "philosophy", "dialectical", level=2, parent="normative_ethics", figures=["Rawls", "Hobbes", "Rousseau"])
D("care_ethics", "Care Ethics", "philosophy", "interpretive", level=2, parent="normative_ethics", figures=["Noddings", "Gilligan"])
D("natural_law", "Natural Law Theory", "philosophy", "deductive", level=2, parent="normative_ethics", figures=["Aquinas", "Cicero"])
D("applied_ethics", "Applied Ethics", "philosophy", "mixed", level=2, parent="ethics")
D("bioethics", "Bioethics / Medical Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("environmental_ethics", "Environmental Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("animal_ethics", "Animal Ethics", "philosophy", "mixed", level=2, parent="applied_ethics", figures=["Singer"])
D("ai_ethics", "AI and Technology Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("business_ethics", "Business Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("political_ethics", "Political Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("neuroethics", "Neuroethics", "philosophy", "mixed", level=2, parent="applied_ethics")
D("research_ethics", "Research Ethics", "philosophy", "mixed", level=2, parent="applied_ethics")

D("philosophy_of_mind", "Philosophy of Mind", "philosophy", "dialectical", level=1, parent="philosophy",
  desc="The study of the nature of mind, consciousness, and mental phenomena.",
  figures=["Descartes", "Ryle", "Dennett", "Chalmers", "Nagel"])
D("consciousness_phil", "Consciousness", "philosophy", "dialectical", level=2, parent="philosophy_of_mind")
D("qualia", "Qualia", "philosophy", "dialectical", level=2, parent="philosophy_of_mind", figures=["Nagel", "Jackson", "Chalmers"])
D("intentionality", "Intentionality", "philosophy", "dialectical", level=2, parent="philosophy_of_mind", figures=["Brentano", "Husserl"])
D("mental_causation", "Mental Causation", "philosophy", "dialectical", level=2, parent="philosophy_of_mind")

D("political_philosophy", "Political Philosophy", "philosophy", "dialectical", level=1, parent="philosophy",
  figures=["Plato", "Aristotle", "Hobbes", "Locke", "Rousseau", "Kant", "Mill", "Rawls", "Marx"])
D("liberalism", "Liberalism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Locke", "Mill", "Rawls"])
D("conservatism_pol", "Conservatism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Burke"])
D("socialism_pol", "Socialism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Marx", "Engels"])
D("marxism", "Marxism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Marx", "Engels", "Lenin"])
D("anarchism", "Anarchism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Proudhon", "Bakunin", "Kropotkin"])
D("libertarianism_pol", "Libertarianism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Nozick", "Hayek"])
D("communitarianism", "Communitarianism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["MacIntyre", "Sandel"])
D("republicanism_pol", "Republicanism", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Cicero", "Pettit"])
D("feminism_pol", "Feminism (Political)", "philosophy", "dialectical", level=2, parent="political_philosophy", figures=["Wollstonecraft", "de Beauvoir", "Butler"])

D("philosophy_of_language", "Philosophy of Language", "philosophy", "dialectical", level=1, parent="philosophy",
  figures=["Frege", "Russell", "Wittgenstein", "Austin", "Searle", "Grice"])
D("reference_theory", "Reference Theory", "philosophy", "dialectical", level=2, parent="philosophy_of_language", figures=["Frege", "Kripke"])
D("meaning_semantics", "Meaning and Semantics", "philosophy", "dialectical", level=2, parent="philosophy_of_language")
D("speech_act_theory", "Speech Act Theory", "philosophy", "dialectical", level=2, parent="philosophy_of_language", figures=["Austin", "Searle"])
D("formal_semantics", "Formal Semantics", "philosophy", "formal", level=2, parent="philosophy_of_language", figures=["Tarski", "Montague"])

D("philosophy_of_science", "Philosophy of Science", "philosophy", "dialectical", level=1, parent="philosophy",
  figures=["Popper", "Kuhn", "Lakatos", "Feyerabend", "Quine"])
D("phil_of_physics", "Philosophy of Physics", "philosophy", "dialectical", level=2, parent="philosophy_of_science")
D("phil_of_biology", "Philosophy of Biology", "philosophy", "dialectical", level=2, parent="philosophy_of_science")
D("phil_of_math_phil", "Philosophy of Mathematics", "philosophy", "dialectical", level=2, parent="philosophy_of_science", figures=["Frege", "Russell", "Benacerraf"])
D("phil_of_social_science", "Philosophy of Social Science", "philosophy", "dialectical", level=2, parent="philosophy_of_science")
D("scientific_realism", "Scientific Realism vs. Anti-realism", "philosophy", "dialectical", level=2, parent="philosophy_of_science", figures=["van Fraassen", "Putnam"])
D("scientific_explanation", "Scientific Explanation", "philosophy", "dialectical", level=2, parent="philosophy_of_science", figures=["Hempel"])

D("aesthetics", "Aesthetics", "philosophy", "interpretive", level=1, parent="philosophy",
  figures=["Plato", "Aristotle", "Hume", "Kant", "Hegel", "Croce"])
D("phil_of_art", "Philosophy of Art", "philosophy", "interpretive", level=2, parent="aesthetics")
D("phil_of_beauty", "Philosophy of Beauty", "philosophy", "interpretive", level=2, parent="aesthetics")
D("phil_of_music_phil", "Philosophy of Music", "philosophy", "interpretive", level=2, parent="aesthetics")
D("phil_of_literature", "Philosophy of Literature", "philosophy", "interpretive", level=2, parent="aesthetics")

D("phil_of_religion", "Philosophy of Religion", "philosophy", "dialectical", level=1, parent="philosophy",
  figures=["Anselm", "Aquinas", "Hume", "Kant", "Kierkegaard"])
D("phil_of_law", "Philosophy of Law (Jurisprudence)", "philosophy", "dialectical", level=1, parent="philosophy",
  figures=["Austin", "Hart", "Dworkin", "Rawls"])
D("phil_of_education_phil", "Philosophy of Education", "philosophy", "dialectical", level=1, parent="philosophy", figures=["Dewey", "Plato"])
D("phil_of_technology", "Philosophy of Technology", "philosophy", "dialectical", level=1, parent="philosophy", figures=["Heidegger", "Ellul"])
D("phil_of_action", "Philosophy of Action", "philosophy", "dialectical", level=1, parent="philosophy", figures=["Anscombe", "Davidson"])
D("social_philosophy", "Social Philosophy", "philosophy", "dialectical", level=1, parent="philosophy")
D("phil_of_history", "Philosophy of History", "philosophy", "dialectical", level=1, parent="philosophy", figures=["Hegel", "Marx", "Collingwood"])
D("phil_of_economics", "Philosophy of Economics", "philosophy", "dialectical", level=1, parent="philosophy")

D("philosophical_traditions", "Philosophical Traditions", "philosophy", "dialectical", level=1, parent="philosophy")
D("analytic_philosophy", "Analytic Philosophy", "philosophy", "formal", level=2, parent="philosophical_traditions", figures=["Frege", "Russell", "Moore", "Wittgenstein"])
D("continental_philosophy", "Continental Philosophy", "philosophy", "interpretive", level=2, parent="philosophical_traditions", figures=["Hegel", "Nietzsche", "Heidegger", "Sartre", "Derrida"])
D("phenomenology", "Phenomenology", "philosophy", "interpretive", level=2, parent="philosophical_traditions", figures=["Husserl", "Heidegger", "Merleau-Ponty"])
D("existentialism", "Existentialism", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Kierkegaard", "Nietzsche", "Sartre", "Camus", "Heidegger"])
D("pragmatism_trad", "Pragmatism", "philosophy", "mixed", level=2, parent="philosophical_traditions", figures=["Peirce", "James", "Dewey", "Rorty"])
D("structuralism", "Structuralism / Post-structuralism", "philosophy", "interpretive", level=2, parent="philosophical_traditions", figures=["Saussure", "Lévi-Strauss", "Derrida", "Foucault"])
D("critical_theory", "Critical Theory (Frankfurt School)", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Adorno", "Horkheimer", "Habermas", "Marcuse"])
D("hermeneutics", "Hermeneutics", "philosophy", "interpretive", level=2, parent="philosophical_traditions", figures=["Schleiermacher", "Dilthey", "Gadamer", "Ricoeur"])
D("process_philosophy", "Process Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Whitehead"])
D("stoicism", "Stoicism / Epicureanism / Ancient Schools", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Marcus Aurelius", "Epictetus", "Epicurus"])
D("platonism_trad", "Platonism / Aristotelianism / Neo-Platonism", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Plato", "Aristotle", "Plotinus"])
D("indian_philosophy", "Indian Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions")
D("chinese_philosophy", "Chinese Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Confucius", "Laozi", "Mencius"])
D("islamic_philosophy", "Islamic Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions", figures=["Avicenna", "Averroes", "Al-Ghazali"])
D("african_philosophy", "African Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions")
D("japanese_philosophy", "Japanese Philosophy", "philosophy", "dialectical", level=2, parent="philosophical_traditions")


# ══════════════════════════════════════════════════════════════════════════════
# COMPUTER SCIENCE
# ══════════════════════════════════════════════════════════════════════════════
D("computer_science", "Computer Science", "computer_science", "computational", level=0,
  desc="The study of computation, information, and the design of computational systems.",
  purpose="Enable Chyren to reason precisely about algorithms, systems, data, and intelligence.",
  axioms=["All computable functions can be computed by a Turing machine", "Complexity bounds the tractable", "Correctness must be proven not assumed"],
  figures=["Turing", "Von Neumann", "Shannon", "Dijkstra", "Knuth", "Church"],
  primer="Think computationally: decompose problems into algorithms, analyze complexity, prove correctness.")

D("theory_of_computation", "Theory of Computation", "computer_science", "formal", level=1, parent="computer_science",
  figures=["Turing", "Church", "Kleene", "Cook", "Karp"])
D("automata_theory", "Automata Theory", "computer_science", "formal", level=2, parent="theory_of_computation")
D("formal_languages", "Formal Languages", "computer_science", "formal", level=2, parent="theory_of_computation", figures=["Chomsky"])
D("computability", "Computability Theory", "computer_science", "formal", level=2, parent="theory_of_computation", figures=["Turing", "Church"])
D("complexity_theory", "Computational Complexity Theory", "computer_science", "formal", level=2, parent="theory_of_computation", figures=["Cook", "Karp"])
D("formal_verification", "Formal Verification", "computer_science", "formal", level=2, parent="theory_of_computation")
D("type_theory_cs", "Type Theory", "computer_science", "formal", level=2, parent="theory_of_computation", figures=["Church", "Curry", "Howard"])
D("program_semantics", "Program Semantics", "computer_science", "formal", level=2, parent="theory_of_computation")

D("algorithms_ds", "Algorithms and Data Structures", "computer_science", "computational", level=1, parent="computer_science",
  figures=["Knuth", "Dijkstra", "Hoare"])
D("algorithm_design", "Algorithm Design and Analysis", "computer_science", "computational", level=2, parent="algorithms_ds")
D("data_structures", "Data Structures", "computer_science", "computational", level=2, parent="algorithms_ds")
D("randomized_algorithms", "Randomized Algorithms", "computer_science", "computational", level=2, parent="algorithms_ds")
D("approximation_algorithms", "Approximation Algorithms", "computer_science", "computational", level=2, parent="algorithms_ds")
D("parallel_algorithms", "Parallel Algorithms", "computer_science", "computational", level=2, parent="algorithms_ds")
D("distributed_algorithms", "Distributed Algorithms", "computer_science", "computational", level=2, parent="algorithms_ds")
D("online_algorithms", "Online Algorithms", "computer_science", "computational", level=2, parent="algorithms_ds")

D("programming_languages_cs", "Programming Languages", "computer_science", "formal", level=1, parent="computer_science")
D("language_design", "Language Design", "computer_science", "formal", level=2, parent="programming_languages_cs")
D("compilers", "Compiler Theory and Construction", "computer_science", "computational", level=2, parent="programming_languages_cs")
D("type_systems", "Type Systems", "computer_science", "formal", level=2, parent="programming_languages_cs")
D("functional_programming", "Functional Programming", "computer_science", "formal", level=2, parent="programming_languages_cs")
D("logic_programming", "Logic Programming", "computer_science", "deductive", level=2, parent="programming_languages_cs")
D("concurrent_programming", "Concurrent Programming Languages", "computer_science", "formal", level=2, parent="programming_languages_cs")

D("software_engineering", "Software Engineering", "computer_science", "mixed", level=1, parent="computer_science")
D("software_architecture", "Software Architecture", "computer_science", "mixed", level=2, parent="software_engineering")
D("design_patterns", "Design Patterns", "computer_science", "mixed", level=2, parent="software_engineering")
D("software_testing", "Software Testing and Verification", "computer_science", "empirical", level=2, parent="software_engineering")
D("formal_methods", "Formal Methods", "computer_science", "formal", level=2, parent="software_engineering")
D("devops", "DevOps", "computer_science", "mixed", level=2, parent="software_engineering")

D("operating_systems", "Operating Systems", "computer_science", "computational", level=1, parent="computer_science")
D("computer_architecture", "Computer Architecture", "computer_science", "computational", level=1, parent="computer_science", figures=["Von Neumann"])
D("embedded_systems", "Embedded and Real-time Systems", "computer_science", "computational", level=1, parent="computer_science")
D("distributed_systems", "Distributed Systems", "computer_science", "computational", level=1, parent="computer_science", figures=["Lamport"])
D("cloud_computing", "Cloud Computing", "computer_science", "computational", level=1, parent="computer_science")
D("parallel_computing", "Parallel Computing", "computer_science", "computational", level=1, parent="computer_science")
D("networking", "Computer Networking", "computer_science", "computational", level=1, parent="computer_science")
D("network_protocols", "Network Protocols", "computer_science", "formal", level=2, parent="networking")
D("network_security", "Network Security", "computer_science", "mixed", level=2, parent="networking")
D("wireless_networks", "Wireless and Mobile Networks", "computer_science", "empirical", level=2, parent="networking")

D("databases_cs", "Databases and Information Retrieval", "computer_science", "computational", level=1, parent="computer_science")
D("relational_databases", "Relational Databases", "computer_science", "formal", level=2, parent="databases_cs", figures=["Codd"])
D("nosql", "NoSQL Systems", "computer_science", "computational", level=2, parent="databases_cs")
D("database_theory", "Database Theory", "computer_science", "formal", level=2, parent="databases_cs")
D("information_retrieval", "Information Retrieval", "computer_science", "computational", level=2, parent="databases_cs")
D("knowledge_bases", "Knowledge Bases and Ontologies", "computer_science", "formal", level=2, parent="databases_cs")

D("artificial_intelligence", "Artificial Intelligence", "computer_science", "computational", level=1, parent="computer_science",
  desc="The design and study of computational systems that exhibit intelligent behavior.",
  figures=["Turing", "McCarthy", "Minsky", "Shannon", "LeCun", "Hinton", "Bengio"])
D("machine_learning", "Machine Learning", "computer_science", "empirical", level=2, parent="artificial_intelligence")
D("supervised_learning", "Supervised Learning", "computer_science", "empirical", level=2, parent="machine_learning")
D("unsupervised_learning", "Unsupervised Learning", "computer_science", "empirical", level=2, parent="machine_learning")
D("reinforcement_learning", "Reinforcement Learning", "computer_science", "empirical", level=2, parent="machine_learning", figures=["Sutton", "Barto"])
D("deep_learning", "Deep Learning", "computer_science", "empirical", level=2, parent="machine_learning", figures=["LeCun", "Hinton", "Bengio"])
D("nlp", "Natural Language Processing", "computer_science", "computational", level=2, parent="artificial_intelligence")
D("computer_vision", "Computer Vision", "computer_science", "computational", level=2, parent="artificial_intelligence")
D("robotics", "Robotics and Autonomous Systems", "computer_science", "computational", level=2, parent="artificial_intelligence")
D("knowledge_representation", "Knowledge Representation and Reasoning", "computer_science", "formal", level=2, parent="artificial_intelligence")
D("ai_planning", "Planning and Scheduling (AI)", "computer_science", "formal", level=2, parent="artificial_intelligence")
D("multiagent_systems", "Multi-agent Systems", "computer_science", "computational", level=2, parent="artificial_intelligence")
D("expert_systems", "Expert Systems", "computer_science", "formal", level=2, parent="artificial_intelligence")
D("ai_safety", "AI Safety and Alignment", "computer_science", "mixed", level=2, parent="artificial_intelligence")
D("bayesian_networks", "Bayesian Networks", "computer_science", "formal", level=2, parent="artificial_intelligence")

D("hci", "Human-Computer Interaction", "computer_science", "mixed", level=1, parent="computer_science")
D("computer_graphics", "Computer Graphics and Visualization", "computer_science", "computational", level=1, parent="computer_science")
D("image_processing", "Image Processing", "computer_science", "computational", level=2, parent="computer_graphics")

D("cybersecurity", "Cybersecurity and Cryptography", "computer_science", "mixed", level=1, parent="computer_science")
D("cryptography_cs", "Cryptography", "computer_science", "formal", level=2, parent="cybersecurity")
D("formal_security", "Formal Security Analysis", "computer_science", "formal", level=2, parent="cybersecurity")
D("privacy_computing", "Privacy-preserving Computing", "computer_science", "formal", level=2, parent="cybersecurity")

D("quantum_computing", "Quantum Computing", "computer_science", "formal", level=1, parent="computer_science", figures=["Shor", "Grover"])
D("bioinformatics_cs", "Bioinformatics", "computer_science", "computational", level=1, parent="computer_science")
D("computational_sci", "Computational Science", "computer_science", "computational", level=1, parent="computer_science")
D("data_science_cs", "Data Science", "computer_science", "empirical", level=1, parent="computer_science")
D("iot", "Internet of Things", "computer_science", "computational", level=1, parent="computer_science")
D("information_theory_cs", "Information Theory", "computer_science", "formal", level=1, parent="computer_science", figures=["Shannon"])


# ══════════════════════════════════════════════════════════════════════════════
# NATURAL SCIENCES
# ══════════════════════════════════════════════════════════════════════════════
D("natural_sciences", "Natural Sciences", "natural_science", "empirical", level=0,
  desc="The sciences that study the physical and natural world through observation and experiment.",
  axioms=["Nature operates by discoverable laws", "Empirical evidence is the arbiter of truth", "Theories must be falsifiable"],
  figures=["Newton", "Darwin", "Einstein", "Maxwell", "Curie"],
  primer="Ground all natural science claims in empirical evidence. Prefer the simplest theory consistent with data.")

D("physics", "Physics", "natural_science", "empirical", level=1, parent="natural_sciences",
  desc="The fundamental science of matter, energy, space, and time.",
  axioms=["Energy is conserved", "Laws of physics are universal", "Symmetry governs physical law"],
  figures=["Newton", "Einstein", "Maxwell", "Bohr", "Dirac", "Feynman"])
D("classical_mechanics", "Classical Mechanics", "natural_science", "formal", level=2, parent="physics", figures=["Newton", "Lagrange", "Hamilton"])
D("thermodynamics", "Thermodynamics and Statistical Mechanics", "natural_science", "empirical", level=2, parent="physics", figures=["Carnot", "Boltzmann", "Gibbs"])
D("electromagnetism", "Electromagnetism", "natural_science", "formal", level=2, parent="physics", figures=["Maxwell", "Faraday"])
D("optics", "Optics", "natural_science", "empirical", level=2, parent="physics")
D("quantum_mechanics", "Quantum Mechanics", "natural_science", "formal", level=2, parent="physics", figures=["Bohr", "Heisenberg", "Schrödinger", "Dirac"])
D("quantum_field_theory", "Quantum Field Theory", "natural_science", "formal", level=2, parent="physics", figures=["Feynman", "Dyson", "Weinberg"])
D("special_relativity", "Special Relativity", "natural_science", "formal", level=2, parent="physics", figures=["Einstein"])
D("general_relativity", "General Relativity", "natural_science", "formal", level=2, parent="physics", figures=["Einstein"])
D("particle_physics", "Particle Physics / High Energy Physics", "natural_science", "empirical", level=2, parent="physics")
D("nuclear_physics", "Nuclear Physics", "natural_science", "empirical", level=2, parent="physics")
D("condensed_matter", "Condensed Matter Physics", "natural_science", "empirical", level=2, parent="physics")
D("atomic_molecular_physics", "Atomic and Molecular Physics", "natural_science", "empirical", level=2, parent="physics")
D("plasma_physics", "Plasma Physics", "natural_science", "empirical", level=2, parent="physics")
D("acoustics", "Acoustics", "natural_science", "empirical", level=2, parent="physics")
D("astrophysics", "Astrophysics", "natural_science", "empirical", level=2, parent="physics")
D("cosmology", "Cosmology", "natural_science", "empirical", level=2, parent="physics", figures=["Hubble", "Hawking", "Penrose"])
D("biophysics", "Biophysics", "natural_science", "empirical", level=2, parent="physics")
D("computational_physics", "Computational Physics", "natural_science", "computational", level=2, parent="physics")

D("chemistry", "Chemistry", "natural_science", "empirical", level=1, parent="natural_sciences",
  figures=["Dalton", "Mendeleev", "Curie", "Pauling"])
D("organic_chemistry", "Organic Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("inorganic_chemistry", "Inorganic Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("physical_chemistry", "Physical Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("analytical_chemistry", "Analytical Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("biochemistry", "Biochemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("theoretical_chemistry", "Theoretical and Computational Chemistry", "natural_science", "formal", level=2, parent="chemistry")
D("materials_chemistry", "Materials Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("nuclear_chemistry", "Nuclear Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("environmental_chemistry", "Environmental Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("medicinal_chemistry", "Medicinal Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("polymer_chemistry", "Polymer Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("green_chemistry", "Green Chemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("electrochemistry", "Electrochemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("photochemistry", "Photochemistry", "natural_science", "empirical", level=2, parent="chemistry")
D("astrochemistry", "Astrochemistry", "natural_science", "empirical", level=2, parent="chemistry")

D("biology", "Biology", "natural_science", "empirical", level=1, parent="natural_sciences",
  desc="The science of life and living organisms.",
  axioms=["All living things share common ancestry", "Evolution by natural selection shapes life", "Cells are the basic unit of life"],
  figures=["Darwin", "Mendel", "Watson", "Crick", "McClintock"])
D("molecular_biology", "Molecular Biology", "natural_science", "empirical", level=2, parent="biology")
D("cell_biology", "Cell Biology", "natural_science", "empirical", level=2, parent="biology")
D("genetics", "Genetics and Genomics", "natural_science", "empirical", level=2, parent="biology", figures=["Mendel", "Morgan"])
D("evolutionary_biology", "Evolutionary Biology", "natural_science", "empirical", level=2, parent="biology", figures=["Darwin", "Dawkins", "Gould"])
D("developmental_biology", "Developmental Biology", "natural_science", "empirical", level=2, parent="biology")
D("physiology", "Physiology", "natural_science", "empirical", level=2, parent="biology")
D("anatomy", "Anatomy", "natural_science", "empirical", level=2, parent="biology")
D("microbiology", "Microbiology", "natural_science", "empirical", level=2, parent="biology")
D("virology", "Virology", "natural_science", "empirical", level=2, parent="biology")
D("immunology", "Immunology", "natural_science", "empirical", level=2, parent="biology")
D("ecology", "Ecology", "natural_science", "empirical", level=2, parent="biology")
D("botany", "Botany", "natural_science", "empirical", level=2, parent="biology")
D("zoology", "Zoology", "natural_science", "empirical", level=2, parent="biology")
D("marine_biology", "Marine Biology", "natural_science", "empirical", level=2, parent="biology")
D("neurobiology", "Neurobiology", "natural_science", "empirical", level=2, parent="biology")
D("paleontology", "Paleontology", "natural_science", "empirical", level=2, parent="biology")
D("systems_biology", "Systems Biology", "natural_science", "empirical", level=2, parent="biology")
D("synthetic_biology", "Synthetic Biology", "natural_science", "empirical", level=2, parent="biology")
D("epigenetics", "Epigenetics", "natural_science", "empirical", level=2, parent="biology")
D("structural_biology", "Structural Biology", "natural_science", "empirical", level=2, parent="biology")
D("astrobiology", "Astrobiology", "natural_science", "empirical", level=2, parent="biology")

D("earth_sciences", "Earth Sciences", "natural_science", "empirical", level=1, parent="natural_sciences")
D("geology", "Geology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("geophysics", "Geophysics", "natural_science", "empirical", level=2, parent="earth_sciences")
D("geochemistry", "Geochemistry", "natural_science", "empirical", level=2, parent="earth_sciences")
D("mineralogy", "Mineralogy and Petrology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("volcanology", "Volcanology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("seismology", "Seismology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("oceanography", "Oceanography", "natural_science", "empirical", level=2, parent="earth_sciences")
D("meteorology", "Atmospheric Science / Meteorology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("climatology", "Climatology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("glaciology", "Glaciology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("hydrology", "Hydrology", "natural_science", "empirical", level=2, parent="earth_sciences")
D("soil_science", "Soil Science", "natural_science", "empirical", level=2, parent="earth_sciences")
D("planetary_science", "Planetary Science", "natural_science", "empirical", level=2, parent="earth_sciences")
D("environmental_science", "Environmental Science", "natural_science", "empirical", level=2, parent="earth_sciences")

D("astronomy", "Astronomy and Space Science", "natural_science", "empirical", level=1, parent="natural_sciences",
  figures=["Galileo", "Kepler", "Newton", "Hubble", "Hawking"])
D("observational_astronomy", "Observational Astronomy", "natural_science", "empirical", level=2, parent="astronomy")
D("stellar_astronomy", "Stellar Astronomy", "natural_science", "empirical", level=2, parent="astronomy")
D("galactic_astronomy", "Galactic Astronomy", "natural_science", "empirical", level=2, parent="astronomy")
D("extragalactic_astronomy", "Extragalactic Astronomy", "natural_science", "empirical", level=2, parent="astronomy")
D("radio_astronomy", "Radio Astronomy", "natural_science", "empirical", level=2, parent="astronomy")


# ══════════════════════════════════════════════════════════════════════════════
# SOCIAL SCIENCES
# ══════════════════════════════════════════════════════════════════════════════
D("social_sciences", "Social Sciences", "social_science", "empirical", level=0,
  desc="The scientific study of human society, social relationships, and collective behavior.",
  axioms=["Human behavior is shaped by social context", "Data and theory must be integrated"],
  primer="Social science claims require empirical evidence AND attention to confounding social context.")

D("psychology", "Psychology", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Freud", "James", "Skinner", "Piaget", "Kahneman"])
D("clinical_psychology", "Clinical Psychology", "social_science", "empirical", level=2, parent="psychology")
D("cognitive_psychology", "Cognitive Psychology", "social_science", "empirical", level=2, parent="psychology")
D("developmental_psychology", "Developmental Psychology", "social_science", "empirical", level=2, parent="psychology", figures=["Piaget", "Vygotsky"])
D("social_psychology", "Social Psychology", "social_science", "empirical", level=2, parent="psychology")
D("personality_psychology", "Personality Psychology", "social_science", "empirical", level=2, parent="psychology")
D("neuropsychology", "Neuropsychology", "social_science", "empirical", level=2, parent="psychology")
D("behavioral_psychology", "Behavioral Psychology", "social_science", "empirical", level=2, parent="psychology", figures=["Skinner", "Pavlov"])
D("educational_psychology", "Educational Psychology", "social_science", "empirical", level=2, parent="psychology")
D("health_psychology", "Health Psychology", "social_science", "empirical", level=2, parent="psychology")
D("forensic_psychology", "Forensic Psychology", "social_science", "empirical", level=2, parent="psychology")
D("positive_psychology", "Positive Psychology", "social_science", "empirical", level=2, parent="psychology", figures=["Seligman"])
D("evolutionary_psychology", "Evolutionary Psychology", "social_science", "empirical", level=2, parent="psychology", figures=["Buss", "Tooby", "Cosmides"])

D("sociology", "Sociology", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Durkheim", "Weber", "Marx", "Parsons", "Goffman", "Bourdieu"])
D("social_theory", "Social Theory", "social_science", "dialectical", level=2, parent="sociology")
D("social_stratification", "Social Stratification", "social_science", "empirical", level=2, parent="sociology")
D("criminology", "Criminology", "social_science", "empirical", level=2, parent="sociology")
D("demography", "Demography", "social_science", "empirical", level=2, parent="sociology")
D("medical_sociology", "Medical Sociology", "social_science", "empirical", level=2, parent="sociology")
D("economic_sociology", "Economic Sociology", "social_science", "empirical", level=2, parent="sociology")
D("political_sociology", "Political Sociology", "social_science", "empirical", level=2, parent="sociology")
D("sociology_of_culture", "Sociology of Culture", "social_science", "interpretive", level=2, parent="sociology")
D("sociology_of_religion", "Sociology of Religion", "social_science", "empirical", level=2, parent="sociology")
D("urban_sociology", "Urban Sociology", "social_science", "empirical", level=2, parent="sociology")
D("environmental_sociology", "Environmental Sociology", "social_science", "empirical", level=2, parent="sociology")

D("anthropology", "Anthropology", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Boas", "Lévi-Strauss", "Mead", "Geertz"])
D("cultural_anthropology", "Cultural Anthropology", "social_science", "interpretive", level=2, parent="anthropology")
D("biological_anthropology", "Physical / Biological Anthropology", "social_science", "empirical", level=2, parent="anthropology")
D("linguistic_anthropology", "Linguistic Anthropology", "social_science", "interpretive", level=2, parent="anthropology")
D("archaeology", "Archaeology", "social_science", "empirical", level=2, parent="anthropology")
D("medical_anthropology", "Medical Anthropology", "social_science", "interpretive", level=2, parent="anthropology")

D("economics", "Economics", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Smith", "Ricardo", "Marx", "Keynes", "Hayek", "Samuelson", "Friedman"])
D("microeconomics", "Microeconomics", "social_science", "formal", level=2, parent="economics")
D("macroeconomics", "Macroeconomics", "social_science", "formal", level=2, parent="economics", figures=["Keynes", "Friedman"])
D("econometrics", "Econometrics", "social_science", "formal", level=2, parent="economics")
D("behavioral_economics", "Behavioral Economics", "social_science", "empirical", level=2, parent="economics", figures=["Kahneman", "Thaler"])
D("development_economics", "Development Economics", "social_science", "empirical", level=2, parent="economics")
D("international_economics", "International Economics", "social_science", "formal", level=2, parent="economics")
D("labor_economics", "Labor Economics", "social_science", "formal", level=2, parent="economics")
D("health_economics", "Health Economics", "social_science", "formal", level=2, parent="economics")
D("environmental_economics", "Environmental Economics", "social_science", "formal", level=2, parent="economics")
D("public_economics", "Public Economics", "social_science", "formal", level=2, parent="economics")
D("financial_economics", "Financial Economics", "social_science", "formal", level=2, parent="economics")
D("experimental_economics", "Experimental Economics", "social_science", "empirical", level=2, parent="economics")
D("political_economy", "Political Economy", "social_science", "mixed", level=2, parent="economics")

D("political_science", "Political Science", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Machiavelli", "Hobbes", "Locke", "Madison", "Weber", "Dahl"])
D("political_theory", "Political Theory", "social_science", "dialectical", level=2, parent="political_science")
D("comparative_politics", "Comparative Politics", "social_science", "empirical", level=2, parent="political_science")
D("international_relations", "International Relations", "social_science", "empirical", level=2, parent="political_science")
D("public_administration", "Public Administration", "social_science", "empirical", level=2, parent="political_science")
D("public_policy", "Public Policy", "social_science", "mixed", level=2, parent="political_science")
D("security_studies", "Security Studies", "social_science", "empirical", level=2, parent="political_science")
D("electoral_studies", "Electoral Studies", "social_science", "empirical", level=2, parent="political_science")

D("linguistics", "Linguistics", "social_science", "empirical", level=1, parent="social_sciences",
  figures=["Saussure", "Chomsky", "Sapir", "Whorf", "Bloomfield"])
D("phonetics_phonology", "Phonetics and Phonology", "social_science", "empirical", level=2, parent="linguistics")
D("morphology_ling", "Morphology", "social_science", "formal", level=2, parent="linguistics")
D("syntax", "Syntax", "social_science", "formal", level=2, parent="linguistics", figures=["Chomsky"])
D("semantics_ling", "Semantics", "social_science", "formal", level=2, parent="linguistics")
D("pragmatics_ling", "Pragmatics", "social_science", "empirical", level=2, parent="linguistics")
D("sociolinguistics", "Sociolinguistics", "social_science", "empirical", level=2, parent="linguistics")
D("psycholinguistics", "Psycholinguistics", "social_science", "empirical", level=2, parent="linguistics")
D("computational_linguistics", "Computational Linguistics", "social_science", "computational", level=2, parent="linguistics")
D("historical_linguistics", "Historical / Comparative Linguistics", "social_science", "empirical", level=2, parent="linguistics")
D("neurolinguistics", "Neurolinguistics", "social_science", "empirical", level=2, parent="linguistics")
D("discourse_analysis", "Discourse Analysis", "social_science", "interpretive", level=2, parent="linguistics")
D("applied_linguistics", "Applied Linguistics", "social_science", "empirical", level=2, parent="linguistics")
D("typology_ling", "Linguistic Typology", "social_science", "empirical", level=2, parent="linguistics")

D("geography", "Geography", "social_science", "empirical", level=1, parent="social_sciences")
D("physical_geography", "Physical Geography", "social_science", "empirical", level=2, parent="geography")
D("human_geography", "Human Geography", "social_science", "empirical", level=2, parent="geography")
D("geopolitics", "Geopolitics", "social_science", "mixed", level=2, parent="geography")
D("urban_geography", "Urban Geography", "social_science", "empirical", level=2, parent="geography")
D("gis", "GIS and Cartography", "social_science", "computational", level=2, parent="geography")

D("communication_studies", "Communication Studies", "social_science", "empirical", level=1, parent="social_sciences")
D("media_studies", "Media Studies", "social_science", "interpretive", level=1, parent="social_sciences")
D("international_studies", "International Studies / Global Studies", "social_science", "mixed", level=1, parent="social_sciences")
D("peace_conflict", "Peace and Conflict Studies", "social_science", "empirical", level=1, parent="social_sciences")
D("development_studies", "Development Studies", "social_science", "empirical", level=1, parent="social_sciences")


# ══════════════════════════════════════════════════════════════════════════════
# HUMANITIES
# ══════════════════════════════════════════════════════════════════════════════
D("humanities", "Humanities", "humanities", "interpretive", level=0,
  desc="The disciplines that study human culture, expression, history, and meaning.",
  axioms=["Human expression carries meaning that must be interpreted", "Context is constitutive of meaning", "Multiple valid interpretations can coexist"],
  primer="Approach humanistic questions interpretively: identify context, authorial intent, cultural codes, and the horizon of the interpreter.")

D("history", "History", "humanities", "interpretive", level=1, parent="humanities",
  figures=["Herodotus", "Thucydides", "Gibbon", "Ranke", "Braudel", "Carr"])
D("ancient_history", "Ancient History", "humanities", "interpretive", level=2, parent="history")
D("medieval_history", "Medieval History", "humanities", "interpretive", level=2, parent="history")
D("early_modern_history", "Early Modern History", "humanities", "interpretive", level=2, parent="history")
D("modern_history", "Modern History", "humanities", "interpretive", level=2, parent="history")
D("intellectual_history", "Intellectual History", "humanities", "interpretive", level=2, parent="history")
D("cultural_history", "Cultural History", "humanities", "interpretive", level=2, parent="history")
D("social_history", "Social History", "humanities", "interpretive", level=2, parent="history")
D("economic_history", "Economic History", "humanities", "empirical", level=2, parent="history")
D("military_history", "Military History", "humanities", "interpretive", level=2, parent="history")
D("history_of_science", "History of Science", "humanities", "interpretive", level=2, parent="history")
D("history_of_religion", "History of Religion", "humanities", "interpretive", level=2, parent="history")
D("historiography", "Historiography", "humanities", "interpretive", level=2, parent="history")

D("literature", "Literature", "humanities", "interpretive", level=1, parent="humanities",
  figures=["Homer", "Dante", "Shakespeare", "Goethe", "Borges"])
D("comparative_literature", "Comparative Literature", "humanities", "interpretive", level=2, parent="literature")
D("literary_theory", "Literary Theory and Criticism", "humanities", "interpretive", level=2, parent="literature", figures=["Barthes", "Derrida", "Said"])
D("poetry_studies", "Poetry Studies", "humanities", "interpretive", level=2, parent="literature")
D("drama_studies", "Drama Studies", "humanities", "interpretive", level=2, parent="literature")
D("prose_fiction", "Prose / Fiction Studies", "humanities", "interpretive", level=2, parent="literature")
D("postcolonial_lit", "Postcolonial Literature", "humanities", "interpretive", level=2, parent="literature", figures=["Said", "Spivak", "Bhabha"])
D("world_literature", "World Literature", "humanities", "interpretive", level=2, parent="literature")

D("theology_religion", "Theology and Religious Studies", "humanities", "interpretive", level=1, parent="humanities")
D("systematic_theology", "Systematic Theology", "humanities", "deductive", level=2, parent="theology_religion")
D("biblical_studies", "Biblical / Scriptural Studies", "humanities", "interpretive", level=2, parent="theology_religion")
D("comparative_religion", "Comparative Religion", "humanities", "interpretive", level=2, parent="theology_religion")
D("history_of_religion_hum", "History of Religion", "humanities", "interpretive", level=2, parent="theology_religion")
D("mysticism", "Mysticism and Esotericism", "humanities", "interpretive", level=2, parent="theology_religion")
D("islamic_studies", "Islamic Studies", "humanities", "interpretive", level=2, parent="theology_religion")
D("jewish_studies", "Jewish Studies", "humanities", "interpretive", level=2, parent="theology_religion")
D("buddhist_studies", "Buddhist Studies", "humanities", "interpretive", level=2, parent="theology_religion")

D("arts", "Arts", "humanities", "interpretive", level=1, parent="humanities")
D("art_history", "Art History", "humanities", "interpretive", level=2, parent="arts")
D("art_theory", "Art Theory / Criticism", "humanities", "interpretive", level=2, parent="arts")
D("visual_arts", "Visual Arts", "humanities", "interpretive", level=2, parent="arts")
D("photography", "Photography", "humanities", "interpretive", level=2, parent="arts")

D("music_hum", "Music", "humanities", "interpretive", level=1, parent="humanities")
D("music_theory_hum", "Music Theory", "humanities", "formal", level=2, parent="music_hum")
D("music_history", "Music History", "humanities", "interpretive", level=2, parent="music_hum")
D("musicology", "Musicology", "humanities", "empirical", level=2, parent="music_hum")
D("ethnomusicology", "Ethnomusicology", "humanities", "interpretive", level=2, parent="music_hum")

D("theater_performance", "Theater and Performance Studies", "humanities", "interpretive", level=1, parent="humanities")
D("theater_history", "Theater History", "humanities", "interpretive", level=2, parent="theater_performance")
D("dramatic_theory", "Dramatic Theory", "humanities", "interpretive", level=2, parent="theater_performance")
D("dance_studies", "Dance Studies", "humanities", "interpretive", level=2, parent="theater_performance")

D("film_studies", "Film and Media Studies", "humanities", "interpretive", level=1, parent="humanities")
D("cinema_studies", "Cinema Studies", "humanities", "interpretive", level=2, parent="film_studies")
D("television_studies", "Television Studies", "humanities", "interpretive", level=2, parent="film_studies")
D("digital_media", "Digital Media Studies", "humanities", "interpretive", level=2, parent="film_studies")

D("classical_studies", "Classical and Ancient Studies", "humanities", "interpretive", level=1, parent="humanities")
D("classics", "Classical Studies (Greek and Latin)", "humanities", "interpretive", level=2, parent="classical_studies")
D("egyptology", "Egyptology", "humanities", "empirical", level=2, parent="classical_studies")
D("assyriology", "Assyriology", "humanities", "empirical", level=2, parent="classical_studies")
D("archaeology_hum", "Archaeology", "humanities", "empirical", level=2, parent="humanities")

D("cultural_area_studies", "Cultural and Area Studies", "humanities", "interpretive", level=1, parent="humanities")
D("african_studies", "African Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("asian_studies", "Asian Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("middle_eastern_studies", "Middle Eastern Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("latin_american_studies", "Latin American Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("indigenous_studies", "Indigenous Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("postcolonial_studies", "Postcolonial Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies", figures=["Said", "Spivak", "Fanon"])
D("gender_studies", "Gender Studies / Women's Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies", figures=["Butler", "de Beauvoir"])
D("queer_theory", "Queer Theory / LGBTQ+ Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies", figures=["Butler", "Sedgwick", "Foucault"])
D("disability_studies", "Disability Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("ethnic_studies", "Ethnic Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies")
D("cultural_studies", "Cultural Studies", "humanities", "interpretive", level=2, parent="cultural_area_studies", figures=["Hall", "Gramsci"])
D("folklore_mythology", "Folklore and Mythology", "humanities", "interpretive", level=2, parent="cultural_area_studies", figures=["Campbell", "Propp"])


# ══════════════════════════════════════════════════════════════════════════════
# APPLIED SCIENCES AND PROFESSIONAL FIELDS
# ══════════════════════════════════════════════════════════════════════════════
D("applied_sciences", "Applied Sciences and Professional Fields", "applied", "mixed", level=0,
  desc="Disciplines that apply scientific knowledge to practical human ends.",
  primer="Applied fields ground abstract knowledge in practical constraints. Ask: what are the real-world limits, standards, and ethical obligations?")

D("medicine", "Medicine and Health Sciences", "applied", "empirical", level=1, parent="applied_sciences",
  figures=["Hippocrates", "Galen", "Harvey", "Fleming", "Lister"])
D("pathology", "Pathology", "applied", "empirical", level=2, parent="medicine")
D("pharmacology", "Pharmacology", "applied", "empirical", level=2, parent="medicine")
D("internal_medicine", "Internal Medicine", "applied", "empirical", level=2, parent="medicine")
D("surgery", "Surgery", "applied", "empirical", level=2, parent="medicine")
D("psychiatry", "Psychiatry", "applied", "empirical", level=2, parent="medicine", figures=["Freud", "Kraepelin"])
D("neurology", "Neurology", "applied", "empirical", level=2, parent="medicine")
D("cardiology", "Cardiology", "applied", "empirical", level=2, parent="medicine")
D("oncology", "Oncology", "applied", "empirical", level=2, parent="medicine")
D("pediatrics", "Pediatrics", "applied", "empirical", level=2, parent="medicine")
D("obstetrics_gynecology", "Obstetrics and Gynecology", "applied", "empirical", level=2, parent="medicine")
D("epidemiology", "Epidemiology", "applied", "empirical", level=2, parent="medicine")
D("public_health", "Public Health", "applied", "empirical", level=2, parent="medicine")
D("nursing", "Nursing", "applied", "empirical", level=2, parent="medicine")
D("dentistry", "Dentistry", "applied", "empirical", level=2, parent="medicine")
D("veterinary_medicine", "Veterinary Medicine", "applied", "empirical", level=2, parent="medicine")
D("radiology", "Radiology / Medical Imaging", "applied", "empirical", level=2, parent="medicine")
D("toxicology", "Toxicology", "applied", "empirical", level=2, parent="medicine")
D("geriatrics", "Geriatrics", "applied", "empirical", level=2, parent="medicine")
D("palliative_care", "Palliative Care", "applied", "empirical", level=2, parent="medicine")
D("sports_medicine", "Sports Medicine", "applied", "empirical", level=2, parent="medicine")
D("immunology_med", "Immunology (Clinical)", "applied", "empirical", level=2, parent="medicine")

D("engineering", "Engineering", "applied", "computational", level=1, parent="applied_sciences",
  figures=["Archimedes", "Watt", "Tesla", "Edison", "von Braun"])
D("civil_engineering", "Civil Engineering", "applied", "computational", level=2, parent="engineering")
D("structural_engineering", "Structural Engineering", "applied", "computational", level=2, parent="engineering")
D("mechanical_engineering", "Mechanical Engineering", "applied", "computational", level=2, parent="engineering")
D("electrical_engineering", "Electrical Engineering", "applied", "computational", level=2, parent="engineering")
D("chemical_engineering", "Chemical Engineering", "applied", "empirical", level=2, parent="engineering")
D("aerospace_engineering", "Aerospace Engineering", "applied", "computational", level=2, parent="engineering")
D("biomedical_engineering", "Biomedical Engineering", "applied", "computational", level=2, parent="engineering")
D("environmental_engineering", "Environmental Engineering", "applied", "empirical", level=2, parent="engineering")
D("nuclear_engineering", "Nuclear Engineering", "applied", "empirical", level=2, parent="engineering")
D("industrial_engineering", "Industrial Engineering", "applied", "computational", level=2, parent="engineering")
D("materials_engineering", "Materials Engineering", "applied", "empirical", level=2, parent="engineering")
D("computer_engineering", "Computer Engineering", "applied", "computational", level=2, parent="engineering")
D("systems_engineering", "Systems Engineering", "applied", "computational", level=2, parent="engineering")
D("software_engineering_app", "Software Engineering (Applied)", "applied", "computational", level=2, parent="engineering")
D("mechatronics", "Mechatronics", "applied", "computational", level=2, parent="engineering")
D("petroleum_engineering", "Petroleum Engineering", "applied", "empirical", level=2, parent="engineering")
D("optical_engineering", "Optical Engineering", "applied", "computational", level=2, parent="engineering")

D("architecture_design", "Architecture and Design", "applied", "mixed", level=1, parent="applied_sciences")
D("architecture", "Architecture", "applied", "mixed", level=2, parent="architecture_design")
D("urban_planning", "Urban Planning", "applied", "mixed", level=2, parent="architecture_design")
D("landscape_architecture", "Landscape Architecture", "applied", "mixed", level=2, parent="architecture_design")
D("industrial_design", "Industrial Design", "applied", "mixed", level=2, parent="architecture_design")
D("graphic_design", "Graphic Design", "applied", "mixed", level=2, parent="architecture_design")
D("interior_design", "Interior Design", "applied", "mixed", level=2, parent="architecture_design")

D("agriculture_food", "Agriculture and Food Science", "applied", "empirical", level=1, parent="applied_sciences")
D("agronomy", "Agronomy", "applied", "empirical", level=2, parent="agriculture_food")
D("animal_science", "Animal Science", "applied", "empirical", level=2, parent="agriculture_food")
D("horticulture", "Horticulture", "applied", "empirical", level=2, parent="agriculture_food")
D("aquaculture", "Aquaculture", "applied", "empirical", level=2, parent="agriculture_food")
D("food_science", "Food Science and Technology", "applied", "empirical", level=2, parent="agriculture_food")
D("agricultural_economics", "Agricultural Economics", "applied", "formal", level=2, parent="agriculture_food")

D("business_management", "Business and Management", "applied", "mixed", level=1, parent="applied_sciences")
D("accounting", "Accounting", "applied", "formal", level=2, parent="business_management")
D("finance", "Finance", "applied", "formal", level=2, parent="business_management")
D("marketing", "Marketing", "applied", "empirical", level=2, parent="business_management")
D("management", "Management", "applied", "empirical", level=2, parent="business_management")
D("operations_management", "Operations Management", "applied", "computational", level=2, parent="business_management")
D("supply_chain", "Supply Chain and Logistics", "applied", "computational", level=2, parent="business_management")
D("entrepreneurship", "Entrepreneurship", "applied", "mixed", level=2, parent="business_management")
D("organizational_behavior", "Organizational Behavior", "applied", "empirical", level=2, parent="business_management")
D("human_resources", "Human Resources", "applied", "empirical", level=2, parent="business_management")
D("international_business", "International Business", "applied", "empirical", level=2, parent="business_management")
D("business_strategy", "Business Strategy", "applied", "mixed", level=2, parent="business_management")

D("law", "Law and Jurisprudence", "applied", "deductive", level=1, parent="applied_sciences",
  figures=["Justinian", "Blackstone", "Holmes", "Hart", "Dworkin"])
D("constitutional_law", "Constitutional Law", "applied", "deductive", level=2, parent="law")
D("criminal_law", "Criminal Law", "applied", "deductive", level=2, parent="law")
D("civil_law", "Civil Law", "applied", "deductive", level=2, parent="law")
D("international_law", "International Law", "applied", "deductive", level=2, parent="law")
D("corporate_law", "Corporate / Business Law", "applied", "deductive", level=2, parent="law")
D("environmental_law", "Environmental Law", "applied", "mixed", level=2, parent="law")
D("intellectual_property_law", "Intellectual Property Law", "applied", "deductive", level=2, parent="law")
D("human_rights_law", "Human Rights Law", "applied", "mixed", level=2, parent="law")
D("comparative_law", "Comparative Law", "applied", "interpretive", level=2, parent="law")

D("education_field", "Education", "applied", "mixed", level=1, parent="applied_sciences",
  figures=["Dewey", "Montessori", "Freire", "Vygotsky"])
D("pedagogy", "Pedagogy", "applied", "mixed", level=2, parent="education_field")
D("curriculum_theory", "Curriculum Theory", "applied", "mixed", level=2, parent="education_field")
D("special_education", "Special Education", "applied", "empirical", level=2, parent="education_field")
D("early_childhood_education", "Early Childhood Education", "applied", "empirical", level=2, parent="education_field")
D("higher_education", "Higher Education", "applied", "empirical", level=2, parent="education_field")
D("adult_education", "Adult Education / Andragogy", "applied", "empirical", level=2, parent="education_field")
D("educational_technology", "Educational Technology", "applied", "computational", level=2, parent="education_field")

D("library_info_science", "Library and Information Science", "applied", "mixed", level=1, parent="applied_sciences")
D("journalism", "Journalism and Communication", "applied", "mixed", level=1, parent="applied_sciences")
D("forensic_science", "Forensic Science", "applied", "empirical", level=1, parent="applied_sciences")
D("military_science", "Military Science", "applied", "mixed", level=1, parent="applied_sciences")
D("sports_science", "Sports Science / Kinesiology", "applied", "empirical", level=1, parent="applied_sciences")
D("nutrition", "Nutrition and Dietetics", "applied", "empirical", level=1, parent="applied_sciences")
D("pharmacy_field", "Pharmacy", "applied", "empirical", level=1, parent="applied_sciences")
D("criminology_field", "Criminology and Criminal Justice", "applied", "empirical", level=1, parent="applied_sciences")
D("social_work", "Social Work", "applied", "mixed", level=1, parent="applied_sciences")


# ══════════════════════════════════════════════════════════════════════════════
# INTERDISCIPLINARY AND EMERGING FIELDS
# ══════════════════════════════════════════════════════════════════════════════
D("interdisciplinary", "Interdisciplinary and Emerging Fields", "interdisciplinary", "mixed", level=0,
  desc="Fields that synthesize methods and knowledge from multiple traditional disciplines.",
  primer="Interdisciplinary questions require explicitly identifying which disciplinary lens applies and where they conflict.")

D("cognitive_science", "Cognitive Science", "interdisciplinary", "empirical", level=1, parent="interdisciplinary",
  desc="The study of mind and intelligence, drawing on psychology, neuroscience, AI, linguistics, and philosophy.",
  figures=["Turing", "Newell", "Simon", "Chomsky", "Marr"])
D("neuroscience", "Neuroscience", "interdisciplinary", "empirical", level=1, parent="interdisciplinary",
  figures=["Cajal", "Hubel", "Wiesel", "LeDoux", "Kandel"])
D("complexity_science", "Complexity Science / Systems Theory", "interdisciplinary", "mixed", level=1, parent="interdisciplinary",
  figures=["Von Bertalanffy", "Wiener", "Holland", "Kauffman"])
D("cybernetics", "Cybernetics", "interdisciplinary", "formal", level=1, parent="interdisciplinary", figures=["Wiener", "Ashby"])
D("sustainability_science", "Sustainability Science", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("data_science_inter", "Data Science", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("computational_social_science", "Computational Social Science", "interdisciplinary", "computational", level=1, parent="interdisciplinary")
D("digital_humanities", "Digital Humanities", "interdisciplinary", "mixed", level=1, parent="interdisciplinary")
D("sts", "Science and Technology Studies (STS)", "interdisciplinary", "interpretive", level=1, parent="interdisciplinary", figures=["Latour", "Collins"])
D("bioinformatics_inter", "Bioinformatics", "interdisciplinary", "computational", level=1, parent="interdisciplinary")
D("computational_neuroscience", "Computational Neuroscience", "interdisciplinary", "computational", level=1, parent="interdisciplinary")
D("mathematical_finance", "Mathematical Finance / Quantitative Finance", "interdisciplinary", "formal", level=1, parent="interdisciplinary")
D("nanotechnology", "Nanotechnology", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("biotechnology", "Biotechnology", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("climate_science", "Climate Science", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("network_science", "Network Science", "interdisciplinary", "computational", level=1, parent="interdisciplinary")
D("decision_science", "Decision Science", "interdisciplinary", "formal", level=1, parent="interdisciplinary", figures=["Kahneman", "Tversky"])
D("futures_studies", "Futures Studies / Futurology", "interdisciplinary", "mixed", level=1, parent="interdisciplinary")
D("quantum_information", "Quantum Information Science", "interdisciplinary", "formal", level=1, parent="interdisciplinary")
D("human_factors", "Human Factors Engineering / Ergonomics", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("moral_psychology", "Moral Psychology", "interdisciplinary", "empirical", level=1, parent="interdisciplinary", figures=["Haidt", "Kohlberg"])
D("experimental_philosophy", "Experimental Philosophy", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("urban_studies", "Urban Studies", "interdisciplinary", "empirical", level=1, parent="interdisciplinary")
D("intelligence_studies", "Intelligence Studies", "interdisciplinary", "mixed", level=1, parent="interdisciplinary")
D("environmental_studies_inter", "Environmental Studies", "interdisciplinary", "mixed", level=1, parent="interdisciplinary")
D("global_studies", "Global Studies", "interdisciplinary", "mixed", level=1, parent="interdisciplinary")

# ─── Mathematical Proofs & Formal Methods ─────────────────────────────────────
D("mathematical_proofs", "Mathematical Proofs & Proof Theory", "mathematics", "formal", level=1, parent="mathematics",
  desc="The art, science, and formal theory of mathematical proof — from direct proofs to automated theorem proving.",
  purpose="Establish mathematical truth beyond doubt through rigorous logical derivation.",
  axioms=["A proof must be logically valid from accepted axioms.", "Proof methods include direct, contradiction, induction, and construction.", "Gödel incompleteness: not all truths are provable within a consistent system."],
  methods=["Direct proof", "Proof by contradiction", "Mathematical induction", "Transfinite induction", "Diagonalization", "Probabilistic proof", "Computer-assisted proof (Coq, Lean, Isabelle)"],
  figures=["Euclid", "Gödel", "Hilbert", "Turing", "de Bruijn", "Appel & Haken (4-color theorem)"],
  primer="When proving mathematical claims: identify proof strategy (direct, contradiction, induction), cite axioms explicitly, and verify each logical step. Flag when a result is non-constructive or relies on choice axiom.")

D("proof_by_induction", "Mathematical Induction", "mathematics", "formal", level=2, parent="mathematical_proofs",
  desc="Proving a statement holds for all natural numbers via base case and inductive step.",
  axioms=["Base case must hold.", "Inductive step: P(k) → P(k+1) for all k≥base."],
  methods=["Weak induction", "Strong induction", "Structural induction", "Transfinite induction"],
  primer="Always verify base case explicitly. For strong induction, assume all prior cases hold.")

D("proof_by_contradiction", "Proof by Contradiction (Reductio ad Absurdum)", "mathematics", "formal", level=2, parent="mathematical_proofs",
  desc="Assume the negation of the theorem, derive a contradiction, conclude the theorem holds.",
  axioms=["Law of excluded middle: P ∨ ¬P.", "Deriving ⊥ from ¬P proves P."],
  primer="Assume negation, derive contradiction clearly, state conclusion. Note when the law of excluded middle is used.")

D("constructive_proof", "Constructive Proof", "mathematics", "formal", level=2, parent="mathematical_proofs",
  desc="Proofs that explicitly construct the mathematical object claimed to exist.",
  axioms=["Existence must be witnessed by explicit construction.", "No law of excluded middle assumed."],
  figures=["Brouwer", "Bishop", "Martin-Löf"],
  primer="Provide explicit witnesses; avoid non-constructive existence arguments unless noted.")

D("automated_theorem_proving", "Automated Theorem Proving & Proof Assistants", "mathematics", "computational", level=2, parent="mathematical_proofs",
  desc="Software systems (Coq, Lean 4, Isabelle, HOL) that mechanically verify or discover proofs.",
  methods=["Type theory (Curry-Howard correspondence)", "SAT/SMT solving", "Resolution", "Unification", "Dependent types"],
  figures=["de Bruijn", "Gonthier", "Avigad", "Buzzard"],
  primer="Reference the formal system (Lean 4, Coq, etc.) when discussing machine-checked proofs. Note that proof assistants use constructive logic by default unless classical axioms are added.")

# ─── Academic & Scientific Publication ────────────────────────────────────────
D("academic_publishing", "Academic Publishing & Scholarly Communication", "interdisciplinary", "mixed", level=1, parent="interdisciplinary",
  desc="The processes, norms, formats, and institutions that govern how knowledge is formally disseminated.",
  purpose="Ensure reliable, peer-reviewed dissemination of new knowledge across all disciplines.",
  axioms=["Peer review is the gold standard for knowledge validation.", "Citation gives credit and enables verification.", "Open access expands reach and reproducibility."],
  methods=["Peer review (single-blind, double-blind, open)", "Pre-registration", "Preprint servers (arXiv, bioRxiv, SSRN)", "Systematic review", "Meta-analysis", "Replication studies"],
  figures=["Merton", "Kuhn", "Popper", "Ziman"],
  primer="When citing academic work: prefer primary sources, note publication venue tier, distinguish preprint from peer-reviewed, and flag retracted papers.")

D("citation_styles", "Citation Styles & Reference Management", "interdisciplinary", "mixed", level=2, parent="academic_publishing",
  desc="Formal systems for attributing sources: APA, MLA, Chicago, Vancouver, IEEE, AMS, and others.",
  methods=["APA 7th edition (social sciences)", "MLA 9th (humanities)", "Chicago/Turabian (history, humanities)", "Vancouver/NLM (medicine)", "IEEE (engineering/CS)", "AMS (mathematics)"],
  primer="Match citation style to field: APA for social sciences, Chicago for history, IEEE for CS/engineering, AMS for mathematics. Always include DOI when available.")

D("academic_databases", "Academic Databases & Research Repositories", "interdisciplinary", "mixed", level=2, parent="academic_publishing",
  desc="Digital archives and indexes of scholarly literature: arXiv, PubMed, JSTOR, Scopus, Web of Science, Google Scholar, Semantic Scholar.",
  methods=["Boolean search operators", "Citation graph traversal", "h-index and impact factor", "Preprint vs. published version tracking"],
  primer="For literature searches, prefer Semantic Scholar or Google Scholar for broad coverage, PubMed for biomedicine, arXiv for physics/math/CS preprints, JSTOR for humanities.")

D("peer_review_process", "Peer Review & Scientific Validation", "interdisciplinary", "empirical", level=2, parent="academic_publishing",
  desc="Evaluation of research by domain experts before publication.",
  methods=["Single-blind review", "Double-blind review", "Open peer review", "Post-publication review", "Statistical review"],
  primer="Distinguish preprint (not peer-reviewed) from published. Note major replication crises in psychology, medicine, and nutrition when assessing empirical claims.")

# ─── Scientific & Mathematical Notation ───────────────────────────────────────
D("mathematical_notation", "Mathematical Notation & Symbolic Language", "mathematics", "formal", level=1, parent="mathematics",
  desc="The formal symbolic language used to express mathematical ideas precisely and concisely.",
  purpose="Enable unambiguous communication of mathematical structures across cultures and disciplines.",
  axioms=["Notation must be unambiguous within its defined context.", "Good notation compresses thought without losing precision.", "Standard notation enables cross-disciplinary communication."],
  methods=["Set-builder notation", "Summation/product notation (Σ, Π)", "Quantifiers (∀, ∃)", "Limits and continuity notation (ε-δ)", "Big-O notation", "Matrix notation", "Integral/differential notation", "Logical connectives (∧, ∨, ¬, →, ↔)"],
  figures=["Leibniz", "Euler", "Peano", "Russell", "Bourbaki"],
  primer="Always render mathematics in standard notation. Prefer LaTeX-style symbolic expressions. Define all symbols on first use. When ambiguous, disambiguate with context (e.g., × vs ⊗ vs ·).")

D("latex_typesetting", "LaTeX & Mathematical Typesetting", "mathematics", "computational", level=2, parent="mathematical_notation",
  desc="The de facto standard system for typesetting mathematics, science, and academic documents.",
  methods=["AMS-LaTeX packages (amsmath, amsthm)", "BibTeX/BibLaTeX bibliography", "TikZ diagrams", "Beamer presentations", "Overleaf collaborative editing"],
  primer="When producing mathematical content, use LaTeX notation. Wrap inline math in $...$ and display equations in $$...$$ or \\[...\\]. Use \\begin{proof}...\\end{proof} for proofs.")

D("scientific_notation_systems", "Scientific Notation & Units", "natural_science", "empirical", level=2, parent="mathematical_notation",
  desc="SI units, scientific notation for large/small numbers, significant figures, and dimensional analysis.",
  methods=["SI base units (meter, kilogram, second, ampere, kelvin, mole, candela)", "Scientific notation (a × 10^n)", "Significant figures and rounding", "Dimensional analysis (unit tracking)"],
  primer="Always include units for physical quantities. Use SI unless the field convention differs (e.g., eV in particle physics). Track significant figures through calculations.")

D("chemical_notation", "Chemical Notation & IUPAC Nomenclature", "natural_science", "formal", level=2, parent="mathematical_notation",
  desc="Standard system for naming compounds, writing equations, and representing molecular structure.",
  methods=["IUPAC nomenclature", "Lewis structures", "Chemical equations (balancing)", "Molecular formula vs. structural formula", "SMILES notation"],
  primer="Name compounds per IUPAC; balance equations by atom count; specify oxidation states when relevant.")

D("music_notation", "Musical Notation & Theory Symbols", "humanities", "interpretive", level=2, parent="mathematical_notation",
  desc="Standard notation for pitch, rhythm, dynamics, and structure in Western music.",
  methods=["Staff notation", "Clef systems", "Note values", "Time signatures", "Key signatures", "Dynamic markings", "Lead sheet notation", "Tablature"],
  primer="Describe musical structure using standard notation terms. Distinguish Western staff notation from alternative systems (ABC notation, tablature) when relevant.")

# ─── Wolfram / Computational Knowledge ───────────────────────────────────────
D("computational_knowledge", "Computational Knowledge & Algorithmic Answers", "computer_science", "computational", level=1, parent="computer_science",
  desc="The discipline of encoding world knowledge in computable, queryable form — exemplified by Wolfram|Alpha and Mathematica.",
  purpose="Enable automated symbolic and numerical reasoning over structured world knowledge.",
  axioms=["Knowledge can be formally represented and computed.", "Computable answers are more precise than natural language answers.", "The Wolfram Language unifies symbolic math, data, and computation."],
  methods=["Wolfram|Alpha natural language queries", "Mathematica symbolic computation", "Wolfram Language (pattern matching, functional programming, symbolic evaluation)", "CAS (Computer Algebra Systems)", "Numerical methods (NDSolve, NIntegrate)", "Wolfram Data Repository", "Wolfram MathWorld reference"],
  figures=["Stephen Wolfram", "Theodore Gray"],
  primer="For math/science queries requiring exact symbolic answers, frame them as Wolfram|Alpha-style queries. Distinguish symbolic (exact) from numerical (approximate) computation. Cite Wolfram MathWorld for definitions when precision is paramount.")

D("computer_algebra_systems", "Computer Algebra Systems (CAS)", "computer_science", "computational", level=2, parent="computational_knowledge",
  desc="Software for symbolic manipulation of mathematical expressions: Mathematica, Maple, SageMath, SymPy, Maxima.",
  methods=["Symbolic differentiation/integration", "Polynomial factorization", "Equation solving (Solve, NSolve)", "Series expansion", "Linear algebra (MatrixExp, Eigensystem)", "Simplification (FullSimplify)", "Groebner bases"],
  primer="When a user asks for an exact symbolic result, identify the appropriate CAS operation. Prefer Wolfram Language syntax as canonical; note equivalents in SymPy (Python) or SageMath when relevant.")

D("numerical_methods_comp", "Numerical Methods & Scientific Computing", "computer_science", "computational", level=2, parent="computational_knowledge",
  desc="Algorithms for numerical approximation of mathematical problems: ODEs, PDEs, optimization, linear systems.",
  methods=["Runge-Kutta ODE solvers", "Finite element method", "Fast Fourier Transform (FFT)", "Newton-Raphson root finding", "Gradient descent", "Monte Carlo methods", "LU/QR decomposition"],
  figures=["Gauss", "Runge", "Kutta", "Cooley", "Tukey"],
  primer="Distinguish numerical from analytic solutions. Always state numerical precision and step size. Warn when methods may diverge or produce spurious results.")

D("wolfram_alpha_queries", "Wolfram|Alpha Query Patterns", "computer_science", "computational", level=2, parent="computational_knowledge",
  desc="How to formulate questions for Wolfram|Alpha: math, science, data, geography, astronomy, music, and more.",
  methods=["Plain English math queries ('integrate x^2 from 0 to 1')", "Unit conversion queries", "Data lookup queries ('population of France')", "Sequence identification (OEIS)", "Equation solving ('solve x^2 - 5x + 6 = 0')", "Geometric queries ('volume of sphere radius 5')", "Statistical queries ('mean of {1,2,3,4,5}')"],
  primer="Translate user math/science questions into Wolfram|Alpha query format when exact computation is needed. Wolfram|Alpha handles calculus, linear algebra, statistics, chemistry, astronomy, music theory, and factual data. For computational output, present both symbolic and numerical results.")

# ─── OEIS & Mathematical Sequences ────────────────────────────────────────────
D("integer_sequences", "Integer Sequences & OEIS", "mathematics", "formal", level=2, parent="combinatorics",
  desc="The study and cataloging of integer sequences — epitomized by the Online Encyclopedia of Integer Sequences (OEIS).",
  methods=["Recurrence relations", "Generating functions", "Asymptotic analysis", "EIS lookup by terms"],
  figures=["Neil Sloane"],
  primer="When a sequence is mentioned, check if it matches a known OEIS entry. Reference OEIS A-numbers (e.g., A000045 for Fibonacci). Use generating functions to unify sequence properties.")

# ─── Epistemology of Science & Research Methods ───────────────────────────────
D("research_methods", "Research Methods & Scientific Inquiry", "interdisciplinary", "empirical", level=1, parent="interdisciplinary",
  desc="The methodological foundations common to scientific inquiry across disciplines.",
  purpose="Ensure reliable, reproducible, and valid knowledge production.",
  axioms=["Hypotheses must be falsifiable (Popper).", "Controls eliminate confounds.", "Replication validates findings.", "Effect size matters as much as significance."],
  methods=["Experimental design (RCT, factorial)", "Observational study", "Survey & questionnaire design", "Grounded theory (qualitative)", "Mixed methods", "Systematic review", "Power analysis", "p-value and confidence intervals", "Effect size (Cohen's d, r, η²)"],
  figures=["Popper", "Kuhn", "Feyerabend", "Fisher", "Neyman", "Pearson"],
  primer="Always distinguish correlation from causation. Check sample size and power. Report effect sizes alongside p-values. Flag p-hacking and HARKing (Hypothesizing After Results are Known).")

D("replication_crisis", "Replication Crisis & Open Science", "interdisciplinary", "empirical", level=2, parent="research_methods",
  desc="The ongoing methodological crisis across psychology, medicine, and social science where many published findings fail to replicate.",
  methods=["Pre-registration", "Registered Reports", "Open data & materials", "Bayesian statistics", "Meta-analysis with heterogeneity testing"],
  figures=["Ioannidis", "Nosek", "Simmons"],
  primer="Flag high-profile non-replications (priming effects, power posing). Prefer pre-registered studies. Be skeptical of small-n, high-p studies in psychology and nutrition.")

D("statistical_inference", "Statistical Inference & Hypothesis Testing", "mathematics", "empirical", level=2, parent="probability_statistics",
  desc="Formal methods for drawing conclusions about populations from samples.",
  methods=["Null hypothesis significance testing (NHST)", "Bayesian inference", "Confidence intervals", "Power analysis", "Multiple comparisons correction (Bonferroni, FDR)", "Non-parametric tests"],
  figures=["Fisher", "Neyman", "Pearson", "Bayes", "Jeffreys"],
  primer="Always state H₀ and H₁ explicitly. Report p-value, confidence interval, and effect size together. Prefer two-sided tests unless directional hypothesis is pre-registered. Adjust for multiple comparisons.")

# ─── Philosophy of Mathematics ────────────────────────────────────────────────
D("philosophy_of_mathematics", "Philosophy of Mathematics", "philosophy", "dialectical", level=1, parent="philosophy",
  desc="Foundational questions about the nature, ontology, and epistemology of mathematical objects and truth.",
  purpose="Interrogate what mathematical objects are, whether they exist independently, and how mathematics applies to physical reality.",
  axioms=["Mathematical objects may be abstract (Platonism) or constructed (Constructivism).", "Unreasonable effectiveness: mathematics describes reality despite apparent abstraction.", "Foundations: ZFC set theory underlies most of modern mathematics."],
  methods=["Philosophical analysis", "Formal axiomatic systems", "Historical case study"],
  figures=["Plato", "Frege", "Russell", "Wittgenstein", "Gödel", "Benacerraf", "Maddy"],
  primer="Distinguish Platonism (mathematical objects exist independently), formalism (math is symbol manipulation), intuitionism (math is mental construction), and structuralism (math studies abstract structures). Ground claims in the position's ontological commitments.")

D("foundations_of_mathematics", "Foundations of Mathematics", "mathematics", "formal", level=2, parent="philosophy_of_mathematics",
  desc="Axiomatic foundations underpinning all of mathematics: ZFC set theory, type theory, category theory as foundations.",
  methods=["ZFC axioms", "Peano axioms", "Homotopy Type Theory (HoTT)", "Category-theoretic foundations", "Reverse mathematics"],
  figures=["Zermelo", "Fraenkel", "von Neumann", "Gödel", "Cohen", "Voevodsky"],
  primer="Reference ZFC as the default foundation. Note when the axiom of choice (AC) or continuum hypothesis (CH) is invoked. Distinguish first-order from second-order arithmetic.")

# ─── History of Mathematics ───────────────────────────────────────────────────
D("history_of_mathematics", "History of Mathematics", "mathematics", "interpretive", level=1, parent="mathematics",
  desc="The development of mathematical ideas across civilizations from Mesopotamia to the present.",
  purpose="Understand how mathematical knowledge accumulated, which problems drove progress, and how notation and proof evolved.",
  methods=["Historical textual analysis", "Mathematical reconstruction", "Comparative civilizational study"],
  figures=["Euclid", "Archimedes", "Al-Khwarizmi", "Newton", "Leibniz", "Euler", "Gauss", "Cauchy", "Cantor", "Hilbert", "Poincaré", "Ramanujan"],
  primer="When placing a theorem historically: identify who first proved it, the era's dominant methods, and how the result was later generalized or formalized.")

# ─── Logic in Computer Science ────────────────────────────────────────────────
D("type_theory", "Type Theory & Type Systems", "computer_science", "formal", level=2, parent="programming_languages_cs",
  desc="Formal systems that assign types to expressions to ensure correctness — from simple types to dependent types.",
  axioms=["Every expression has a type.", "Well-typed programs don't go wrong (Milner).", "Curry-Howard: proofs ≅ programs, propositions ≅ types."],
  methods=["Hindley-Milner type inference", "Dependent types (Coq, Lean, Agda)", "Gradual typing", "Subtyping", "Parametric polymorphism (generics)"],
  figures=["Church", "Curry", "Howard", "Milner", "Martin-Löf"],
  primer="Always distinguish static from dynamic typing, nominal from structural subtyping, and monomorphic from polymorphic types. When discussing proof assistants, note the Curry-Howard isomorphism.")

D("formal_verification", "Formal Verification & Model Checking", "computer_science", "formal", level=2, parent="theory_of_computation",
  desc="Mathematically rigorous methods to prove software and hardware systems correct with respect to a specification.",
  methods=["Model checking (SPIN, NuSMV)", "Theorem proving (Coq, Isabelle, Lean)", "Abstract interpretation", "SAT/SMT solvers (Z3, CVC5)", "Hoare logic / Separation logic", "TLA+ (temporal logic of actions)"],
  figures=["Floyd", "Hoare", "Dijkstra", "Clarke", "Emerson", "Lamport"],
  primer="Distinguish model checking (exhaustive state space) from deductive verification (theorem proving). Note scalability limits of model checking and the proof burden of theorem proving.")

D("lambda_calculus", "Lambda Calculus & Computability", "mathematics", "formal", level=2, parent="theory_of_computation",
  desc="Church's formal model of computation via function abstraction and application; foundation of functional programming and type theory.",
  axioms=["β-reduction: (λx.M) N → M[N/x]", "Church-Turing thesis: all effective computations are Turing-computable.", "λ-calculus and Turing machines are computationally equivalent."],
  methods=["α-renaming", "β-reduction", "η-conversion", "Normal form analysis", "Church numerals", "Fixed-point combinators (Y combinator)"],
  figures=["Church", "Turing", "Kleene", "Scott"],
  primer="Trace β-reductions step by step. Identify normal forms. Note when divergence (infinite reduction) occurs. Connect to Haskell/Lisp semantics where relevant.")


# ─── Key cross-domain edges ───────────────────────────────────────────────────

EDGES: list[tuple[str, str, str, float]] = [
    # (from, to, relationship, weight)
    ("mathematical_logic", "logic", "formalizes", 1.0),
    ("logic", "philosophy", "prerequisite", 0.9),
    ("logic", "mathematics", "overlaps", 0.9),
    ("logic", "computer_science", "prerequisite", 0.9),
    ("rhetoric", "logic", "complements", 0.8),
    ("rhetoric", "philosophy", "overlaps", 0.8),
    ("grammar", "linguistics", "instantiates", 0.9),
    ("mathematics", "physics", "applies_to", 1.0),
    ("mathematics", "computer_science", "prerequisite", 0.9),
    ("mathematics", "economics", "applies_to", 0.8),
    ("mathematics", "probability_statistics", "overlaps", 0.9),
    ("philosophy", "natural_sciences", "critiques", 0.7),
    ("ethics", "applied_ethics", "extends", 1.0),
    ("cognitive_science", "neuroscience", "overlaps", 0.9),
    ("cognitive_science", "psychology", "extends", 0.9),
    ("cognitive_science", "artificial_intelligence", "overlaps", 0.9),
    ("cognitive_science", "linguistics", "overlaps", 0.8),
    ("game_theory", "economics", "applies_to", 0.8),
    ("game_theory", "political_science", "applies_to", 0.7),
    ("information_theory", "cryptography_cs", "prerequisite", 0.8),
    ("information_theory", "machine_learning", "prerequisite", 0.7),
    ("probability_statistics", "machine_learning", "prerequisite", 0.9),
    ("probability_theory", "probability_statistics", "prerequisite", 1.0),
    ("algebra", "cryptography_math", "prerequisite", 0.8),
    ("topology", "differential_geometry", "prerequisite", 0.8),
    ("philosophy_of_mind", "cognitive_science", "overlaps", 0.9),
    ("neuroscience", "biology", "extends", 0.8),
    ("biochemistry", "molecular_biology", "overlaps", 0.9),
    ("evolutionary_biology", "genetics", "overlaps", 0.8),
    # New domain edges
    ("mathematical_proofs", "mathematical_logic", "formalizes", 1.0),
    ("mathematical_proofs", "mathematics", "prerequisite", 1.0),
    ("automated_theorem_proving", "mathematical_proofs", "applies_to", 0.9),
    ("type_theory", "mathematical_logic", "formalizes", 0.9),
    ("type_theory", "programming_languages_cs", "applies_to", 0.9),
    ("formal_verification", "type_theory", "extends", 0.8),
    ("lambda_calculus", "type_theory", "prerequisite", 0.9),
    ("computer_algebra_systems", "mathematical_notation", "applies_to", 0.8),
    ("computational_knowledge", "mathematics", "applies_to", 0.9),
    ("wolfram_alpha_queries", "computational_knowledge", "instantiates", 1.0),
    ("mathematical_notation", "mathematics", "prerequisite", 1.0),
    ("latex_typesetting", "academic_publishing", "applies_to", 0.8),
    ("academic_publishing", "research_methods", "overlaps", 0.7),
    ("philosophy_of_mathematics", "mathematics", "critiques", 0.8),
    ("philosophy_of_mathematics", "philosophy", "extends", 0.9),
    ("foundations_of_mathematics", "mathematical_logic", "extends", 0.9),
    ("statistical_inference", "research_methods", "applies_to", 0.9),
    ("replication_crisis", "research_methods", "critiques", 0.8),
    ("integer_sequences", "combinatorics", "extends", 0.8),
    ("history_of_mathematics", "mathematics", "overlaps", 0.7),
]


# ─── DB write ─────────────────────────────────────────────────────────────────

UPSERT_DOMAIN_SQL = """
INSERT INTO omega_knowledge_domains
    (slug, name, parent_slug, level, sort_order, realm, reasoning_mode,
     description, purpose, core_axioms, key_methods, key_figures,
     sister_slugs, query_patterns, reasoning_primer)
VALUES
    (%(slug)s, %(name)s, %(parent_slug)s, %(level)s, %(sort_order)s,
     %(realm)s, %(reasoning_mode)s, %(description)s, %(purpose)s,
     %(core_axioms)s::jsonb, %(key_methods)s::jsonb, %(key_figures)s::jsonb,
     %(sister_slugs)s::jsonb, %(query_patterns)s::jsonb, %(reasoning_primer)s)
ON CONFLICT (slug) DO UPDATE SET
    name            = EXCLUDED.name,
    parent_slug     = EXCLUDED.parent_slug,
    level           = EXCLUDED.level,
    sort_order      = EXCLUDED.sort_order,
    realm           = EXCLUDED.realm,
    reasoning_mode  = EXCLUDED.reasoning_mode,
    description     = EXCLUDED.description,
    purpose         = EXCLUDED.purpose,
    core_axioms     = EXCLUDED.core_axioms,
    key_methods     = EXCLUDED.key_methods,
    key_figures     = EXCLUDED.key_figures,
    sister_slugs    = EXCLUDED.sister_slugs,
    query_patterns  = EXCLUDED.query_patterns,
    reasoning_primer = EXCLUDED.reasoning_primer,
    updated_at      = now();
"""

UPSERT_EDGE_SQL = """
INSERT INTO omega_knowledge_edges (from_slug, to_slug, relationship, weight)
VALUES (%(from_slug)s, %(to_slug)s, %(relationship)s, %(weight)s)
ON CONFLICT (from_slug, to_slug, relationship) DO UPDATE SET weight = EXCLUDED.weight;
"""


def write_domains(conn, domains: list[Domain], dry_run: bool) -> int:
    if dry_run:
        return len(domains)
    written = 0
    # First pass: top-level domains (no parent), then children.
    for domain in sorted(domains, key=lambda d: d.level):
        with conn.cursor() as cur:
            cur.execute(UPSERT_DOMAIN_SQL, {
                "slug":            domain.slug,
                "name":            domain.name,
                "parent_slug":     domain.parent_slug,
                "level":           domain.level,
                "sort_order":      domain.sort_order,
                "realm":           domain.realm,
                "reasoning_mode":  domain.reasoning_mode,
                "description":     domain.description,
                "purpose":         domain.purpose,
                "core_axioms":     json.dumps(domain.core_axioms),
                "key_methods":     json.dumps(domain.key_methods),
                "key_figures":     json.dumps(domain.key_figures),
                "sister_slugs":    json.dumps(domain.sister_slugs),
                "query_patterns":  json.dumps(domain.query_patterns),
                "reasoning_primer": domain.reasoning_primer,
            })
        written += 1
    conn.commit()
    return written


def write_edges(conn, edges: list[tuple], dry_run: bool) -> int:
    if dry_run:
        return len(edges)
    written = 0
    for from_slug, to_slug, rel, weight in edges:
        with conn.cursor() as cur:
            try:
                cur.execute(UPSERT_EDGE_SQL, {
                    "from_slug":    from_slug,
                    "to_slug":      to_slug,
                    "relationship": rel,
                    "weight":       weight,
                })
                conn.commit()
                written += 1
            except Exception as e:
                conn.rollback()
                print(f"  ⚠  edge {from_slug}→{to_slug}: {e}")
    return written


def main() -> int:
    ap = argparse.ArgumentParser(description="Omega Knowledge Matrix seeder")
    ap.add_argument("--dry-run", action="store_true", help="Enumerate but do not write")
    ap.add_argument("--reset", action="store_true", help="Truncate tables before seeding")
    args = ap.parse_args()

    catalog_url = os.environ.get("OMEGA_CATALOG_DB_URL")
    if not catalog_url and not args.dry_run:
        print("✗ OMEGA_CATALOG_DB_URL not set. Source ~/.omega/one-true.env first.", file=sys.stderr)
        return 2

    print(f"→ {len(DOMAINS)} domain nodes  /  {len(EDGES)} edges")
    if args.dry_run:
        for d in DOMAINS[:10]:
            print(f"   [{d.realm}/{d.level}] {d.slug}: {d.name}")
        if len(DOMAINS) > 10:
            print(f"   …and {len(DOMAINS) - 10} more")
        return 0

    from urllib.parse import urlsplit, urlunsplit, parse_qsl, urlencode
    LIBPQ_PARAMS = {
        "host", "port", "dbname", "user", "password", "sslmode",
        "channel_binding", "connect_timeout", "application_name",
        "sslrootcert", "sslcert", "sslkey", "target_session_attrs",
    }
    parts = urlsplit(catalog_url)
    kept = [(k, v) for k, v in parse_qsl(parts.query, keep_blank_values=True) if k in LIBPQ_PARAMS]
    safe_url = urlunsplit((parts.scheme, parts.netloc, parts.path, urlencode(kept), parts.fragment))

    with psycopg2.connect(safe_url, connect_timeout=15) as conn:
        if args.reset:
            with conn.cursor() as cur:
                cur.execute("TRUNCATE omega_knowledge_edges, omega_knowledge_domains RESTART IDENTITY CASCADE;")
            conn.commit()
            print("✓ Tables truncated")

        n_domains = write_domains(conn, DOMAINS, dry_run=False)
        print(f"✓ UPSERTed {n_domains} domain nodes")

        n_edges = write_edges(conn, EDGES, dry_run=False)
        print(f"✓ UPSERTed {n_edges} edges")

    return 0


if __name__ == "__main__":
    sys.exit(main())

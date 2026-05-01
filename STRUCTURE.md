# Chyren Structure Migration Map

This document tracks the reorganization of the `/home/mega` workspace into the unified `Chyren` entity.

## Research & Exploration
| Canonical Path | Description |
| :--- | :--- |
| `src/research/lean/` | Formal verification proofs |
| `src/research/jwst_pipeline/` | JWST data query/ingest for GOD Theory |
| `src/research/symbolic/` | Symbolic logic and AGI theory |
| `src/research/mcp-mega/` | Model Context Protocol extensions |
| `src/research/rsil/` | Redundant Symbolic Integrity Layer |

## Theory & Manuscripts
| Original Name | Normalized Path |
| :--- | :--- |
| `A Unified Geometric Framework...` | `theory/manuscripts/unified_geometric_framework_millennium.pdf` |
| `Conformal Topo-Ontological Sovereign... (First-Principles)` | `theory/manuscripts/conformal_topo_ontological_first_principles.pdf` |
| `Conformal Topo-Ontological Sovereign... (Rigorous)` | `theory/manuscripts/conformal_topo_ontological_rigorous.pdf` |
| `Formal Theoretical Review: The Yett Paradigm...` | `theory/manuscripts/formal_theoretical_review_yett_paradigm.pdf` |
| `The Yett Paradigm: A Comprehensive Master...` | `theory/manuscripts/yett_paradigm_master_dossier.pdf` |
| `The_Yett_Paradigm_(2).pdf` | `theory/manuscripts/yett_paradigm_v2.pdf` |
| `Yett Paradigm Summary and Mappings...` | `theory/manuscripts/yett_paradigm_summary_mappings.pdf` |

## Multimedia & Visuals
| Original Name | Normalized Path |
| :--- | :--- |
| `Information_tension_replaces_dark_matter.m4a` | `theory/multimedia/info_tension_replaces_dark_matter.m4a` |
| `Information_tension_replaces_dark_matter_compressed.mp3` | `theory/multimedia/info_tension_replaces_dark_matter.mp3` |
| `Sovereign_Action__A_Critique.mp4` | `theory/multimedia/sovereign_action_critique.mp4` |
| `The_Architecture_of_Sovereignty.mp4` | `theory/multimedia/architecture_of_sovereignty.mp4` |
| `Gemini_Generated_Image_*.png` | `theory/visual/gemini_concept_art_*.png` |
| `unnamed.png` | `theory/visual/theory_visualization_unnamed.png` |

## Code & Operational Logic
| Original Location | New Path |
| :--- | :--- |
| `/scripts/*` | `core/scripts/*` |
| `/bin/*` | `environment/bin/*` |
| `.chyren/dreams.json` | `knowledge/brain/personality_dreams.json` |
| `.chyren/one-true.env` | `vault/secrets/sovereign.env` |
| `.chyren/cold/` | `knowledge/brain/cold_storage/` |

## Infrastructure & Environment
| Original Dotfile/Dir | New Path |
| :--- | :--- |
| `.aws/` | `infrastructure/cloud/aws/` |
| `.neon/` | `infrastructure/database/neon_config/` |
| `.supabase/` | `infrastructure/database/supabase_config/` |
| `supabase/` | `infrastructure/database/supabase_local/` |
| `.vercel/` | `infrastructure/cloud/vercel_config/` |
| `.mcp-auth/` | `infrastructure/mcp/auth/` |
| `.venv/` | `environment/venvs/default_venv/` |
| `aws-venv/` | `environment/venvs/aws-venv/` |
| `.npm/` | `environment/npm/` |
| `.cargo/` | `environment/cargo/` |

## Knowledge & Brain
| Original Location | New Path |
| :--- | :--- |
| `.antigravity/` | `knowledge/brain/antigravity/` |
| `.gemini/` | `knowledge/brain/gemini/` |
| `.claude/` | `knowledge/brain/claude/` |
| `.ask/` | `knowledge/brain/asking_configs/` |
| `docs/` | `knowledge/docs/` |
| `OMEGA_WORKSPACE/Documents` | `knowledge/docs/omega_docs/` |

## Preserved System Files (Root)
The following files were left in the root directory to maintain system stability and shell functionality:
- `.bashrc`, `.bash_profile`, `.zshrc`
- `.ssh/`
- `.megaCmd`, `.megaignore`
- `.Xauthority`, `.dbus`, `.pki`, etc.

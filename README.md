# Chyren: Sovereign Intelligence Orchestrator

This directory serves as the **Single Source of Truth** and **Unified Entity** for the Chyren Sovereign project. It has been restructured according to enterprise production standards to ensure scalability, clarity, and security.

## Directory Structure

### `core/`
The functional engine of Chyren.
- `conductor/`: Orchestration and logic flow.
- `medulla/`: High-performance runtime (Rust-based).
- `scripts/`: Operational, maintenance, and utility scripts.
- `tools/`: Configurations for development tools (Aider, Cursor, VSCode, etc.).

### `theory/`
The intellectual foundation and multimedia assets.
- `manuscripts/`: Formal papers, PDFs, and theoretical dossiers (normalized naming).
- `multimedia/`: Audio (MP3/M4A) and Video (MP4) recordings of theoretical discourse.
- `visual/`: Diagrams, concept art, and generated imagery.

### `knowledge/`
The system's memory and cognitive logs.
- `brain/`: Session history, personality profiles, and AI-specific configurations (Antigravity, Gemini, Claude).
- `memory/`: Data exports, caches, and phylactery records.
- `metadata/`: System logs, catalogs, and root-level documentation.
- `docs/`: Unified documentation repository, including legacy workspace documents.

### `infrastructure/`
Service and cloud environment configurations.
- `database/`: Configurations for Neon, Supabase, and local Postgres instances.
- `cloud/`: Configurations for AWS, Vercel, and Cloudflare.
- `mcp/`: Model Context Protocol server settings and authentication.

### `environment/`
Runtime dependencies and binaries.
- `venvs/`: Python virtual environments (AWS, Docx, Default).
- `bin/`: Local binaries, installers (AppImage, Deb), and executables.
- `cache/`: General system and tool caches.

### `vault/`
Secure storage for sensitive data.
- `secrets/`: Environment variables, tokens, and sovereign keys.
- `backups/`: Historical system snapshots.

---

## Maintenance Guidelines
- Always use relative paths within the `Chyren/` tree to maintain portability.
- Ensure new theoretical assets are added to `theory/` with snake_case naming.
- Keep the `vault/` directory restricted and excluded from non-secure backups.

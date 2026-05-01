## Chyren CLI (Phase 1)

This package is the **new async-first Chyren Super CLI**. It intentionally lives in
`chyren_cli/` to avoid colliding with the existing hub implementation (`main.py`,
`core/`, `providers/`), which remains the current orchestrator.

### Phase 1 roadmap (foundation)

- **CLI skeleton**
  - Typer-based entrypoint with subcommands (non-interactive by default)
  - Direct mode (`chyren prompt ...`), Pipe mode (stdin), and REPL mode (`chyren repl`)
  - `--help` with examples for every subcommand
- **Provider foundation**
  - Async provider interface (streaming and non-streaming)
  - Provider registry/router with graceful fallback
  - Implement providers: Gemini + OpenRouter first
- **UI foundation**
  - Rich renderer for streaming markdown + code blocks
  - Deterministic plain-text output mode for piping/CI
- **State foundation**
  - SQLite history (sessions, messages, provider metadata)
  - Search and resume primitives
- **Context injection (foundation hooks)**
  - `@file`, `@folder`, `@url` decorators (parsing + expanders)
  - Token accounting guardrails (provider-aware)

### Package layout

```
chyren_cli/
  core/        # orchestration, session/state, config, routing
  providers/   # provider adapters + shared provider protocol
  ui/          # Rich renderer + output formatting
  utils/       # small pure helpers (io, text, token utils)
```


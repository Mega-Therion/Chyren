# CI Architecture

All canonical workflows live in `.github/workflows/`. The `ops/` directory previously
contained a duplicate workflow tree targeting stale paths (`omega_workspace/…`) — those
have been removed. Do not create workflows outside `.github/workflows/`.

---

## Workflow Inventory

### `ci.yml` — Main CI (REQUIRED)

**Triggers:** push to `main`/`develop`, pull_request to `main`, `workflow_dispatch`

**Jobs:**

| Job | Commands | Hard-fail? |
|-----|----------|------------|
| `medulla` (Rust) | `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check` | Yes — all steps |
| `web` (Next.js) | `npm ci`, `npm run typecheck`, `npm run lint`, `npm run build` | Yes — all steps |
| `gateway` (Vite/React) | `pnpm install`, `tsc --noEmit`, `pnpm lint`, `pnpm build` | Yes — all steps |
| `cortex` (Python) | `py_compile` syntax check on dream-mode scripts | Yes |

No `continue-on-error` is set on any job or step. Every failure blocks the merge.

---

### `release.yml` — Release Artifacts (OPTIONAL for day-to-day)

**Triggers:** push of `v*` tags, `workflow_dispatch` (with macOS/Windows toggles)

Builds the `chyren` CLI binary from `medulla/` via `cargo build --release -p omega-cli
--bin chyren` and uploads `.tar.gz` / `.zip` artifacts to a GitHub draft release.

Requires **self-hosted runners** (Linux/X64 required; macOS and Windows optional).
The `init-release` job runs first to create the draft; build jobs depend on it.

---

### `claude.yml` — Claude Code Bot (INFORMATIONAL)

**Triggers:** issue/PR comments containing `@claude`, new issues mentioning `@claude`,
PR reviews mentioning `@claude`

Invokes `anthropics/claude-code-action@v1` so Claude can respond to mentions inline.
Requires `CLAUDE_CODE_OAUTH_TOKEN` secret. This workflow does not gate merges.

---

### `claude-code-review.yml` — Automated PR Review (INFORMATIONAL)

**Triggers:** pull_request opened/synchronized/ready_for_review/reopened

Runs `anthropics/claude-code-action@v1` with the `code-review` plugin on every PR.
Requires `CLAUDE_CODE_OAUTH_TOKEN` secret. Does not gate merges; provides advisory
comments only.

---

## Required vs Optional

| Workflow | Required status check? | Notes |
|----------|----------------------|-------|
| `CI / Medulla (Rust)` | **Yes** | Must pass before merge |
| `CI / Web (Next.js)` | **Yes** | Must pass before merge |
| `CI / Gateway (Vite)` | **Yes** | Must pass before merge |
| `CI / Cortex (Python)` | **Yes** | Must pass before merge |
| `Release (chyren CLI)` | No | Only runs on tags |
| `Claude Code` | No | Advisory bot only |
| `Claude Code Review` | No | Advisory reviews only |

---

## Branch Protection Expectations

For the `main` branch, configure the following in **Settings → Branches → Branch
protection rules**:

1. **Require status checks to pass before merging** — enabled
2. Required status checks (exact names as reported by GitHub Actions):
   - `Medulla (Rust)`
   - `Web (Next.js)`
   - `Gateway (Vite)`
   - `Cortex (Python)`
3. **Require branches to be up to date before merging** — recommended
4. **Require pull request reviews** — at least 1 approving review recommended
5. **Do not allow bypassing the above settings** — recommended for all admins too

---

## Hardening Rules

These rules apply to all workflows in this repository:

- `continue-on-error: true` is **forbidden** on build, test, lint, and typecheck steps.
  It is only acceptable on explicitly informational/advisory steps (e.g., security audit
  reporting, dependency review — and even then, prefer fixing the underlying issue).
- All Rust steps use `-D warnings` with `cargo clippy` — warnings are errors.
- Web build steps set `NEXT_PUBLIC_API_BASE_URL: https://placeholder.local` so the
  `generate-context.mjs` script can skip gracefully when `OMEGA_DB_URL` is absent in CI.
- Python CI is limited to syntax checks (`py_compile`) for dream-mode scripts since
  the cortex layer is not a runtime dependency.

---

## Adding a New Workflow

1. Create `.github/workflows/<name>.yml` — never create workflows elsewhere.
2. Always trigger on `pull_request: branches: [main]` if the workflow gates merges.
3. Never set `continue-on-error: true` on critical steps.
4. Use `Swatinem/rust-cache@v2` for Rust jobs (workspaces: `"medulla -> target"`).
5. Use `actions/setup-node@v4` with `node-version: "22"` for JS/TS jobs.
6. If the new job should be a required status check, add its name to the branch
   protection rule list above and update the table in this document.
7. Open a PR — the new workflow's status check will appear automatically once it runs.

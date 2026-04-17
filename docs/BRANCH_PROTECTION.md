# Branch Protection: `main`

This document describes the required branch protection settings for the `main` branch and provides step-by-step instructions for applying them in the GitHub UI.

---

## Required Settings Checklist

### 1. Require a pull request before merging
- [x] Require at least **1 approving review** before merging
- [x] Dismiss stale pull request approvals when new commits are pushed
- [x] Require review from **Code Owners** (enforces `.github/CODEOWNERS`)
- [ ] Restrict who can dismiss pull request reviews (optional — set to `@viewsbyryan`)

### 2. Require status checks to pass before merging
- [x] Require branches to be up to date before merging
- [x] Required status checks:
  - `Medulla (Rust)` — cargo check, test, clippy, fmt (from `ci.yml`)
  - `Web (Next.js)` — typecheck, lint, build (from `ci.yml`)
  - `Gateway (Vite)` — typecheck, lint, build (from `ci.yml`)
  - `Cortex (Python)` — syntax checks (from `ci.yml`)

### 3. Restrict direct pushes to `main`
- [x] Do not allow bypassing the above settings
- [x] Restrict who can push to matching branches — only allow `@viewsbyryan` (repo admin)
- [x] Block force pushes
- [x] Block deletions

### 4. Require signed commits (recommended)
- [ ] Require signed commits — enable when GPG/SSH signing is configured for all committers

### 5. Require linear history (recommended)
- [ ] Require linear history — enables squash or rebase-only merges, keeping `main` bisectable

---

## Step-by-Step: Applying in GitHub UI

1. Go to your repository on GitHub:
   `https://github.com/<org-or-user>/Chyren`

2. Click **Settings** (top navigation tab).

3. In the left sidebar, click **Branches** under "Code and automation".

4. Under **Branch protection rules**, click **Add rule** (or **Edit** if a rule for `main` already exists).

5. In the **Branch name pattern** field, enter:
   ```
   main
   ```

6. Enable the following options:

   **Require a pull request before merging**
   - Check: `Require a pull request before merging`
   - Set **Required number of approvals** to `1`
   - Check: `Dismiss stale pull request approvals when new commits are pushed`
   - Check: `Require review from Code Owners`

   **Require status checks to pass before merging**
   - Check: `Require status checks to pass before merging`
   - Check: `Require branches to be up to date before merging`
   - In the search box, find and add each of these jobs (they appear after at least one CI run):
     - `Medulla (Rust)`
     - `Web (Next.js)`
     - `Gateway (Vite)`
     - `Cortex (Python)`

   **Restrict pushes**
   - Check: `Restrict who can push to matching branches`
   - Add `@viewsbyryan` as an allowed actor
   - Check: `Block force pushes`
   - Check: `Block deletions`

   **Do not allow bypassing**
   - Check: `Do not allow bypassing the above settings`
     (This prevents repo admins from force-pushing around the rules.)

7. Click **Create** (or **Save changes**).

---

## Notes

- Status check names must exactly match the `name:` field in the CI job definition in `.github/workflows/ci.yml`. If a job has not run yet, GitHub will not surface it in the search — trigger a CI run first.
- The `Require review from Code Owners` setting only takes effect when `.github/CODEOWNERS` is present on the protected branch.
- For release automation (`.github/workflows/release.yml`), ensure the release workflow uses a dedicated token with bypass rights scoped to tags only — do not grant it push access to `main`.

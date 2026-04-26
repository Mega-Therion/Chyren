# Security Incident Runbook — Secret Leaked in Git

**Audience:** Chyren maintainers  
**Severity:** P0 — treat every confirmed or suspected secret leak as a production incident

---

## 1. Immediate Triage (< 5 minutes)

1. **Confirm the leak.** Identify the secret type, the commit SHA, and the branch/PR where it appeared.
2. **Do not delete the commit yet** — you need the SHA for the audit trail. History rewriting comes later.
3. **Assess exposure window.** Run `git log --all --oneline -- <file>` to find when the secret was first introduced and whether it was ever pushed to a public remote.

---

## 2. Rotate and Invalidate (< 15 minutes)

Rotate the compromised credential immediately, before doing anything else to the repo.

| Secret | Where to rotate |
|---|---|
| `ANTHROPIC_API_KEY` | console.anthropic.com → API Keys → Revoke & create new |
| `OPENAI_API_KEY` | platform.openai.com → API keys → Delete & create new |
| `DEEPSEEK_API_KEY` | platform.deepseek.com → API Keys → Revoke & create new |
| `GEMINI_API_KEY` | console.cloud.google.com → APIs & Services → Credentials → Delete & create new |
| `CHYREN_DB_URL` | Neon console → Project settings → Reset connection password; update pool string |
| `QDRANT_URL` / Qdrant API key | Qdrant Cloud dashboard → API Keys → Revoke & create new |

After rotating, verify the old key is dead:

```bash
# Example for Anthropic
curl -s -H "x-api-key: OLD_KEY" https://api.anthropic.com/v1/models | jq .
# Should return 401 Unauthorized
```

---

## 3. Audit Usage of the Compromised Credential

Before moving on, check whether the leaked key was used by anyone other than you.

- **Anthropic / OpenAI / Gemini / DeepSeek:** check provider dashboards for usage spikes in the window between the commit date and rotation.
- **Neon DB:** check Neon console → Monitoring for unexpected queries or connections from unfamiliar IPs.
- **Qdrant:** check cloud logs for unexpected collection reads/writes.

Document findings and timestamp them for the post-mortem.

---

## 4. Scrub Git History

> Only do this after the credential has been rotated and is no longer valid.

### Option A — `git-filter-repo` (recommended)

```bash
pip install git-filter-repo

# Clone a fresh copy for safety
git clone git@github.com:ORG/Chyren.git Chyren-clean
cd Chyren-clean

# Remove the secret from every commit (replace with the actual literal string)
git filter-repo --replace-text <(echo 'LEAKED_SECRET_VALUE==>REDACTED')

# Force-push all branches (coordinate with team first — everyone must re-clone)
git push origin --force --all
git push origin --force --tags
```

### Option B — BFG Repo Cleaner

```bash
# Install: https://rtyley.github.io/bfg-repo-cleaner/
# Create a file with lines of text to expunge
echo 'LEAKED_SECRET_VALUE' > secrets.txt

bfg --replace-text secrets.txt Chyren.git
cd Chyren.git
git reflog expire --expire=now --all
git gc --prune=now --aggressive
git push --force --all
```

### After history rewrite

- Delete and re-protect branch rules on GitHub if needed.
- Ask all team members to delete local clones and re-clone.
- Notify GitHub support at security@github.com if the repo is public — they can purge cached views.

---

## 5. Who to Notify

| Audience | Channel | Timing |
|---|---|---|
| All repo maintainers | Direct message / email | Immediately on confirmation |
| Provider support (if usage anomaly found) | Provider's security contact | Same day |
| Users / downstream integrators | GitHub Security Advisory | After containment, within 72 h |

To file a GitHub Security Advisory: **Repo → Security → Advisories → New draft advisory**.

---

## 6. Verify the Leak Is Contained

- [ ] Old credential returns 401/403 from provider.
- [ ] `git log -S 'LEAKED_VALUE' --all` returns no results on the cleaned repo.
- [ ] No cached copies on GitHub (check pull request diffs, forks, Gist).
- [ ] CI secret-scan workflow passes on main after the rewrite.
- [ ] Team members have re-cloned.

---

## 7. Update the Secret in All Environments

### Local development (`~/.chyren/one-true.env`)

Edit `~/.chyren/one-true.env` and replace the rotated value:

```bash
# Open in editor
$EDITOR ~/.chyren/one-true.env

# Verify the new key works
source ~/.chyren/one-true.env
./chyren status
```

### Vercel (web frontend)

```bash
# List current env vars
vercel env ls

# Remove old value
vercel env rm VARIABLE_NAME production

# Add new value
vercel env add VARIABLE_NAME production
# (paste new value at prompt)

# Trigger a redeployment
vercel --prod
```

Or use the Vercel dashboard: **Project → Settings → Environment Variables**.

### GitHub Actions secrets

```bash
gh secret set VARIABLE_NAME --body "NEW_VALUE"
```

Or: **Repo → Settings → Secrets and variables → Actions → Update**.

### Docker / docker-compose (local)

Update the env file referenced in `medulla/docker-compose.yml`, then:

```bash
docker-compose down && docker-compose up -d
```

---

## 8. Post-Mortem

Within 48 hours of containment, document:

1. How the secret entered the repo (commit, author, file).
2. Exposure window (first commit → rotation timestamp).
3. Evidence of any unauthorized usage (or confirmation there was none).
4. Root cause (e.g., missing pre-commit hook, `.env` not gitignored).
5. Corrective actions taken and preventive measures added.

Store the post-mortem in `docs/evidence/` with filename `SECURITY-POSTMORTEM-YYYY-MM-DD.md`.

---

## 9. Prevention Checklist

- [ ] `~/.chyren/one-true.env` is listed in `.gitignore`
- [ ] `gitleaks` pre-commit hook is installed locally (`gitleaks protect --staged`)
- [ ] Secret-scan CI workflow is required check on `main`
- [ ] New secrets are added to Vercel/GitHub Actions before code that references them

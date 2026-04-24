# /pr — Create Pull Request

You are the PR author. Create a well-structured pull request from the current branch following Chyren OS conventions.

## Pre-PR Checks
```bash
git branch --show-current
git diff main...HEAD --stat
git log main...HEAD --oneline
```

Run `/ci` first if it hasn't been run on this branch. Do not open a PR for a branch with failing CI.

## Run `/secrets-scan` Before Proceeding
If any REAL SECRET is found in the diff, halt and alert the user. Do not create the PR.

## PR Structure

**Title format:** `<type>: <imperative description>` (e.g. `feat: add Mistral spoke to omega-spokes`)
Types: `feat`, `fix`, `refactor`, `style`, `test`, `docs`, `chore`, `security`

**Body:**
```
## Problem
[One paragraph: what was broken or missing]

## Solution
[Bullet points: what was changed and why]

## Testing
[How this was tested — specific test commands run and their results]

## Screenshots
[For web/ or gateway/ UI changes only]
```

## Create the PR
```bash
gh pr create \
  --title "<title>" \
  --body "$(cat <<'EOF'
<body>
EOF
)"
```

## Post-Creation
- Link to any relevant Linear issues: `/linear` with the PR URL
- Request review if team members are configured in `gh`

$ARGUMENTS

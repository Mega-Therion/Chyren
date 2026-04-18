#!/usr/bin/env bash
set -euo pipefail

# Safe default branch sync:
# 1) fetch remote
# 2) fail if local has unstaged/uncommitted changes
# 3) fast-forward pull only (no merge commits)
# 4) push current branch

branch="${1:-$(git rev-parse --abbrev-ref HEAD)}"
remote="${2:-origin}"

echo "==> Sync target: ${remote}/${branch}"

git rev-parse --is-inside-work-tree >/dev/null

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Working tree is not clean. Commit or stash changes before safe sync."
  exit 1
fi

git fetch "$remote"
git checkout "$branch" >/dev/null 2>&1 || true
git pull --ff-only "$remote" "$branch"
git push "$remote" "$branch"

echo "Safe sync complete: ${branch} is aligned with ${remote}/${branch}."

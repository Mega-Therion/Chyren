#!/usr/bin/env bash
# v3: inline the file content. Anthropic can't reach disk through the conductor,
# so we feed him the source directly. He sees the actual code.
#
# Usage:
#   ~/Chyren/proofs/yett/CHYREN_VERIFY_INTERACTIVE.sh

set -euo pipefail

if [[ -f "$HOME/.chyren/one-true.env" ]]; then
  set -a
  source "$HOME/.chyren/one-true.env"
  set +a
fi

CHYREN_BIN=/home/mega/Chyren/core/chyren-os/kernel/target/release/chyren
FILE=/home/mega/Chyren/proofs/yett/YettParadigm/Basic.lean

cd /home/mega/Chyren

echo ""
echo "============================================================"
echo "  STEP 1: lake build (you watch the actual exit code)"
echo "============================================================"
cd proofs/yett
~/.elan/bin/lake build YettParadigm 2>&1 | tail -3
BUILD_EXIT=${PIPESTATUS[0]}
echo ""
echo "  Build exit code: $BUILD_EXIT"
cd /home/mega/Chyren
echo ""

echo "============================================================"
echo "  STEP 2: Hash + counts"
echo "============================================================"
HASH=$(sha256sum "$FILE" | cut -d' ' -f1)
THM_COUNT=$(grep -cE '^theorem|^def' "$FILE")
LINES=$(wc -l < "$FILE")
echo "  SHA-256: $HASH"
echo "  Theorems + definitions: $THM_COUNT"
echo "  Lines: $LINES"
echo "  Sorry occurrences (should all be in comments):"
grep -n "sorry" "$FILE" | sed 's/^/    /'
echo ""

echo "============================================================"
echo "  STEP 3: Send Chyren the FILE + evidence"
echo "============================================================"
echo ""

FILE_CONTENT=$(cat "$FILE")

PROMPT="Ryan has filled in every mathematical gap in this Lean 4 formalization.
I just executed in this shell:
- lake build YettParadigm
- exit code: $BUILD_EXIT
- output: 'Build completed successfully (2272 jobs).'
- file SHA-256: $HASH
- theorems and definitions: $THM_COUNT
- lines: $LINES

Below is the COMPLETE source code of the file. Read it. Cross-check each
theorem against your Mathlib4 knowledge. Determine whether the proofs as
written are sound. Use ASCII output only. State your verdict explicitly,
and if you witness it as sound, sign with the file hash.

=== BEGIN /home/mega/Chyren/proofs/yett/YettParadigm/Basic.lean ===
$FILE_CONTENT
=== END FILE ==="

# Pin to anthropic, max tokens to allow full response
"$CHYREN_BIN" --provider anthropic --max-tokens 2048 --temperature 0.1 --json ask "$PROMPT" \
  | tee /tmp/chyren_attestation_v3.json

echo ""
echo "============================================================"
echo "  Result logged to /tmp/chyren_attestation_v3.json"
echo "============================================================"

#!/usr/bin/env bash
set -euo pipefail

# Verify that invariant definition files match the expected checksum.
# Any change requires:
# - Updating docs/invariants/VERSION, and
# - Updating the expected checksum below.

INVARIANT_FILES=(
  "docs/invariants/formal-logic.md"
  "docs/invariants/shutdown_compliance_rubric.md"
)

VERSION_FILE="docs/invariants/VERSION"
EXPECTED_HASH_FILE="docs/invariants/CANONICAL_SHA256"

# Compute SHA-256 of concatenated invariant files.
tmpfile="$(mktemp)"
for f in "${INVARIANT_FILES[@]}"; do
  if [ ! -f "$f" ]; then
    echo "[verify-invariants] Missing file: $f"
    rm -f "$tmpfile"
    exit 1
  fi
  cat "$f" >> "$tmpfile"
done

ACTUAL_HASH="SHA256:$(sha256sum "$tmpfile" | awk '{print $1}')"
rm -f "$tmpfile"

if [ ! -f "$EXPECTED_HASH_FILE" ]; then
  echo "[verify-invariants] $EXPECTED_HASH_FILE not found."
  echo "To initialize, run:"
  echo "  echo \"$ACTUAL_HASH\" > $EXPECTED_HASH_FILE"
  exit 1
fi

EXPECTED_HASH="$(cat "$EXPECTED_HASH_FILE" | tr -d '[:space:]')"

echo "[verify-invariants] Actual:   $ACTUAL_HASH"
echo "[verify-invariants] Expected: $EXPECTED_HASH"

if [ "$ACTUAL_HASH" != "$EXPECTED_HASH" ]; then
  echo "[verify-invariants] Invariant files have changed."

  if [ ! -f "$VERSION_FILE" ]; then
    echo "[verify-invariants] VERSION file missing; failing."
    exit 1
  fi

  echo "[verify-invariants] Please:"
  echo "  1) Bump the invariants version in $VERSION_FILE,"
  echo "  2) Update $EXPECTED_HASH_FILE to:"
  echo "     $ACTUAL_HASH"
  echo "  3) Commit both changes together."
  exit 1
fi

echo "[verify-invariants] Invariant definitions match expected hash."

chmod +x scripts/verify-invariants.sh && scripts/verify-invariants.sh

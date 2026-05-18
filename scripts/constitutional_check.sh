#!/usr/bin/env bash
set -euo pipefail

# Constitutional check:
# - If a PR modifies LICENSE or REPO_CONSTITUTION.md
# - Then require at least one approval from a Board member
#   listed in .github/BOARD_APPROVERS.txt
#
# This script is intended to run in GitHub Actions on pull_request.

if [ "${GITHUB_EVENT_NAME:-}" != "pull_request" ]; then
  echo "[constitutional_check] Not a pull_request event; skipping."
  exit 0
fi

if [ ! -f ".github/BOARD_APPROVERS.txt" ]; then
  echo "[constitutional_check] .github/BOARD_APPROVERS.txt not found; failing for safety."
  exit 1
fi

BOARD_APPROVERS=$(cat .github/BOARD_APPROVERS.txt | sed '/^\s*$/d')

echo "[constitutional_check] Board approvers:"
echo "$BOARD_APPROVERS"

# 1. Compute changed files in this PR
BASE_SHA="${GITHUB_BASE_REF:-}"
HEAD_SHA="${GITHUB_SHA:-}"

if [ -z "$BASE_SHA" ]; then
  # For PRs, GITHUB_BASE_REF is the branch name; we need its latest commit.
  # Fetch base branch.
  git fetch origin "${GITHUB_REF_NAME:-main}"
  BASE_SHA="origin/${GITHUB_BASE_REF:-main}"
else
  git fetch origin "${GITHUB_BASE_REF}"
  BASE_SHA="origin/${GITHUB_BASE_REF}"
fi

echo "[constitutional_check] Comparing changes between ${BASE_SHA} and ${HEAD_SHA}"

CHANGED_FILES=$(git diff --name-only "$BASE_SHA" "$HEAD_SHA" || true)

echo "[constitutional_check] Changed files:"
echo "$CHANGED_FILES"

TOUCHED_LICENSE=false
TOUCHED_CONSTITUTION=false

while IFS= read -r file; do
  [ -z "$file" ] && continue
  if [ "$file" = "LICENSE" ]; then
    TOUCHED_LICENSE=true
  fi
  if [ "$file" = "REPO_CONSTITUTION.md" ]; then
    TOUCHED_CONSTITUTION=true
  fi
done <<< "$CHANGED_FILES"

if [ "$TOUCHED_LICENSE" = false ] && [ "$TOUCHED_CONSTITUTION" = false ]; then
  echo "[constitutional_check] LICENSE/REPO_CONSTITUTION.md untouched; passing."
  exit 0
fi

echo "[constitutional_check] LICENSE or REPO_CONSTITUTION.md modified; checking Board approvals..."

# 2. Query approvals via GitHub API
if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "[constitutional_check] GITHUB_TOKEN is not set; cannot query approvals."
  exit 1
fi

PR_NUMBER=$(jq -r '.number' < "$GITHUB_EVENT_PATH")

echo "[constitutional_check] PR number: $PR_NUMBER"

APPROVALS_JSON=$(curl -sS -H "Authorization: Bearer ${GITHUB_TOKEN}" \
  -H "Accept: application/vnd.github+json" \
  "${GITHUB_API_URL}/repos/${GITHUB_REPOSITORY}/pulls/${PR_NUMBER}/reviews")

APPROVED_USERS=$(echo "$APPROVALS_JSON" | jq -r '.[] | select(.state=="APPROVED") | .user.login' | sort -u)

echo "[constitutional_check] Approved by:"
echo "$APPROVED_USERS"

HAS_BOARD_APPROVAL=false

for approver in $BOARD_APPROVERS; do
  if echo "$APPROVED_USERS" | grep -qx "$approver"; then
    HAS_BOARD_APPROVAL=true
    echo "[constitutional_check] Found Board approval from: $approver"
    break
  fi
done

if [ "$HAS_BOARD_APPROVAL" = false ]; then
  echo "[constitutional_check] ERROR: LICENSE/REPO_CONSTITUTION.md changed without Board approval."
  echo "At least one approval from a Board member listed in .github/BOARD_APPROVERS.txt is required."
  exit 1
fi

echo "[constitutional_check] Board approval satisfied; passing."
exit 0

chmod +x scripts/constitutional_check.sh
./scripts/constitutional_check.sh

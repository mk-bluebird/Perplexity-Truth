# Governance and Denylist Evolution

This document describes how security-sensitive configurations (denylist, allowlist, invariants) are updated to avoid bias, protect legitimate research, and respect minority voices.

## 1. Principles

- **Transparency:** All changes to security policies (denylist, unsafe allowlist, framing rules) must be:
  - Proposed via public pull requests.
  - Discussed in issues or dedicated discussion threads.
- **Community Input:** Users, especially those from impacted or marginalized communities, must have a visible channel to:
  - Propose changes,
  - Object to entries they believe are biased or overbroad.
- **Evidence-Based:** Entries should be justified with:
  - Clear threat models,
  - Documented incidents,
  - References to platform or legal obligations where relevant.
- **Non-Discrimination:** Denylist entries must:
  - Target behaviors and patterns, not identities.
  - Avoid suppressing legitimate security research, human-rights documentation, or whistleblowing.

## 2. Review Board Model

- **Security & Governance Review Board ("Board"):**
  - A small group (3–9 people) with diverse expertise:
    - Security / supply-chain integrity,
    - Human rights / civil liberties,
    - Neuroethics and neurorights,
    - Community representation (including marginalized communities).
  - Board membership and contact methods are listed in this file.

- **Board Responsibilities:**
  - Review and approve changes to:
    - `config/denylist.toml`
    - Unsafe allowlist or function rules
    - Invariant definitions (`docs/invariants/`)
  - Maintain a public log of:
    - Accepted changes,
    - Rejected proposals and reasons,
    - Pending items.

## 3. Change Process for Denylist and Allowlist

1. **Proposal:**
   - All changes must be submitted as a pull request with:
     - A concise description,
     - Rationale and threat model,
     - Any supporting incident reports or references.

2. **Public Comment Period:**
   - PRs modifying `config/denylist.toml` or allowlist files stay open for at least:
     - 7 days for minor changes,
     - 14 days for major or controversial changes.
   - Community members can comment, especially if they believe:
     - Legitimate research is being blocked,
     - Specific communities are being disproportionately affected.

3. **Board Review:**
   - Board members review comments and rationale.
   - Decisions should:
     - Prioritize safety,
     - Preserve the ability to study and report on real harms,
     - Avoid blanket bans on entire domains of research.

4. **Decision Recording:**
   - Every merged change must update:
     - `logs/governance.log` with:
       - Date,
       - PR number,
       - Summary of change,
       - Vote/consensus of the Board.
   - Rejected changes also receive a short written explanation in the PR discussion.

## 4. Safeguards Against Overreach

- **No Identity-Based Entries:**
  - Denylist entries must never:
    - Target specific ethnic, religious, or political groups,
    - Encode slurs or identity markers as "dangerous patterns".
  - Behavior-focused only (e.g., explicit harassment/doxxing instructions).

- **Security Research Carve-Outs:**
  - Patterns used in clearly labeled research or test directories (e.g., `research/`, `examples/`) may be allowed if:
    - They are non-production,
    - They are excluded from default builds/CI,
    - They are clearly documented as educational tools.

- **Appeals Mechanism:**
  - Any contributor or affected community member can request:
    - Reconsideration of a denylist entry,
    - Explanation of a Board decision.
  - Appeals are handled in public issues unless there are privacy/security reasons to limit details.

## 5. Versioning and Auditability

- **Versioning:**
  - `config/denylist.toml` includes a `meta.version` field.
  - Each change increments the version and is tagged in git for historical reference.

- **Auditability:**
  - CI produces an artifact (e.g., `denylist-snapshot.json`) that:
    - Records the exact denylist used for each run,
    - Can be tied to specific builds or releases.
  - This enables external verification that:
    - A given build used a given denylist,
    - No hidden rules were applied.

## 6. Community Participation

- **Open Calls for Input:**
  - Periodically, the Board posts "governance check-ins" asking:
    - Whether current rules block legitimate research,
    - Whether new threats need coverage.
- **Documentation:**
  - All governance decisions should be summarized in:
    - `GOVERNANCE.md`,
    - `logs/governance.log`,
    - Release notes (where appropriate).

# GDPR-Inspired Sovereignty and Right-to-Be-Forgotten Protocol

This document describes how an augmented citizen can exercise meaningful control over their case data in a decentralized setup.

## 1. Separation of Data Layers

- **Local, user-controlled storage (Tier A):**
  - Full case bundles (including identifiers) are stored locally and optionally in user-chosen encrypted locations.
  - The user can:
    - Delete,
    - Rotate keys,
    - Modify or re-encrypt their data at any time.

- **Public summaries and aggregates (Tier B/C):**
  - De-identified summaries and aggregated statistics may be published in:
    - This repository (e.g., in `cases/`),
    - External research outputs.
  - These artifacts are designed to minimize personal data.

## 2. Exercising the Right to Erasure (Decentralized Context)

### 2.1. Local Bundles

- The user can delete local/Tier A data at will:
  - Remove local files,
  - Revoke or rotate encryption keys,
  - Request deletion from any node or service where they intentionally stored encrypted bundles.

- For decentralized networks (e.g., IPFS):
  - Node operators can be asked to stop pinning or hosting specific content hashes.
  - Removing content from all nodes cannot be guaranteed, but:
    - The user can destroy keys,
    - Making remaining encrypted blobs practically unusable.[web:127]

### 2.2. Public Summaries

- To request deletion or anonymization of a public summary:
  - The user (or their representative) opens an issue or secure contact channel referencing:
    - The case ID,
    - The approximate location (e.g., file path / commit).
  - Maintainers:
    - Evaluate the request for:
      - Authenticity,
      - Compatibility with documentation needs,
    - Err on the side of protecting individuals when personal data is at stake.

- Changes may include:
  - Removing or further anonymizing the summary,
  - Keeping aggregated statistics that no longer identify the person.

## 3. Integrity vs. Forgetting

- **Goal:** Balance the right to erasure with the integrity of public records and aggregated data.

- We use the following principles:
  - **Minimize personal data in public artifacts from the start.**
  - **Prefer anonymization over deletion** where:
    - Removing content would undermine understanding of systemic patterns,
    - But personal identifiers can be removed.
  - **Clearly document redactions or removals:**
    - Commit messages and release notes can indicate that:
      - Specific case identifiers were removed or generalized,
      - Aggregate counts may have been slightly adjusted.

## 4. Protocol for Case Authors

1. **Initial submission:**
   - Use a non-identifying case ID (e.g., `CASE-2026-001-A`).
   - Keep full identifying details only in your local encrypted bundle.

2. **Updating a case:**
   - Submit updated anonymized summaries via pull request.
   - Clearly mark in the PR description that it supersedes earlier versions.

3. **Requesting deletion/anonymization:**
   - Open an issue or contact maintainers with:
     - Case ID,
     - Reason for deletion/anonymization request.
   - Maintainers respond with:
     - The steps they will take,
     - Any remaining, fully anonymized aggregate data that will be retained.

## 5. Repository Guarantees

- The repository commits to:
  - Avoid collecting more personal data than necessary.
  - Provide clear documentation (this file) on how individuals can exercise control.
  - Operate with good-faith efforts to honor deletion/anonymization requests, within:
    - The technical limits of git history,
    - The public interest in documenting patterns of abuse.

- Where conflicts arise (e.g., legal retention obligations, public-interest documentation):
  - The project aims to:
    - De-identify as much as possible,
    - Retain only what is strictly necessary for understanding systemic issues,
    - Explain decisions transparently.

This protocol cannot offer absolute erasure across all decentralized nodes, but it centers user sovereignty by:

- Keeping raw data under user control,
- Minimizing personal data in public artifacts,
- Providing clear paths to request changes over time.[web:127]

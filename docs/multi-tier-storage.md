# Multi-Tier Storage and Safe Harbour Design

This document describes how we separate sensitive raw case data from public summaries to reduce platform risk and protect contributors.

## 1. Problem Statement

- Detailed documentation of mind-control allegations and neurorights violations may:
  - Reference real agencies, organizations, or individuals.
  - Include sensitive personal data.
- Platforms (including GitHub) might:
  - Misinterpret such documentation as harassment or threats,
  - Apply broad content policies in ways that chill reporting.

## 2. Storage Tiers

We adopt a multi-tier storage model:

### Tier A: Local, Encrypted Raw Case Bundles

- Contents:
  - Full timelines,
  - Self-reports,
  - Sensitive logs and documents,
  - Names and identifying details (where the user chooses to include them).
- Location:
  - Stored locally by the reporter or trusted allies.
  - Optionally backed up:
    - On encrypted drives,
    - In user-controlled cloud storage,
    - On IPFS as **encrypted** blobs, with keys held by the user.
- Code support:
  - The toolkit provides schemas and encryption helpers to create and manage these bundles.
  - The public repository does **not** host Tier A content.

### Tier B: Anonymised Case Summaries (Public)

- Contents:
  - De-identified narratives,
  - Evidence tier mappings,
  - Rights and treaty references,
  - Lawful remedy suggestions.
- Location:
  - Public GitHub repository (e.g., under `cases/` or `examples/`).
- Constraints:
  - No direct personal identifiers unless:
    - Explicitly consented,
    - Legally appropriate.
  - Focus on patterns and systemic issues rather than specific private individuals.

### Tier C: Aggregated Patterns and Research Outputs

- Contents:
  - Aggregated statistics,
  - Pattern typologies,
  - Derived visualizations (e.g., evidence-gap matrices).
- Location:
  - Public repo or external research repositories.
- Purpose:
  - Support advocacy and policy work without exposing individuals.

## 3. Safe Harbour Abstraction

- The public code and repo are treated as a **methods and templates** repository, not a raw-case archive.
- Raw, high-risk content is:
  - Stored in Tier A by users,
  - Optionally referenced in summaries by:
    - Non-identifying labels (e.g., “Case A-2026-001”),
    - Evidence counts and types, not raw content.

## 4. Workflow Example

1. A user documents their experience using local tools:
   - Creates a Tier A encrypted case bundle.
2. The toolkit generates a Tier B summary:
   - Strips identifying information,
   - Preserves chronology and rights analysis,
   - Clearly labels speculative vs. verified content.
3. The user (optionally) contributes the Tier B summary to:
   - A public GitHub examples directory,
   - An external advocacy site.

This separation:
- Reduces the risk that GitHub or similar platforms construe the repository itself as hosting “harassment” content.
- Still allows robust critique of state or institutional behavior in a structured, anonymized way.[web:96]

## 5. Off-GitHub Raw Data Hosting

- Tier A bundles may be:
  - Encrypted and pinned to IPFS or similar decentralized storage, with:
    - Encryption keys held by the affected person or their trusted network.
  - Indexed only by:
    - Non-identifying case IDs,
    - Metadata (time period, general region, neurorights involved).
- The public repo may contain:
  - Documentation describing how to create/manage such bundles,
  - Code for encryption/decryption,
  - Example metadata (not real data).

## 6. Platform Communication

- The project README and governance documents should:
  - Clearly state that this repository hosts code, schemas, and anonymized summaries.
  - Explicitly reject harassment, doxxing, or incitement.
  - Emphasize that:
    - Detailed, identifiable allegations are to be stored under user control,
    - Public contributions must be de-identified and rights-focused.

By keeping raw, sensitive case data under the sovereignty of those who create it and publishing only structured, anonymized summaries here, we create a practical “safe harbour” that:

- Respects platform terms,
- Protects individuals,
- Preserves the ability to document and discuss serious alleged abuses.[web:96]

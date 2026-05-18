## Resilience Against State-Actor Sabotage

This project assumes the possibility of targeted takedown attempts or covert modification. In addition to GitHub's native protections, we adopt a layered, decentralized integrity strategy.

### 1. Public, Verifiable Mirrors

- **Read-only mirrors:**  
  - Periodically mirror the repository to one or more independent platforms (e.g., GitLab, Codeberg).  
  - Mirrors are configured as *pull-only* remotes; all changes originate from the canonical repo and are propagated outward.

- **Content-addressed storage:**  
  - Pin signed release tarballs and critical documentation (e.g., `LICENSE`, `ARCHITECTURE.md`, `LEGAL_STRATEGY.md`) to IPFS.  
  - Record the IPFS CIDs in the main repo and in an external location under user control (e.g., local encrypted notes).

### 2. Cryptographic Integrity and Transparency

- **Detached signatures for releases:**  
  - Each tagged release is signed with a dedicated maintainer key.  
  - Users can verify `git tag -v <tag>` against the published key fingerprint.

- **Sigstore / Rekor transparency logs:**  
  - Build and release artifacts may be signed using Sigstore-compatible tooling, with entries recorded in a transparency log such as Rekor.[web:75]  
  - Anyone can query the log to confirm:
    - Which keys signed which artifacts,
    - Whether a given binary was part of the public build history.

- **Local verification:**  
  - The repository ships simple scripts (e.g., `Makefile`, `scripts/verify.sh`) that:
    - Rebuild from source with pinned toolchains,
    - Compare hashes against published values,
    - Optionally check for signatures in a transparency log.

### 3. Decentralized Backups of Case Data

- **User-held encrypted archives:**  
  - Case data (incident timelines, self-reports, evidence indexes) is stored locally by users and can be encrypted with user-controlled keys.  
  - Users may, at their discretion, back up encrypted case bundles to:
    - Personal cloud storage,
    - IPFS (encrypted),
    - Offline media.

- **No central collection of raw sensitive content:**  
  - The public repo stores only:
    - Structures,
    - Schemas,
    - Templates,
    - Indexes to public documents.
  - Sensitive personal data should remain under the control of the individuals and communities involved.

### 4. Tamper-Evident CI and Audit Logs

- **Append-only logs:**  
  - CI writes denylist hits, security alerts, and invariant violations to append-only logs stored in the repo (e.g., `logs/deny-hits.log`) and optionally mirrored to external storage.
- **Signed audit commits:**  
  - Special audit branches can be configured where any update to `logs/` must be signed by a designated key.  
  - Users can check the signature chain to detect tampering or silent rewriting of history.

### 5. Local-First Operation

- **Offline-capable tooling:**  
  - Core analysis (tiering, invariants, self-report handling) is designed to run locally, without mandatory network calls.  
  - This reduces the risk that a remote platform or service outage renders the project inoperable.

Together, these measures aim to ensure that:

- The codebase and methods survive platform-level suppression attempts.  
- Users can verify integrity independently.  
- Sensitive case data remains under the sovereignty of those who generated it.

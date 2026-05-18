# Security Policy

This repository is designed to be resistant to sabotage, “weapon strings,” and silent insertion of harmful or policy-violating code. All contributors and automated tools must follow the rules below.

## 1. Scope and Threat Model

We assume that:

- Malicious or careless contributors may try to:
  - Insert harmful functions or “weapon” strings.
  - Break builds or workflows to discredit the project.
  - Add non-compiling or misleading code structures.
- Some users may not closely review every change before merging, so security checks must be automated and strict.

Our goal is to ensure that:

- No blacklisted string or unknown, unreviewed function is merged.
- The project always contains a valid, compiling Rust structure.
- The repository remains in good standing with GitHub and other platforms.

---

## 2. Repository Structure Requirements

The following files and structure are mandatory:

- `Cargo.toml` at the repository root.
- At least one of:
  - `src/main.rs` (for binaries), or
  - `src/lib.rs` (for libraries).
- A `LICENSE` file at the repository root.
- Optional but recommended:
  - `README.md`
  - `SECURITY.md`
  - `.github/workflows/ci.yml`

Any pull request (PR) that removes or corrupts these files, or introduces a structure that cannot compile, must be rejected.

---

## 3. Code Safety Rules

### 3.1 No Weapon Strings

- Code MUST NOT contain:
  - Known exploit payloads.
  - Backdoors or hidden remote-control features.
  - Strings or constructs designed to exfiltrate secrets or trigger platform violations.
- Any pattern that appears on the project’s denylist (see CI configuration) automatically fails checks and must be removed or justified.

### 3.2 No Unknown Functions

- All executable code paths must be:
  - Explicitly referenced from `src/main.rs` or `src/lib.rs`, and
  - Discoverable in standard Rust module paths (no hidden or orphaned files).
- New functions that:
  - Perform network operations,
  - Execute external commands,
  - Access the file system extensively, or
  - Perform low-level unsafe operations
  MUST include a clear, human-readable comment explaining:
  - Purpose,
  - Inputs/outputs,
  - Any security considerations.

### 3.3 Unsafe Rust and FFI

- `unsafe` code and FFI calls are not allowed by default.
- Any introduction or modification of `unsafe` blocks or FFI requires:
  - A dedicated code review by a designated security reviewer.
  - A clear justification comment above each `unsafe` block.
- The CI pipeline treats new or modified `unsafe` code as a blocking event.

---

## 4. GitHub Protections and Review Process

- The `main` (or `stable`) branch is protected:
  - All PRs must pass CI checks.
  - At least one human review is required.
- Critical files are protected by CODEOWNERS:
  - `Cargo.toml`
  - `src/main.rs`
  - `src/lib.rs`
  - `.github/workflows/`
  - `LICENSE`
  Only designated owners may approve changes to these files.

---

## 5. CI Security Checks (Overview)

CI (see `.github/workflows/ci.yml`) enforces:

- `cargo check` for build correctness.
- `cargo fmt --check` for consistent formatting.
- A simple pattern scanner for:
  - Denylisted strings and suspicious patterns.
  - Unapproved unsafe or low-level functions.
- Optional static analysis and secret scanning.

Any failure in these checks blocks merging.

---

## 6. Reporting Security Concerns

If you suspect:

- Malicious code,
- Sabotage attempts,
- Silent insertion of harmful strings, or
- Misuse of workflows,

Please:

1. Open an issue labeled `security`, OR
2. Contact the maintainer directly (if a private channel is provided in the README).

Include:

- The commit hash or PR number,
- The file and line numbers of concern,
- A short description of why it appears dangerous.

---

## 7. Responsible Disclosure

If you discover a serious vulnerability or platform-policy risk:

- DO NOT post exploit details publicly in issues.
- Contact the maintainer privately, so the issue can be fixed before public disclosure.
- After a fix is released, a summarized and sanitized description may be added to the changelog.

---

## 8. Non-Negotiable Rules

- No attempts to bypass CI or branch protections.
- No obfuscated code or misleading identifiers.
- No code or configuration intended to:
  - Trigger GitHub or platform violations,
  - Harass or harm users, or
  - Damage the project’s reputation.

Violations may result in reverting commits, blocking contributors, and reporting abuse through proper channels.

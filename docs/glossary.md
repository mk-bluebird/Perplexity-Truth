# Glossary and Weapon-String Taxonomy

This glossary provides definitions for key terms used in this repository. It is designed so that CI scanners and static analysis tools can parse and enforce rules around “weapon strings” and related concepts.

---

## 1. Weapon String

A **weapon string** is any textual pattern in source code, configuration, or workflow files that is intentionally designed, or reasonably likely, to:

- Deliver or trigger malicious behavior (e.g., code execution, data exfiltration, destructive actions).
- Circumvent or undermine platform policies (e.g., implicit self-harm content, harassment payloads, or terms-of-service violations).
- Activate known exploit chains, malware, or backdoors.

Weapon strings are divided into three main categories:

### 1.1. Malicious Payload String

A **malicious payload string** is a sequence of characters or code whose primary purpose is to:

- Execute arbitrary commands on a system without user intent.
- Alter, destroy, or encrypt user or system data.
- Exfiltrate secrets (tokens, passwords, private keys) to external destinations.
- Install or propagate malware or backdoors.

**Examples (abstracted):**

- Shell commands that recursively delete core system directories.
- Encoded payloads intended to be decoded and executed at runtime.
- Hard-coded one-line commands that fetch and run remote binaries.

### 1.2. Platform-Triggering Banned String

A **platform-triggering banned string** is any text or code construct that:

- Violates GitHub or other platform policies when executed or displayed (e.g., direct hate speech, doxxing content, explicit calls to violence).
- Is known to trigger automated moderation or security actions due to past abuse.
- Encodes prohibited operations (e.g., mass credential harvesting, deliberate exploitation of platform vulnerabilities).

This repository treats such strings as prohibited even if they are not immediately executable code.

### 1.3. Contextually Sensitive Security-Research String

A **contextually sensitive security-research string** is text or code that:

- Demonstrates security vulnerabilities, exploits, or payloads in a controlled, educational, or research context.
- Is clearly marked as non-production, test-only, or educational.
- Is surrounded by explicit comments explaining that it must not be used in production or harmful ways.

These strings are permitted only under strict conditions:

- They must reside in clearly named directories (e.g., `examples/`, `research/`).
- They must not be compiled or executed in default builds or CI.
- They must be accompanied by clear disclaimers in comments and documentation.

---

## 2. Unknown Function

An **unknown function** is any function in the codebase that, from the project’s perspective, is not safely classified and therefore cannot be trusted without review.

A function is considered unknown if:

- It is not reachable from the approved module tree (e.g., not referenced from `src/main.rs` or `src/lib.rs` or their transitive modules).
- It performs security-sensitive operations (network calls, file system writes, process execution, cryptography, unsafe code) without:
  - Clear documentation comments, and
  - An approved classification tag or annotation.
- It is introduced or significantly modified in a pull request without corresponding unit tests or review by a trusted maintainer.
- Its name or behavior is misleading (e.g., a function named `noop` that opens network sockets).

Unknown functions are subject to blocking or mandatory manual review by CI rules.

---

## 3. Augmented Citizen

An **augmented citizen** is an individual whose cognitive, sensory, or motor capacities are measurably affected by:

- Neurotechnology (implants, BCIs, neurostimulation devices).
- Cognitive-enhancing or modifying tools (software-based neurofeedback, immersive XR with neural monitoring, etc.).
- Long-term use of digital systems that collect, infer, or modulate neural data.

Within this project:

- Augmented citizens are treated as rights-holders with enhanced protection for:
  - Cognitive liberty (self-determination over mental processes).
  - Mental privacy (control over neural and mental data).
  - Mental integrity and psychological continuity (protection against unauthorized interference with thought and identity).
- Any logic, evidence tiering, or incident classification that involves augmented citizens must observe anti-discrimination and equal protection principles.

---

## 4. Neurorights

**Neurorights** are rights and safeguards related to an individual’s neural and mental domain, including but not limited to:

- **Cognitive liberty**: the right to control one’s own mental processes, cognition, and consciousness, including the freedom to use or refuse neurotechnology.
- **Mental privacy**: the right to keep one’s thoughts, brain data, and mental states private against unauthorized access or disclosure.
- **Mental integrity**: the right to be free from non-consensual interference with one’s brain or mental states, especially harmful or manipulative interventions.
- **Psychological continuity**: the right to maintain one’s sense of self and identity over time without unauthorized external manipulation of memory, personality, or core preferences.

These neurorights are reflected in the project’s data models and policy logic for incident analysis and evidence tiering.

---

## 5. Truth-Serum Invariants

**Truth-serum invariants** are strict rules that govern how claims and evidence are processed, summarized, and presented. Each invariant is represented both as:

- A human-readable tag and description, and
- A machine-enforceable rule for CI and analysis tools.

Examples include:

- `tier-lock-veracity`: claims cannot be represented as “verified” unless supported by Tier 1 sources.
- `source-traceability-required`: every non-trivial factual assertion must be traceable to at least one source.

Formal logic for these invariants is defined in `docs/invariants/formal-logic.md`.

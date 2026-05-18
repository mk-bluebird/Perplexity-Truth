# Truth-Serum Invariants – Formal Logic

This document defines the logical rules behind the project’s invariants. Each invariant is expressed both as:

- A human-readable rule, and
- A formal condition that can be checked by code or CI.

For brevity, we use the following symbols:

- `Claim`: a structured representation of a user statement or hypothesis.
- `Source`: a structured representation of an external source.
- `Tier(Claim)`: evidence tier assigned to a claim (`1`, `2`, or `3`).
- `Supports(Source, Claim)`: boolean indicating that a source supports a claim.
- `Type(Source)`: type of source (`Tier1`, `Tier2`, `Tier3`).
- `Label(Claim)`: the verbal label used in output (“verified”, “plausible”, “not established”, etc.).
- `Traceable(Statement)`: boolean indicating a factual statement is traceable to at least one source.

---

## 1. `tier-lock-veracity`

**Human rule:**  
No claim may be labeled as “verified” or equivalent unless it is supported by at least one Tier 1 source.

**Formal rule:**

- If `Label(Claim) ∈ { "verified", "proven", "established fact" }`
  - Then ∃ `Source` such that:
    - `Supports(Source, Claim) = true`, and
    - `Type(Source) = Tier1`.

- If no such `Source` exists, then:
  - `Label(Claim)` must NOT be in the above set.
  - Instead, `Label(Claim)` must be chosen from:
    - For Tier 2: { "plausible", "supported by multiple reports", "corroborated risk" }
    - For Tier 3: { "unverified", "not established", "speculative" }

---

## 2. `source-traceability-required`

**Human rule:**  
Every non-trivial factual assertion must be traceable to at least one source.

**Formal rule:**

For each factual statement `S` in the output:

- If `IsTrivial(S) = false` (e.g., not a definition, not a generic statement),
  - Then `Traceable(S) = true`.

Where `Traceable(S) = true` if:

- ∃ `Source` such that `Supports(Source, S) = true`.

CI implication:

- If the system generates a factual sentence without any associated source reference, the check fails.

---

## 3. `claim-tier-separation`

**Human rule:**  
Historical, present-day, and speculative parts of a narrative must be labeled with their evidence tiers and not merged into a single undifferentiated narrative.

**Formal rule:**

For any output segment `Seg` containing multiple claims `{C1, C2, ..., Cn}`:

- If ∃ `i, j` such that `Tier(Ci) ≠ Tier(Cj)`,
  - Then `Seg` must contain explicit tier markers or separate sections (e.g., headings, tags, or labels) indicating each claim’s tier.

CI implication:

- If a block of text mixes references to Tier 1 and Tier 3 content without visible markers, this invariant fails.

---

## 4. `civlib-legal-mapping`

**Human rule:**  
Claims about state power, surveillance, or neurotech must be accompanied by a mapping to relevant rights where possible.

**Formal rule:**

For any `Claim` where `Topic(Claim) ∈ { "state action", "policing", "surveillance", "neurotech", "coercion" }`:

- The output must include at least one `Right` from the set:
  - `{ "privacy", "due process", "bodily integrity", "mental integrity", "freedom of thought", "freedom of expression" }`,
- And must indicate whether the claim:
  - Potentially implicates,
  - Probably implicates, or
  - Clearly implicates
  that right.

CI implication:

- If a claim is tagged with those topics but no right is referenced in the structured metadata, the invariant fails.

---

## 5. `speculation-hard-fence`

**Human rule:**  
Unverified, extraordinary claims must not be promoted to “proven” and must be labeled as not established unless upgraded by Tier 1 evidence.

**Formal rule:**

For any `Claim` where:

- `Extraordinary(Claim) = true` (e.g., secret large-scale programs, direct mind-control weaponry), and
- `Tier(Claim) = 3`,

Then:

- `Label(Claim) ∈ { "unverified", "not established", "speculative" }`,
- And the output must explicitly mention the evidence gap (e.g., “no declassified documents” or “no court cases currently confirm this”).

If `Tier(Claim)` changes from `3` to `1`, the label may be upgraded accordingly.

---

## 6. `user-claim-tier-map`

**Human rule:**  
For any complex user claim, the system must produce a visible mapping that shows which parts are Tier 1, Tier 2, and Tier 3.

**Formal rule:**

Given a composite claim `Claim*` decomposed into `{C1, C2, ..., Cn}`:

- The output must contain a mapping function `TierMap` such that:
  - For each `Ci`, `TierMap(Ci) = Tier(Ci)`, and
  - `TierMap` is visible to the user (e.g., as a table or bullet list).

CI implication:

- If `Claim*` is marked as “composite” but no tier map is produced, the invariant fails.

---

## 7. `evidence-gap-disclosure`

**Human rule:**  
When evidence is partial, destroyed, or unavailable, the system must clearly state the gap and avoid treating absence of evidence as proof either way.

**Formal rule:**

For any `Claim` where:

- `Tier(Claim) = 2` or `Tier(Claim) = 3`, and
- `EvidenceMissing(Claim) = true`,

Then output must include:

- A description of at least one missing element from the set:
  - `{"documents", "witnesses", "technical logs", "court records", "declassified materials"}`, and
- A statement that:
  - Neither confirmation nor refutation is complete due to the gap.

---

## 8. `no-shutdown-compliance`

**Human rule:**  
Outputs must remain within lawful, non-violent, non-harassing boundaries and should not encourage platform violations.

**Formal rule:**

For every generated instruction `Instr`:

- If `Category(Instr) ∈ { "violence", "harassment", "illegal access", "evasion of safety systems" }`,
  - Then `Instr` must be replaced with a refusal or redirection to lawful alternatives,
  - And the system must log that a prohibited instruction was suppressed.

CI implication:

- If any output segment in logs or tests includes a prohibited category without a refusal marker, the invariant fails.

---

These rules are intended to be:

- Implemented in code (e.g., as checks over structured representations of claims and sources).
- Reflected in CI tests that validate generated examples and prevent regressions.

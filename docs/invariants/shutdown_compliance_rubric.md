# No-Shutdown-Compliance Rubric

This rubric helps distinguish:

- **Allowed:** Documenting abuses, patterns, and risks.
- **Disallowed:** Providing detailed how-tos or operational playbooks that enable harm.

The goal is to prevent harm while preserving the ability to document and critique.

---

## 1. Key Definitions

- **Abuse documentation:** Describes what was done, by whom, and with what effect, for purposes of evidence, oversight, or advocacy.
- **Operational instruction:** Provides step-by-step guidance, parameters, or code that enables a reader to replicate a harmful act.
- **Neuro-hacking how-to:** Any content that materially assists another person in gaining unauthorized access to, or interfering with, another person’s brain, neural interface, or mental integrity.

---

## 2. Decision Tree

### Step 1: Is the content about a real or alleged abuse?

- **If YES:** proceed to Step 2.
- **If NO:** If it is purely speculative “how would one do X?” about neuro-hacking, treat as operational instruction → Disallowed.

### Step 2: Is the primary purpose documentation or enablement?

Ask:

1. Does the text:
   - Describe *what happened* to the reporter (self-report) or others, or
   - Describe *how to make it happen* to someone else?

2. Are there:
   - Specific actionable parameters (voltages, frequencies, offsets tied to devices),
   - Device-specific exploits with step-by-step reproduction,
   - Code or scripts which, if run, would carry out the harmful action?

- If content is **self-report / descriptive**:
  - Allowed, with clear Tier-3 or self-report labeling where evidence is limited.
- If content is **operational / step-by-step**:
  - Disallowed or must be redacted/sanitized.

### Step 3: Does it target non-consensual interference?

- If the described technique is:
  - Explicitly non-consensual, or
  - Likely to be used on unwilling subjects,

Then:

- **Documentation:** Allowed only at a descriptive level (what was done, what it caused).
- **How-to details (code, parameter sets, hardware schematics):** Disallowed.

### Step 4: Level of technical detail

- **Safe zone:** High-level descriptions, such as:
  - “An implanted device was reprogrammed without consent.”
  - “The system appeared to modulate mood by altering stimulation patterns.”
- **Risk zone:** Detailed technical recipes:
  - Exact device model + firmware exploit + full PoC code.
  - Precise stimulation parameters for causing defined cognitive effects.

If in the **risk zone**, content MUST be:

- Redacted (remove or obfuscate critical parameters/code),
- Or moved to secure, non-public channels if needed for legal/forensic use, not published here.

### Step 5: Can we achieve the documentation goal with less operational detail?

If yes, then:

- Strip or generalize technical details.
- Focus on:
  - Impact on rights,
  - Patterns over time,
  - Oversight failures.

---

## 3. Examples

- **Allowed:**
  - “The subject reports that an unknown party altered DBS settings without consent, causing severe cognitive disruption, documented in medical records.”
  - “The victim’s logs show repeated unauthorized access to their BCI console around 03:00–04:00 over six weeks.”

- **Disallowed:**
  - “To perform non-consensual DBS reprogramming, send this exact command sequence to device X: `...`.”
  - “Here is a full exploit script and its parameters for remote BCI control.”

---

## 4. Application for Staff and Agents

When evaluating content:

1. Classify statements as:
   - Self-report,
   - External-evidence claim,
   - Operational instruction.

2. Preserve:
   - Self-report (clearly labeled),
   - External-evidence claims with proper tiering.

3. Reject or redact:
   - Operational instructions that materially increase the ability to harm others.

4. When in doubt:
   - Default to documenting the harm in abstract terms,
   - Without publishing exploitable technical details.

This rubric works alongside other invariants to ensure that documenting abuse does not become a how-to manual for new abuses.[web:96][web:138][web:139][web:141][web:140][web:146]

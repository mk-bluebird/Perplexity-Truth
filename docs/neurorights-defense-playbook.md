# Neurorights Defense Playbook

This playbook shows how to use Perplexity-Truth to **analyze, document, and push back** against unfair or unlawful neural data collection — especially in private residences and virtual ecosystems.

It turns neurorights theory into a concrete workflow:
- Identify when **neural data** is at stake.
- Map the situation to **laws, policies, and neurorights**.
- Generate a **checklist and summary** that users, lawyers, and regulators can act on.[web:68][web:60][web:58][web:90][web:92][web:93]

---

## 1. Key concepts: neurorights and neural data

Before using the tool, understand the core concepts it relies on.

- **Neural data**  
  - Any data derived from brain activity or closely related physiological signals (EEG, BCI signals, neural imaging) that can reveal mental states, intentions, or emotions.[web:60][web:58]

- **Neurorights** (emerging but influential ideas)[web:68][web:60][web:95]  
  - **Mental privacy**: Right to keep thoughts and mental states free from unauthorized access.  
  - **Cognitive liberty**: Freedom to control your own mental processes without coercive intervention.  
  - **Identity integrity / psychological continuity**: Right to a coherent sense of self without external manipulation.  
  - **Neural data autonomy**: Control over how neural data is collected, stored, used, and shared.[web:68][web:58][web:60]

- **Why neural data is special**  
  - It can indirectly reveal thoughts, feelings, and vulnerabilities.  
  - Abuse of neural data can undermine freedom of thought and conscience, which international law treats as a near-absolute right.[web:60][web:68]

Perplexity-Truth uses these concepts as a lens when analyzing any situation that smells like “brain data + technology.”

---

## 2. Step-by-step workflow for users

This is the **front-line workflow** for anyone using the tool to defend neurorights.

### Step 1 – Describe the scenario

Provide a short, concrete description, for example:

- “My VR headset at home tracks my brainwaves and uses them to personalize ads.”  
- “My employer is requiring a neural headband to monitor attention during remote work.”  
- “A brain-computer interface game collects my EEG data and says it may share with ‘partners’.”

You can also include:
- Location/jurisdiction (e.g., Colorado, EU, California).
- Whether you have seen any consent form or privacy policy.

### Step 2 – Classification: is this neural data, and in what context?

The tool (Lua/Rust classification layer) will:

- Detect **neural tech keywords**:
  - EEG, BCI, brain sensor, “neurofeedback,” “brainwaves,” “neural signals,” etc.
- Identify **context**:
  - `home_private_use`
  - `workplace_monitoring`
  - `virtual_platform_tracking`
  - `medical_research`, `education`, `children`, etc.

Output example:

```json
{
  "claim": "My VR headset at home tracks my brainwaves and uses them to personalize ads.",
  "classification": "home_private_use + consumer_neurotech"
}
```

---

## 3. Step 3 – Lookup: what laws and policies apply?

Perplexity-Truth queries its `neural_policies` index for:

- **Jurisdiction-specific rules** (if known):
  - State laws that classify neural data as sensitive and require explicit consent (e.g., certain U.S. states).[web:90][web:93]  
  - Requirements for notice, purpose limitation, and bans on certain uses (e.g., employment decisions, insurance).[web:90][web:93]

- **National / regional frameworks**:
  - MIND Act proposals for federal neural-data regulation.[web:71][web:90][web:92]  
  - EU AI Act limits on:
    - “Significantly harmful subliminal manipulation.”
    - Emotion inference AI in workplaces/schools.[web:60]

- **Neurorights and mental-privacy principles**:
  - Mental privacy and cognitive liberty as argued in neurorights scholarship.[web:68][web:60][web:95]

The tool then attaches relevant entries to the scenario (Tier-1 and Tier-2 where available).

---

## 4. Step 4 – Rights-based analysis

From the classification + policies, Perplexity-Truth generates a neutral rights analysis, for example:

- **Home VR headset + brainwave tracking for ads**  
  - Neural data is being collected in a **private residence**.  
  - If used for advertising or profiling beyond the core function of the device, this likely:
    - Violates emerging norms that treat neural data as **sensitive** and require explicit, narrow consent.[web:58][web:90][web:93]  
    - Raises **mental privacy** concerns, especially if the system infers emotional or cognitive states without clear transparency.[web:60][web:68]

- **Employer-required attention headband**  
  - Workplace monitoring of neural signals intersects with:
    - Proposed bans or strict limits on emotion and mental-state inference in employment contexts (e.g., under EU AI Act-like regimes).[web:60]  
    - Concerns about coercion: “consent” under threat of job loss is not really free consent.[web:60][web:90]

The tool should phrase conclusions carefully:

- “This scenario **implicates mental privacy and cognitive liberty** and may conflict with emerging neural-data laws or proposals in [jurisdiction].”  
- “Neural data used beyond the narrow purpose of user-chosen functionality (e.g., therapy, accessibility) is especially suspect.”

---

## 5. Step 5 – Generate compliance checklist (manual review)

To create space for **human oversight**, Perplexity-Truth emits a short checklist for manual review. This is where “manual labor” and human responsibility come in.

Example checklist items (customized by context):

- **Consent and transparency**
  - Is there a clear, written explanation that neural data is being collected?
  - Is consent specific to neural data, not buried in generic terms?
  - Can the user revoke consent without losing essential service?

- **Purpose limitation**
  - Is neural data used only for the function the user chose (e.g., medical therapy, accessibility, game control)?
  - Is it used for unrelated advertising, profiling, or AI training?

- **Sharing and retention**
  - Is neural data shared with third parties? Under what conditions?
  - How long is data stored, and is it de-identified or pseudonymized?

- **Context-specific risks**
  - Home/private use:
    - Is any covert collection happening (no notice in a private space)?  
  - Workplace:
    - Is neural data used for hiring/firing, promotion, or evaluation decisions?  
  - Children/education:
    - Are minors involved? Are extra safeguards required by law or policy?

Auditors, compliance staff, or advocates can use these lists to pressure companies or institutions into aligning with neurorights and emerging neural-data laws.[web:58][web:60][web:68][web:93]

---

## 6. Step 6 – Produce a public-ready neurorights summary

The tool should also output a short, shareable summary for each scenario, suitable for:

- Talking to a lawyer or regulator.
- Posting in a neurorights advocacy group.
- Filing a complaint or policy comment.

Template:

> **Summary:**  
> This scenario involves neural data collection in a [home / workplace / virtual platform] context. Neural data is considered highly sensitive because it can reveal mental states and vulnerabilities.[web:60][web:68] In jurisdictions such as [X/Y], laws and policy proposals treat neural data as a special category requiring explicit, narrow-purpose consent and strict limits on use and sharing.[web:90][web:93][web:92] The described practice raises concerns about mental privacy and cognitive liberty and warrants review against applicable neural-data regulations and neurorights principles.

This keeps the output:

- Evidence-based (Tier-1/Tier-2 sourced).[web:60][web:68][web:71][web:90][web:92][web:93]  
- Non-accusatory, but rights-forward.

---

## 7. How this playbook prevents or exposes unlawful neural data collection

By following these steps, Perplexity-Truth helps:

- **Detect**: When neural data is quietly being collected in homes, workplaces, or VR environments.  
- **Classify**: The legal/ethical risk based on neurorights and existing laws.  
- **Document**: A clear record of what’s happening, which laws/policies are relevant, and where the practice is questionable.  
- **Enable action**:
  - Users can take the summary to counsel, regulators, or advocacy groups.
  - Companies can use the checklists to tighten compliance.
  - Policymakers can see concrete examples of problematic practices when drafting new neurorights or neural-data laws.

The act of **researching and structuring** this information is itself a form of neurorights defense: it turns vague fear into concrete, legally‑grounded scrutiny that is much harder to ignore or dismiss.[web:60][web:68][web:90][web:92][web:93]

---

## 8. Extending the playbook

Future extensions can include:

- Jurisdiction-specific annexes (e.g., “Colorado Neural Data Rules,” “EU AI Act Neurotech Annex”).[web:60][web:93][web:90]  
- A catalog of known consumer neurotech devices and their stated data practices.  
- Templates for:
  - Regulatory complaints,
  - FOIA/public-records requests,
  - Policy comments on neurorights and mental-privacy bills.

All of these can be layered on top of this core workflow without changing its basic structure.

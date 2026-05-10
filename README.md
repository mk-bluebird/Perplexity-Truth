# Perplexity-Truth

Perplexity-Truth is a small, pluggable toolkit for **sorting, checking, and documenting claims** using public sources. It is designed to be:

- Evidence-focused, not sensational.
- Platform-compliant across GitHub, Perplexity Spaces, and social platforms.
- Easy to extend in Rust, C++, Lua, and SQLite.

The goal is to help users separate:
- What is **verified**.
- What is **plausible but not proven**.
- What is **not established** (or contradicted) in the public record.

## Evidence tiers

Perplexity-Truth uses a simple three-tier model inspired by evidence hierarchies in research and source-credibility frameworks.[web:75][web:78][web:80]

### Tier 1 – Verified

Tier 1 evidence is:

- Directly traceable to **official or primary records**, such as:
  - Declassified government documents, court filings, statutes, regulations.
  - Peer-reviewed academic research, systematic reviews, and major legal analyses.
- Publicly viewable and checkable by anyone (open documents, FOIA releases, court dockets). 

Tier 1 is the strongest category and should be favored when making factual claims.[web:75][web:78]

### Tier 2 – Plausible / Corroborated

Tier 2 evidence includes:

- Investigative journalism from reputable outlets.
- NGO and think-tank reports with named authors and citations.
- Expert commentary that is:
  - Based on identifiable data or case studies.
  - Cross-checked by multiple independent sources when possible.[web:78]

Tier 2 can suggest patterns, risks, or likely interpretations, but it should be clearly labeled as **plausible**, not conclusive.

### Tier 3 – Unverified / Speculative

Tier 3 evidence includes:

- Single-source claims without independent corroboration.
- Opinion pieces, personal testimony, anonymous leaks that lack supporting documents.
- Highly agenda-driven or low-credibility sites.

Tier 3 is still tracked because:
- It may contain **leads** or keywords.
- It should be explicitly separated from Tier 1 and Tier 2 to avoid conflation.[web:78][web:80]

Claims supported **only** by Tier 3 should be treated as **not established**.

### How the tool uses tiers

- For each claim, Perplexity-Truth aims to:
  - Normalize and classify the text.
  - Attach known sources into Tier 1 / Tier 2 / Tier 3 buckets.
  - Return a structured summary that clearly states what is:
    - Verified (Tier 1)
    - Plausible (Tier 2)
    - Not established (Tier 3-only or unsupported)

### Safety and platform compliance

## Safety and platform compliance

Perplexity-Truth is designed to:

- Respect platform rules (GitHub, Perplexity, social networks).
- Avoid promoting violence, targeted harassment, or illegal activity.
- Focus on **evidence mapping**, not on telling people what to believe.

It is especially suitable for neurorights, surveillance, civil-liberties, and tech-policy topics, where careful separation of evidence tiers is essential.[web:58][web:68]

## 2. Minimal API contract (stdin → stdout JSON)

This is the “wire format” you can document in the README and use inside a Perplexity Space, scripts, or CI.

### Input (stdin)

One JSON object per call, via STDIN:

```json
{
  "claim": "Short free-text claim or question",
  "context": "Optional extra context or notes",
  "max_sources": 10
}
```

- `claim` (string, required): The user’s statement or question.
- `context` (string, optional): Hints, topic tags, or notes.
- `max_sources` (int, optional): Upper bound for how many sources to attach (can be ignored by stubs).

### Output (stdout)

One JSON object written to STDOUT:

```json
{
  "claim": "Short free-text claim or question",
  "classification": "generic_claim",
  "confidence": 0.42,
  "evidence_tiers": {
    "tier1": [],
    "tier2": [],
    "tier3": []
  },
  "summary": "Short, neutral summary of where the evidence stands.",
  "notes": "Implementation stub: populate evidence_tiers from SQLite or external tools."
}
```

Fields:

- `claim`: Echo of the input.
- `classification`: Label from your Lua/Rust logic (e.g., `historical_mind_control_context`, `health_policy`, `policing_tech`).
- `confidence`: Float in \([0, 1]\), a **cheap heuristic** score from the C++ stub, later upgradeable.
- `evidence_tiers`:
  - `tier1`: Array of `{ "id": string, "title": string, "url": string, "notes": string }`.
  - `tier2`: Same structure.
  - `tier3`: Same structure.
- `summary`: 1–3 sentence neutral summary usable in a post or note.
- `notes`: Implementation notes, warnings, or TODOs.

Your current Rust stub can easily be adjusted to read that JSON from stdin and emit the structured JSON above.

***

## 3. Research plan: law, policy, programming, and Spaces integration

Below is a practical research and build plan to take this from stub → fully operable, while staying compliant and hard to “shut down” on policy grounds.

### Step 1 – Lock in legal/ethical guardrails

- Define **scope**:
  - Focus on: civil liberties, surveillance tech, neurorights, disinformation, policing tech, declassified programs, and evidence evaluation. [sites.uab](https://sites.uab.edu/humanrights/2025/11/11/neurorights-and-mental-privacy/)
  - Explicitly exclude: calls to violence, doxxing, targeted harassment, or operational security breaches.
- Create a short `CODE_OF_CONDUCT.md`:
  - No targeting of individuals by name for harassment.
  - No encouragement of unlawful access, hacking, or evasion of safety systems.
  - Emphasis on public records, declassified docs, and lawful advocacy.
- Add a `LEGAL_NOTICE.md`:
  - Clarify that the project:
    - Is an educational/research aid.
    - Does not provide legal or medical advice.
    - Encourages users to cross-check sources.

This makes the project easier to defend under platform policies (evidence‑mapping and research, not “coordination of harm”).

### Step 2 – Build a curated source index (SQLite)

- Design a **source taxonomy** matching your tiers:
  - Tier 1:  
    - National Security Archive, CIA FOIA Reading Room, official court records, statutes, major peer‑reviewed work. [pmc.ncbi.nlm.nih](https://pmc.ncbi.nlm.nih.gov/articles/PMC12064251/)
  - Tier 2:  
    - ACLU, EFF, Brennan Center, major investigative journalism, neurorights think‑tank work, academic blogs with sources. [open.oregonstate](https://open.oregonstate.education/goodargument/chapter/four-tiers-of-sources/)
  - Tier 3:  
    - Credible but unverified personal accounts, opinion essays, and “lead” materials.
- Populate `db/sources` table offline:
  - Write a small script (Python or Rust) that:
    - Reads a YAML/JSON file listing sources and their tier/type.
    - Inserts them into SQLite with normalized titles, URLs, and tags.
  - Keep the index **public, static, and documented** so Perplexity and other tools can safely traverse it.

### Step 3 – Implement real classification and matching logic

- In **Lua / Rust**:
  - Expand `classify.lua` rules to:
    - Detect topic domains (e.g., `mkultra`, `surveillance_tech`, `neuroprivacy`, `policing_ai`).
    - Map each domain to **recommended Tier 1 / Tier 2 collections**.
  - In Rust:
    - Implement simple keyword/tag matching against your SQLite `sources` table.
    - For each claim, pull:
      - Up to `N` Tier‑1 sources,
      - Up to `M` Tier‑2 sources,
      - Optionally a few Tier‑3 “lead” sources.
- Keep resource usage low:
  - Use small queries (index on tags, titles).
  - No heavy embedding/vector search initially; stay within “free” compute budgets.

### Step 4 – Document user workflows for Perplexity Spaces

For Spaces, the *instructions* are just as important as the repo:

- In your Space description/instructions:
  - Define the **mission**:  
    - “Map claims into Tier 1/2/3 evidence, explain what is verified, plausible, and not established, and highlight relevant neurorights and civil‑liberties implications.” [trustarc](https://trustarc.com/resource/neurotechnology-privacy-safeguarding-the-next-frontier-of-data/)
  - Require:
    - Always favor Tier‑1 sources when available.
    - Explicitly label Tier‑3 content as unverified/speculative.
    - Provide short, shareable summaries in plain language.
- Add GitHub repo link in the Space’s “links” section:
  - So the agent can read the README, tiers, and schemas to frame responses.
- Encourage users to:
  - Paste a claim.
  - Ask the Space to:
    - Classify it,
    - List Tier‑1 / Tier‑2 / Tier‑3 sources,
    - Generate a **public‑ready summary** or post.

### Step 5 – Ensure policy continuity (avoid violations and shutdown)

To minimize shutdown risk:

- **Transparency first**:
  - All source links are public, legal, and documented.
  - No scraped “dark” databases or proprietary leaks.
- **No direct “call to action” for harm**:
  - Summaries can end with:
    - “Read the declassified docs here…”
    - “Contact your representative about neurorights / surveillance oversight.”
    - “File a FOIA request if you want deeper access to records.”
  - Avoid:
    - Calls for harassment, doxxing, or violence.
- **Platform‑aligned topics**:
  - Highlight neurorights and mental privacy as constructive, policy‑reform topics. [sites.uab](https://sites.uab.edu/humanrights/2025/11/11/neurorights-and-mental-privacy/)
  - Emphasize critical thinking and multiple sources (very aligned with both academic and platform best practices). [uen.pressbooks](https://uen.pressbooks.pub/writingelevated/chapter/source-types/)

### Step 6 – Implementation roadmap (programming side)

1. **MVP (what you already have plus small additions)**  
   - Rust CLI reading simple JSON from stdin, outputting JSON with `claim`, `classification`, `confidence`, `summary`, `evidence_tiers`.  
   - SQLite populated with a dozen Tier‑1 and Tier‑2 sources (MKULTRA docs, neurorights articles, ACLU/Brennan reports). [nsarchive.gwu](https://nsarchive.gwu.edu/briefing-book/dnsa-intelligence/2024-12-23/cia-behavior-control-experiments-focus-new-scholarly)
   - Lua classifier tagging major topics.

2. **v0.2 – Better relevance**  
   - Add tag‑based matching in SQLite (`topic_tags` column).  
   - Simple scoring in Rust that prioritizes topic‑matching sources.

3. **v0.3 – Spaces integration polish**  
   - Add examples in README:
     - “Input claim,” “tool JSON output,” and “how a Perplexity Space would turn that into a post.”  
   - Add a “Facebook‑ready summary” field in the JSON (short, plain‑language paragraph).

4. **v1.0 – Community use**  
   - Accept contributions of new sources:
     - PRs that add to a `sources.yml` file.
   - CI checks:
     - Ensure each new source has:
       - URL, title, tier, brief notes,
       - No obviously illegal content.


```
Perplexity-Truth/
  README.md
  Cargo.toml
  src/
    main.rs               # Rust entrypoint (CLI / service)
    debunk.rs             # Rust facade: calls into C++ and Lua
    db.rs                 # SQLite helpers
  cpp/
    CMakeLists.txt
    engine.cpp            # C++ stubs for “reasoning / ranking”
    engine.hpp
  lua/
    classify.lua          # basic claim classification stub
    normalize.lua         # text normalization stub
  db/
    schema.sql            # SQLite schema
  .github/
    workflows/
      ci.yml              # Simple build/test CI
```

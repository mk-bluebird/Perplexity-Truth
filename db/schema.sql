-- db/schemas.sql
--
-- Perplexity-Truth SQLite schema
-- --------------------------------
-- Goals:
-- - Minimal, platform-safe indexing schema.
-- - Store claims, sources, evidence tiers, tags, and legal / neurorights
--   metadata for later cross-checking and public reporting.
-- - Keep structure simple enough for GitHub, CI, and Perplexity Spaces.
--
-- Notes:
-- - All tables use INTEGER PRIMARY KEY for rowid efficiency.
-- - Foreign keys are declared but kept simple for portability.
-- - Use PRAGMA foreign_keys = ON in application code if strict FK behavior
--   is desired.

-- Enable foreign keys when supported.
PRAGMA foreign_keys = ON;

------------------------------------------------------------------------------
-- 1. Core claims and sources
------------------------------------------------------------------------------

-- Stores raw user claims and normalized text.
CREATE TABLE IF NOT EXISTS claims (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_text         TEXT NOT NULL,             -- Original user text
    normalized_text  TEXT NOT NULL,             -- Lowercased / normalized
    classification   TEXT,                      -- e.g. "mkultra", "neurorights", "policing_tech"
    confidence       REAL,                      -- Optional numeric score 0.0–1.0
    created_at       DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Fast lookup by normalized text.
CREATE INDEX IF NOT EXISTS idx_claims_normalized_text
    ON claims (normalized_text);

-- Lookup by classification.
CREATE INDEX IF NOT EXISTS idx_claims_classification
    ON claims (classification, created_at);


-- Stores sources attached to claims.
-- Example: a declassified MKULTRA document, an ACLU report, a Tier 3 allegation.
CREATE TABLE IF NOT EXISTS sources (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    claim_id     INTEGER NOT NULL,             -- References claims.id
    url          TEXT NOT NULL,                -- Publicly-viewable URL
    title        TEXT,                         -- Human-readable title
    source_type  TEXT,                         -- e.g. "tier1_verified", "tier2_plausible", "tier3_unverified"
    notes        TEXT,                         -- Short notes on relevance / limitations
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (claim_id) REFERENCES claims(id)
);

-- Index to quickly fetch sources for a claim.
CREATE INDEX IF NOT EXISTS idx_sources_claim_id
    ON sources (claim_id);

-- Index for filtering by evidence tier.
CREATE INDEX IF NOT EXISTS idx_sources_source_type
    ON sources (source_type);

------------------------------------------------------------------------------
-- 2. Reusable evidence catalog (tiered sources)
------------------------------------------------------------------------------

-- Global catalog of sources not tied to a single claim.
-- These can be reused across many claims and mapped into tiers.
CREATE TABLE IF NOT EXISTS evidence_sources (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT UNIQUE,          -- Short ID, e.g. "mkultra_cia_foia_06760269"
    title       TEXT NOT NULL,        -- Human-readable title
    url         TEXT NOT NULL,        -- Public URL
    tier        INTEGER NOT NULL,     -- 1 = verified, 2 = plausible, 3 = unverified
    source_type TEXT,                 -- e.g. "declassified_doc", "court_record", "ngo_report", "journalism", "testimony"
    topics      TEXT,                 -- Comma-separated tags, e.g. "mkultra,behavior_control,cia"
    notes       TEXT,                 -- Short description / rationale
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_evidence_sources_tier
    ON evidence_sources (tier);

CREATE INDEX IF NOT EXISTS idx_evidence_sources_topics
    ON evidence_sources (topics);

CREATE INDEX IF NOT EXISTS idx_evidence_sources_slug
    ON evidence_sources (slug);


-- Link table: which evidence_sources are associated with which claims.
-- This allows one Tier 1 source (e.g., MKULTRA IG report) to support many claims.
CREATE TABLE IF NOT EXISTS claim_evidence (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    claim_id            INTEGER NOT NULL,
    evidence_source_id  INTEGER NOT NULL,
    relevance_score     REAL,                     -- Optional 0.0–1.0 heuristic score
    notes               TEXT,                     -- Short explanation of why it is linked
    FOREIGN KEY (claim_id) REFERENCES claims(id),
    FOREIGN KEY (evidence_source_id) REFERENCES evidence_sources(id)
);

CREATE INDEX IF NOT EXISTS idx_claim_evidence_claim_id
    ON claim_evidence (claim_id);

CREATE INDEX IF NOT EXISTS idx_claim_evidence_evidence_source_id
    ON claim_evidence (evidence_source_id);

------------------------------------------------------------------------------
-- 3. Tags and topic mapping
------------------------------------------------------------------------------

-- Optional tag table for more structured topic labels.
CREATE TABLE IF NOT EXISTS tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT UNIQUE NOT NULL,    -- e.g. "mkultra", "neurorights", "surveillance_ai"
    description TEXT
);

-- Many-to-many between claims and tags.
CREATE TABLE IF NOT EXISTS claim_tags (
    claim_id    INTEGER NOT NULL,
    tag_id      INTEGER NOT NULL,
    PRIMARY KEY (claim_id, tag_id),
    FOREIGN KEY (claim_id) REFERENCES claims(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE INDEX IF NOT EXISTS idx_claim_tags_tag_id
    ON claim_tags (tag_id);

-- Many-to-many between evidence_sources and tags.
CREATE TABLE IF NOT EXISTS evidence_source_tags (
    evidence_source_id  INTEGER NOT NULL,
    tag_id              INTEGER NOT NULL,
    PRIMARY KEY (evidence_source_id, tag_id),
    FOREIGN KEY (evidence_source_id) REFERENCES evidence_sources(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE INDEX IF NOT EXISTS idx_evidence_source_tags_tag_id
    ON evidence_source_tags (tag_id);

------------------------------------------------------------------------------
-- 4. Neurorights and legal / policy index
------------------------------------------------------------------------------

-- Machine-readable index of neurorights / mental-privacy and surveillance policies.
-- Used to check scenarios like home BCI use, workplace monitoring, or virtual ecosystems.
CREATE TABLE IF NOT EXISTS neural_policies (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    jurisdiction   TEXT NOT NULL,          -- e.g. "US-CO", "US-CA", "EU", "INTL"
    law_or_code    TEXT NOT NULL,          -- e.g. "Colorado Privacy Act", "EU AI Act", "MIND Act"
    topic          TEXT NOT NULL,          -- e.g. "neural_data", "consent", "workplace_monitoring", "subliminal_manipulation"
    rule_summary   TEXT NOT NULL,          -- Short, neutral summary of the rule
    tier           INTEGER NOT NULL,       -- 1 or 2 (policy / law, no Tier 3 here)
    source_url     TEXT NOT NULL,          -- Official text or credible legal explainer
    notes          TEXT,
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_neural_policies_jurisdiction
    ON neural_policies (jurisdiction);

CREATE INDEX IF NOT EXISTS idx_neural_policies_topic
    ON neural_policies (topic);

CREATE INDEX IF NOT EXISTS idx_neural_policies_tier
    ON neural_policies (tier);


-- Optional: policy tags for grouping (e.g. "home_private_use", "workplace", "vr_platform").
CREATE TABLE IF NOT EXISTS neural_policy_tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS neural_policy_tag_links (
    neural_policy_id  INTEGER NOT NULL,
    tag_id            INTEGER NOT NULL,
    PRIMARY KEY (neural_policy_id, tag_id),
    FOREIGN KEY (neural_policy_id) REFERENCES neural_policies(id),
    FOREIGN KEY (tag_id) REFERENCES neural_policy_tags(id)
);

CREATE INDEX IF NOT EXISTS idx_neural_policy_tag_links_tag_id
    ON neural_policy_tag_links (tag_id);

------------------------------------------------------------------------------
-- 5. Manual checklists for human review
------------------------------------------------------------------------------

-- Checklist items representing manual review steps for neurorights / surveillance scenarios.
-- Example: "Verify explicit, purpose-specific consent for neural data collection."
CREATE TABLE IF NOT EXISTS checklist_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    code        TEXT UNIQUE,         -- e.g. "consent_written", "no_retaliation_optout"
    topic       TEXT NOT NULL,       -- e.g. "neural_data", "home_use", "workplace"
    text        TEXT NOT NULL,       -- The checklist item phrased neutrally
    notes       TEXT,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Mapping between tags (scenario classifications) and checklist items.
-- Example: tag "home_private_use" -> several checklist_items to run.
CREATE TABLE IF NOT EXISTS tag_checklist_links (
    tag_id            INTEGER NOT NULL,
    checklist_item_id INTEGER NOT NULL,
    PRIMARY KEY (tag_id, checklist_item_id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    FOREIGN KEY (checklist_item_id) REFERENCES checklist_items(id)
);

CREATE INDEX IF NOT EXISTS idx_tag_checklist_links_tag_id
    ON tag_checklist_links (tag_id);

------------------------------------------------------------------------------
-- 6. Lightweight logging (optional, for auditing behavior)
------------------------------------------------------------------------------

-- Minimal interaction log, useful for debugging and transparency.
-- Can be truncated or disabled if not needed.
CREATE TABLE IF NOT EXISTS interactions (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    claim_id       INTEGER,                  -- Optional link to claims.id
    input_json     TEXT,                     -- Raw JSON request (stdin)
    output_json    TEXT,                     -- Raw JSON response (stdout)
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP,
    notes          TEXT,
    FOREIGN KEY (claim_id) REFERENCES claims(id)
);

CREATE INDEX IF NOT EXISTS idx_interactions_claim_id
    ON interactions (claim_id);

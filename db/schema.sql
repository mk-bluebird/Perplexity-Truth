-- Minimal, platform-safe indexing schema.
-- Stores claims and attached metadata/summaries for later cross-checking.

CREATE TABLE IF NOT EXISTS claims (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    raw_text TEXT NOT NULL,
    normalized_text TEXT NOT NULL,
    classification TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    claim_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    source_type TEXT,    -- e.g. "tier1_verified", "tier2_plausible", "tier3_unverified"
    notes TEXT,
    FOREIGN KEY (claim_id) REFERENCES claims(id)
);

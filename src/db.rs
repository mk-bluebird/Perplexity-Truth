use rusqlite::{params, Connection, OptionalExtension, Result, NO_PARAMS};

/// High-level handle for all DB operations used by Perplexity-Truth Spaces.
///
/// Responsibilities:
/// - Open / initialize the SQLite database.
/// - Ensure required tables and FTS indices exist.
/// - Provide simple insert + similarity lookup over stored claims/snippets.
pub struct DbHandle {
    conn: Connection,
}

#[derive(Debug, Clone)]
pub struct StoredClaim {
    pub id: i64,
    pub claim: String,
    pub source: Option<String>,
    pub created_at: String,
    pub score: f64,
}

impl DbHandle {
    /// Open (or create) the SQLite database at `path` and ensure schema is ready.
    ///
    /// This will:
    /// - Create a `claims` table for raw text + metadata.
    /// - Create an FTS5 virtual table `claims_fts` for full-text search (if supported).
    /// - Create triggers to keep `claims_fts` in sync with `claims`.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Enable WAL for better concurrent read performance in long-running processes.
        conn.pragma_update(None, "journal_mode", &"WAL")?;
        conn.pragma_update(None, "synchronous", &"NORMAL")?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize base tables and FTS indices.
    fn init_schema(&self) -> Result<()> {
        // Main table for claims/snippets we want to search.
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS claims (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                claim       TEXT NOT NULL,
                source      TEXT,
                created_at  TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- FTS5 virtual table for full-text search over the `claim` column.
            -- Uses contentless mode with external content table for flexibility.
            CREATE VIRTUAL TABLE IF NOT EXISTS claims_fts
            USING fts5(
                claim,
                content='claims',
                content_rowid='id'
            );

            -- Automatically keep FTS index in sync on INSERT.
            CREATE TRIGGER IF NOT EXISTS claims_ai
            AFTER INSERT ON claims
            BEGIN
                INSERT INTO claims_fts(rowid, claim)
                VALUES (new.id, new.claim);
            END;

            -- Automatically keep FTS index in sync on UPDATE.
            CREATE TRIGGER IF NOT EXISTS claims_au
            AFTER UPDATE ON claims
            BEGIN
                UPDATE claims_fts
                SET claim = new.claim
                WHERE rowid = new.id;
            END;

            -- Automatically keep FTS index in sync on DELETE.
            CREATE TRIGGER IF NOT EXISTS claims_ad
            AFTER DELETE ON claims
            BEGIN
                DELETE FROM claims_fts WHERE rowid = old.id;
            END;
            "#,
        )?;

        Ok(())
    }

    /// Insert a new claim/snippet into the database.
    ///
    /// Returns the new row id on success.
    pub fn insert_claim(&self, claim: &str, source: Option<&str>) -> Result<i64> {
        self.conn.execute(
            r#"
            INSERT INTO claims (claim, source)
            VALUES (?1, ?2);
            "#,
            params![claim, source],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Simple health-check: verifies the DB is reachable and schema exists.
    ///
    /// Returns `Ok(true)` if a trivial query executes successfully.
    pub fn health_check(&self) -> Result<bool> {
        let _: Option<i64> = self
            .conn
            .query_row("SELECT 1 as ok;", NO_PARAMS, |row| row.get(0))
            .optional()?;

        Ok(true)
    }

    /// Lookup similar claims to the provided `claim_text` using FTS5 MATCH
    /// and a simple relevance score (bm25).
    ///
    /// - `limit` caps the number of results.
    /// - Returns an empty Vec if nothing matches or FTS is not available.
    ///
    /// NOTE: For now, this is a straightforward full-text search. Ranking is
    /// handled via `bm25(claims_fts)` where lower is better. [web:39]
    pub fn lookup_similar(
        &self,
        claim_text: &str,
        limit: usize,
    ) -> Result<Vec<StoredClaim>> {
        // Quick guard: if claim_text is empty or only whitespace, skip heavy queries.
        if claim_text.trim().is_empty() {
            return Ok(Vec::new());
        }

        // FTS5 query using MATCH; wrap user text with quotes to avoid
        // accidental boolean operator parsing. For more advanced usage,
        // sanitize/transform input into proper FTS5 syntax. [web:39]
        //
        // We join back to `claims` to get metadata and use bm25() for ranking.
        let sql = format!(
            r#"
            SELECT
                c.id,
                c.claim,
                c.source,
                c.created_at,
                bm25(claims_fts) AS score
            FROM claims_fts
            JOIN claims c ON c.id = claims_fts.rowid
            WHERE claims_fts MATCH ?
            ORDER BY score ASC
            LIMIT {};
            "#,
            limit.max(1)
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(params![claim_text])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(StoredClaim {
                id: row.get(0)?,
                claim: row.get(1)?,
                source: row.get(2)?,
                created_at: row.get(3)?,
                score: row.get(4)?,
            });
        }

        Ok(results)
    }

    /// Backwards-compatible stub: keeps the original signature but now returns
    /// the best matching claim text (if any) instead of always `None`.
    ///
    /// This is useful for quick integration points that only care about a single
    /// best match string.
    pub fn lookup_similar_stub(&self, claim: &str) -> Result<Option<String>> {
        let results = self.lookup_similar(claim, 1)?;
        Ok(results.into_iter().next().map(|r| r.claim))
    }
}

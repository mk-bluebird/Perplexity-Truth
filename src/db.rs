use rusqlite::{Connection, Result};

pub struct DbHandle {
    conn: Connection,
}

impl DbHandle {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    // Very minimal stub – extend later (e.g., full‑text search, ranking)
    pub fn lookup_similar_stub(&self, _claim: &str) -> Result<Option<String>> {
        // For now, just ensure DB is reachable; no heavy queries.
        let _ = self.conn.execute("SELECT 1", [])?;
        Ok(None)
    }
}

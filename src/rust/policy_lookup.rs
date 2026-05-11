// src/rust/policy_lookup.rs
//! Policy lookup module
//! 
//! Provides efficient querying of neural_policies and compliance_check_items
//! tables with in-memory caching for frequently accessed rules.

use rusqlite::{Connection, params};
use crate::types::{PolicyRule, ChecklistItem, PolicyTheme, ContextTag};
use std::collections::HashMap;
use lru::LruCache;
use std::num::NonZeroUsize;

/// Query relevant policy rules for a given jurisdiction and policy themes
pub fn query_policies(
    conn: &Connection,
    jurisdictions: &[String],
    themes: &[PolicyTheme],
) -> Result<Vec<PolicyRule>, rusqlite::Error> {
    if jurisdictions.is_empty() || themes.is_empty() {
        return Ok(Vec::new());
    }
    
    // Build dynamic SQL with parameter binding
    let theme_placeholders = themes.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let jurisdiction_placeholders = jurisdictions.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    
    let sql = format!(
        "SELECT id, jurisdiction, law_or_instrument, topic, rule_text, source_url, tier, effective_date
         FROM neural_policies
         WHERE jurisdiction IN ({})
           AND topic IN ({})
           AND tier <= 2
         ORDER BY tier ASC, jurisdiction, topic",
        jurisdiction_placeholders, theme_placeholders
    );
    
    let mut stmt = conn.prepare(&sql)?;
    
    // Bind parameters: jurisdictions first, then themes
    let mut params_vec = Vec::new();
    for j in jurisdictions {
        params_vec.push(j.as_str());
    }
    for t in themes {
        params_vec.push(t.as_str());
    }
    
    let policy_iter = stmt.query_map(params_vec.as_slice(), |row| {
        Ok(PolicyRule {
            id: row.get(0)?,
            jurisdiction: row.get(1)?,
            law_or_instrument: row.get(2)?,
            topic: row.get(3)?,
            rule_text: row.get(4)?,
            source_url: row.get(5)?,
            tier: row.get(6)?,
            effective_date: row.get(7)?,
        })
    })?;
    
    policy_iter.collect()
}

/// Query checklist items for a given context and policy themes
pub fn query_checklist_items(
    conn: &Connection,
    context: &ContextTag,
    themes: &[PolicyTheme],
) -> Result<Vec<ChecklistItem>, rusqlite::Error> {
    if themes.is_empty() {
        return Ok(Vec::new());
    }
    
    let theme_placeholders = themes.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    
    let sql = format!(
        "SELECT id, topic, context, checklist_text, verification_method, severity
         FROM compliance_check_items
         WHERE context = ?1
           AND topic IN ({})
         ORDER BY severity DESC, topic",
        theme_placeholders
    );
    
    let mut stmt = conn.prepare(&sql)?;
    
    let mut params_vec = vec![context.as_str()];
    for t in themes {
        params_vec.push(t.as_str());
    }
    
    let item_iter = stmt.query_map(params_vec.as_slice(), |row| {
        Ok(ChecklistItem {
            id: row.get(0)?,
            topic: row.get(1)?,
            context: row.get(2)?,
            checklist_text: row.get(3)?,
            verification_method: row.get(4)?,
            severity: row.get(5)?,
        })
    })?;
    
    item_iter.collect()
}

/// In-memory cache for policy rules to reduce database hits
pub struct PolicyCache {
    cache: LruCache<String, Vec<PolicyRule>>,
    conn: Connection,
}

impl PolicyCache {
    pub fn new(conn: Connection, cache_size: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            cache: LruCache::new(NonZeroUsize::new(cache_size).unwrap()),
            conn,
        })
    }
    
    pub fn get_policies(
        &mut self,
        jurisdictions: &[String],
        themes: &[PolicyTheme],
    ) -> Result<Vec<PolicyRule>, rusqlite::Error> {
        // Create cache key from sorted inputs
        let mut jurs = jurisdictions.to_vec();
        let mut thms = themes.iter().map(|t| t.as_str()).collect::<Vec<_>>();
        jurs.sort();
        thms.sort();
        let cache_key = format!("{}|{}", jurs.join(","), thms.join(","));
        
        // Return cached result if available
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Query database and cache result
        let policies = query_policies(&self.conn, jurisdictions, themes)?;
        self.cache.put(cache_key, policies.clone());
        Ok(policies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;
    
    #[test]
    fn test_policy_query_with_valid_db() {
        let temp_db = NamedTempFile::new().unwrap();
        let schema_sql = fs::read_to_string("schemas/neural_policies.sql").unwrap();
        let conn = Connection::open(temp_db.path()).unwrap();
        conn.execute_batch(&schema_sql).unwrap();
        
        let policies = query_policies(
            &conn,
            &["US-CO".to_string()],
            &[PolicyTheme::Consent, PolicyTheme::SensitiveClassification],
        ).unwrap();
        
        assert!(!policies.is_empty());
        assert!(policies.iter().all(|p| p.tier <= 2));
    }
}

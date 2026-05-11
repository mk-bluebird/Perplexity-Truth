// src/rust/lib.rs
//! Neurorights Defense Framework Core Library
//! 
//! Provides type-safe interfaces for scenario classification, policy lookup,
//! and compliance report generation. Designed for integration with Lua scripting
//! layer and external CLI/GUI applications.
//!
//! # Safety Guarantees
//! - All policy lookups are read-only and sandboxed
//! - No external network calls without explicit user consent
//! - All outputs are deterministic given same input and database state

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod classifier;
pub mod compliance_engine;
pub mod policy_lookup;
pub mod types;

use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;

use crate::types::{ClassificationResult, ComplianceReport, ScenarioInput};

/// Initialize the compliance engine with a SQLite database connection
/// 
/// # Arguments
/// * `db_path` - Path to the neural_policies SQLite database
/// 
/// # Returns
/// * `Result<ComplianceEngine, rusqlite::Error>` - Initialized engine or database error
pub fn initialize_engine<P: AsRef<Path>>(db_path: P) -> SqliteResult<ComplianceEngine> {
    let conn = Connection::open(db_path)?;
    
    // Verify required tables exist
    let required_tables = ["neural_policies", "compliance_check_items"];
    for table in required_tables {
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
            [table],
            |row| row.get(0),
        )?;
        if !exists {
            return Err(rusqlite::Error::SchemaError(format!(
                "Required table '{}' not found in database", table
            )));
        }
    }
    
    Ok(ComplianceEngine { conn })
}

/// Main compliance engine struct
pub struct ComplianceEngine {
    conn: Connection,
}

impl ComplianceEngine {
    /// Classify a scenario and generate a compliance report
    /// 
    /// # Arguments
    /// * `input` - Structured scenario description
    /// 
    /// # Returns
    /// * `Result<ComplianceReport, Box<dyn std::error::Error>>` - Generated report or error
    pub fn analyze_scenario(&self, input: ScenarioInput) -> Result<ComplianceReport, Box<dyn std::error::Error>> {
        // Step 1: Classify scenario
        let classification = classifier::classify_scenario(&input)?;
        
        // Step 2: Lookup relevant policies
        let policies = policy_lookup::query_policies(
            &self.conn, 
            &classification.jurisdiction_hints,
            &classification.policy_themes
        )?;
        
        // Step 3: Generate checklist items
        let checklist_items = policy_lookup::query_checklist_items(
            &self.conn,
            &classification.primary_context,
            &classification.policy_themes
        )?;
        
        // Step 4: Assemble report
        Ok(ComplianceReport {
            scenario_id: input.scenario_id,
            classification,
            applicable_policies: policies,
            checklist_items,
            generated_at: chrono::Utc::now(),
        })
    }
    
    /// Export report to markdown format for human review
    pub fn export_markdown(&self, report: &ComplianceReport) -> String {
        compliance_engine::format_markdown_report(report)
    }
    
    /// Export report to JSON for programmatic consumption
    pub fn export_json(&self, report: &ComplianceReport) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    #[test]
    fn test_initialize_engine_with_valid_db() {
        // Create temporary database with required schema
        let temp_db = NamedTempFile::new().unwrap();
        let schema_sql = fs::read_to_string("schemas/neural_policies.sql").unwrap();
        let conn = Connection::open(temp_db.path()).unwrap();
        conn.execute_batch(&schema_sql).unwrap();
        
        // Initialize engine - should succeed
        let engine = initialize_engine(temp_db.path()).unwrap();
        assert!(engine.conn.is_busy() == false);
    }
}

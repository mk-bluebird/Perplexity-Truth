//! Temporal scan for time-based sabotage.
//!
//! Static heuristics to flag code that:
//! - Checks the current time, and
//! - Compares it to hard-coded or suspicious thresholds.

use regex::Regex;

#[derive(Debug, Default)]
pub struct TemporalIssues {
    pub uses_time_now: bool,
    pub has_hardcoded_date_threshold: bool,
    pub notes: Vec<String>,
}

/// Scan Rust source code for temporal sabotage patterns.
pub fn scan_source_for_temporal_traps(source: &str) -> TemporalIssues {
    let mut issues = TemporalIssues::default();

    let lower = source.to_lowercase();

    // Detect usage of time APIs.
    let re_time_now =
        Regex::new(r"(systemtime::now\(\)|chrono::.*now\(\)|std::time::systemtime)").unwrap();
    if re_time_now.is_match(&lower) {
        issues.uses_time_now = true;
        issues
            .notes
            .push("Detected use of current time APIs (SystemTime::now / chrono::now).".into());
    }

    // Detect hard-coded timestamps (very simple heuristic: 4-digit year plus month/day).
    let re_date = Regex::new(r"(20[2-9][0-9][-/\.](0[1-9]|1[0-2]))").unwrap();
    if re_date.is_match(&lower) {
        issues.has_hardcoded_date_threshold = true;
        issues
            .notes
            .push("Detected year-month pattern that may indicate a time-based trigger.".into());
    }

    issues
}

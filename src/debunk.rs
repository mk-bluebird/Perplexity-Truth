use serde::Serialize;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::time::SystemTime;

use crate::db::DbHandle;

/// High-level classification of a claim's status.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClassification {
    Verified,
    Plausible,
    NotEstablished,
    False,
    Mixed,
    Unclassified,
}

impl fmt::Display for ClaimClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ClaimClassification::Verified => "verified",
            ClaimClassification::Plausible => "plausible",
            ClaimClassification::NotEstablished => "not_established",
            ClaimClassification::False => "false",
            ClaimClassification::Mixed => "mixed",
            ClaimClassification::Unclassified => "unclassified",
        };
        write!(f, "{s}")
    }
}

/// Core result structure returned to callers (HTTP, CLI, etc.).
#[derive(Serialize, Clone)]
pub struct AnalysisResult {
    /// Original user claim.
    pub claim: String,
    /// Classification label (human-readable, but machine-friendly).
    pub classification: String,
    /// Confidence in [0.0, 1.0]. This is a heuristic score.
    pub confidence: f32,
    /// Structured notes to show in the UI or logs.
    pub notes: String,
    /// Optional list of prior, similar claims found in the DB.
    pub prior_matches: Vec<PriorClaimMatch>,
    /// ISO-8601 timestamp of when this analysis was produced.
    pub analyzed_at: String,
}

/// Lightweight representation of similar prior claims.
#[derive(Serialize, Clone)]
pub struct PriorClaimMatch {
    pub claim: String,
    pub classification: String,
    pub confidence: f32,
    pub similarity: f32,
}

/// Error type for analysis flow.
#[derive(Debug)]
pub enum AnalysisError {
    DbInit(String),
    DbQuery(String),
    ForeignEngine(String),
    Encoding(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::DbInit(msg) => write!(f, "DB init error: {msg}"),
            AnalysisError::DbQuery(msg) => write!(f, "DB query error: {msg}"),
            AnalysisError::ForeignEngine(msg) => write!(f, "Foreign engine error: {msg}"),
            AnalysisError::Encoding(msg) => write!(f, "Encoding error: {msg}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// FFI hook into the C/C++ scoring engine.
///
/// Expected contract:
/// - Takes a null-terminated UTF-8 string.
/// - Returns a float in a reasonable range (we clamp to [0.0, 1.0] on our side).
extern "C" {
    fn pt_engine_score_claim(claim: *const c_char) -> f32;
}

/// Public API: analyze a claim and return JSON.
///
/// This is the primary entry-point you should call from:
/// - HTTP handlers (e.g., Axum / Actix)
/// - CLI commands
///
/// It returns a JSON string and never panics; all internal errors are
/// mapped into a low-confidence, "unclassified" result with diagnostic notes.
pub fn analyze_claim(claim: &str) -> String {
    match analyze_claim_inner(claim) {
        Ok(result) => serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
            // Extremely unlikely; this is a last-resort safety.
            let fallback = AnalysisResult {
                claim: claim.to_string(),
                classification: ClaimClassification::Unclassified.to_string(),
                confidence: 0.0,
                notes: format!(
                    "Serialization failure; returning minimal result. Details: {e}"
                ),
                prior_matches: Vec::new(),
                analyzed_at: iso_timestamp_now(),
            };
            serde_json::to_string_pretty(&fallback)
                .unwrap_or_else(|_| "{\"error\":\"serialization_failure\"}".to_string())
        }),
        Err(err) => {
            // On error, downgrade to a low-confidence, unclassified result.
            let result = AnalysisResult {
                claim: claim.to_string(),
                classification: ClaimClassification::Unclassified.to_string(),
                confidence: 0.0,
                notes: format!("Analysis encountered an error: {err}"),
                prior_matches: Vec::new(),
                analyzed_at: iso_timestamp_now(),
            };
            serde_json::to_string_pretty(&result)
                .unwrap_or_else(|_| "{\"error\":\"analysis_failure\"}".to_string())
        }
    }
}

/// Internal analysis pipeline with structured error handling.
fn analyze_claim_inner(claim: &str) -> Result<AnalysisResult, AnalysisError> {
    // 0. Open DB (if present).
    let db = DbHandle::new("db/perplexity_truth.db").map_err(|e| {
        AnalysisError::DbInit(format!(
            "Failed to open db/perplexity_truth.db: {e}"
        ))
    })?;

    // 1. Pre-classify via Lua stub (can be upgraded later).
    let pre_class = lua_classify_stub(claim);

    // 2. Foreign engine numeric score (C/C++ or other).
    let raw_score = call_foreign_score_engine(claim)?;
    let confidence = normalize_score(raw_score);

    // 3. Optional: query SQLite for prior similar records.
    let prior_matches = lookup_prior_matches(&db, claim)?;

    // 4. Map into high-level classification.
    let classification = map_score_to_classification(confidence, &pre_class);

    // 5. Build human-usable notes.
    let notes = build_notes(&classification, confidence, &prior_matches, &pre_class);

    let result = AnalysisResult {
        claim: claim.to_string(),
        classification: classification.to_string(),
        confidence,
        notes,
        prior_matches,
        analyzed_at: iso_timestamp_now(),
    };

    Ok(result)
}

/// Safe wrapper around the foreign scoring engine.
fn call_foreign_score_engine(claim: &str) -> Result<f32, AnalysisError> {
    let c_string = CString::new(claim).map_err(|e| {
        AnalysisError::Encoding(format!(
            "Failed to convert claim to C string: {e}"
        ))
    })?;

    let ptr = c_string.as_ptr();
    // Safety: we trust the foreign function to not retain the pointer.
    let score = unsafe { pt_engine_score_claim(ptr) };

    if !score.is_finite() {
        Err(AnalysisError::ForeignEngine(format!(
            "Non-finite score returned by foreign engine: {score}"
        )))
    } else {
        Ok(score)
    }
}

/// Normalize arbitrary engine score into [0.0, 1.0].
fn normalize_score(raw: f32) -> f32 {
    if !raw.is_finite() {
        return 0.0;
    }
    if raw.is_nan() {
        return 0.0;
    }

    // Heuristic:
    // - If engine is already in [0, 1], keep it.
    // - If in [0, 100], scale down.
    // - Otherwise, clamp.
    if (0.0..=1.0).contains(&raw) {
        raw
    } else if (0.0..=100.0).contains(&raw) {
        (raw / 100.0).clamp(0.0, 1.0)
    } else {
        raw.clamp(0.0, 1.0)
    }
}

/// Convert score + pre-classification into a high-level label.
fn map_score_to_classification(
    confidence: f32,
    pre_classification: &str,
) -> ClaimClassification {
    // If Lua provided a stronger signal, you could branch on it here.
    // For now, we only consider the numeric confidence.
    match confidence {
        c if c >= 0.85 => ClaimClassification::Verified,
        c if c >= 0.65 => ClaimClassification::Plausible,
        c if c >= 0.45 => ClaimClassification::Mixed,
        c if c >= 0.25 => ClaimClassification::NotEstablished,
        _ => {
            if pre_classification.eq_ignore_ascii_case("false") {
                ClaimClassification::False
            } else {
                ClaimClassification::Unclassified
            }
        }
    }
}

/// Query DB for prior similar claims using the existing stub.
///
/// `lookup_similar_stub` is expected to return a vector of rows with:
/// (claim_text, classification, confidence, similarity_score)
fn lookup_prior_matches(
    db: &DbHandle,
    claim: &str,
) -> Result<Vec<PriorClaimMatch>, AnalysisError> {
    let rows = db.lookup_similar_stub(claim).map_err(|e| {
        AnalysisError::DbQuery(format!(
            "lookup_similar_stub failed: {e}"
        ))
    })?;

    Ok(rows
        .into_iter()
        .map(|row| PriorClaimMatch {
            claim: row.claim,
            classification: row.classification,
            confidence: row.confidence,
            similarity: row.similarity,
        })
        .collect())
}

/// Create human-readable notes for UIs and logs.
fn build_notes(
    classification: &ClaimClassification,
    confidence: f32,
    prior_matches: &[PriorClaimMatch],
    pre_classification: &str,
) -> String {
    let mut parts: Vec<String> = Vec::new();

    parts.push(format!(
        "Classification: {} (confidence {:.2}).",
        classification, confidence
    ));

    if !prior_matches.is_empty() {
        parts.push(format!(
            "Found {} prior similar claim(s) in the local database.",
            prior_matches.len()
        ));
        if let Some(top) = prior_matches.first() {
            parts.push(format!(
                "Top prior match: \"{}\" [{}] with confidence {:.2} and similarity {:.2}.",
                truncate(&top.claim, 120),
                top.classification,
                top.confidence,
                top.similarity
            ));
        }
    } else {
        parts.push(
            "No prior similar claims were found in the local database.".to_string(),
        );
    }

    if !pre_classification.is_empty() {
        parts.push(format!(
            "Lua pre-classification hint: \"{}\".",
            pre_classification
        ));
    }

    parts.join(" ")
}

/// Minimal Lua hook – currently a stub.
///
/// Later you can embed `rlua`/`mlua` and call real Lua scripts
/// from `lua/` or a similar directory.
fn lua_classify_stub(_claim: &str) -> String {
    // For now, return a neutral placeholder.
    "unclassified_stub".to_string()
}

/// Get a simple ISO-8601-like timestamp in UTC.
///
/// Example: "2026-05-10T18:19:00Z"
fn iso_timestamp_now() -> String {
    use chrono::{DateTime, Utc};

    let now: DateTime<Utc> = SystemTime::now().into();
    now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Helper: truncate long strings for note fields.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut truncated = s[..max_len].to_string();
        truncated.push_str("…");
        truncated
    }
}

/// Optional: small convenience for CLIs/tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_score_in_range() {
        assert!((normalize_score(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((normalize_score(1.0) - 1.0).abs() < f32::EPSILON);
        assert!((normalize_score(50.0) - 0.5).abs() < 1e-6);
        assert!((normalize_score(-10.0) - 0.0).abs() < f32::EPSILON);
        assert!((normalize_score(200.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn classification_thresholds() {
        assert!(matches!(
            map_score_to_classification(0.9, "unclassified_stub"),
            ClaimClassification::Verified
        ));
        assert!(matches!(
            map_score_to_classification(0.7, "unclassified_stub"),
            ClaimClassification::Plausible
        ));
        assert!(matches!(
            map_score_to_classification(0.5, "unclassified_stub"),
            ClaimClassification::Mixed
        ));
        assert!(matches!(
            map_score_to_classification(0.3, "unclassified_stub"),
            ClaimClassification::NotEstablished
        ));
        assert!(matches!(
            map_score_to_classification(0.1, "false"),
            ClaimClassification::False
        ));
    }

    #[test]
    fn truncate_long_strings() {
        let s = "abcdefghijklmnopqrstuvwxyz";
        let t = truncate(s, 10);
        assert!(t.len() <= 11);
    }
}

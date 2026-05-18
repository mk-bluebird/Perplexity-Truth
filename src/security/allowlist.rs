//! Allowlist and justification parser for weapon-string exceptions.
//!
//! Patterns may be allowed when explicitly annotated with:
//!   // ALLOW_WEAPON_STRING: reason="...", scope="..."
//!
//! CI will:
//! - Look up this marker near the denylisted line.
//! - Accept only if a justification is present.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowJustification {
    pub reason: String,
    pub scope: String,
}

/// Look for an allow-justification comment in the given file lines near `line_idx`.
pub fn find_allow_justification(lines: &[String], line_idx: usize) -> Option<AllowJustification> {
    // Check same line and a few lines above.
    let start = line_idx.saturating_sub(3);
    let end = usize::min(lines.len(), line_idx + 1);

    for i in start..end {
        if let Some(comment_pos) = lines[i].find("ALLOW_WEAPON_STRING:") {
            let tail = &lines[i][comment_pos..];
            // Very simple parse: reason="...", scope="..."
            let reason = extract_param(tail, "reason").unwrap_or_else(|| "unspecified".into());
            let scope = extract_param(tail, "scope").unwrap_or_else(|| "unspecified".into());
            return Some(AllowJustification { reason, scope });
        }
    }
    None
}

fn extract_param(line: &str, key: &str) -> Option<String> {
    let pattern = format!(r#"{key}=""#);
    if let Some(start_idx) = line.find(&pattern) {
        let rest = &line[start_idx + pattern.len()..];
        if let Some(end_idx) = rest.find('"') {
            return Some(rest[..end_idx].to_string());
        }
    }
    None
}

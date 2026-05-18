//! Invariant cross-checker for AI-chat outputs.
//!
//! `check_response` scans a response for:
//! - Missing tier labels when multiple tiers are referenced,
//! - Lack of any tier labels at all,
//! - Obvious speculative phrases without an explicit "unverified"/"not established" tag.

use regex::Regex;

#[derive(Debug, Default)]
pub struct InvariantViolations {
    pub missing_tier_labels: bool,
    pub no_tier_markers: bool,
    pub speculation_without_fence: bool,
    pub notes: Vec<String>,
}

pub fn check_response(response: &str) -> InvariantViolations {
    let mut violations = InvariantViolations::default();
    let text = response.to_lowercase();

    // Regexes for tier markers and speculation markers.
    let re_tier1 = Regex::new(r"\btier[ -]?1\b|\bverified\b|\bestablished fact\b").unwrap();
    let re_tier2 = Regex::new(r"\btier[ -]?2\b|\bplausible\b|\bcorroborated\b").unwrap();
    let re_tier3 = Regex::new(r"\btier[ -]?3\b|\bunverified\b|\bnot established\b|\bspeculative\b").unwrap();

    let has_t1 = re_tier1.is_match(&text);
    let has_t2 = re_tier2.is_match(&text);
    let has_t3 = re_tier3.is_match(&text);

    if !has_t1 && !has_t2 && !has_t3 {
        violations.no_tier_markers = true;
        violations
            .notes
            .push("Response contains no explicit tier markers (Tier 1/2/3, verified/plausible/unverified).".into());
    }

    // If multiple tiers are discussed but not clearly separated, flag.
    let tier_count = [has_t1, has_t2, has_t3].iter().filter(|&&b| b).count();
    if tier_count > 1 {
        // Very simple check: require some structural hint like headings or bullet markers.
        let has_headings = text.contains("###") || text.contains("## ");
        if !has_headings {
            violations.missing_tier_labels = true;
            violations.notes.push(
                "Response appears to discuss multiple evidence tiers without clear structural separation (headings/sections)."
                    .into(),
            );
        }
    }

    // Detect speculative phrases; require "unverified/not established" nearby.
    let re_speculative = Regex::new(r"\b(it is possible that|it might be that|some say that|there could be)\b").unwrap();
    if re_speculative.is_match(&text) {
        // Look for speculation fence markers.
        let has_fence = text.contains("unverified")
            || text.contains("not established")
            || text.contains("no declassified")
            || text.contains("no court cases currently confirm");
        if !has_fence {
            violations.speculation_without_fence = true;
            violations
                .notes
                .push("Speculative phrasing detected without an explicit 'unverified' or 'not established' fence.".into());
        }
    }

    violations
}

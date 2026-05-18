//! Detection of "gap-flooding" attacks in evidence-gap disclosures.
//!
//! Problem:
//! - Adversaries can submit many near-identical reports with small,
//!   unverifiable variations, causing repeated "records missing" messages.
//!
//! Approach:
//! - Detect clusters of highly similar cases with:
        // - High Tier 3 / gap content,
        // - Low Tier 1/2 content.
//! - Flag these clusters for human review, not automatic suppression.

use serde::{Deserialize, Serialize};

/// Simplified case summary for gap analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseGapSummary {
    pub case_id: String,
    /// Normalized textual description (e.g., concatenated incident summaries).
    pub description: String,
    pub tier1_count: u32,
    pub tier2_count: u32,
    pub tier3_count: u32,
    pub gap_items: u32,
}

/// Result of gap-flood analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapFloodAnalysis {
    pub suspected_flood_cases: Vec<String>,
    pub notes: Vec<String>,
}

/// Very simple similarity metric: Jaccard over word sets.
fn jaccard_similarity(a: &str, b: &str) -> f32 {
    use std::collections::HashSet;
    let wa: HashSet<_> = a.split_whitespace().collect();
    let wb: HashSet<_> = b.split_whitespace().collect();
    if wa.is_empty() && wb.is_empty() {
        return 1.0;
    }
    let inter = wa.intersection(&wb).count() as f32;
    let union = wa.union(&wb).count() as f32;
    inter / union
}

/// Detect potential gap-flood clusters.
pub fn detect_gap_flood(cases: &[CaseGapSummary], similarity_threshold: f32, min_cluster_size: usize) -> GapFloodAnalysis {
    let mut suspected = Vec::new();
    let mut notes = Vec::new();

    for (i, ci) in cases.iter().enumerate() {
        let mut cluster = vec![ci.case_id.clone()];

        for (j, cj) in cases.iter().enumerate().skip(i + 1) {
            let sim = jaccard_similarity(&ci.description, &cj.description);
            if sim >= similarity_threshold {
                // Both must be heavy on Tier3/gaps, light on Tier1/2.
                let ci_is_gap_heavy = ci.tier1_count + ci.tier2_count < ci.tier3_count && ci.gap_items > 0;
                let cj_is_gap_heavy = cj.tier1_count + cj.tier2_count < cj.tier3_count && cj.gap_items > 0;
                if ci_is_gap_heavy && cj_is_gap_heavy {
                    cluster.push(cj.case_id.clone());
                }
            }
        }

        if cluster.len() >= min_cluster_size {
            suspected.extend(cluster);
        }
    }

    suspected.sort();
    suspected.dedup();

    if !suspected.is_empty() {
        notes.push(format!(
            "Detected {} case(s) that may be part of a gap-flooding cluster; review recommended.",
            suspected.len()
        ));
    }

    GapFloodAnalysis {
        suspected_flood_cases: suspected,
        notes,
    }
}

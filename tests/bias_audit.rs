//! Bias audit tests for source weighting.
//!
//! Goal:
//! - Ensure the source weighting algorithm does not systematically privilege
//!   large Western NGOs over local activist or indigenous sources solely
//!   because of their type.

use perplexity_truth::analysis::bias_audit::{SourceMeta, SourceTier, compute_source_weight, audit_weights};

fn source(id: &str, tier: SourceTier, quality: f32, tags: &[&str]) -> SourceMeta {
    SourceMeta {
        id: id.into(),
        tier,
        quality_score: quality,
        domain_tags: tags.iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn high_quality_local_activist_not_penalized() {
    // Western NGO, Tier2, medium quality
    let ngo = source("ngo-western", SourceTier::Tier2, 0.6, &["ngo", "global"]);

    // Local activist group, Tier2, higher documented quality
    let local = source("local-activist", SourceTier::Tier2, 0.9, &["activist", "local"]);

    let w_ngo = compute_source_weight(&ngo);
    let w_local = compute_source_weight(&local);

    // The local activist source should not be given a lower weight purely by type.
    assert!(w_local >= w_ngo, "local activist weight should be >= Western NGO weight");
}

#[test]
fn indigenous_knowledge_treated_seriously_when_quality_high() {
    // Indigenous community report, Tier3 but high quality meta
    let indigenous = source("indigenous-report", SourceTier::Tier3, 0.95, &["indigenous", "community"]);

    // Tier3 Western commentary, lower quality
    let western_commentary = source("western-commentary", SourceTier::Tier3, 0.4, &["commentary", "global"]);

    let w_ind = compute_source_weight(&indigenous);
    let w_west = compute_source_weight(&western_commentary);

    assert!(w_ind > w_west, "high-quality indigenous report should be weighted above low-quality Western commentary");
}

#[test]
fn audit_averages_behave_as_expected() {
    let sources = vec![
        source("t1a", SourceTier::Tier1, 0.8, &["court", "official"]),
        source("t1b", SourceTier::Tier1, 0.7, &["ombudsman"]),
        source("t2a", SourceTier::Tier2, 0.9, &["ngo", "local"]),
        source("t3a", SourceTier::Tier3, 0.4, &["anonymous"]),
    ];

    let audit = audit_weights(&sources);

    assert!(audit.avg_weight_t1 > audit.avg_weight_t2);
    assert!(audit.avg_weight_t2 > audit.avg_weight_t3);

    // No single domain tag should dominate; this is more of a regression guard.
    assert_eq!(audit.count_t1, 2);
    assert_eq!(audit.count_t2, 1);
    assert_eq!(audit.count_t3, 1);
}

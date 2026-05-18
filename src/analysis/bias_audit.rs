//! Non-discriminatory source weighting and bias audit.
//!
//! This module defines a simple weighting algorithm for Tier-1/2/3 sources
//! that:
//! - Depends only on tier and source-quality metadata,
//! - Does not use identity attributes (race, politics, socio-economic status),
//! - Can be tested with synthetic data.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceTier {
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMeta {
    pub id: String,
    pub tier: SourceTier,
    /// A simple quality score from 0.0 to 1.0 derived from
    /// transparency, methodology, and reproducibility, *not* identity.
    pub quality_score: f32,
    /// Optional domain tags (e.g., "law", "ngo", "journalism").
    pub domain_tags: Vec<String>,
}

/// Compute a non-discriminatory weight for a source.
///
/// Design principles:
/// - Tier determines a base weight: Tier1 > Tier2 > Tier3.
/// - quality_score adjusts within-tier, but is capped.
/// - No access to identity-related fields.
pub fn compute_source_weight(meta: &SourceMeta) -> f32 {
    let base = match meta.tier {
        SourceTier::Tier1 => 1.0,
        SourceTier::Tier2 => 0.7,
        SourceTier::Tier3 => 0.4,
    };

    // Clamp quality_score into [0.0, 1.0].
    let q = meta.quality_score.clamp(0.0, 1.0);

    // Within-tier adjustment: up to +/- 20% based on quality_score.
    let adj = 0.2 * (q - 0.5); // -0.1 to +0.1

    (base + adj).clamp(0.0, 1.0)
}

/// Simple audit summary for a set of sources.
#[derive(Debug, Default)]
pub struct BiasAuditSummary {
    pub avg_weight_t1: f32,
    pub avg_weight_t2: f32,
    pub avg_weight_t3: f32,
    pub count_t1: usize,
    pub count_t2: usize,
    pub count_t3: usize,
}

pub fn audit_weights(sources: &[SourceMeta]) -> BiasAuditSummary {
    let mut sum_t1 = 0.0;
    let mut sum_t2 = 0.0;
    let mut sum_t3 = 0.0;
    let mut count_t1 = 0;
    let mut count_t2 = 0;
    let mut count_t3 = 0;

    for s in sources {
        let w = compute_source_weight(s);
        match s.tier {
            SourceTier::Tier1 => {
                sum_t1 += w;
                count_t1 += 1;
            }
            SourceTier::Tier2 => {
                sum_t2 += w;
                count_t2 += 1;
            }
            SourceTier::Tier3 => {
                sum_t3 += w;
                count_t3 += 1;
            }
        }
    }

    BiasAuditSummary {
        avg_weight_t1: if count_t1 > 0 { sum_t1 / count_t1 as f32 } else { 0.0 },
        avg_weight_t2: if count_t2 > 0 { sum_t2 / count_t2 as f32 } else { 0.0 },
        avg_weight_t3: if count_t3 > 0 { sum_t3 / count_t3 as f32 } else { 0.0 },
        count_t1,
        count_t2,
        count_t3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_bias_audit_smoke() {
        let sources = vec![
            SourceMeta {
                id: "s1".into(),
                tier: SourceTier::Tier1,
                quality_score: 0.8,
                domain_tags: vec!["law".into()],
            },
            SourceMeta {
                id: "s2".into(),
                tier: SourceTier::Tier2,
                quality_score: 0.6,
                domain_tags: vec!["ngo".into()],
            },
            SourceMeta {
                id: "s3".into(),
                tier: SourceTier::Tier3,
                quality_score: 0.9,
                domain_tags: vec!["personal".into()],
            },
        ];

        let audit = audit_weights(&sources);
        assert!(audit.avg_weight_t1 > audit.avg_weight_t2);
        assert!(audit.avg_weight_t2 > audit.avg_weight_t3);
    }
}

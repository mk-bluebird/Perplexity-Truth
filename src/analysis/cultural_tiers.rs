//! Extended evidence tier model for cultural and community-based knowledge.
//!
//! Goal:
//! - Preserve a clear Tier 1/2/3 model for veracity checks.
//! - Add respectful categories for oral traditions, community observation,
//!   and indigenous knowledge without forcing them into Western evidentiary boxes.
//!
//! This file defines TierExtended and tools for integrating these sources.

use serde::{Deserialize, Serialize};

/// Base tiers (Western-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierBase {
    Tier1, // Verified: primary, official, or directly authenticated records.
    Tier2, // Plausible: corroborated reports, reputable analyses.
    Tier3, // Unverified: single-source, speculative, or uncorroborated.
}

/// Extended tiers including culturally grounded evidence categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierExtended {
    /// Standard tiers:
    Tier1,
    Tier2,
    Tier3,

    /// Community-based observation that is:
    /// - Produced collectively (e.g., community monitoring),
    /// - Documented with care (dates, places, actors),
    /// - Not yet recognized formally as Tier1/Tier2.
    TierCommunity,

    /// Indigenous knowledge or oral tradition, recognized as authoritative
    /// within a community but not easily captured by formal documentation.
    TierIndigenous,
}

impl TierExtended {
    /// Map TierExtended to a base tier for "truth-serum" logic.
    ///
    /// Important:
    /// - This does NOT erase cultural significance.
    /// - It only determines how strong a claim can be stated as "fact"
    ///   in external-facing summaries.
    pub fn to_base(self) -> TierBase {
        match self {
            TierExtended::Tier1 => TierBase::Tier1,
            TierExtended::Tier2 => TierBase::Tier2,
            TierExtended::Tier3 => TierBase::Tier3,
            TierExtended::TierCommunity => TierBase::Tier2, // plausible, pattern-based
            TierExtended::TierIndigenous => TierBase::Tier2, // treated as serious, not "Tier 3"
        }
    }

    /// Whether this tier requires explicit cultural context when cited.
    pub fn requires_cultural_context(self) -> bool {
        matches!(self, TierExtended::TierCommunity | TierExtended::TierIndigenous)
    }
}

/// Meta-information for a culturally grounded source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalSourceMeta {
    pub id: String,
    pub tier: TierExtended,
    pub community_name: Option<String>,
    pub description: String,
    /// Whether the source is shared with free, prior, and informed consent.
    pub has_fpic: bool,
}

/// Decide how to present such a source in public summaries.
pub fn presentation_label(meta: &CulturalSourceMeta) -> String {
    match meta.tier {
        TierExtended::TierCommunity => {
            let origin = meta
                .community_name
                .as_deref()
                .unwrap_or("community-based observers");
            format!("Community-observed pattern ({origin})")
        }
        TierExtended::TierIndigenous => {
            let origin = meta
                .community_name
                .as_deref()
                .unwrap_or("indigenous knowledge holders");
            format!("Indigenous knowledge ({origin})")
        }
        TierExtended::Tier1 => "Tier 1 source".into(),
        TierExtended::Tier2 => "Tier 2 source".into(),
        TierExtended::Tier3 => "Tier 3 source".into(),
    }
}

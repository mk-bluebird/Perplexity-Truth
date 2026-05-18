use serde::{Deserialize, Serialize};

/// Evidence tier classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    /// Tier 1: Verified (primary/official records).
    Tier1,
    /// Tier 2: Plausible / corroborated.
    Tier2,
    /// Tier 3: Unverified / speculative.
    Tier3,
}

/// Basic representation of a claim with an assigned tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredClaim {
    pub id: String,
    pub text: String,
    pub tier: Tier,
}

/// Basic representation of a source with an assigned tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredSource {
    pub id: String,
    pub url: String,
    pub title: String,
    pub tier: Tier,
}

/// Shared trait for tiered entities (claims, sources, etc.).
pub trait HasTier {
    fn tier(&self) -> Tier;
}

impl HasTier for TieredClaim {
    fn tier(&self) -> Tier {
        self.tier
    }
}

impl HasTier for TieredSource {
    fn tier(&self) -> Tier {
        self.tier
    }
}

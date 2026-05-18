use crate::{Tier, TieredClaim, TieredSource};

/// Trait for logic related to Tier 3 (unverified / speculative) evidence.
pub trait Tier3Logic {
    fn is_valid_tier3_claim(&self) -> bool;
}

impl Tier3Logic for TieredClaim {
    fn is_valid_tier3_claim(&self) -> bool {
        self.tier == Tier::Tier3 && !self.text.trim().is_empty()
    }
}

/// Example helper for Tier 3 sources.
pub fn is_flagged_tier3_source(source: &TieredSource) -> bool {
    source.tier == Tier::Tier3 && !source.url.trim().is_empty()
}

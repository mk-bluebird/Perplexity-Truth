use crate::{Tier, TieredClaim, TieredSource};

/// Trait for logic related to Tier 2 (plausible / corroborated) evidence.
pub trait Tier2Logic {
    fn is_valid_tier2_claim(&self) -> bool;
}

impl Tier2Logic for TieredClaim {
    fn is_valid_tier2_claim(&self) -> bool {
        self.tier == Tier::Tier2 && !self.text.trim().is_empty()
    }
}

/// Example helper for Tier 2 sources.
pub fn is_reasonable_tier2_source(source: &TieredSource) -> bool {
    source.tier == Tier::Tier2 && !source.title.trim().is_empty()
}

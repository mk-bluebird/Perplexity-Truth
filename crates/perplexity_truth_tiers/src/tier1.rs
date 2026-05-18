use crate::{Tier, TieredClaim, TieredSource};

/// Trait for logic related specifically to Tier 1 (verified) evidence.
pub trait Tier1Logic {
    /// Returns true if this claim qualifies as Tier 1 under project rules.
    fn is_valid_tier1_claim(&self) -> bool;
}

impl Tier1Logic for TieredClaim {
    fn is_valid_tier1_claim(&self) -> bool {
        self.tier == Tier::Tier1 && !self.text.trim().is_empty()
    }
}

/// Example helper for Tier 1 sources.
pub fn is_strong_tier1_source(source: &TieredSource) -> bool {
    source.tier == Tier::Tier1 && source.url.starts_with("http")
}

//! Machine-checkable specification for function classification and “unknown function” rules.
//!
//! This module defines how functions are classified and which characteristics
//! make a function “unknown” and therefore subject to blocking or extra review.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSensitivity {
    /// Normal pure or side-effect-limited functions (e.g., simple computation).
    Normal,
    /// Functions that perform I/O, but not security-sensitive operations.
    IoNonSensitive,
    /// Functions that perform security-sensitive operations (network, FS writes, process exec, crypto).
    SecuritySensitive,
    /// Functions that use `unsafe` or FFI.
    UnsafeOrFfi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionReachability {
    /// Referenced from main/lib module tree and included in normal builds.
    ReachableFromRoot,
    /// Only used in tests or examples.
    TestOrExampleOnly,
    /// Not referenced from any approved root; “orphaned” or hidden.
    Orphaned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionDocumentation {
    /// Has clear doc comments explaining purpose and security implications.
    Documented,
    /// Has minimal or no documentation.
    Undocumented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionTrustStatus {
    /// Known-safe under current policy (may still need periodic review).
    KnownSafe,
    /// Requires manual review due to sensitivity, reachability, or missing docs.
    RequiresReview,
    /// Treated as unknown and blocked from merge under default rules.
    BlockedUnknown,
}

/// Static “policy” thresholds for classification.
/// These can be tuned or loaded from configuration if needed.
pub struct FunctionPolicy {
    /// Whether orphaned functions should be treated as BlockedUnknown by default.
    pub block_orphaned: bool,
    /// Whether undocumented security-sensitive functions should be blocked.
    pub block_undocumented_security_sensitive: bool,
    /// Whether new unsafe/FFI functions should be blocked or just flagged.
    pub block_new_unsafe_or_ffi: bool,
}

impl Default for FunctionPolicy {
    fn default() -> Self {
        Self {
            block_orphaned: true,
            block_undocumented_security_sensitive: true,
            block_new_unsafe_or_ffi: false,
        }
    }
}

/// A summary of function characteristics as derived by static analysis or CI scripts.
#[derive(Debug, Clone)]
pub struct FunctionDescriptor {
    pub name: String,
    pub sensitivity: FunctionSensitivity,
    pub reachability: FunctionReachability,
    pub documentation: FunctionDocumentation,
    /// True if this function is newly added or modified in the current PR.
    pub is_new_or_modified: bool,
}

impl FunctionDescriptor {
    /// Compute the trust status of a function under the given policy.
    pub fn classify_trust(&self, policy: &FunctionPolicy) -> FunctionTrustStatus {
        use FunctionDocumentation::*;
        use FunctionReachability::*;
        use FunctionSensitivity::*;
        use FunctionTrustStatus::*;

        // 1. Orphaned functions are treated as unknown by default.
        if self.reachability == Orphaned && policy.block_orphaned {
            return BlockedUnknown;
        }

        // 2. Undocumented security-sensitive functions are blocked by default.
        if self.sensitivity == SecuritySensitive
            && self.documentation == Undocumented
            && policy.block_undocumented_security_sensitive
        {
            return BlockedUnknown;
        }

        // 3. Unsafe or FFI functions may be blocked or require review.
        if self.sensitivity == UnsafeOrFfi {
            if policy.block_new_unsafe_or_ffi && self.is_new_or_modified {
                return BlockedUnknown;
            } else {
                return RequiresReview;
            }
        }

        // 4. Non-sensitive functions without docs: require review, not block.
        if self.documentation == Undocumented {
            return RequiresReview;
        }

        // 5. All other cases: considered known-safe under current policy.
        KnownSafe
    }

    /// Helper: true if this function qualifies as “unknown” in the strong sense.
    pub fn is_unknown(&self, policy: &FunctionPolicy) -> bool {
        matches!(
            self.classify_trust(policy),
            FunctionTrustStatus::BlockedUnknown
        )
    }
}

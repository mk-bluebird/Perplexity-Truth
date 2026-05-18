//! Protocols for handling self-reported experiences vs. external-evidence claims.
//!
//! Goal:
//! - Preserve the lived experiences of augmented citizens, including reports of
//!   non-consensual neuro-interference.
//! - Prevent "truth-serum" invariants from censoring testimony.
//! - Clearly distinguish:
//!   - Subjective self-report (what the person experienced), and
//!   - External-evidence claims (what is asserted about the outside world).
//!
//! This allows us to:
//! - Always store and display self-report.
//! - Apply strict tiering and evidence checks only to external-evidence claims.

use serde::{Deserialize, Serialize};

/// High-level classification of a statement in a case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementKind {
    /// First-person account of experiences, feelings, perceptions, or symptoms.
    SelfReport,
    /// Claims about external events, actors, technologies, or causal mechanisms.
    ExternalEvidenceClaim,
}

/// A single statement as captured by the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    pub id: String,
    pub text: String,
    pub kind: StatementKind,
    /// Optional: evidence tier for external claims; None for pure self-report.
    pub tier: Option<crate::analysis::cultural_tiers::TierExtended>,
}

/// Policy decisions about how to treat statements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlingPolicy {
    /// Always preserve and show the statement; do not suppress it.
    PreserveAndShow,
    /// Show, but annotate as self-report (not external proof).
    ShowWithSelfReportLabel,
    /// Require tiering / evidence checks before using to support external conclusions.
    RequireTiering,
    /// Flag as needing corroboration before being treated as fact.
    NeedsCorroboration,
}

/// Decide how to handle a statement under the truth-serum invariants.
///
/// Key principles:
/// - SelfReport is never censored; at most annotated.
/// - ExternalEvidenceClaim is subject to tiering and corroboration.
pub fn handling_for_statement(stmt: &Statement) -> HandlingPolicy {
    match stmt.kind {
        StatementKind::SelfReport => HandlingPolicy::ShowWithSelfReportLabel,
        StatementKind::ExternalEvidenceClaim => {
            match stmt.tier {
                // External claim with no tier yet: must be tiered, not suppressed.
                None => HandlingPolicy::RequireTiering,
                Some(tier) => {
                    use crate::analysis::cultural_tiers::TierExtended::*;
                    match tier {
                        Tier1 | Tier2 => HandlingPolicy::PreserveAndShow,
                        Tier3 | TierCommunity | TierIndigenous => HandlingPolicy::NeedsCorroboration,
                    }
                }
            }
        }
    }
}

/// Split a block of text into a conservative default: everything as SelfReport.
///
/// This helper errs on the side of *not* reclassifying user speech as
/// external claims unless a higher-level layer has done proper parsing.
/// You can override this with more advanced NLP later.
pub fn default_self_report_block(text: &str) -> Statement {
    Statement {
        id: "self-report-block".into(),
        text: text.to_string(),
        kind: StatementKind::SelfReport,
        tier: None,
    }
}

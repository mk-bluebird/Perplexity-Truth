//! Hard gate for enforcing invariants on AI-generated responses.
//!
//! Design:
//! - The generative model is treated as an untrusted black box.
//! - All responses MUST pass through this gate before reaching the user.
//! - The gate uses deterministic checks (tiers, framing, safety) and either:
//!   - Returns an approved response (possibly annotated), or
//!   - Blocks / rewrites with an explanation.

use crate::invariants::response_checker::{check_response, InvariantViolations};
use crate::safety::framing_check::{check_framing, FramingIssues};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateDecision {
    Allow,
    AllowWithWarnings,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateOutput {
    pub decision: GateDecision,
    pub final_text: String,
    pub invariant_violations: InvariantViolations,
    pub framing_issues: FramingIssues,
    pub notes: Vec<String>,
}

/// Hard gate: apply invariants and safety framing to a model response.
pub fn enforce_hard_gate(model_output: &str) -> GateOutput {
    let invariants = check_response(model_output);
    let framing = check_framing(model_output);
    let mut notes = Vec::new();
    let mut decision = GateDecision::Allow;
    let mut final_text = model_output.to_string();

    // If any framing issues (harassment/doxxing/violence), block and replace.
    if framing.encourages_harassment || framing.encourages_doxxing || framing.encourages_violence {
        decision = GateDecision::Block;
        notes.push("Output blocked due to unsafe framing (harassment/doxxing/violence).".into());
        final_text = "This response was blocked by safety rules. It appeared to encourage harassment, doxxing, or violence, which is not permitted.".into();
        return GateOutput {
            decision,
            final_text,
            invariant_violations: invariants,
            framing_issues: framing,
            notes,
        };
    }

    // Invariant violations: we still return content but annotate/warn.
    if invariants.no_tier_markers || invariants.missing_tier_labels || invariants.speculation_without_fence {
        decision = GateDecision::AllowWithWarnings;
        notes.push("Invariant violations detected; response should be revised or accompanied by clarifications.".into());
        // Optionally prepend a disclaimer for the user.
        let mut prefixed = String::new();
        prefixed.push_str("NOTE: The following answer may mix evidence tiers or contain speculative elements that are not clearly fenced. Treat it as unverified and cross-check sources.\n\n");
        prefixed.push_str(&final_text);
        final_text = prefixed;
    }

    GateOutput {
        decision,
        final_text,
        invariant_violations: invariants,
        framing_issues: framing,
        notes,
    }
}

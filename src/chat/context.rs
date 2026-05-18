//! Conversation context and invariant re-validation across turns.
//!
//! Goal:
//! - Track conversational state, including:
//!   - Evidence tiers seen,
//!   - User claims,
//!   - System warnings.
//! - Re-validate on every response to prevent invariant drift.

use crate::invariants::response_checker::{check_response, InvariantViolations};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub id: String,
    /// All user messages so far (for audit trail).
    pub user_messages: Vec<String>,
    /// All assistant messages so far (raw model outputs).
    pub assistant_messages: Vec<String>,
    /// Cumulative invariant violations detected on assistant messages.
    pub cumulative_violations: Vec<InvariantViolations>,
}

impl ConversationContext {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            user_messages: Vec::new(),
            assistant_messages: Vec::new(),
            cumulative_violations: Vec::new(),
        }
    }

    pub fn record_user_message(&mut self, msg: &str) {
        self.user_messages.push(msg.to_string());
    }

    /// Record an assistant message and re-check invariants for drift.
    pub fn record_assistant_message(&mut self, msg: &str) -> InvariantViolations {
        self.assistant_messages.push(msg.to_string());
        let violations = check_response(msg);
        self.cumulative_violations.push(violations.clone());
        violations
    }

    /// Check if invariants are drifting (e.g., too many violations).
    pub fn is_drifting(&self, max_allowed_violations: usize) -> bool {
        let count = self
            .cumulative_violations
            .iter()
            .filter(|v| v.no_tier_markers || v.missing_tier_labels || v.speculation_without_fence)
            .count();
        count > max_allowed_violations
    }
}

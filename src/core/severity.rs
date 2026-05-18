//! Severity model for rights violations and system events.
//!
//! Incorporates the principle that deliberate attempts to degrade
//! cognitive function or mental integrity are highest-severity events.

use serde::{Deserialize, Serialize};

/// Severity levels used throughout the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Minor issue; informational or small inconsistency.
    Info,
    /// Potential concern; monitor and log.
    Notice,
    /// Confirmed issue requiring remediation.
    Warning,
    /// Serious rights impact; prioritize investigation and response.
    Major,
    /// Highest severity: egregious violation, especially deliberate
    /// degradation of cognitive function or mental integrity.
    Critical,
}

/// High-level category of an incident or event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentCategory {
    /// General procedural or documentation issue.
    Procedural,
    /// Data or privacy-related incident.
    Privacy,
    /// Neurorights/mental integrity incident (e.g., non-consensual neuro-interference).
    Neurorights,
    /// Physical violence or direct bodily harm.
    Physical,
}

/// Compute a recommended severity based on category and details.
pub fn severity_for_incident(category: IncidentCategory, deliberate_cognitive_degrade: bool) -> Severity {
    if deliberate_cognitive_degrade && matches!(category, IncidentCategory::Neurorights) {
        // "Intelligence is god" principle:
        // Any deliberate attempt to degrade cognitive function is Critical.
        return Severity::Critical;
    }

    match category {
        IncidentCategory::Procedural => Severity::Notice,
        IncidentCategory::Privacy => Severity::Warning,
        IncidentCategory::Neurorights => Severity::Major,
        IncidentCategory::Physical => Severity::Major,
    }
}

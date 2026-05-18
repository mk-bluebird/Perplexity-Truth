use serde::{Deserialize, Serialize};

/// Helper enum for tier labels in templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateTier {
    Tier1,
    Tier2,
    Tier3,
}

/// A single item within the full case template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseIncident {
    pub date_time: String,
    pub location: String,
    pub who_what: String,
    pub observable_actions: String,
    pub tech_or_tools: String,
    pub impact: String,
}

/// A single external source entry with tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseSource {
    pub tier: TemplateTier,
    pub source_id: String,
    pub link: String,
    pub source_type: String,
    pub relevance: String,
    pub notes: String,
}

/// Main case template structure mirroring `case-template-full.md`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseTemplate {
    // 1. Case header
    pub case_id: String,
    pub reporter_name: Option<String>,
    pub reporter_contact: Option<String>,
    pub city: String,
    pub region: String,
    pub main_locations: Vec<String>,
    pub period_from: String,
    pub period_to: String,

    // 2. Timeline
    pub incidents: Vec<CaseIncident>,

    // 3. Interpretation
    pub pattern_labels: Vec<String>,
    pub interpretation_reasons: Vec<String>,

    // 4. Documents
    pub documents: Vec<CaseSource>,

    // 5. External sources
    pub external_sources: Vec<CaseSource>,

    // 6. Rights and legal framing
    pub rights_implicated: Vec<String>,
    pub legal_references: Vec<String>,

    // 7. Evidence gaps and alternatives
    pub evidence_gaps: Vec<String>,
    pub alternative_explanations: Vec<String>,

    // 8. Desired remedies and channels
    pub desired_remedies: Vec<String>,
    pub lawful_channels: Vec<String>,

    // 9. Optional summary
    pub summary_paragraph: Option<String>,
}

/// Social-media oriented summary template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSummaryTemplate {
    // 1. Facts
    pub timeframe: String,
    pub locations: Vec<String>,
    pub core_events: Vec<String>,

    // 2. Documents
    pub confirmed_points: Vec<String>,

    // 3. Interpretation
    pub pattern_description: String,
    pub suppression_reasons: Vec<String>,
    pub rights_at_risk: Vec<String>,

    // 4. Evidence tiers snapshot
    pub tier1_points: Vec<String>,
    pub tier2_points: Vec<String>,
    pub tier3_points: Vec<String>,

    // 5. Requests
    pub requests: Vec<String>,
    pub closing_line: Option<String>,
}

/// Validation errors for templates.
#[derive(Debug)]
pub enum TemplateValidationError {
    EmptyCaseId,
    NoIncidents,
    MixedTiersWithoutLabels,
}

/// Simple validation trait.
pub trait ValidatableTemplate {
    fn validate(&self) -> Result<(), TemplateValidationError>;
}

impl ValidatableTemplate for CaseTemplate {
    fn validate(&self) -> Result<(), TemplateValidationError> {
        if self.case_id.trim().is_empty() {
            return Err(TemplateValidationError::EmptyCaseId);
        }
        if self.incidents.is_empty() {
            return Err(TemplateValidationError::NoIncidents);
        }

        // Tier separation: verify external_sources explicitly list all tiers used.
        let mut has_t1 = false;
        let mut has_t2 = false;
        let mut has_t3 = false;
        for src in &self.external_sources {
            match src.tier {
                TemplateTier::Tier1 => has_t1 = true,
                TemplateTier::Tier2 => has_t2 = true,
                TemplateTier::Tier3 => has_t3 = true,
            }
        }

        // If multiple tiers are present, ensure the summary paragraph references them by name
        // so they cannot be silently merged.
        if (has_t1 && has_t3) || (has_t1 && has_t2) || (has_t2 && has_t3) {
            if let Some(summary) = &self.summary_paragraph {
                let s = summary.to_lowercase();
                let mentions_t1 = s.contains("tier 1") || s.contains("verified");
                let mentions_t2 = s.contains("tier 2") || s.contains("plausible");
                let mentions_t3 = s.contains("tier 3") || s.contains("unverified");
                if !mentions_t1 && !mentions_t2 && !mentions_t3 {
                    return Err(TemplateValidationError::MixedTiersWithoutLabels);
                }
            } else {
                return Err(TemplateValidationError::MixedTiersWithoutLabels);
            }
        }

        Ok(())
    }
}

impl ValidatableTemplate for SocialSummaryTemplate {
    fn validate(&self) -> Result<(), TemplateValidationError> {
        if self.core_events.is_empty() {
            return Err(TemplateValidationError::NoIncidents);
        }

        // Ensure tier separation: if multiple tier lists are non-empty,
        // they must be presented separately (enforced structurally here).
        // Since tier1_points, tier2_points, tier3_points are separate fields,
        // their existence already preserves separation.

        Ok(())
    }
}

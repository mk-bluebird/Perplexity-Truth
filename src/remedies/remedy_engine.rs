//! Lawful remedy suggestion engine.
//!
//! Given a case ID and mapped rights, suggest only lawful, non-violent
//! remedies (FOIA, complaints, oversight, advocacy). No self-help or escalation.

use crate::rights::neurorights::Neuroright;
use serde::{Deserialize, Serialize};

/// Basic identifier for a case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseKey {
    pub case_id: String,
    /// Rights that this case appears to implicate.
    pub neurorights: Vec<Neuroright>,
    /// Free-form tags, e.g. "policing", "workplace", "healthcare".
    pub context_tags: Vec<String>,
}

/// A single suggested remedy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remedy {
    /// Short label, e.g. "FOIA request to local police".
    pub title: String,
    /// Plain language description.
    pub description: String,
    /// Optional URL (oversight body, template, etc.).
    pub link: Option<String>,
}

/// Aggregated remedy suggestions for a case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemedyPlan {
    pub case_id: String,
    pub remedies: Vec<Remedy>,
}

impl RemedyPlan {
    pub fn new(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            remedies: Vec::new(),
        }
    }

    pub fn add(&mut self, remedy: Remedy) {
        self.remedies.push(remedy);
    }
}

/// Generate a lawful-only remedy plan. This function must never suggest:
/// - Violence
/// - Harassment
/// - Doxxing
/// - Illegal access or hacking
pub fn suggest_lawful_remedies(case: &CaseKey) -> RemedyPlan {
    let mut plan = RemedyPlan::new(&case.case_id);

    // Baseline: generic documentation and oversight.
    plan.add(Remedy {
        title: "Document incidents and preserve evidence".into(),
        description: "Maintain a written timeline, save relevant documents, and store backups of non-sensitive evidence in case you choose to contact oversight bodies or legal aid.",
        link: None,
    });

    // If privacy / mental privacy / surveillance-like tags are present.
    if case
        .context_tags
        .iter()
        .any(|t| t.contains("police") || t.contains("surveillance"))
    {
        plan.add(Remedy {
            title: "File public records / FOIA requests".into(),
            description: "Use public records or FOIA laws to request incident reports, policies, and logs relevant to your case. Focus on specific dates, locations, and agencies.",
            // Example generic FOIA sample letter from FTC, not a direct dependency.
            link: Some("https://www.ftc.gov/foia/make-foia-request/sample-foia-request-letter".into()),
        });

        plan.add(Remedy {
            title: "Contact civil-liberties organizations".into(),
            description: "Reach out to civil-liberties or digital-rights organizations with your documented timeline and public records, asking for guidance on surveillance and privacy concerns.",
            link: None,
        });
    }

    // If neurorights are implicated, suggest neurorights-aware advocacy.
    if case
        .neurorights
        .iter()
        .any(|r| matches!(r, Neuroright::MentalPrivacy | Neuroright::MentalIntegrity | Neuroright::CognitiveLiberty))
    {
        plan.add(Remedy {
            title: "Seek neurorights-aware legal or advocacy support".into(),
            description: "Consult organizations, clinics, or advocates familiar with neurotechnology, mental privacy, and cognitive liberty to understand how your case fits into emerging neurorights protections.",
            link: None,
        });
    }

    // Generic democratic oversight channels, never self-help.
    plan.add(Remedy {
        title: "Use oversight and complaint mechanisms".into(),
        description: "File structured complaints with relevant regulators, ombuds offices, or professional bodies, attaching your documentation and specifying which rights you believe are implicated.",
        link: None,
    });

    plan.add(Remedy {
        title: "Engage in peaceful, lawful advocacy".into(),
        description: "Consider writing to your representatives, contributing to public consultations, or sharing a documented summary of your case in ways that respect privacy, avoid harassment, and focus on systemic reform.",
        link: None,
    });

    plan
}

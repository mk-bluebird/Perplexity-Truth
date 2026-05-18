//! Alternate oversight remedies when local agencies are alleged perpetrators.
//!
//! Problem:
//! - A user may allege abuse by the same agencies that would normally be their
//!   remedy (police, regulators, intelligence services).
//!
//! Approach:
//! - Provide alternate, safer channels:
//!   - International bodies,
//!   - Independent neurorights/watchdog organizations,
//!   - Trusted civil-society groups.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversightChannel {
    pub name: String,
    pub description: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateOversightPlan {
    pub case_id: String,
    pub channels: Vec<OversightChannel>,
}

pub fn suggest_alternate_oversight(case_id: &str) -> AlternateOversightPlan {
    let mut channels = Vec::new();

    // International human rights mechanisms.
    channels.push(OversightChannel {
        name: "UN Special Procedures / Special Rapporteurs".into(),
        description: "Consider submitting information to UN Special Rapporteurs or Working Groups on issues like torture, freedom of expression, privacy, or emerging technologies, following their guidance on individual communications.",
        link: Some("https://www.ohchr.org/en/special-procedures-hrc/communications".into()),
    });

    channels.push(OversightChannel {
        name: "International or regional human-rights bodies".into(),
        description: "Explore whether regional human-rights bodies, ombuds institutions, or courts (where available) accept individual petitions related to surveillance, neurotechnology, or abuse by state actors.",
        link: None,
    });

    // Neurorights-related actors and civil society (examples; user chooses).
    channels.push(OversightChannel {
        name: "Independent neurorights / neuroethics organizations".into(),
        description: "Contact independent neurorights, neuroethics, or digital rights organizations that monitor neurotechnology and AI-related abuses, sharing a documented, carefully anonymized summary of your case.",
        link: None,
    });

    channels.push(OversightChannel {
        name: "Trusted civil society and legal aid".into(),
        description: "Reach out to trusted civil liberties groups, legal clinics, or human rights defenders who have experience with state abuse cases and can advise on safe escalation paths.",
        link: None,
    });

    AlternateOversightPlan {
        case_id: case_id.into(),
        channels,
    }
}

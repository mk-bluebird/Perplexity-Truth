//! UN human-rights mapping library for digital technology and neurotechnology.
//!
//! Maps neuroright impacts to relevant UN treaty articles and due-diligence components,
//! based on UN guidance for digital technology use and neurorights analysis.

use crate::rights::neurorights::Neuroright;
use serde::{Deserialize, Serialize};

/// Key UN treaties / instruments we reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Untreaty {
    Udnhr, // Universal Declaration of Human Rights
    Iccpr, // International Covenant on Civil and Political Rights
    Icescr, // International Covenant on Economic, Social and Cultural Rights
    Crpd, // Convention on the Rights of Persons with Disabilities
}

/// Specific articles we care about in the UN framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnArticle {
    // UDHR
    UdhrArt3,  // Right to life, liberty, and security of person.
    UdhrArt12, // Privacy, family, home, correspondence.
    UdhrArt18, // Freedom of thought, conscience, and religion.
    UdhrArt19, // Freedom of opinion and expression.

    // ICCPR
    IccprArt7,  // No torture, cruel, inhuman or degrading treatment.
    IccprArt17, // Privacy, family, home, correspondence.
    IccprArt18, // Freedom of thought, conscience, religion.

    // ICESCR
    IcescrArt12, // Right to the highest attainable standard of physical and mental health.

    // CRPD (mental health and disability context)
    CrpdArt1,  // Purpose and persons with disabilities.
    CrpdArt17, // Protecting the integrity of the person.
}

/// Components of UN human-rights due diligence for digital technology use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HrddComponent {
    /// Embed human rights risk management in governance and processes.
    EmbedRiskManagement,
    /// Identify and assess actual and potential human rights impacts.
    IdentifyAssessImpacts,
    /// Take action to prevent, mitigate, and redress as appropriate.
    PreventMitigateRedress,
    /// Track implementation and effectiveness.
    TrackImplementation,
    /// Communicate about how impacts are being addressed.
    CommunicateActions,
}

/// Mapping entry between a neuroright and UN framework references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnNeurorightMapping {
    pub neuroright: Neuroright,
    pub articles: Vec<UnArticle>,
    pub hrdd_components: Vec<HrddComponent>,
    /// Optional free-text note (e.g., referencing specific UN guidance documents).
    pub note: String,
}

/// Return the UN mapping for a given neuroright.
pub fn mapping_for_neuroright(right: Neuroright) -> UnNeurorightMapping {
    match right {
        Neuroright::CognitiveLiberty => UnNeurorightMapping {
            neuroright: right,
            articles: vec![
                UnArticle::UdhrArt18,
                UnArticle::UdhrArt19,
                UnArticle::IccprArt18,
            ],
            hrdd_components: vec![
                HrddComponent::EmbedRiskManagement,
                HrddComponent::IdentifyAssessImpacts,
                HrddComponent::PreventMitigateRedress,
            ],
            note: "Cognitive liberty intersects with freedoms of thought, conscience, and expression. Digital and neurotechnology systems must respect these freedoms throughout their lifecycle.",
        },
        Neuroright::MentalPrivacy => UnNeurorightMapping {
            neuroright: right,
            articles: vec![
                UnArticle::UdhrArt12,
                UnArticle::IccprArt17,
            ],
            hrdd_components: vec![
                HrddComponent::IdentifyAssessImpacts,
                HrddComponent::PreventMitigateRedress,
                HrddComponent::TrackImplementation,
            ],
            note: "Mental privacy relates to protections against arbitrary or unlawful interference with privacy, family, home, and correspondence, extended to neural data and mental states.",
        },
        Neuroright::MentalIntegrity => UnNeurorightMapping {
            neuroright: right,
            articles: vec![
                UnArticle::UdhrArt3,
                UnArticle::IccprArt7,
                UnArticle::CrpdArt17,
            ],
            hrdd_components: vec![
                HrddComponent::EmbedRiskManagement,
                HrddComponent::IdentifyAssessImpacts,
                HrddComponent::PreventMitigateRedress,
            ],
            note: "Mental integrity draws on bans on torture and cruel, inhuman or degrading treatment and the integrity of the person, especially for people using or affected by neurotechnology.",
        },
        Neuroright::PsychologicalContinuity => UnNeurorightMapping {
            neuroright: right,
            articles: vec![
                UnArticle::UdhrArt3,
                UnArticle::IcescrArt12,
                UnArticle::CrpdArt1,
            ],
            hrdd_components: vec![
                HrddComponent::IdentifyAssessImpacts,
                HrddComponent::TrackImplementation,
                HrddComponent::CommunicateActions,
            ],
            note: "Psychological continuity relates to security of person, mental health, and protection of persons with disabilities from exploitation or non-consensual modification.",
        },
    }
}

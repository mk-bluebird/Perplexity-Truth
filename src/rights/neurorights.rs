//! Data types for representing neurorights and checking incidents against them.
//!
//! Neurorights include cognitive liberty, mental privacy, mental integrity,
//! and psychological continuity. These are modeled so they can be attached
//! to incidents, claims, and evidence tiering.

/// Core neurorights recognized by this project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Neuroright {
    /// The right to mental self-determination: control over one’s own mental processes,
    /// cognition, and consciousness, including the freedom to use or refuse neurotechnology.
    CognitiveLiberty,

    /// The right to keep one’s thoughts, mental states, and brain data private
    /// against unauthorized access, inference, or disclosure.
    MentalPrivacy,

    /// The right to be free from non-consensual interference with one’s brain or
    /// mental states, particularly harmful or manipulative interventions.
    MentalIntegrity,

    /// The right to maintain one’s sense of self and identity over time, free from
    /// unauthorized external manipulation of memory, personality, or core preferences.
    PsychologicalContinuity,
}

/// A simple enumeration of how strongly an incident appears to affect a neuroright.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeurorightImpactLevel {
    /// No apparent connection to neurorights.
    None,
    /// Possible impact; more information needed.
    Possible,
    /// Probable impact based on available evidence.
    Probable,
    /// Clear and direct impact.
    Clear,
}

/// A structured assessment of an incident with respect to neurorights.
#[derive(Debug, Clone)]
pub struct NeurorightAssessment {
    pub right: Neuroright,
    pub impact: NeurorightImpactLevel,
    /// Optional short explanation or rationale.
    pub rationale: String,
}

impl NeurorightAssessment {
    pub fn new(right: Neuroright, impact: NeurorightImpactLevel, rationale: impl Into<String>) -> Self {
        Self {
            right,
            impact,
            rationale: rationale.into(),
        }
    }
}

/// A collection of neurorights assessments for a single incident or claim.
#[derive(Debug, Clone, Default)]
pub struct IncidentNeurorights {
    pub assessments: Vec<NeurorightAssessment>,
}

impl IncidentNeurorights {
    pub fn add_assessment(&mut self, assessment: NeurorightAssessment) {
        self.assessments.push(assessment);
    }

    /// Get the highest impact level recorded for a given neuroright.
    pub fn max_impact_for(&self, right: Neuroright) -> NeurorightImpactLevel {
        self.assessments
            .iter()
            .filter(|a| a.right == right)
            .map(|a| a.impact)
            .max_by_key(|level| match level {
                NeurorightImpactLevel::None => 0,
                NeurorightImpactLevel::Possible => 1,
                NeurorightImpactLevel::Probable => 2,
                NeurorightImpactLevel::Clear => 3,
            })
            .unwrap_or(NeurorightImpactLevel::None)
    }

    /// Return a list of neurorights that appear to be clearly impacted.
    pub fn clearly_impacted_rights(&self) -> Vec<Neuroright> {
        self.assessments
            .iter()
            .filter(|a| a.impact == NeurorightImpactLevel::Clear)
            .map(|a| a.right)
            .collect()
    }
}

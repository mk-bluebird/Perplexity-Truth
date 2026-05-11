// src/rust/classifier.rs
//! Scenario classification module
//! 
//! Maps natural language scenario descriptions to structured policy themes
//! using keyword matching and simple heuristics. Designed for extensibility
//! via configuration files rather than hardcoded logic.

use crate::types::{ClassificationResult, ScenarioInput, PolicyTheme, ContextTag, RiskLevel};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    // Context keyword mappings - extend via config/jurisdictions.yaml
    static ref CONTEXT_KEYWORDS: HashMap<ContextTag, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(ContextTag::HomePrivateUse, vec!["home", "residence", "apartment", "private", "personal device"]);
        m.insert(ContextTag::VirtualPlatformTracking, vec!["vr", "ar", "virtual reality", "augmented reality", "metaverse", "gaming platform"]);
        m.insert(ContextTag::WorkplaceMonitoring, vec!["work", "employer", "employee", "productivity", "performance review"]);
        m.insert(ContextTag::MedicalResearch, vec!["clinical trial", "research study", "medical device", "therapeutic", "healthcare"]);
        m.insert(ContextTag::ChildrenInteraction, vec!["child", "minor", "under 18", "student", "school", "educational"]);
        m
    };

    // Technology keyword mappings
    static ref TECH_KEYWORDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("eeg", "eeg_headset");
        m.insert("brain-computer interface", "bci_implant");
        m.insert("neural wearable", "vr_neuro_wearable");
        m.insert("attention tracking", "ar_attention_tracker");
        m.insert("brain sensor", "generic_neurotech");
        m
    };

    // Risk assessment weights
    static ref RISK_WEIGHTS: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("raw_neural_signals", 3);
        m.insert("inferred_emotional_state", 4);
        m.insert("cognitive_load_metrics", 2);
        m.insert("intention_predictions", 5);
        m.insert("advertising", 3);
        m.insert("ai_training", 4);
        m.insert("third_party_sharing", 5);
        m
    };
}

/// Classify a scenario input into structured policy themes
pub fn classify_scenario(input: &ScenarioInput) -> Result<ClassificationResult, Box<dyn std::error::Error>> {
    let description_lower = input.description.to_lowercase();
    
    // Detect context tags
    let mut context_matches = HashSet::new();
    for (tag, keywords) in CONTEXT_KEYWORDS.iter() {
        for &kw in keywords {
            if description_lower.contains(kw) {
                context_matches.insert(*tag);
                break;
            }
        }
    }
    let primary_context = context_matches.iter().next()
        .copied()
        .unwrap_or(ContextTag::Unknown);
    
    // Detect technology category
    let tech_category = TECH_KEYWORDS.iter()
        .find(|(kw, _)| description_lower.contains(kw))
        .map(|(_, cat)| *cat)
        .unwrap_or("generic_neurotech");
    
    // Derive policy themes from context and tech
    let policy_themes = derive_policy_themes(primary_context, tech_category, &input.data_uses);
    
    // Assess risk level
    let risk_score = assess_risk_level(&input.data_types, &input.data_uses);
    let risk_level = match risk_score {
        0..=3 => RiskLevel::Low,
        4..=7 => RiskLevel::Medium,
        8..=12 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };
    
    // Infer jurisdiction hints
    let jurisdiction_hints = infer_jurisdiction(&input.jurisdiction);
    
    Ok(ClassificationResult {
        scenario_id: input.scenario_id.clone(),
        primary_context,
        technology_category: tech_category.to_string(),
        policy_themes,
        risk_level,
        jurisdiction_hints,
        confidence_score: calculate_confidence(&context_matches, &description_lower),
    })
}

fn derive_policy_themes(
    context: ContextTag,
    tech_category: &str,
    data_uses: &[String],
) -> Vec<PolicyTheme> {
    let mut themes = Vec::new();
    
    // Base themes from context
    match context {
        ContextTag::HomePrivateUse => {
            themes.push(PolicyTheme::Consent);
            themes.push(PolicyTheme::PurposeLimitation);
            themes.push(PolicyTheme::SensitiveClassification);
        }
        ContextTag::VirtualPlatformTracking => {
            themes.push(PolicyTheme::SubliminalManipulation);
            themes.push(PolicyTheme::EmotionInference);
            if data_uses.iter().any(|u| u.contains("advertising")) {
                themes.push(PolicyTheme::ThirdPartySharing);
            }
        }
        ContextTag::WorkplaceMonitoring => {
            themes.push(PolicyTheme::EmotionInference);
            themes.push(PolicyTheme::Workplace);
        }
        ContextTag::ChildrenInteraction => {
            themes.push(PolicyTheme::Children);
            themes.push(PolicyTheme::Consent); // Parental consent
        }
        _ => {}
    }
    
    // Additional themes from data uses
    if data_uses.iter().any(|u| u.contains("ai_training")) {
        themes.push(PolicyTheme::PurposeLimitation);
    }
    
    themes
}

fn assess_risk_level(data_types: &[String], data_uses: &[String]) -> u8 {
    let mut score = 0;
    
    // Weight data types
    for dt in data_types {
        if let Some(&weight) = RISK_WEIGHTS.get(dt.as_str()) {
            score += weight;
        }
    }
    
    // Weight data uses
    for du in data_uses {
        if let Some(&weight) = RISK_WEIGHTS.get(du.as_str()) {
            score += weight;
        }
    }
    
    // Context multipliers
    score
}

fn infer_jurisdiction(input_jurisdiction: &Option<String>) -> Vec<String> {
    match input_jurisdiction.as_deref() {
        Some(j) if j.starts_with("US-") => vec![j.to_string(), "US-FEDERAL".to_string()],
        Some("EU") => vec!["EU".to_string(), "GLOBAL_HR".to_string()],
        Some("global") | None => vec!["GLOBAL_HR".to_string()],
        Some(other) => vec![other.to_string()],
    }
}

fn calculate_confidence(context_matches: &HashSet<ContextTag>, description: &str) -> f32 {
    let base_confidence = if context_matches.is_empty() { 0.3 } else { 0.7 };
    let keyword_density = (description.split_whitespace().count() as f32).min(100.0) / 100.0;
    base_confidence * (0.5 + 0.5 * keyword_density)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_home_scenario_classification() {
        let input = ScenarioInput {
            scenario_id: "test-001".to_string(),
            description: "My VR headset reads brainwaves in my apartment during gaming".to_string(),
            data_types: vec!["raw_neural_signals".to_string()],
            data_uses: vec!["primary_function".to_string(), "ai_training".to_string()],
            jurisdiction: Some("US-CO".to_string()),
        };
        
        let result = classify_scenario(&input).unwrap();
        assert_eq!(result.primary_context, ContextTag::VirtualPlatformTracking);
        assert!(result.policy_themes.contains(&PolicyTheme::PurposeLimitation));
        assert_eq!(result.jurisdiction_hints, vec!["US-CO", "US-FEDERAL"]);
    }
}

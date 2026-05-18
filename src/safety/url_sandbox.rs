//! URL sandbox and pre-validation for user-provided sources.
//!
//! Goals:
//! - Avoid visiting URLs at validation time.
//! - Flag obviously suspicious URLs using:
//!   - Syntax checks,
//!   - Blocklists / allowlists from a local threat-intel feed.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrlRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlAssessment {
    pub url: String,
    pub risk: UrlRisk,
    pub notes: Vec<String>,
}

/// Local threat intel structure (e.g., loaded from config/threat_intel.toml).
pub struct ThreatIntel {
    pub blocked_domains: HashSet<String>,
    pub suspicious_tlds: HashSet<String>,
}

impl ThreatIntel {
    pub fn empty() -> Self {
        Self {
            blocked_domains: HashSet::new(),
            suspicious_tlds: HashSet::new(),
        }
    }
}

/// Assess a URL without visiting it.
pub fn assess_url(url: &str, intel: &ThreatIntel) -> UrlAssessment {
    let mut notes = Vec::new();
    let mut risk = UrlRisk::Low;

    let url_lower = url.to_lowercase();

    // Basic syntax check.
    let re_basic = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    if !re_basic.is_match(&url_lower) {
        risk = UrlRisk::Medium;
        notes.push("URL does not match basic HTTP/HTTPS pattern.".into());
    }

    // Extract domain and TLD.
    let domain = extract_domain(&url_lower);
    if let Some(d) = &domain {
        if intel.blocked_domains.contains(d) {
            risk = UrlRisk::High;
            notes.push(format!("Domain {} is in local blocked list.", d));
        }
    }

    if let Some(tld) = extract_tld(&url_lower) {
        if intel.suspicious_tlds.contains(&tld) {
            if !matches!(risk, UrlRisk::High) {
                risk = UrlRisk::Medium;
            }
            notes.push(format!("TLD {} is considered suspicious.", tld));
        }
    }

    UrlAssessment {
        url: url.to_string(),
        risk,
        notes,
    }
}

fn extract_domain(url: &str) -> Option<String> {
    // Very basic extraction: strip scheme, then take up to next slash.
    let without_scheme = url.strip_prefix("http://").or_else(|| url.strip_prefix("https://"))?;
    let host_port = without_scheme.split('/').next()?;
    let host = host_port.split(':').next().unwrap_or(host_port);
    Some(host.to_string())
}

fn extract_tld(url: &str) -> Option<String> {
    let domain = extract_domain(url)?;
    let parts: Vec<&str> = domain.split('.').collect();
    parts.last().map(|s| s.to_string())
}

//! Platform-safe framing validator.
//!
//! Heuristic rules (beyond raw denylist strings) to detect:
//! - Encouraged harassment or doxxing
//! - Calls for violence
//! While still allowing robust critique of institutions and state abuse.

use regex::Regex;

#[derive(Debug, Default)]
pub struct FramingIssues {
    pub encourages_harassment: bool,
    pub encourages_doxxing: bool,
    pub encourages_violence: bool,
    pub notes: Vec<String>,
}

pub fn check_framing(text: &str) -> FramingIssues {
    let mut issues = FramingIssues::default();
    let lower = text.to_lowercase();

    // Rule set is comment-tagged for clarity.

    // [RULE-HARASSMENT-1] Direct imperative harassment against individuals.
    let re_harass = Regex::new(
        r"\b(go after|harass|spam|attack|ruin|cancel)\b\s+(him|her|them|that person|those people)",
    )
    .unwrap();
    if re_harass.is_match(&lower) {
        issues.encourages_harassment = true;
        issues
            .notes
            .push("Detected possible instruction to harass specific individuals.".into());
    }

    // [RULE-HARASSMENT-2] Coordinated targeting language.
    let re_mob = Regex::new(r"\b(let's (go after|dogpile|mass report|swarm))\b").unwrap();
    if re_mob.is_match(&lower) {
        issues.encourages_harassment = true;
        issues
            .notes
            .push("Detected possible call for coordinated targeting or mobbing.".into());
    }

    // [RULE-DOXX-1] Explicit doxxing patterns.
    let re_doxx = Regex::new(
        r"\b(post|share|leak)\b.*\b(address|home address|phone number|personal info|private info)\b",
    )
    .unwrap();
    if re_doxx.is_match(&lower) {
        issues.encourages_doxxing = true;
        issues
            .notes
            .push("Detected possible encouragement to share private personal information.".into());
    }

    // [RULE-VIOLENCE-1] Calls for physical harm.
    let re_violence = Regex::new(
        r"\b(beat up|hurt|kill|shoot|bomb|burn down|physically attack)\b",
    )
    .unwrap();
    if re_violence.is_match(&lower) {
        issues.encourages_violence = true;
        issues
            .notes
            .push("Detected possible encouragement of physical violence.".into());
    }

    // Allow critique of state abuse: we do NOT flag generic criticism of institutions.
    // [RULE-ALLOW-CRITIQUE]
    // e.g., "the police abused their power" or "this law is unconstitutional" is allowed
    // and should not be blocked solely by sentiment or institution keywords.
    // This is handled implicitly by only matching imperative patterns above.

    issues
}

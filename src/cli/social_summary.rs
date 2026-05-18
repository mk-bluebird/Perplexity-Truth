//! CLI tool: convert a full CaseTemplate JSON into a social-media summary Markdown.
//!
//! Usage:
//!   cargo run --bin social_summary -- <input.json>

use crate::models::template::{CaseTemplate, SocialSummaryTemplate, ValidatableTemplate};
use std::io::{self, Read};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON from stdin.
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let case: CaseTemplate = serde_json::from_str(&buf)?;

    case.validate()?; // ensure invariants like tier separation

    let summary = to_social_summary(&case);
    let markdown = render_social_markdown(&summary);
    println!("{markdown}");
    Ok(())
}

fn to_social_summary(case: &CaseTemplate) -> SocialSummaryTemplate {
    // Extract core pieces for the summary.
    let timeframe = format!("{} – {}", case.period_from, case.period_to);
    let locations = case.main_locations.clone();
    let core_events: Vec<String> = case
        .incidents
        .iter()
        .map(|i| format!("{} at {}: {}", i.date_time, i.location, i.observable_actions))
        .collect();

    // Confirmed points from documents.
    let confirmed_points = case
        .documents
        .iter()
        .map(|d| format!("{} ({})", d.source_id, d.relevance))
        .collect();

    let pattern_description = case.pattern_labels.join(", ");
    let suppression_reasons = case.interpretation_reasons.clone();
    let rights_at_risk = case.rights_implicated.clone();

    // Tier snapshot from external sources.
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    let mut t3 = Vec::new();
    for src in &case.external_sources {
        let label = format!("{} – {}", src.source_id, src.notes);
        match src.tier {
            crate::models::template::TemplateTier::Tier1 => t1.push(label),
            crate::models::template::TemplateTier::Tier2 => t2.push(label),
            crate::models::template::TemplateTier::Tier3 => t3.push(label),
        }
    }

    let requests = case.desired_remedies.clone();
    let closing_line = case.summary_paragraph.clone();

    SocialSummaryTemplate {
        timeframe,
        locations,
        core_events,
        confirmed_points,
        pattern_description,
        suppression_reasons,
        rights_at_risk,
        tier1_points: t1,
        tier2_points: t2,
        tier3_points: t3,
        requests,
        closing_line,
    }
}

fn render_social_markdown(s: &SocialSummaryTemplate) -> String {
    let mut out = String::new();

    out.push_str("# Case Summary (Social Media Ready)\n\n");

    out.push_str("## 1. What Happened (Facts Only)\n\n");
    out.push_str(&format!("- **Timeframe:** {}\n", s.timeframe));
    if !s.locations.is_empty() {
        out.push_str("- **Locations:** ");
        out.push_str(&s.locations.join(", "));
        out.push('\n');
    }
    if !s.core_events.is_empty() {
        out.push_str("- **Core events:**\n");
        for ev in &s.core_events {
            out.push_str(&format!("  - {}\n", ev));
        }
    }
    out.push('\n');

    out.push_str("## 2. What the Documents Show\n\n");
    if !s.confirmed_points.is_empty() {
        out.push_str("- **Confirmed by documents:**\n");
        for p in &s.confirmed_points {
            out.push_str(&format!("  - {}\n", p));
        }
    } else {
        out.push_str("- No specific points confirmed by documents were provided.\n");
    }
    out.push('\n');

    out.push_str("## 3. Patterns and Concerns (Your Interpretation)\n\n");
    if !s.pattern_description.is_empty() {
        out.push_str(&format!("- **Pattern you see:** {}\n", s.pattern_description));
    }
    if !s.suppression_reasons.is_empty() {
        out.push_str("- **Why it feels like suppression / interference:**\n");
        for r in &s.suppression_reasons {
            out.push_str(&format!("  - {}\n", r));
        }
    }
    if !s.rights_at_risk.is_empty() {
        out.push_str("- **Rights or values at risk:**\n");
        for r in &s.rights_at_risk {
            out.push_str(&format!("  - {}\n", r));
        }
    }
    out.push('\n');

    out.push_str("## 4. Evidence Tiers (Snapshot)\n\n");

    if !s.tier1_points.is_empty() {
        out.push_str("- **Tier 1 – Verified:**\n");
        for p in &s.tier1_points {
            out.push_str(&format!("  - {}\n", p));
        }
    }
    if !s.tier2_points.is_empty() {
        out.push_str("\n- **Tier 2 – Plausible / Corroborated:**\n");
        for p in &s.tier2_points {
            out.push_str(&format!("  - {}\n", p));
        }
    }
    if !s.tier3_points.is_empty() {
        out.push_str("\n- **Tier 3 – Unverified / Speculative:**\n");
        for p in &s.tier3_points {
            out.push_str(&format!("  - {}\n", p));
        }
    }
    out.push('\n');

    out.push_str("## 5. What You Are Asking For (Lawful, Peaceful)\n\n");
    if !s.requests.is_empty() {
        out.push_str("- **Requests / calls-to-action:**\n");
        for r in &s.requests {
            out.push_str(&format!("  - {}\n", r));
        }
    }
    if let Some(closing) = &s.closing_line {
        if !closing.trim().is_empty() {
            out.push('\n');
            out.push_str(&format!("{closing}\n"));
        }
    }

    out
}

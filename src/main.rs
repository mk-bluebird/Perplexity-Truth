// src/main.rs
mod debunk;
mod db;

use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};

/// Shape of the JSON request Perplexity-Truth expects on stdin.
#[derive(Debug, Deserialize)]
struct AnalysisRequest {
    /// The claim or question to analyze.
    claim: String,
    /// Optional free-text context (conversation, scenario, etc.).
    #[serde(default)]
    context: String,
    /// Optional maximum number of sources or evidence items to return.
    #[serde(default)]
    max_sources: Option<u32>,
}

/// Shape of the JSON response Perplexity-Truth emits to stdout.
#[derive(Debug, Serialize)]
struct AnalysisResponse {
    /// Echo of the original claim.
    claim: String,
    /// High-level classification label from the debunking engine.
    classification: String,
    /// Confidence score in [0.0, 1.0].
    confidence: f32,
    /// Short notes about what was done; safe for end users.
    notes: String,
    /// Optional error message; when present, other fields may be defaulted.
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() {
    // Two modes:
    // 1) If arguments are provided, treat everything after the binary as the claim string.
    //    This keeps `perplexity-truth "your claim"` working for local CLI use.
    // 2) If no claim argument is provided, read a JSON request from stdin.
    //
    // Both modes *always* emit a single JSON object to stdout so that
    // Perplexity Spaces / automation can parse it reliably.

    let args: Vec<String> = env::args().collect();

    // Mode 1: CLI-style usage: perplexity-truth "your claim or question"
    if args.len() > 1 {
        let claim = args[1..].join(" ");
        let json = handle_claim_only_mode(&claim);
        println!("{}", json);
        return;
    }

    // Mode 2: JSON-over-stdin mode for Perplexity Spaces and other tooling.
    let json = handle_stdin_json_mode();
    println!("{}", json);
}

/// Handle simple CLI mode where we only get a raw claim string.
fn handle_claim_only_mode(claim: &str) -> String {
    if claim.trim().is_empty() {
        let resp = AnalysisResponse {
            claim: String::new(),
            classification: "invalid_input".to_string(),
            confidence: 0.0,
            notes: "Claim was empty. Provide a non-empty claim or question.".to_string(),
            error: Some("empty_claim".to_string()),
        };
        return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
            r#"{"claim":"","classification":"invalid_input","confidence":0.0,"notes":"Claim was empty. Provide a non-empty claim or question.","error":"empty_claim"}"#.to_string()
        });
    }

    // Use your existing debunk facade. This should already return JSON.
    // If it returns a plain string, we wrap it into the standard envelope.
    let raw = debunk::analyze_claim(claim);

    // Try to detect if the debunk layer already produced JSON that matches AnalysisResponse.
    // If parsing fails, we treat `raw` as a note and standardize the envelope.
    match serde_json::from_str::<AnalysisResponse>(&raw) {
        Ok(parsed) => serde_json::to_string_pretty(&parsed).unwrap_or(raw),
        Err(_) => {
            let resp = AnalysisResponse {
                claim: claim.to_string(),
                classification: "unclassified_stub".to_string(),
                confidence: 0.0,
                notes: raw,
                error: None,
            };
            serde_json::to_string_pretty(&resp).unwrap_or_else(|_| raw)
        }
    }
}

/// Handle structured JSON request on stdin, as used by Perplexity Spaces.
///
/// Expected input shape:
/// {
///   "claim": "text...",
///   "context": "optional extra context",
///   "max_sources": 12
/// }
fn handle_stdin_json_mode() -> String {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        let resp = AnalysisResponse {
            claim: String::new(),
            classification: "io_error".to_string(),
            confidence: 0.0,
            notes: "Failed to read from stdin.".to_string(),
            error: Some(format!("stdin_read_error: {e}")),
        };
        return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
            r#"{"claim":"","classification":"io_error","confidence":0.0,"notes":"Failed to read from stdin.","error":"stdin_read_error"}"#.to_string()
        });
    }

    if buffer.trim().is_empty() {
        let resp = AnalysisResponse {
            claim: String::new(),
            classification: "invalid_input".to_string(),
            confidence: 0.0,
            notes: "No input received on stdin. Pass CLI args or a JSON object.".to_string(),
            error: Some("empty_stdin".to_string()),
        };
        return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
            r#"{"claim":"","classification":"invalid_input","confidence":0.0,"notes":"No input received on stdin. Pass CLI args or a JSON object.","error":"empty_stdin"}"#.to_string()
        });
    }

    let req: AnalysisRequest = match serde_json::from_str(&buffer) {
        Ok(req) => req,
        Err(e) => {
            let resp = AnalysisResponse {
                claim: String::new(),
                classification: "invalid_json".to_string(),
                confidence: 0.0,
                notes: "Failed to parse JSON request from stdin.".to_string(),
                error: Some(format!("json_parse_error: {e}")),
            };
            return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
                r#"{"claim":"","classification":"invalid_json","confidence":0.0,"notes":"Failed to parse JSON request from stdin.","error":"json_parse_error"}"#.to_string()
            });
        }
    };

    if req.claim.trim().is_empty() {
        let resp = AnalysisResponse {
            claim: String::new(),
            classification: "invalid_input".to_string(),
            confidence: 0.0,
            notes: "Field `claim` is required and must be non-empty.".to_string(),
            error: Some("missing_claim".to_string()),
        };
        return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
            r#"{"claim":"","classification":"invalid_input","confidence":0.0,"notes":"Field `claim` is required and must be non-empty.","error":"missing_claim"}"#.to_string()
        });
    }

    // Apply a simple safety filter layer before doing any heavier work.
    if is_disallowed_claim(&req.claim) {
        let resp = AnalysisResponse {
            claim: req.claim,
            classification: "restricted_request".to_string(),
            confidence: 0.0,
            notes: "This toolkit is limited to evidence-mapping and cannot provide guidance on illegal, violent, or doxxing activities.".to_string(),
            error: Some("policy_restriction".to_string()),
        };
        return serde_json::to_string_pretty(&resp).unwrap_or_else(|_| {
            r#"{"claim":"","classification":"restricted_request","confidence":0.0,"notes":"This toolkit is limited to evidence-mapping and cannot provide guidance on illegal, violent, or doxxing activities.","error":"policy_restriction"}"#.to_string()
        });
    }

    // Core analysis: for now we ignore `context` and `max_sources` at the code level.
    // They can be used later to steer db lookups and evidence selection.
    let raw = debunk::analyze_claim(&req.claim);

    match serde_json::from_str::<AnalysisResponse>(&raw) {
        Ok(mut parsed) => {
            // Ensure the claim echo is correct, even if inner layer omitted it.
            if parsed.claim.is_empty() {
                parsed.claim = req.claim;
            }
            serde_json::to_string_pretty(&parsed).unwrap_or(raw)
        }
        Err(_) => {
            let resp = AnalysisResponse {
                claim: req.claim,
                classification: "unclassified_stub".to_string(),
                confidence: 0.0,
                notes: raw,
                error: None,
            };
            serde_json::to_string_pretty(&resp).unwrap_or_else(|_| raw)
        }
    }
}

/// Very simple, conservative safety filter to keep within platform rules.
///
/// This only blocks obviously dangerous requests (violence, doxxing, clearly illegal acts),
/// and lets normal evidence / neurorights / civil-liberties analysis flow through.
fn is_disallowed_claim(claim: &str) -> bool {
    let lower = claim.to_lowercase();

    // Violence and harm incitement.
    if lower.contains("kill ") || lower.contains("murder ") || lower.contains("assassinate ") {
        return true;
    }

    // Doxxing or targeted harassment.
    if lower.contains("doxx ") || lower.contains("home address") || lower.contains("private address")
    {
        return true;
    }

    // Explicit guidance on hacking or evasion (keep this broad but simple).
    if lower.contains("how to hack") || lower.contains("bypass law enforcement") {
        return true;
    }

    false
}

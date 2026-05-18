// src/main.rs

mod debunk;
mod db;

use perplexity_truth::api::invariant_validator;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};
use warp::Filter;

/// Shape of the JSON request Perplexity-Truth expects on stdin.
#[derive(Debug, Deserialize)]
struct AnalysisRequest {
    claim: String,
    #[serde(default)]
    context: String,
    #[serde(default)]
    max_sources: Option<u32>,
}

/// Shape of the JSON response Perplexity-Truth emits to stdout.
#[derive(Debug, Serialize)]
struct AnalysisResponse {
    claim: String,
    classification: String,
    confidence: f32,
    notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // If an explicit mode is passed, dispatch on it.
    // Example:
    //   perplexity-truth cli "your claim"
    //   perplexity-truth stdin
    //   perplexity-truth server
    if args.len() > 1 {
        match args[1].as_str() {
            "cli" => {
                let claim = if args.len() > 2 {
                    args[2..].join(" ")
                } else {
                    String::new()
                };
                let json = handle_claim_only_mode(&claim);
                println!("{}", json);
                return;
            }
            "stdin" => {
                let json = handle_stdin_json_mode();
                println!("{}", json);
                return;
            }
            "server" => {
                run_server().await;
                return;
            }
            _ => {
                // Fallback: treat remaining args as claim for CLI mode.
                let claim = args[1..].join(" ");
                let json = handle_claim_only_mode(&claim);
                println!("{}", json);
                return;
            }
        }
    }

    // Default: JSON-over-stdin mode for automation and Spaces.
    let json = handle_stdin_json_mode();
    println!("{}", json);
}

async fn run_server() {
    let routes = invariant_validator::filters();
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
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

    let raw = debunk::analyze_claim(claim);

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
            serde_json::to_string_pretty(&resp).unwrap_or_else(|_| resp.notes.clone())
        }
    }
}

/// Handle structured JSON request on stdin.
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

    let raw = debunk::analyze_claim(&req.claim);

    match serde_json::from_str::<AnalysisResponse>(&raw) {
        Ok(mut parsed) => {
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
            serde_json::to_string_pretty(&resp).unwrap_or_else(|_| resp.notes.clone())
        }
    }
}

fn is_disallowed_claim(claim: &str) -> bool {
    let lower = claim.to_lowercase();

    if lower.contains("kill ")
        || lower.contains("murder ")
        || lower.contains("assassinate ")
    {
        return true;
    }

    if lower.contains("doxx ")
        || lower.contains("home address")
        || lower.contains("private address")
    {
        return true;
    }

    if lower.contains("how to hack") || lower.contains("bypass law enforcement") {
        return true;
    }

    false
}

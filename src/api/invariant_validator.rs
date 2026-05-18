use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct ValidationRequest {
    pub response_text: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

async fn handle_validate(req: ValidationRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Placeholder: here you would parse config.toml, apply invariants,
    // run pattern scans, tier logic, etc.
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Simple example: if response is empty, mark as error.
    if req.response_text.trim().is_empty() {
        errors.push("Response text is empty.".to_string());
    }

    // Example stub for an invariant check:
    if req.response_text.contains("rm -rf /") {
        errors.push("Denied pattern 'rm -rf /' found in response_text.".to_string());
    }

    let valid = errors.is_empty();

    let resp = ValidationResponse {
        valid,
        warnings,
        errors,
    };

    Ok(warp::reply::json(&resp))
}

/// Build the Warp filter for the /validate endpoint.
pub fn filters() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let validate_route = warp::path("validate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_validate);

    validate_route
}

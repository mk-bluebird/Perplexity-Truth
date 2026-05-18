use analyzers::validation::validate_text;
use core::cases::{CaseRecord, CaseStore};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

#[derive(Clone)]
struct AppState {
    cases: Arc<RwLock<CaseStore>>,
}

#[derive(Debug, Deserialize)]
struct ValidateRequest {
    text: String,
}

#[derive(Debug, Serialize)]
struct ValidateResponse {
    ok: bool,
    notes: Vec<String>,
}

#[tokio::main]
async fn main() {
    // In-memory store for now; later you can back with SQLite or similar.
    let state = AppState {
        cases: Arc::new(RwLock::new(CaseStore::new())),
    };

    let state_filter = warp::any().map(move || state.clone());

    // GET /health
    let health = warp::path!("health")
        .and(warp::get())
        .and_then(handle_health);

    // POST /validate  { "text": "..." }
    let validate = warp::path!("validate")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(handle_validate);

    // POST /case  { ... CaseRecord JSON ... }
    let create_case = warp::path!("case")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(handle_create_case);

    // GET /case/{id}
    let get_case = warp::path!("case" / String)
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(handle_get_case);

    // Combine routes.
    let routes = health
        .or(validate)
        .or(create_case)
        .or(get_case)
        .with(warp::log("sovereign-truthd"));

    // Bind to localhost only.
    let addr: SocketAddr = "127.0.0.1:8088".parse().expect("valid address");
    println!("sovereign-truthd listening on http://{}", addr);

    warp::serve(routes).run(addr).await;
}

async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&serde_json::json!({
        "status": "ok",
        "service": "sovereign-truthd"
    })))
}

async fn handle_validate(
    req: ValidateRequest,
    _state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    // Use your existing invariants + safety logic.
    let result = validate_text(&req.text);

    let resp = ValidateResponse {
        ok: result.is_ok,
        notes: result.notes,
    };

    Ok(warp::reply::json(&resp))
}

async fn handle_create_case(
    case: CaseRecord,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = state.cases.write().await;
    let id = store.insert(case);
    let resp = serde_json::json!({ "id": id });
    Ok(warp::reply::json(&resp))
}

async fn handle_get_case(
    id: String,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    let store = state.cases.read().await;
    if let Some(case) = store.get(&id) {
        Ok(warp::reply::json(&case))
    } else {
        let resp = serde_json::json!({ "error": "not_found" });
        Ok(warp::reply::with_status(
            warp::reply::json(&resp),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

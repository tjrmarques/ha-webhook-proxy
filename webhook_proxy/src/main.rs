use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use serde::Deserialize;
use std::{collections::HashSet, fs, sync::Arc};
use tokio::net::TcpListener;

const HA_BASE: &str = "http://homeassistant:8123";
const OPTIONS_PATH: &str = "/data/options.json";

#[derive(Deserialize)]
struct Options {
    #[serde(default)]
    allowed_webhook_ids: Vec<String>,
}

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    allowed: Arc<HashSet<String>>,
}

fn load_allowed_ids() -> HashSet<String> {
    match fs::read_to_string(OPTIONS_PATH) {
        Ok(raw) => match serde_json::from_str::<Options>(&raw) {
            Ok(opts) => opts.allowed_webhook_ids.into_iter().collect(),
            Err(e) => {
                eprintln!("failed to parse {OPTIONS_PATH}: {e}");
                HashSet::new()
            }
        },
        Err(e) => {
            eprintln!("failed to read {OPTIONS_PATH}: {e}");
            HashSet::new()
        }
    }
}

#[tokio::main]
async fn main() {
    let allowed = load_allowed_ids();
    println!("allowed webhook ids: {:?}", allowed);

    let state = AppState {
        client: reqwest::Client::new(),
        allowed: Arc::new(allowed),
    };

    let app = Router::new()
        .route("/api/webhook/*path", any(proxy_webhook))
        .fallback(reject)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8181").await.unwrap();
    println!("listening on 0.0.0.0:8181");
    axum::serve(listener, app).await.unwrap();
}

async fn reject() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
}

async fn proxy_webhook(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let webhook_id = path.split('/').next().unwrap_or("");

    // contains() on an empty set is always false, so this naturally
    // fails closed if no allowed_webhook_ids are configured.
    if !state.allowed.contains(webhook_id) {
        return (StatusCode::NOT_FOUND, "not found").into_response();
    }

    let target = format!("{HA_BASE}/api/webhook/{path}");
    let mut req = state.client.request(method, &target).body(body);
    for (k, v) in headers.iter() {
        if k.as_str().to_lowercase() != "host" {
            req = req.header(k, v);
        }
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let bytes = resp.bytes().await.unwrap_or_default();
            (status, bytes).into_response()
        }
        Err(_) => (StatusCode::BAD_GATEWAY, "upstream error").into_response(),
    }
}

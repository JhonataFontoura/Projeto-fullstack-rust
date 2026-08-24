use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/api", get(api_root))
}

async fn api_root() -> Json<Value> {
    Json(json!({"name":"FinTrack Rust API","status":"foundation-ready"}))
}

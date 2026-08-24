use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

mod config;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind address");
    println!("FinTrack Rust running on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({"status":"ok","service":"fintrack-rust","version":"0.1.0"}))
}

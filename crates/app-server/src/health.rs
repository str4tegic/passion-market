use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub fn router() -> Router {
    Router::new().route("/api/v1/health", get(health))
}

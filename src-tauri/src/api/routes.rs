use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ping", get(ping))
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "kaipanla"
    }))
}

async fn ping() -> impl IntoResponse {
    Json(json!({
        "message": "pong"
    }))
}
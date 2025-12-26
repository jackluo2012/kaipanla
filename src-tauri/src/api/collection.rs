use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 采集状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStatus {
    pub is_running: bool,
    pub last_update: Option<String>,
    pub success_count: u64,
    pub failed_count: u64,
    pub servers: Vec<ServerStatus>,
}

/// 服务器状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub host: String,
    pub status: String,  // "healthy" | "failed" | "unknown"
}

/// 启动采集请求
#[derive(Debug, Deserialize)]
pub struct StartCollectionRequest {
    pub codes: Option<Vec<String>>,
    pub mode: String,  // "realtime" | "history"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/collection/start", post(start_collection))
        .route("/api/v1/collection/stop", post(stop_collection))
        .route("/api/v1/collection/status", get(get_collection_status))
        .route("/api/v1/data/quality", get(get_data_quality))
}

/// 启动数据采集
async fn start_collection(
    Json(req): Json<StartCollectionRequest>,
) -> impl IntoResponse {
    tracing::info!("启动数据采集: mode={}, codes={:?}", req.mode, req.codes);

    // TODO: 实际启动采集
    Json(serde_json::json!({
        "status": "started",
        "message": "数据采集已启动"
    }))
}

/// 停止数据采集
async fn stop_collection() -> impl IntoResponse {
    tracing::info!("停止数据采集");

    // TODO: 实际停止采集
    Json(serde_json::json!({
        "status": "stopped",
        "message": "数据采集已停止"
    }))
}

/// 获取采集状态
async fn get_collection_status() -> impl IntoResponse {
    // TODO: 实际查询状态
    let status = CollectionStatus {
        is_running: false,
        last_update: None,
        success_count: 0,
        failed_count: 0,
        servers: vec![
            ServerStatus {
                host: "124.71.187.122:7709".to_string(),
                status: "healthy".to_string(),
            }
        ],
    };

    Json(status)
}

/// 获取数据质量报告
async fn get_data_quality(
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let date = params.get("date").cloned().unwrap_or_default();

    tracing::debug!("查询数据质量: date={}", date);

    // TODO: 实际查询数据质量
    Json(serde_json::json!({
        "date": date,
        "total_records": 0,
        "good_quality": 0,
        "suspect": 0,
        "error": 0,
        "issues": []
    }))
}

use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ping", get(ping))
        .route("/api/v1/quote/:code", get(get_quote))
        .route("/api/v1/moneyflow/:code", get(get_money_flow))
        .route("/api/v1/dragon-tiger", get(get_dragon_tiger_list))
        .route("/api/v1/auction/anomalies", get(get_auction_anomalies))
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

/// 获取股票实时行情
async fn get_quote(axum::extract::Path(code): axum::extract::Path<String>) -> impl IntoResponse {
    // TODO: 调用 QuoteService
    Json(json!({
        "code": code,
        "name": "测试股票",
        "price": 10.5,
        "preclose": 10.0,
        "change": 0.5,
        "change_pct": 5.0
    }))
}

/// 获取资金流向
async fn get_money_flow(axum::extract::Path(code): axum::extract::Path<String>) -> impl IntoResponse {
    // TODO: 调用 MoneyFlowService
    Json(json!({
        "code": code,
        "main_inflow": 5000.0,
        "main_outflow": 3000.0,
        "main_net": 2000.0,
        "retail_inflow": 2000.0,
        "retail_outflow": 4000.0,
        "net_amount": 0.0
    }))
}

/// 获取龙虎榜列表
async fn get_dragon_tiger_list() -> impl IntoResponse {
    // TODO: 调用 DragonTigerService
    Json(json!([]))
}

/// 获取竞价异动
async fn get_auction_anomalies() -> impl IntoResponse {
    // TODO: 调用 AuctionService
    Json(json!([]))
}
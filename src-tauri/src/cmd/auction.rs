use crate::models::auction::AuctionAnomaly;
use crate::service::AuctionService;

/// 获取竞价异动
#[tauri::command]
pub async fn get_auction_anomalies() -> std::result::Result<Vec<AuctionAnomaly>, String> {
    let service = AuctionService::new();

    // 获取竞价数据
    let auctions = service
        .get_auction_data()
        .await
        .map_err(|e| e.to_string())?;

    // 分析异动
    service
        .analyze_anomalies(auctions)
        .map_err(|e| e.to_string())
}

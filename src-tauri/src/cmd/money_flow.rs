use crate::models::MoneyFlow;
use crate::service::MoneyFlowService;

/// 获取资金流向
#[tauri::command]
pub async fn get_money_flow(code: String) -> std::result::Result<MoneyFlow, String> {
    let service = MoneyFlowService::new();
    service
        .get_daily_money_flow(&code)
        .await
        .map_err(|e| e.to_string())
}

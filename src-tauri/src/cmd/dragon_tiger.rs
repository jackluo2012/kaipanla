use crate::models::DragonTiger;
use crate::service::DragonTigerService;
use chrono::NaiveDate;

/// 获取龙虎榜列表
#[tauri::command]
pub async fn get_dragon_tiger_list(date: String) -> std::result::Result<Vec<DragonTiger>, String> {
    let service = DragonTigerService::new();

    // 解析日期字符串
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| format!("日期解析失败: {}", e))?;

    service
        .get_dragon_tiger_list(parsed_date)
        .await
        .map_err(|e| e.to_string())
}

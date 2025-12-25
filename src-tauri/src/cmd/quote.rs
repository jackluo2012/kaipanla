use serde::{Deserialize, Serialize};
use std::result::Result;

/// 股票实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
}

/// 获取实时行情命令
#[tauri::command]
pub async fn get_quote(code: String) -> Result<Quote, String> {
    Ok(Quote {
        code: code.clone(),
        name: "测试股票".to_string(),
        price: 10.5,
        change: 0.5,
        change_pct: 5.0,
        volume: 100000.0,
        amount: 1050000.0,
    })
}

/// 获取股票列表命令
#[tauri::command]
pub async fn get_stock_list() -> Result<Vec<Quote>, String> {
    Ok(vec![])
}
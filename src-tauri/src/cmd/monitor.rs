//! 监控相关的 Tauri 命令

use crate::monitor::{CollectorMonitor, CollectionMetrics, Alert};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 监控器状态（全局共享）
pub struct MonitorState {
    pub monitor: Arc<CollectorMonitor>,
}

impl MonitorState {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            monitor: Arc::new(CollectorMonitor::new(servers)),
        }
    }
}

/// 获取采集指标
#[tauri::command]
pub async fn get_collection_metrics(
    state: tauri::State<'_, Arc<RwLock<MonitorState>>>,
) -> Result<CollectionMetrics, String> {
    let state = state.read().await;
    Ok(state.monitor.get_metrics().await)
}

/// 检查告警
#[tauri::command]
pub async fn check_alerts(
    state: tauri::State<'_, Arc<RwLock<MonitorState>>>,
) -> Result<Vec<Alert>, String> {
    let state = state.read().await;
    Ok(state.monitor.check_alerts().await)
}

/// 重置计数器
#[tauri::command]
pub async fn reset_metrics(
    state: tauri::State<'_, Arc<RwLock<MonitorState>>>,
) -> Result<(), String> {
    let state = state.read().await;
    state.monitor.reset_counters();
    Ok(())
}

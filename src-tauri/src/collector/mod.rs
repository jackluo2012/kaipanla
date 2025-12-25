//! 数据采集模块 - 集成 rustdx 获取通达信数据

pub mod tdx;
pub mod parser;

use crate::config::DataSourceConfig;
use crate::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 数据采集器
pub struct Collector {
    _config: Arc<RwLock<DataSourceConfig>>,
}

impl Collector {
    /// 创建新的数据采集器
    pub fn new(config: Arc<RwLock<DataSourceConfig>>) -> Self {
        Self { _config: config }
    }

    /// 启动数据采集任务
    pub async fn start(&self) -> Result<()> {
        tracing::info!("数据采集器启动");

        // TODO: 启动定时采集任务
        Ok(())
    }
}

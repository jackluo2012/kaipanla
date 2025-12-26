use crate::config::DataSourceConfig;
use crate::{AppError, Result};
use chrono::{Datelike, Timelike, Utc, Weekday};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

/// 采集调度器
pub struct CollectionScheduler {
    config: Arc<RwLock<DataSourceConfig>>,
    is_running: Arc<RwLock<bool>>,
}

impl CollectionScheduler {
    /// 创建新的调度器
    pub fn new(config: Arc<RwLock<DataSourceConfig>>) -> Self {
        Self {
            config,
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// 判断是否为交易日
    pub fn is_trading_day() -> bool {
        let now = Utc::now();
        let weekday = now.weekday();

        // 周末不是交易日
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            return false;
        }

        // TODO: 添加节假日判断
        true
    }

    /// 判断是否在交易时间
    pub fn is_trading_time() -> bool {
        let now = Utc::now();
        let hour = now.hour();

        // 交易时间: 9:00-15:00
        hour >= 9 && hour < 15
    }

    /// 启动定时采集
    pub async fn start(&self) -> Result<()> {
        {
            let mut is_running = self.is_running.write().await;
            if *is_running {
                return Err(AppError::Internal("调度器已在运行".to_string()));
            }
            *is_running = true;
        }

        let interval_secs = {
            let config = self.config.read().await;
            config.update_interval_secs
        };

        let mut timer = interval(Duration::from_secs(interval_secs));
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            tracing::info!("数据采集调度器已启动，间隔: {}秒", interval_secs);

            loop {
                timer.tick().await;

                // 检查是否应该继续运行
                if !*is_running.read().await {
                    tracing::info!("数据采集调度器已停止");
                    break;
                }

                // 仅在交易日交易时间采集
                if Self::is_trading_day() && Self::is_trading_time() {
                    tracing::debug!("触发数据采集任务");

                    // TODO: 触发实际的采集任务
                    // 这里暂时只是日志
                } else {
                    tracing::debug!("非交易时间，跳过采集");
                }
            }
        });

        Ok(())
    }

    /// 停止定时采集
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        tracing::info!("数据采集调度器停止请求已发送");
        Ok(())
    }

    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_trading_day_weekday() {
        // 这个测试依赖于实际运行日期
        let result = CollectionScheduler::is_trading_day();
        // 仅验证方法可调用
        assert!(result == true || result == false);
    }

    #[tokio::test]
    async fn test_scheduler_start_stop() {
        let config = Arc::new(RwLock::new(DataSourceConfig::default()));
        let scheduler = CollectionScheduler::new(config);

        // 启动
        assert!(scheduler.start().await.is_ok());
        assert!(scheduler.is_running().await);

        // 停止
        assert!(scheduler.stop().await.is_ok());

        // 等待停止完成
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(!scheduler.is_running().await);
    }

    #[tokio::test]
    async fn test_scheduler_double_start() {
        let config = Arc::new(RwLock::new(DataSourceConfig::default()));
        let scheduler = CollectionScheduler::new(config);

        // 第一次启动
        assert!(scheduler.start().await.is_ok());

        // 第二次启动应该失败
        assert!(scheduler.start().await.is_err());

        // 清理
        scheduler.stop().await.ok();
    }
}

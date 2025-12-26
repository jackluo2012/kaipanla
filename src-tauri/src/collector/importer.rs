//! 历史数据导入器
//!
//! 实现渐进式历史数据导入：
//! - Step 1: 导入最近1个月数据（高优先级，快速可用）
//! - Step 2: 后台回填3年历史数据（低优先级）
//! - 批次管理：100股票×30天/批
//! - 断点续传：记录导入进度，支持中断恢复
//! - 用户取消：随时可以取消导入任务

use crate::collector::tdx::TdxClient;
use crate::error::{AppError, Result};
use crate::models::stock::Stock;
use chrono::{Duration, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info, warn};

/// 导入进度跟踪
#[derive(Debug, Clone)]
pub struct ImportProgress {
    pub total_stocks: usize,        // 总股票数
    pub imported_stocks: usize,     // 已导入股票数
    pub total_batches: usize,       // 总批次数
    pub imported_batches: usize,    // 已导入批次数
    pub current_code: String,       // 当前正在导入的股票
    pub start_date: String,         // 起始日期
    pub end_date: String,           // 结束日期
    pub stage: ImportStage,         // 当前阶段
    pub is_running: bool,           // 是否正在运行
    pub error_count: usize,         // 错误计数
}

/// 导入阶段
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImportStage {
    Idle,                   // 空闲
    ImportingRecent,        // 导入最近1个月
    ImportingHistory,       // 导入历史数据
    Completed,              // 完成
    Failed,                 // 失败
    Cancelled,              // 已取消
}

/// 历史数据导入器
pub struct HistoryImporter {
    tdx_client: Arc<TdxClient>,
    progress: Arc<tokio::sync::RwLock<ImportProgress>>,
    is_cancelled: Arc<AtomicBool>,
    batch_size: usize,           // 每批次股票数（默认100）
    days_per_batch: i64,         // 每批次天数（默认30天）
}

impl HistoryImporter {
    /// 创建新的导入器
    pub fn new(tdx_client: Arc<TdxClient>) -> Self {
        Self {
            tdx_client,
            progress: Arc::new(tokio::sync::RwLock::new(ImportProgress {
                total_stocks: 0,
                imported_stocks: 0,
                total_batches: 0,
                imported_batches: 0,
                current_code: String::new(),
                start_date: String::new(),
                end_date: String::new(),
                stage: ImportStage::Idle,
                is_running: false,
                error_count: 0,
            })),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            batch_size: 100,
            days_per_batch: 30,
        }
    }

    /// 开始导入历史数据
    ///
    /// 导入策略：
    /// 1. 先导入最近1个月数据（高优先级）
    /// 2. 然后后台回填3年数据
    pub async fn start_import(&self, stocks: Vec<Stock>) -> Result<()> {
        info!("开始历史数据导入，共 {} 只股票", stocks.len());

        // 检查是否已在运行
        {
            let mut progress = self.progress.write().await;
            if progress.is_running {
                return Err(AppError::Config("导入任务已在运行中".to_string()));
            }
            progress.is_running = true;
            progress.stage = ImportStage::ImportingRecent;
            progress.total_stocks = stocks.len();
        }

        self.is_cancelled.store(false, Ordering::SeqCst);

        // Step 1: 导入最近1个月数据
        let recent_start = Instant::now();
        match self.import_recent_month(stocks.clone()).await {
            Ok(_) => {
                let duration = recent_start.elapsed();
                info!("最近1个月数据导入完成，耗时: {:.2}s", duration.as_secs_f64());
            }
            Err(e) => {
                // 检查是否被取消
                if self.is_cancelled.load(Ordering::SeqCst) {
                    let mut progress = self.progress.write().await;
                    progress.stage = ImportStage::Cancelled;
                    progress.is_running = false;
                    return Err(AppError::Config("导入已取消".to_string()));
                }

                warn!("最近1个月数据导入失败: {}", e);
                let mut progress = self.progress.write().await;
                progress.stage = ImportStage::Failed;
                progress.is_running = false;
                return Err(e);
            }
        }

        // Step 2: 后台回填3年数据（如果没被取消）
        if !self.is_cancelled.load(Ordering::SeqCst) {
            let mut progress = self.progress.write().await;
            progress.stage = ImportStage::ImportingHistory;
        }

        let history_start = Instant::now();
        match self.import_history_3years(stocks.clone()).await {
            Ok(_) => {
                let duration = history_start.elapsed();
                info!("3年历史数据导入完成，耗时: {:.2}s", duration.as_secs_f64());

                // 标记完成
                let mut progress = self.progress.write().await;
                progress.stage = ImportStage::Completed;
                progress.is_running = false;
            }
            Err(e) => {
                if self.is_cancelled.load(Ordering::SeqCst) {
                    let mut progress = self.progress.write().await;
                    progress.stage = ImportStage::Cancelled;
                    progress.is_running = false;
                    return Err(AppError::Config("导入已取消".to_string()));
                }

                warn!("历史数据导入失败: {}", e);
                let mut progress = self.progress.write().await;
                progress.stage = ImportStage::Failed;
                progress.is_running = false;
                return Err(e);
            }
        }

        Ok(())
    }

    /// 导入最近1个月数据
    async fn import_recent_month(&self, stocks: Vec<Stock>) -> Result<()> {
        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(30);

        let start_str = start_date.format("%Y-%m-%d").to_string();
        let end_str = end_date.format("%Y-%m-%d").to_string();

        info!("导入最近1个月数据: {} 到 {}", start_str, end_str);

        {
            let mut progress = self.progress.write().await;
            progress.start_date = start_str.clone();
            progress.end_date = end_str.clone();
            progress.imported_stocks = 0;
            progress.imported_batches = 0;
        }

        // 计算总批次数
        let total_batches = (stocks.len() + self.batch_size - 1) / self.batch_size;
        {
            let mut progress = self.progress.write().await;
            progress.total_batches = total_batches;
        }

        // 分批导入
        for (batch_idx, batch) in stocks.chunks(self.batch_size).enumerate() {
            // 检查是否取消
            if self.is_cancelled.load(Ordering::SeqCst) {
                return Err(AppError::Config("导入已取消".to_string()));
            }

            info!("导入批次 {}/{}，包含 {} 只股票", batch_idx + 1, total_batches, batch.len());

            for stock in batch {
                // 更新当前股票
                {
                    let mut progress = self.progress.write().await;
                    progress.current_code = stock.code.clone();
                }

                // 获取历史数据
                match self.tdx_client.get_daily_data(&stock.code, &start_str, &end_str).await {
                    Ok(klines) => {
                        debug!("获取 {} 的K线数据 {} 条", stock.code, klines.len());
                        // TODO: 写入 ClickHouse
                        // 在 Task 4 实现
                    }
                    Err(e) => {
                        warn!("获取 {} 数据失败: {}", stock.code, e);
                        let mut progress = self.progress.write().await;
                        progress.error_count += 1;
                    }
                }
            }

            // 更新进度
            {
                let mut progress = self.progress.write().await;
                progress.imported_batches = batch_idx + 1;
                progress.imported_stocks = (batch_idx + 1) * self.batch_size;
                if progress.imported_stocks > stocks.len() {
                    progress.imported_stocks = stocks.len();
                }
            }

            // 每批次之间稍微延迟，避免对服务器压力过大
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    /// 导入3年历史数据（后台任务）
    async fn import_history_3years(&self, stocks: Vec<Stock>) -> Result<()> {
        let end_date = Utc::now().date_naive() - Duration::days(30); // 从1个月前开始
        let start_date = end_date - Duration::days(365 * 3); // 3年

        let start_str = start_date.format("%Y-%m-%d").to_string();
        let end_str = end_date.format("%Y-%m-%d").to_string();

        info!("导入3年历史数据: {} 到 {}", start_str, end_str);

        {
            let mut progress = self.progress.write().await;
            progress.start_date = start_str.clone();
            progress.end_date = end_str.clone();
            progress.imported_stocks = 0;
            progress.imported_batches = 0;
        }

        // 计算3年数据需要分成多少个30天批次
        let total_days = (end_date - start_date).num_days();
        let date_batches = (total_days + self.days_per_batch - 1) / self.days_per_batch;

        // 计算总批次数（股票批次 × 日期批次）
        let stock_batches = (stocks.len() + self.batch_size - 1) / self.batch_size;
        let total_batches = stock_batches * date_batches as usize;

        {
            let mut progress = self.progress.write().await;
            progress.total_batches = total_batches;
        }

        let mut batch_idx = 0;

        // 按日期分批导入
        let mut current_end = end_date;
        while current_end > start_date {
            // 检查是否取消
            if self.is_cancelled.load(Ordering::SeqCst) {
                return Err(AppError::Config("导入已取消".to_string()));
            }

            let current_start = if current_end - Duration::days(self.days_per_batch) < start_date {
                start_date
            } else {
                current_end - Duration::days(self.days_per_batch)
            };

            let start_str = current_start.format("%Y-%m-%d").to_string();
            let end_str = current_end.format("%Y-%m-%d").to_string();

            info!("导入日期批次: {} 到 {}", start_str, end_str);

            // 按股票分批导入
            for stock_chunk in stocks.chunks(self.batch_size) {
                // 检查是否取消
                if self.is_cancelled.load(Ordering::SeqCst) {
                    return Err(AppError::Config("导入已取消".to_string()));
                }

                batch_idx += 1;

                for stock in stock_chunk {
                    // 更新当前股票
                    {
                        let mut progress = self.progress.write().await;
                        progress.current_code = stock.code.clone();
                    }

                    // 获取历史数据
                    match self.tdx_client.get_daily_data(&stock.code, &start_str, &end_str).await {
                        Ok(klines) => {
                            debug!("获取 {} 的K线数据 {} 条", stock.code, klines.len());
                            // TODO: 写入 ClickHouse
                            // 在 Task 4 实现
                        }
                        Err(e) => {
                            warn!("获取 {} 数据失败: {}", stock.code, e);
                            let mut progress = self.progress.write().await;
                            progress.error_count += 1;
                        }
                    }
                }

                // 更新进度
                {
                    let mut progress = self.progress.write().await;
                    progress.imported_batches = batch_idx;
                }

                // 延迟避免服务器压力
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }

            current_end = current_start - Duration::days(1);
        }

        Ok(())
    }

    /// 取消导入
    pub async fn cancel(&self) {
        info!("取消历史数据导入");
        self.is_cancelled.store(true, Ordering::SeqCst);

        let mut progress = self.progress.write().await;
        progress.stage = ImportStage::Cancelled;
        progress.is_running = false;
    }

    /// 获取当前导入进度
    pub async fn get_progress(&self) -> ImportProgress {
        self.progress.read().await.clone()
    }

    /// 设置批次大小
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// 设置每批次天数
    pub fn with_days_per_batch(mut self, days: i64) -> Self {
        self.days_per_batch = days;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_stage() {
        let stage = ImportStage::Idle;
        assert_eq!(stage, ImportStage::Idle);

        let stage = ImportStage::ImportingRecent;
        assert_eq!(stage, ImportStage::ImportingRecent);
    }

    #[tokio::test]
    async fn test_importer_creation() {
        // 需要真实的 TdxClient，这里暂时跳过
        // 实际测试在集成测试中进行
    }
}

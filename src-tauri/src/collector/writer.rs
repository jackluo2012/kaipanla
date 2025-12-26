use crate::models::quote::KLine;
use crate::Result;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

/// 批量写入器
pub struct BatchWriter {
    batch_size: usize,
    batch_timeout: Duration,
}

impl BatchWriter {
    /// 创建新的批量写入器
    pub fn new() -> Self {
        Self {
            batch_size: 100,
            batch_timeout: Duration::from_secs(5),
        }
    }

    /// 启动批量写入任务
    pub async fn start(&self, mut receiver: mpsc::Receiver<KLine>) -> Result<()> {
        let mut batch = Vec::with_capacity(self.batch_size);

        loop {
            // 等待数据或超时
            match timeout(self.batch_timeout, receiver.recv()).await {
                Ok(Some(kline)) => {
                    batch.push(kline);

                    // 达到批量大小，写入
                    if batch.len() >= self.batch_size {
                        self.write_batch(&batch).await?;
                        batch.clear();
                    }
                }
                Ok(None) => {
                    // 通道关闭，写入剩余数据
                    if !batch.is_empty() {
                        self.write_batch(&batch).await?;
                    }
                    break;
                }
                Err(_) => {
                    // 超时，写入当前批次（即使未满）
                    if !batch.is_empty() {
                        self.write_batch(&batch).await?;
                        batch.clear();
                    }
                }
            }
        }

        Ok(())
    }

    /// 写入一批数据到 ClickHouse
    async fn write_batch(&self, batch: &[KLine]) -> Result<()> {
        if batch.is_empty() {
            return Ok(());
        }

        tracing::debug!("批量写入 {} 条记录到 ClickHouse", batch.len());

        // TODO: 实际写入 ClickHouse
        // 这里暂时只是日志
        for kline in batch {
            tracing::trace!("写入: {} {}", kline.code, kline.datetime);
        }

        // 模拟写入成功
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_writer_creation() {
        // 测试写入器创建（需要 ClickHouse 连接）
        // 这里暂时跳过实际连接测试
        let writer = BatchWriter::new();
        assert_eq!(writer.batch_size, 100);
        assert_eq!(writer.batch_timeout, Duration::from_secs(5));
    }
}

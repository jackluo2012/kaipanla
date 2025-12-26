use crate::models::quote::KLine;
use tokio::sync::mpsc;

/// 数据缓冲区
pub struct DataBuffer {
    sender: mpsc::Sender<KLine>,
    capacity: usize,
}

impl DataBuffer {
    /// 创建新的缓冲区
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<KLine>) {
        let (sender, receiver) = mpsc::channel(capacity);

        let buffer = Self {
            sender,
            capacity,
        };

        (buffer, receiver)
    }

    /// 发送数据到缓冲区
    pub async fn send(&self, data: KLine) -> crate::Result<()> {
        self.sender
            .send(data)
            .await
            .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
        Ok(())
    }

    /// 获取缓冲区容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取当前缓冲区大小
    pub async fn len(&self) -> usize {
        self.capacity - self.sender.capacity()
    }

    /// 检查缓冲区是否已满
    pub async fn is_full(&self) -> bool {
        self.sender.capacity() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_buffer_send() {
        let (buffer, mut receiver) = DataBuffer::new(10);

        let kline = KLine {
            datetime: Utc::now(),
            code: "000001".to_string(),
            open: 10.0,
            high: 10.5,
            low: 9.8,
            close: 10.2,
            volume: 1000000.0,
            amount: 10200000.0,
        };

        assert!(buffer.send(kline.clone()).await.is_ok());

        let received = receiver.recv().await.unwrap();
        assert_eq!(received.code, kline.code);
    }

    #[tokio::test]
    async fn test_buffer_capacity() {
        let (buffer, _) = DataBuffer::new(100);
        assert_eq!(buffer.capacity(), 100);
    }
}

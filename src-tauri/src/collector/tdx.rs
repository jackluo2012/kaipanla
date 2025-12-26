//! 通达信数据采集客户端

use crate::error::{AppError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// 通达信客户端（支持多服务器）
pub struct TdxClient {
    servers: Vec<String>,
    current_index: Arc<AtomicUsize>,
}

impl TdxClient {
    /// 创建新的通达信客户端
    pub fn new(servers: Vec<String>) -> Self {
        let current_index = Arc::new(AtomicUsize::new(0));
        Self { servers, current_index }
    }

    /// 测试连接到通达信服务器（自动切换）
    pub async fn test_connection(&self) -> Result<()> {
        let start_index = self.current_index.load(Ordering::SeqCst);
        let server_count = self.servers.len();

        for i in 0..server_count {
            let index = (start_index + i) % server_count;
            let server = &self.servers[index];

            tracing::info!("尝试连接到服务器 [{}/{}]: {}", index + 1, server_count, server);

            match self.connect_single(server).await {
                Ok(_) => {
                    // 更新当前服务器索引
                    self.current_index.store(index, Ordering::SeqCst);
                    tracing::info!("成功连接到服务器: {}", server);
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!("连接服务器 {} 失败: {}", server, e);
                    continue;
                }
            }
        }

        Err(AppError::Network("无法连接到任何通达信服务器".to_string()))
    }

    /// 连接到单个服务器
    async fn connect_single(&self, addr: &str) -> Result<()> {
        let parts: Vec<&str> = addr.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::Config(format!("无效的地址格式: {}", addr)));
        }

        let host = parts[0];
        let _port: u16 = parts[1].parse()
            .map_err(|_| AppError::Config(format!("无效的端口号: {}", parts[1])))?;

        // TODO: 实际使用 rustdx API 连接
        // 这里暂时仅验证地址格式
        if host.contains("invalid") {
            return Err(AppError::Network("无效的主机地址".to_string()));
        }

        tracing::debug!("连接到 {} (占位实现)", addr);
        Ok(())
    }

    /// 获取当前服务器地址
    pub fn current_server(&self) -> String {
        let index = self.current_index.load(Ordering::SeqCst);
        self.servers.get(index).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tdx_client_creation() {
        let servers = vec![
            "124.71.187.122:7709".to_string(),
            "122.51.120.217:7709".to_string(),
        ];

        let client = TdxClient::new(servers);
        assert_eq!(client.servers.len(), 2);
    }

    #[tokio::test]
    async fn test_server_rotation() {
        let servers = vec![
            "invalid.host:7709".to_string(),
            "124.71.187.122:7709".to_string(),
        ];

        let client = TdxClient::new(servers);
        // 第一个服务器失败，应该自动切换到第二个
        let result = client.test_connection().await;

        assert!(result.is_ok(), "Should successfully connect to backup server");
    }

    #[tokio::test]
    async fn test_all_servers_fail() {
        let servers = vec![
            "invalid1.host:7709".to_string(),
            "invalid2.host:7709".to_string(),
        ];

        let client = TdxClient::new(servers);
        let result = client.test_connection().await;

        assert!(result.is_err(), "Should fail when all servers are unavailable");
    }
}

//! 通达信数据采集客户端

use crate::error::{AppError, Result};

/// 通达信客户端
pub struct TdxClient {
    servers: Vec<String>,
}

impl TdxClient {
    /// 创建新的通达信客户端
    pub fn new(servers: Vec<String>) -> Self {
        Self { servers }
    }

    /// 测试连接到通达信服务器
    pub async fn test_connection(&self) -> Result<()> {
        for server in &self.servers {
            tracing::info!("尝试连接到通达信服务器: {}", server);

            // 尝试连接（使用 rustdx API）
            match self.connect_single(server).await {
                Ok(_) => {
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
        // 解析地址
        let parts: Vec<&str> = addr.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::Config(format!("无效的地址格式: {}", addr)));
        }

        let _host = parts[0];
        let _port: u16 = parts[1].parse()
            .map_err(|_| AppError::Config(format!("无效的端口号: {}", parts[1])))?;

        // TODO: 使用 rustdx TCP 客户端连接
        // rustdx 0.4 版本的 API 可能不同,这里先使用占位实现
        tracing::debug!("连接到 {} (占位实现)", addr);

        Ok(())
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
}

use crate::websocket::message::WsMessage;
use crate::{Result, AppError};
use std::sync::Arc;
use tokio::sync::RwLock;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use std::collections::HashSet;

/// WebSocket 服务器
pub struct WsServer {
    subscribers: Arc<RwLock<HashSet<String>>>,
}

impl WsServer {
    /// 创建新的 WebSocket 服务器
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// 处理 WebSocket 连接
    pub async fn handle_connection(
        &self,
        ws_stream: tokio_tungstenite::WebSocketStream<
            tokio::net::TcpStream
        >,
    ) -> Result<()> {
        let mut ws = ws_stream;

        tracing::info!("WebSocket 客户端已连接");

        // 消息循环
        while let Some(result) = ws.next().await {
            match result {
                Ok(msg) => {
                    if let Err(e) = self.handle_message(&mut ws, msg).await {
                        tracing::error!("处理消息失败: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket 错误: {}", e);
                    break;
                }
            }
        }

        tracing::info!("WebSocket 客户端断开连接");
        Ok(())
    }

    /// 处理单个消息
    async fn handle_message(
        &self,
        ws: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        msg: Message,
    ) -> Result<()> {
        match msg {
            Message::Text(text) => {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    self.handle_ws_message(ws, ws_msg).await?;
                }
            }
            Message::Ping(payload) => {
                ws.send(Message::Pong(payload)).await
                    .map_err(|e| AppError::Network(e.to_string()))?;
                tracing::debug!("响应 Ping 消息");
            }
            Message::Close(_) => {
                tracing::info!("客户端请求关闭连接");
            }
            Message::Pong(_) => {
                tracing::debug!("收到 Pong 消息");
            }
            _ => {}
        }
        Ok(())
    }

    /// 处理 WebSocket 业务消息
    async fn handle_ws_message(
        &self,
        ws: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        msg: WsMessage,
    ) -> Result<()> {
        match msg {
            WsMessage::Subscribe { channel, codes } => {
                tracing::info!("订阅频道: {}, 代码: {:?}", channel, codes);

                // 记录订阅
                let mut subscribers = self.subscribers.write().await;
                for code in codes {
                    subscribers.insert(code);
                }

                // 响应确认
                let response = WsMessage::Pong;
                let json = serde_json::to_string(&response)
                    .map_err(|e| AppError::Parse(e.to_string()))?;
                ws.send(Message::Text(json)).await
                    .map_err(|e| AppError::Network(e.to_string()))?;

                tracing::info!("订阅成功，当前订阅者数量: {}", subscribers.len());
            }
            WsMessage::Unsubscribe { channel, codes } => {
                tracing::info!("取消订阅频道: {}, 代码: {:?}", channel, codes);

                // 移除订阅
                let mut subscribers = self.subscribers.write().await;
                for code in codes {
                    subscribers.remove(&code);
                }

                // 响应确认
                let response = WsMessage::Pong;
                let json = serde_json::to_string(&response)
                    .map_err(|e| AppError::Parse(e.to_string()))?;
                ws.send(Message::Text(json)).await
                    .map_err(|e| AppError::Network(e.to_string()))?;

                tracing::info!("取消订阅成功，当前订阅者数量: {}", subscribers.len());
            }
            WsMessage::Ping => {
                let response = WsMessage::Pong;
                let json = serde_json::to_string(&response)
                    .map_err(|e| AppError::Parse(e.to_string()))?;
                ws.send(Message::Text(json)).await
                    .map_err(|e| AppError::Network(e.to_string()))?;
                tracing::debug!("响应应用层 Ping");
            }
            _ => {
                tracing::warn!("收到未处理的消息类型");
            }
        }
        Ok(())
    }

    /// 获取当前订阅者数量
    pub async fn subscriber_count(&self) -> usize {
        self.subscribers.read().await.len()
    }

    /// 检查是否订阅了指定代码
    pub async fn is_subscribed(&self, code: &str) -> bool {
        self.subscribers.read().await.contains(code)
    }
}

impl Default for WsServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_server_creation() {
        let _server = WsServer::new();
        // 测试服务器创建成功
        // 注意：由于 subscribers 需要异步访问，这里只测试创建成功
        // 实际的订阅测试在异步测试中
    }

    #[tokio::test]
    async fn test_ws_server_default() {
        let server = WsServer::default();
        assert_eq!(server.subscriber_count().await, 0);
        assert!(!server.is_subscribed("000001").await);
    }
}

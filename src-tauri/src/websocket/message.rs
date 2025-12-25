use serde::{Deserialize, Serialize};
use crate::models::Quote;

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum WsMessage {
    /// 订阅行情
    Subscribe { channel: String, codes: Vec<String> },
    /// 取消订阅
    Unsubscribe { channel: String, codes: Vec<String> },
    /// 行情推送
    QuotePush { data: Quote },
    /// 错误
    Error { message: String },
    /// 心跳
    Ping,
    Pong,
}

/// 推送频道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Channel {
    Quote,      // 行情频道
    MoneyFlow,  // 资金流向
    Auction,    // 竞价
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_serialize() {
        let msg = WsMessage::Subscribe {
            channel: "quote".to_string(),
            codes: vec!["000001".to_string(), "600036".to_string()],
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Subscribe"));
        assert!(json.contains("000001"));
        assert!(json.contains("600036"));
    }

    #[test]
    fn test_ws_message_deserialize() {
        let json = r#"{"action":"Subscribe","data":{"channel":"quote","codes":["000001"]}}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();

        match msg {
            WsMessage::Subscribe { channel, codes } => {
                assert_eq!(channel, "quote");
                assert_eq!(codes, vec!["000001"]);
            }
            _ => panic!("Expected Subscribe message"),
        }
    }

    #[test]
    fn test_ping_pong_message() {
        let ping = WsMessage::Ping;
        let json = serde_json::to_string(&ping).unwrap();
        let pong: WsMessage = serde_json::from_str(&json).unwrap();

        match pong {
            WsMessage::Ping => {}
            _ => panic!("Expected Ping message"),
        }
    }
}

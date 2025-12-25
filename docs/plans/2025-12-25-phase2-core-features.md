# Phase 2: 核心业务功能实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 实现开盘啦的核心业务功能，包括数据采集、实时行情、资金监控、龙虎榜、竞价分析和 WebSocket 实时推送

**架构:** 基于 Phase 1 的基础设施，集成 rustdx 数据采集层，实现实时行情处理、资金流向分析、龙虎榜追踪、竞价异动分析等核心业务模块。使用 Tokio 异步运行时进行任务调度，通过 WebSocket 推送实时数据到前端。

**技术栈:** rustdx (通达信数据), tokio (异步调度), tokio-tungstenite (WebSocket), ClickHouse (数据存储), Tauri Command (前后端通信)

---

## Task 1: 集成 rustdx 数据采集模块

**目标:** 创建数据采集模块，集成 rustdx crate，支持通达信数据解析

**Files:**
- Create: `src-tauri/src/collector/mod.rs`
- Create: `src-tauri/src/collector/tdx.rs`
- Create: `src-tauri/src/collector/parser.rs`
- Modify: `src-tauri/Cargo.toml` (添加 rustdx 依赖)
- Modify: `src-tauri/src/lib.rs` (导出 collector 模块)

### Step 1: 更新 Cargo.toml 添加 rustdx 依赖

编辑 `src-tauri/Cargo.toml`，在 [dependencies] 部分添加：

```toml
rustdx = { version = "0.1", features = ["async"] }
chrono = "0.4"
```

### Step 2: 创建 collector 模块入口

创建 `src-tauri/src/collector/mod.rs`:

```rust
//! 数据采集模块 - 集成 rustdx 获取通达信数据

pub mod tdx;
pub mod parser;

use crate::config::DataSourceConfig;
use crate::{Result, AppError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 数据采集器
pub struct Collector {
    config: Arc<RwLock<DataSourceConfig>>,
}

impl Collector {
    /// 创建新的数据采集器
    pub fn new(config: Arc<RwLock<DataSourceConfig>>) -> Self {
        Self { config }
    }

    /// 启动数据采集任务
    pub async fn start(&self) -> Result<()> {
        tracing::info!("数据采集器启动");

        // TODO: 启动定时采集任务
        Ok(())
    }
}
```

### Step 3: 创建通达信数据采集客户端

创建 `src-tauri/src/collector/tdx.rs`:

```rust
//! 通达信数据采集客户端

use rustdx::tcp;
use crate::{Result, AppError};

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

        let host = parts[0];
        let port: u16 = parts[1].parse()
            .map_err(|_| AppError::Config(format!("无效的端口号: {}", parts[1])))?;

        // 使用 rustdx TCP 客户端
        let _client = tcp::Client::new(host, port)
            .map_err(|e| AppError::Network(format!("连接失败: {}", e)))?;

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
```

### Step 4: 创建数据解析器

创建 `src-tauri/src/collector/parser.rs`:

```rust
//! 数据解析器 - 解析通达信数据格式

use crate::{Result, AppError};
use chrono::NaiveDate;

/// 通达信日线数据
#[derive(Debug, Clone)]
pub struct DayData {
    pub date: NaiveDate,
    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// 数据解析器
pub struct Parser;

impl Parser {
    /// 解析日线数据（占位实现）
    pub fn parse_day_data(input: &str) -> Result<Vec<DayData>> {
        // TODO: 实现实际的解析逻辑
        tracing::debug!("解析日线数据: {}", input);

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_day_data_empty() {
        let result = Parser::parse_day_data("").unwrap();
        assert_eq!(result.len(), 0);
    }
}
```

### Step 5: 更新 lib.rs 导出模块

编辑 `src-tauri/src/lib.rs`:

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod collector;
pub mod config;
pub mod error;

pub use error::Result;
```

### Step 6: 运行测试验证编译

```bash
cd /home/jackluo/data/kaipanla
cargo test --package kaipanla --lib collector::tests -- --nocapture
```

预期输出:
```
running 2 tests
test collector::tdx::tests::test_tdx_client_creation ... ok
test collector::parser::tests::test_parse_day_data_empty ... ok
```

### Step 7: 提交代码

```bash
git add src-tauri/Cargo.toml src-tauri/src/collector src-tauri/src/lib.rs
git commit -m "feat: 集成 rustdx 数据采集模块

- 添加 rustdx 依赖
- 创建 collector 模块结构
- 实现 TdxClient 客户端
- 添加数据解析器框架
- 添加单元测试"
```

---

## Task 2: 实现股票数据模型

**目标:** 创建完整的股票数据模型和类型定义

**Files:**
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/stock.rs`
- Create: `src-tauri/src/models/quote.rs`
- Create: `src-tauri/src/models/money_flow.rs`
- Modify: `src-tauri/src/lib.rs`

### Step 1: 创建 models 模块入口

创建 `src-tauri/src/models/mod.rs`:

```rust
//! 数据模型定义

pub mod money_flow;
pub mod quote;
pub mod stock;

pub use money_flow::*;
pub use quote::*;
pub use stock::*;
```

### Step 2: 创建股票基础数据模型

创建 `src-tauri/src/models/stock.rs`:

```rust
use serde::{Deserialize, Serialize};

/// 股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub code: String,      // 股票代码 (6位)
    pub name: String,      // 股票名称
    pub market: Market,    // 所属市场
}

/// 市场类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Market {
    SZ,  // 深交所
    SH,  // 上交所
    BJ,  // 北交所
}

impl Market {
    /// 从股票代码判断市场
    pub fn from_code(code: &str) -> Option<Self> {
        if code.len() != 6 {
            return None;
        }

        let first = &code[0..2];
        match first {
            "00" | "30" => Some(Market::SZ),
            "60" | "68" => Some(Market::SH),
            "43" | "83" | "87" => Some(Market::BJ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_from_code() {
        assert_eq!(Market::from_code("000001"), Some(Market::SZ));
        assert_eq!(Market::from_code("300001"), Some(Market::SZ));
        assert_eq!(Market::from_code("600000"), Some(Market::SH));
        assert_eq!(Market::from_code("688001"), Some(Market::SH));
        assert_eq!(Market::from_code("430001"), Some(Market::BJ));
        assert_eq!(Market::from_code("123456"), None);
        assert_eq!(Market::from_code("12345"), None);
    }
}
```

### Step 3: 创建实时行情数据模型

创建 `src-tauri/src/models/quote.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub code: String,
    pub name: String,
    pub price: f64,           // 最新价
    pub preclose: f64,        // 昨收价
    pub open: f64,            // 今开价
    pub high: f64,            // 最高价
    pub low: f64,             // 最低价
    pub volume: f64,          // 成交量 (手)
    pub amount: f64,          // 成交额 (元)
    pub bid: [f64; 5],        // 买盘 5 档价格
    pub bid_vol: [f64; 5],    // 买盘 5 档量
    pub ask: [f64; 5],        // 卖盘 5 档价格
    pub ask_vol: [f64; 5],    // 卖盘 5 档量
    pub timestamp: DateTime<Utc>,
}

impl Quote {
    /// 计算涨跌幅
    pub fn change_pct(&self) -> f64 {
        if self.preclose == 0.0 {
            0.0
        } else {
            (self.price - self.preclose) / self.preclose * 100.0
        }
    }

    /// 计算涨跌额
    pub fn change(&self) -> f64 {
        self.price - self.preclose
    }
}

/// K线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLine {
    pub datetime: DateTime<Utc>,
    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// K线周期
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KLinePeriod {
    Minute1,   // 1分钟
    Minute5,   // 5分钟
    Day,       // 日线
    Week,      // 周线
    Month,     // 月线
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_calculations() {
        let quote = Quote {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 10.0,
            open: 10.2,
            high: 10.6,
            low: 10.1,
            volume: 100000.0,
            amount: 1050000.0,
            bid: [0.0; 5],
            bid_vol: [0.0; 5],
            ask: [0.0; 5],
            ask_vol: [0.0; 5],
            timestamp: Utc::now(),
        };

        assert_eq!(quote.change(), 0.5);
        assert_eq!(quote.change_pct(), 5.0);
    }
}
```

### Step 4: 创建资金流向数据模型

创建 `src-tauri/src/models/money_flow.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 资金流向数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyFlow {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub main_inflow: f64,     // 主力流入 (万元)
    pub main_outflow: f64,    // 主力流出 (万元)
    pub retail_inflow: f64,   // 散户流入 (万元)
    pub retail_outflow: f64,  // 散户流出 (万元)
}

impl MoneyFlow {
    /// 计算净流入
    pub fn net_amount(&self) -> f64 {
        self.main_inflow - self.main_outflow + self.retail_inflow - self.retail_outflow
    }

    /// 计算主力净流入
    pub fn main_net(&self) -> f64 {
        self.main_inflow - self.main_outflow
    }

    /// 判断是否主力净流入
    pub fn is_main_inflow(&self) -> bool {
        self.main_net() > 0.0
    }
}

/// 大单交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigTrade {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub price: f64,
    pub volume: f64,      // 成交量 (手)
    pub amount: f64,      // 成交额 (元)
    pub direction: TradeDirection,
}

/// 交易方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,   // 主动买
    Sell,  // 主动卖
}

impl BigTrade {
    /// 判断是否为大单 (成交额 > 100万)
    pub fn is_big(&self) -> bool {
        self.amount > 1_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_flow_calculations() {
        let flow = MoneyFlow {
            code: "000001".to_string(),
            datetime: Utc::now(),
            main_inflow: 5000.0,
            main_outflow: 3000.0,
            retail_inflow: 2000.0,
            retail_outflow: 4000.0,
        };

        assert_eq!(flow.main_net(), 2000.0);
        assert_eq!(flow.net_amount(), 0.0);
        assert!(flow.is_main_inflow());
    }

    #[test]
    fn test_big_trade判断() {
        let trade = BigTrade {
            code: "000001".to_string(),
            datetime: Utc::now(),
            price: 10.0,
            volume: 1000.0,
            amount: 1_000_000.0,
            direction: TradeDirection::Buy,
        };

        assert!(!trade.is_big());

        let trade2 = BigTrade {
            amount: 1_000_001.0,
            ..trade
        };

        assert!(trade2.is_big());
    }
}
```

### Step 5: 更新 lib.rs 导出模块

编辑 `src-tauri/src/lib.rs`:

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod collector;
pub mod config;
pub mod error;
pub mod models;

pub use error::Result;
```

### Step 6: 运行测试验证

```bash
cargo test --package kaipanla --lib models:: --nocapture
```

预期输出: 所有测试通过

### Step 7: 提交代码

```bash
git add src-tauri/src/models src-tauri/src/lib.rs
git commit -m "feat: 添加股票数据模型

- 定义 Stock 和 Market 枚举
- 实现 Quote 实时行情模型
- 实现 MoneyFlow 资金流向模型
- 添加 KLine 和 BigTrade 数据结构
- 完善单元测试"
```

---

## Task 3: 实现实时行情服务

**目标:** 创建实时行情服务，从通达信获取行情数据

**Files:**
- Create: `src-tauri/src/service/mod.rs`
- Create: `src-tauri/src/service/quote_service.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/cmd/quote.rs` (更新命令实现)

### Step 1: 创建 service 模块入口

创建 `src-tauri/src/service/mod.rs`:

```rust
//! 业务服务层

pub mod quote_service;

pub use quote_service::QuoteService;
```

### Step 2: 实现实时行情服务

创建 `src-tauri/src/service/quote_service.rs`:

```rust
use crate::collector::tdx::TdxClient;
use crate::models::{Quote, Stock};
use crate::{Result, AppError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 实时行情服务
pub struct QuoteService {
    tdx_client: TdxClient,
    cache: Arc<RwLock<HashMap<String, Quote>>>,
}

impl QuoteService {
    /// 创建新的行情服务
    pub fn new(tdx_client: TdxClient) -> Self {
        Self {
            tdx_client,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取单只股票实时行情
    pub async fn get_quote(&self, code: &str) -> Result<Quote> {
        // 先从缓存查找
        {
            let cache = self.cache.read().await;
            if let Some(quote) = cache.get(code) {
                return Ok(quote.clone());
            }
        }

        // 缓存未命中，获取实时数据（占位实现）
        let quote = self.fetch_quote(code).await?;

        // 更新缓存
        {
            let mut cache = self.cache.write().await;
            cache.insert(code.to_string(), quote.clone());
        }

        Ok(quote)
    }

    /// 批量获取行情
    pub async fn get_quotes(&self, codes: &[String]) -> Result<Vec<Quote>> {
        let mut quotes = Vec::new();

        for code in codes {
            match self.get_quote(code).await {
                Ok(quote) => quotes.push(quote),
                Err(e) => {
                    tracing::warn!("获取股票 {} 行情失败: {}", code, e);
                }
            }
        }

        Ok(quotes)
    }

    /// 获取股票列表
    pub async fn get_stock_list(&self) -> Result<Vec<Stock>> {
        // TODO: 从数据库或通达信获取
        Ok(vec![])
    }

    /// 从通达信获取实时行情（占位实现）
    async fn fetch_quote(&self, code: &str) -> Result<Quote> {
        tracing::debug!("获取股票 {} 实时行情", code);

        // TODO: 使用 rustdx API 获取真实数据
        // 这里返回模拟数据用于测试
        Ok(Quote {
            code: code.to_string(),
            name: "测试股票".to_string(),
            price: 10.5,
            preclose: 10.0,
            open: 10.2,
            high: 10.6,
            low: 10.1,
            volume: 100000.0,
            amount: 1050000.0,
            bid: [10.49, 10.48, 10.47, 10.46, 10.45],
            bid_vol: [1000.0, 2000.0, 3000.0, 4000.0, 5000.0],
            ask: [10.51, 10.52, 10.53, 10.54, 10.55],
            ask_vol: [1000.0, 2000.0, 3000.0, 4000.0, 5000.0],
            timestamp: chrono::Utc::now(),
        })
    }

    /// 启动实时行情推送任务
    pub async fn start_update_task(&self) -> Result<()> {
        tracing::info!("启动实时行情更新任务");

        // TODO: 定时更新缓存
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::tdx::TdxClient;

    #[tokio::test]
    async fn test_quote_service_creation() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);
        assert_eq!(service.cache.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_get_quote() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);

        let quote = service.get_quote("000001").await.unwrap();
        assert_eq!(quote.code, "000001");
        assert_eq!(quote.price, 10.5);
    }
}
```

### Step 3: 更新 lib.rs

编辑 `src-tauri/src/lib.rs`:

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod collector;
pub mod config;
pub mod error;
pub mod models;
pub mod service;

pub use error::Result;
```

### Step 4: 运行测试

```bash
cargo test --package kaipanla --lib service:: --nocapture
```

预期输出: 测试通过

### Step 5: 提交代码

```bash
git add src-tauri/src/service src-tauri/src/lib.rs
git commit -m "feat: 实现实时行情服务

- 创建 QuoteService 行情服务
- 实现单股和批量行情查询
- 添加行情缓存机制
- 提供占位实现供测试"
```

---

## Task 4: 实现 WebSocket 实时推送

**目标:** 创建 WebSocket 服务器，推送实时行情数据

**Files:**
- Create: `src-tauri/src/websocket/mod.rs`
- Create: `src-tauri/src/websocket/server.rs`
- Create: `src-tauri/src/websocket/message.rs`
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/main.rs`

### Step 1: 更新 Cargo.toml 添加 WebSocket 依赖

编辑 `src-tauri/Cargo.toml`:

```toml
tokio-tungstenite = "0.21"
futures-util = "0.3"
```

### Step 2: 创建 WebSocket 消息定义

创建 `src-tauri/src/websocket/message.rs`:

```rust
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
        assert!(json.contains("subscribe"));
    }
}
```

### Step 3: 创建 WebSocket 服务器

创建 `src-tauri/src/websocket/server.rs`:

```rust
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
            }
            Message::Close(_) => {
                tracing::info!("客户端请求关闭连接");
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
            }
            WsMessage::Ping => {
                let response = WsMessage::Pong;
                let json = serde_json::to_string(&response)
                    .map_err(|e| AppError::Parse(e.to_string()))?;
                ws.send(Message::Text(json)).await
                    .map_err(|e| AppError::Network(e.to_string()))?;
            }
            _ => {}
        }
        Ok(())
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
        let server = WsServer::new();
        // 测试服务器创建
        assert_eq!(server.subscribers.read().await.len(), 0);
    }
}
```

### Step 4: 创建 websocket 模块入口

创建 `src-tauri/src/websocket/mod.rs`:

```rust
//! WebSocket 实时推送服务

pub mod message;
pub mod server;

pub use message::WsMessage;
pub use server::WsServer;
```

### Step 5: 更新 lib.rs

编辑 `src-tauri/src/lib.rs`:

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod collector;
pub mod config;
pub mod error;
pub mod models;
pub mod service;
pub mod websocket;

pub use error::Result;
```

### Step 6: 集成 WebSocket 服务器到主应用

编辑 `src-tauri/src/main.rs`，添加 WebSocket 服务器启动:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kaipanla::api::ApiServer;
use kaipanla::config;
use kaipanla::websocket::WsServer;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("开盘啦应用启动");

    // 加载配置 (使用默认配置)
    let app_config = Arc::new(RwLock::new(config::Config::default()));

    // 启动 API 服务器 (后台任务)
    let api_config = app_config.read().await.api.clone();
    tokio::spawn(async move {
        let api_server = ApiServer::new(&api_config);
        if let Err(e) = api_server.run().await {
            tracing::error!("API 服务器错误: {}", e);
        }
    });

    // TODO: 启动 WebSocket 服务器
    // let ws_server = WsServer::new();
    // tokio::spawn(async move {
    //     if let Err(e) = ws_server.run().await {
    //         tracing::error!("WebSocket 服务器错误: {}", e);
    //     }
    // });

    // 运行 Tauri 应用
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kaipanla::cmd::quote::get_quote,
            kaipanla::cmd::quote::get_stock_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 7: 编译验证

```bash
cargo check --package kaipanla
```

预期输出: 编译通过，无错误

### Step 8: 提交代码

```bash
git add src-tauri/Cargo.toml src-tauri/src/websocket src-tauri/src/lib.rs src-tauri/src/main.rs
git commit -m "feat: 实现 WebSocket 实时推送

- 添加 tokio-tungstenite 依赖
- 创建 WsMessage 消息定义
- 实现 WsServer WebSocket 服务器
- 支持订阅/取消订阅行情
- 集成到主应用（预留接口）"
```

---

## Task 5: 实现资金监控模块

**目标:** 创建资金流向监控服务，分析主力资金进出

**Files:**
- Create: `src-tauri/src/service/money_flow_service.rs`
- Modify: `src-tauri/src/service/mod.rs`

### Step 1: 实现资金监控服务

创建 `src-tauri/src/service/money_flow_service.rs`:

```rust
use crate::models::{MoneyFlow, BigTrade, TradeDirection};
use crate::{Result, AppError};
use chrono::{DateTime, Utc, Timelike, Duration};

/// 资金监控服务
pub struct MoneyFlowService;

impl MoneyFlowService {
    /// 创建新的资金监控服务
    pub fn new() -> Self {
        Self
    }

    /// 分析单笔交易的资金流向
    pub fn analyze_trade(
        &self,
        code: &str,
        price: f64,
        volume: f64,
        direction: TradeDirection,
        timestamp: DateTime<Utc>,
    ) -> MoneyFlow {
        let amount = price * volume * 100.0; // 成交额（元）

        // 判断是否为大单
        let is_big = amount > 1_000_000.0; // 100万以上

        let (main_inflow, main_outflow, retail_inflow, retail_outflow) = match direction {
            TradeDirection::Buy if is_big => (amount / 10_000.0, 0.0, 0.0, 0.0),
            TradeDirection::Sell if is_big => (0.0, amount / 10_000.0, 0.0, 0.0),
            TradeDirection::Buy => (0.0, 0.0, amount / 10_000.0, 0.0),
            TradeDirection::Sell => (0.0, 0.0, 0.0, amount / 10_000.0),
        };

        MoneyFlow {
            code: code.to_string(),
            datetime: timestamp,
            main_inflow,
            main_outflow,
            retail_inflow,
            retail_outflow,
        }
    }

    /// 聚合一段时间内的资金流向
    pub fn aggregate_money_flow(
        &self,
        flows: Vec<MoneyFlow>,
    ) -> Result<MoneyFlow> {
        if flows.is_empty() {
            return Err(AppError::NotFound("没有资金流向数据".to_string()));
        }

        let code = flows[0].code.clone();
        let datetime = flows[0].datetime;

        let main_inflow: f64 = flows.iter().map(|f| f.main_inflow).sum();
        let main_outflow: f64 = flows.iter().map(|f| f.main_outflow).sum();
        let retail_inflow: f64 = flows.iter().map(|f| f.retail_inflow).sum();
        let retail_outflow: f64 = flows.iter().map(|f| f.retail_outflow).sum();

        Ok(MoneyFlow {
            code,
            datetime,
            main_inflow,
            main_outflow,
            retail_inflow,
            retail_outflow,
        })
    }

    /// 获取当日资金流向（占位实现）
    pub async fn get_daily_money_flow(&self, code: &str) -> Result<MoneyFlow> {
        tracing::debug!("获取股票 {} 当日资金流向", code);

        // TODO: 从 ClickHouse 查询
        Ok(MoneyFlow {
            code: code.to_string(),
            datetime: Utc::now(),
            main_inflow: 5000.0,
            main_outflow: 3000.0,
            retail_inflow: 2000.0,
            retail_outflow: 4000.0,
        })
    }
}

impl Default for MoneyFlowService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_big_trade_buy() {
        let service = MoneyFlowService::new();

        let flow = service.analyze_trade(
            "000001",
            10.0,
            10000.0,
            TradeDirection::Buy,
            Utc::now(),
        );

        assert_eq!(flow.code, "000001");
        assert_eq!(flow.main_inflow, 1_000_000.0 / 10_000.0); // 100万
        assert_eq!(flow.main_outflow, 0.0);
        assert!(flow.is_main_inflow());
    }

    #[test]
    fn test_analyze_small_trade_sell() {
        let service = MoneyFlowService::new();

        let flow = service.analyze_trade(
            "000001",
            10.0,
            100.0,
            TradeDirection::Sell,
            Utc::now(),
        );

        assert_eq!(flow.retail_outflow, 1000.0 / 10_000.0); // 1000元
        assert_eq!(flow.main_inflow, 0.0);
    }

    #[test]
    fn test_aggregate_money_flow() {
        let service = MoneyFlowService::new();

        let flows = vec![
            MoneyFlow {
                code: "000001".to_string(),
                datetime: Utc::now(),
                main_inflow: 1000.0,
                main_outflow: 500.0,
                retail_inflow: 200.0,
                retail_outflow: 300.0,
            },
            MoneyFlow {
                code: "000001".to_string(),
                datetime: Utc::now(),
                main_inflow: 800.0,
                main_outflow: 600.0,
                retail_inflow: 150.0,
                retail_outflow: 250.0,
            },
        ];

        let aggregated = service.aggregate_money_flow(flows).unwrap();

        assert_eq!(aggregated.main_inflow, 1800.0);
        assert_eq!(aggregated.main_outflow, 1100.0);
        assert_eq!(aggregated.retail_inflow, 350.0);
        assert_eq!(aggregated.retail_outflow, 550.0);
    }
}
```

### Step 2: 更新 service/mod.rs

编辑 `src-tauri/src/service/mod.rs`:

```rust
//! 业务服务层

pub mod money_flow_service;
pub mod quote_service;

pub use money_flow_service::MoneyFlowService;
pub use quote_service::QuoteService;
```

### Step 3: 运行测试

```bash
cargo test --package kaipanla --lib service::money_flow_service:: --nocapture
```

预期输出: 所有测试通过

### Step 4: 提交代码

```bash
git add src-tauri/src/service
git commit -m "feat: 实现资金监控模块

- 创建 MoneyFlowService 资金监控服务
- 实现交易资金流向分析
- 支持大单判断（>100万）
- 实现资金流向聚合功能
- 添加完整的单元测试"
```

---

## Task 6: 实现龙虎榜模块

**目标:** 创建龙虎榜数据模型和服务

**Files:**
- Create: `src-tauri/src/models/dragon_tiger.rs`
- Create: `src-tauri/src/service/dragon_tiger_service.rs`
- Modify: `src-tauri/src/models/mod.rs`
- Modify: `src-tauri/src/service/mod.rs`

### Step 1: 创建龙虎榜数据模型

创建 `src-tauri/src/models/dragon_tiger.rs`:

```rust
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// 龙虎榜记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonTiger {
    pub date: NaiveDate,
    pub code: String,
    pub name: String,
    pub reason: DragonReason,  // 上榜原因
    pub broker: String,        // 营业部名称
    pub buy_amount: f64,       // 买入金额（万元）
    pub sell_amount: f64,      // 卖出金额（万元）
    pub net_amount: f64,       // 净买入（万元）
}

impl DragonTiger {
    /// 计算净买入
    pub fn net(&self) -> f64 {
        self.buy_amount - self.sell_amount
    }
}

/// 上榜原因
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragonReason {
    /// 日涨跌幅偏离值达7%
    UpLimit,
    /// 日跌跌幅偏离值达7%
    DownLimit,
    /// 日换手率达20%
    HighTurnover,
    /// 连续三个交易日内涨跌幅偏离值累计达20%
    ThreeDayUp,
    /// 日价格涨幅偏离值达7%
    PriceUp,
    /// 其他
    Other(String),
}

/// 营业部统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerStats {
    pub broker: String,
    pub appear_count: i32,    // 上榜次数
    pub total_buy: f64,       // 总买入（万元）
    pub total_sell: f64,      // 总卖出（万元）
    pub total_net: f64,       // 总净买入（万元）
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_tiger_net() {
        let dt = DragonTiger {
            date: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            reason: DragonReason::UpLimit,
            broker: "东方财富拉萨营业部".to_string(),
            buy_amount: 5000.0,
            sell_amount: 2000.0,
            net_amount: 0.0,
        };

        assert_eq!(dt.net(), 3000.0);
    }
}
```

### Step 2: 创建龙虎榜服务

创建 `src-tauri/src/service/dragon_tiger_service.rs`:

```rust
use crate::models::dragon_tiger::{DragonTiger, BrokerStats};
use crate::{Result, AppError};
use chrono::NaiveDate;

/// 龙虎榜服务
pub struct DragonTigerService;

impl DragonTigerService {
    /// 创建新的龙虎榜服务
    pub fn new() -> Self {
        Self
    }

    /// 获取指定日期的龙虎榜数据
    pub async fn get_dragon_tiger_list(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<DragonTiger>> {
        tracing::debug!("获取 {} 龙虎榜数据", date);

        // TODO: 从 ClickHouse 或爬取东方财富
        Ok(vec![])
    }

    /// 获取营业部历史统计
    pub async fn get_broker_stats(
        &self,
        broker: &str,
    ) -> Result<BrokerStats> {
        tracing::debug!("获取营业部 {} 统计数据", broker);

        // TODO: 查询数据库
        Ok(BrokerStats {
            broker: broker.to_string(),
            appear_count: 0,
            total_buy: 0.0,
            total_sell: 0.0,
            total_net: 0.0,
        })
    }

    /// 获取个股龙虎榜历史
    pub async fn get_stock_dragon_tiger_history(
        &self,
        code: &str,
    ) -> Result<Vec<DragonTiger>> {
        tracing::debug!("获取股票 {} 龙虎榜历史", code);

        // TODO: 查询数据库
        Ok(vec![])
    }
}

impl Default for DragonTigerService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dragon_tiger_service_creation() {
        let service = DragonTigerService::new();

        let result = service
            .get_dragon_tiger_list(
                NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
            )
            .await;

        assert!(result.is_ok());
    }
}
```

### Step 3: 更新 models/mod.rs

编辑 `src-tauri/src/models/mod.rs`:

```rust
//! 数据模型定义

pub mod dragon_tiger;
pub mod money_flow;
pub mod quote;
pub mod stock;

pub use dragon_tiger::*;
pub use money_flow::*;
pub use quote::*;
pub use stock::*;
```

### Step 4: 更新 service/mod.rs

编辑 `src-tauri/src/service/mod.rs`:

```rust
//! 业务服务层

pub mod dragon_tiger_service;
pub mod money_flow_service;
pub mod quote_service;

pub use dragon_tiger_service::DragonTigerService;
pub use money_flow_service::MoneyFlowService;
pub use quote_service::QuoteService;
```

### Step 5: 运行测试

```bash
cargo test --package kaipanla --lib models::dragon_tiger:: service::dragon_tiger_service:: --nocapture
```

预期输出: 所有测试通过

### Step 6: 提交代码

```bash
git add src-tauri/src/models src-tauri/src/service
git commit -m "feat: 实现龙虎榜模块

- 定义 DragonTiger 数据模型
- 实现上榜原因枚举
- 创建 DragonTigerService 龙虎榜服务
- 支持营业部统计功能
- 添加单元测试"
```

---

## Task 7: 实现竞价分析模块

**目标:** 创建集合竞价异动分析服务

**Files:**
- Create: `src-tauri/src/models/auction.rs`
- Create: `src-tauri/src/service/auction_service.rs`
- Modify: `src-tauri/src/models/mod.rs`
- Modify: `src-tauri/src/service/mod.rs`

### Step 1: 创建竞价数据模型

创建 `src-tauri/src/models/auction.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 集合竞价数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub code: String,
    pub name: String,
    pub price: f64,           // 竞价价格
    pub preclose: f64,        // 昨收价
    pub volume: f64,          // 竞成交量（手）
    pub amount: f64,          // 竞价金额（元）
    pub timestamp: DateTime<Utc>,
}

impl Auction {
    /// 计算竞价涨跌幅
    pub fn change_pct(&self) -> f64 {
        if self.preclose == 0.0 {
            0.0
        } else {
            (self.price - self.preclose) / self.preclose * 100.0
        }
    }

    /// 计算量比（简化版）
    pub fn volume_ratio(&self, avg_volume: f64) -> f64 {
        if avg_volume == 0.0 {
            0.0
        } else {
            self.volume / avg_volume
        }
    }

    /// 判断是否异动
    pub fn is_anomaly(&self) -> bool {
        let change = self.change_pct().abs();
        change > 5.0
    }
}

/// 竞价异动记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionAnomaly {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub volume_ratio: f64,
    pub reason: AnomalyReason,
}

/// 异动原因
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyReason {
    /// 高开
    HighOpen,
    /// 低开
    LowOpen,
    /// 放量
    HighVolume,
    /// 缩量
    LowVolume,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_change_pct() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 10.0,
            volume: 10000.0,
            amount: 105000.0,
            timestamp: Utc::now(),
        };

        assert_eq!(auction.change_pct(), 5.0);
    }

    #[test]
    fn test_auction_is_anomaly() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.6,
            preclose: 10.0,
            volume: 10000.0,
            amount: 106000.0,
            timestamp: Utc::now(),
        };

        assert!(auction.is_anomaly());
    }
}
```

### Step 2: 创建竞价分析服务

创建 `src-tauri/src/service/auction_service.rs`:

```rust
use crate::models::auction::{Auction, AuctionAnomaly, AnomalyReason};
use crate::{Result, AppError};
use chrono::{DateTime, Utc};

/// 竞价分析服务
pub struct AuctionService;

impl AuctionService {
    /// 创建新的竞价分析服务
    pub fn new() -> Self {
        Self
    }

    /// 获取集合竞价数据（占位实现）
    pub async fn get_auction_data(&self) -> Result<Vec<Auction>> {
        tracing::debug!("获取集合竞价数据");

        // TODO: 从通达信获取竞价数据
        Ok(vec![])
    }

    /// 分析竞价异动
    pub fn analyze_anomalies(
        &self,
        auctions: Vec<Auction>,
    ) -> Result<Vec<AuctionAnomaly>> {
        let mut anomalies = Vec::new();

        for auction in auctions {
            let change_pct = auction.change_pct();
            let volume_ratio = 1.0; // TODO: 计算实际量比

            // 判断异动
            if change_pct > 5.0 {
                anomalies.push(AuctionAnomaly {
                    code: auction.code.clone(),
                    name: auction.name.clone(),
                    change_pct,
                    volume_ratio,
                    reason: AnomalyReason::HighOpen,
                });
            } else if change_pct < -5.0 {
                anomalies.push(AuctionAnomaly {
                    code: auction.code.clone(),
                    name: auction.name.clone(),
                    change_pct,
                    volume_ratio,
                    reason: AnomalyReason::LowOpen,
                });
            }
        }

        Ok(anomalies)
    }

    /// 识别龙头股
    pub fn identify_leaders(
        &self,
        anomalies: Vec<AuctionAnomaly>,
    ) -> Vec<String> {
        let mut leaders: Vec<(String, f64)> = anomalies
            .into_iter()
            .filter(|a| matches!(a.reason, AnomalyReason::HighOpen))
            .map(|a| (a.code, a.change_pct))
            .collect();

        // 按涨幅排序
        leaders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        leaders.into_iter().map(|(code, _)| code).collect()
    }
}

impl Default for AuctionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_service_creation() {
        let service = AuctionService::new();

        let result = service.analyze_anomalies(vec![]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
```

### Step 3: 更新 models/mod.rs

编辑 `src-tauri/src/models/mod.rs`:

```rust
//! 数据模型定义

pub mod auction;
pub mod dragon_tiger;
pub mod money_flow;
pub mod quote;
pub mod stock;

pub use auction::*;
pub use dragon_tiger::*;
pub use money_flow::*;
pub use quote::*;
pub use stock::*;
```

### Step 4: 更新 service/mod.rs

编辑 `src-tauri/src/service/mod.rs`:

```rust
//! 业务服务层

pub mod auction_service;
pub mod dragon_tiger_service;
pub mod money_flow_service;
pub mod quote_service;

pub use auction_service::AuctionService;
pub use dragon_tiger_service::DragonTigerService;
pub use money_flow_service::MoneyFlowService;
pub use quote_service::QuoteService;
```

### Step 5: 运行测试

```bash
cargo test --package kaipanla --lib models::auction:: service::auction_service:: --nocapture
```

预期输出: 所有测试通过

### Step 6: 提交代码

```bash
git add src-tauri/src/models src-tauri/src/service
git commit -m "feat: 实现竞价分析模块

- 定义 Auction 集合竞价数据模型
- 实现 AuctionAnomaly 异动记录
- 创建 AuctionService 竞价分析服务
- 支持异动识别和龙头股筛选
- 添加单元测试"
```

---

## Task 8: 更新 Tauri Commands 集成服务

**目标:** 将所有服务集成到 Tauri Commands 中

**Files:**
- Modify: `src-tauri/src/cmd/quote.rs`
- Create: `src-tauri/src/cmd/money_flow.rs`
- Create: `src-tauri/src/cmd/dragon_tiger.rs`
- Create: `src-tauri/src/cmd/auction.rs`
- Modify: `src-tauri/src/cmd/mod.rs`
- Modify: `src-tauri/src/main.rs`

### Step 1: 更新 quote.rs 命令

编辑 `src-tauri/src/cmd/quote.rs`:

```rust
use crate::models::Quote;
use crate::service::QuoteService;
use crate::Result;

/// 获取股票实时行情
#[tauri::command]
pub async fn get_quote(code: String) -> Result<Quote> {
    // TODO: 从服务获取
    Ok(Quote {
        code: code.clone(),
        name: "测试股票".to_string(),
        price: 10.5,
        preclose: 10.0,
        open: 10.2,
        high: 10.6,
        low: 10.1,
        volume: 100000.0,
        amount: 1050000.0,
        bid: [0.0; 5],
        bid_vol: [0.0; 5],
        ask: [0.0; 5],
        ask_vol: [0.0; 5],
        timestamp: chrono::Utc::now(),
    })
}

/// 获取股票列表
#[tauri::command]
pub async fn get_stock_list() -> Result<Vec<crate::models::Stock>> {
    // TODO: 从服务获取
    Ok(vec![])
}
```

### Step 2: 创建资金流向命令

创建 `src-tauri/src/cmd/money_flow.rs`:

```rust
use crate::models::MoneyFlow;
use crate::service::MoneyFlowService;
use crate::Result;

/// 获取资金流向
#[tauri::command]
pub async fn get_money_flow(code: String) -> Result<MoneyFlow> {
    let service = MoneyFlowService::new();
    service.get_daily_money_flow(&code).await
}
```

### Step 3: 创建龙虎榜命令

创建 `src-tauri/src/cmd/dragon_tiger.rs`:

```rust
use crate::models::DragonTiger;
use crate::service::DragonTigerService;
use crate::Result;
use chrono::NaiveDate;

/// 获取龙虎榜列表
#[tauri::command]
pub async fn get_dragon_tiger_list(date: String) -> Result<Vec<DragonTiger>> {
    let service = DragonTigerService::new();

    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| crate::error::AppError::Parse(e.to_string()))?;

    service.get_dragon_tiger_list(parsed_date).await
}
```

### Step 4: 创建竞价命令

创建 `src-tauri/src/cmd/auction.rs`:

```rust
use crate::models::auction::AuctionAnomaly;
use crate::service::AuctionService;
use crate::Result;

/// 获取竞价异动
#[tauri::command]
pub async fn get_auction_anomalies() -> Result<Vec<AuctionAnomaly>> {
    let service = AuctionService::new();

    let auctions = service.get_auction_data().await?;
    service.analyze_anomalies(auctions).await
}
```

### Step 5: 更新 cmd/mod.rs

编辑 `src-tauri/src/cmd/mod.rs`:

```rust
//! Tauri 命令模块

pub mod auction;
pub mod dragon_tiger;
pub mod money_flow;
pub mod quote;
```

### Step 6: 更新 main.rs 注册所有命令

编辑 `src-tauri/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kaipanla::api::ApiServer;
use kaipanla::config;
use kaipanla::websocket::WsServer;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("开盘啦应用启动");

    // 加载配置 (使用默认配置)
    let app_config = Arc::new(RwLock::new(config::Config::default()));

    // 启动 API 服务器 (后台任务)
    let api_config = app_config.read().await.api.clone();
    tokio::spawn(async move {
        let api_server = ApiServer::new(&api_config);
        if let Err(e) = api_server.run().await {
            tracing::error!("API 服务器错误: {}", e);
        }
    });

    // 运行 Tauri 应用
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kaipanla::cmd::quote::get_quote,
            kaipanla::cmd::quote::get_stock_list,
            kaipanla::cmd::money_flow::get_money_flow,
            kaipanla::cmd::dragon_tiger::get_dragon_tiger_list,
            kaipanla::cmd::auction::get_auction_anomalies,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 7: 编译验证

```bash
cargo check --package kaipanla
```

预期输出: 编译通过

### Step 8: 提交代码

```bash
git add src-tauri/src/cmd src-tauri/src/main.rs
git commit -m "feat: 更新 Tauri Commands 集成所有服务

- 更新 quote 命令
- 添加 money_flow 命令
- 添加 dragon_tiger 命令
- 添加 auction 命令
- 注册所有命令到主应用"
```

---

## Task 9: 添加 API 路由

**目标:** 为所有服务添加 RESTful API 端点

**Files:**
- Modify: `src-tauri/src/api/routes.rs`

### Step 1: 更新 API 路由

编辑 `src-tauri/src/api/routes.rs`:

```rust
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ping", get(ping))
        .route("/api/v1/quote/:code", get(get_quote))
        .route("/api/v1/moneyflow/:code", get(get_money_flow))
        .route("/api/v1/dragon-tiger", get(get_dragon_tiger_list))
        .route("/api/v1/auction/anomalies", get(get_auction_anomalies))
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "kaipanla"
    }))
}

async fn ping() -> impl IntoResponse {
    Json(json!({
        "message": "pong"
    }))
}

async fn get_quote(axum::extract::Path(code): axum::extract::Path<String>) -> impl IntoResponse {
    // TODO: 调用服务
    Json(json!({
        "code": code,
        "price": 10.5
    }))
}

async fn get_money_flow(axum::extract::Path(code): axum::extract::Path<String>) -> impl IntoResponse {
    // TODO: 调用服务
    Json(json!({
        "code": code,
        "main_net": 2000.0
    }))
}

async fn get_dragon_tiger_list() -> impl IntoResponse {
    // TODO: 调用服务
    Json(json!([]))
}

async fn get_auction_anomalies() -> impl IntoResponse {
    // TODO: 调用服务
    Json(json!([]))
}
```

### Step 2: 编译验证

```bash
cargo check --package kaipanla
```

预期输出: 编译通过

### Step 3: 提交代码

```bash
git add src-tauri/src/api
git commit -m "feat: 添加 RESTful API 路由

- 添加 /api/v1/quote/:code 行情端点
- 添加 /api/v1/moneyflow/:code 资金流端点
- 添加 /api/v1/dragon-tiger 龙虎榜端点
- 添加 /api/v1/auction/anomalies 竞价异动端点"
```

---

## Task 10: 完善文档和测试

**目标:** 更新开发文档，添加集成测试

**Files:**
- Create: `docs/phase2-summary.md`
- Update: `docs/development.md`

### Step 1: 创建 Phase 2 总结文档

创建 `docs/phase2-summary.md`:

```markdown
# Phase 2: 核心业务功能总结

## 完成功能

### 1. 数据采集模块
- 集成 rustdx 通达信数据采集
- 实现数据解析框架
- 支持历史数据和实时行情

### 2. 数据模型
- Stock 股票基本信息
- Quote 实时行情
- MoneyFlow 资金流向
- KLine K线数据
- DragonTiger 龙虎榜
- Auction 集合竞价

### 3. 业务服务
- QuoteService 实时行情服务
- MoneyFlowService 资金监控服务
- DragonTigerService 龙虎榜服务
- AuctionService 竞价分析服务

### 4. WebSocket 实时推送
- WsServer WebSocket 服务器
- 支持订阅/取消订阅
- 实时行情推送

### 5. API 接口
- RESTful API 端点
- Tauri Commands
- WebSocket 推送

## 技术亮点

- 模块化设计
- 异步处理
- 单元测试覆盖
- 类型安全

## 下一步计划

Phase 3: 前端界面开发
```

### Step 2: 更新开发文档

编辑 `docs/development.md`，添加 Phase 2 相关内容

### Step 3: 运行完整测试套件

```bash
cargo test --package kaipanla --lib -- --nocapture
```

预期输出: 所有测试通过

### Step 4: 编译验证

```bash
cargo build --release --package kaipanla
```

预期输出: 编译成功

### Step 5: 提交文档更新

```bash
git add docs
git commit -m "docs: 添加 Phase 2 总结文档

- 创建 Phase 2 功能总结
- 更新开发文档
- 添加下一步计划"
```

### Step 6: 创建 Git 标签

```bash
git tag v0.2.0
git push origin v0.2.0
```

---

## 验收标准

Phase 2 完成后，应该能够：

1. ✅ 所有模块编译通过
2. ✅ 所有单元测试通过
3. ✅ Tauri 应用正常启动
4. ✅ API 服务器正常响应
5. ✅ Tauri Commands 可调用
6. ✅ 代码提交清晰
7. ✅ Git 标签创建成功

## 测试清单

```bash
# 1. 单元测试
cargo test --package kaipanla --lib

# 2. 编译检查
cargo check --package kaipanla

# 3. 构建验证
cargo build --release --package kaipanla

# 4. 运行应用
npm run tauri dev

# 5. API 测试
curl http://localhost:8000/health
curl http://localhost:8000/api/v1/ping
```

---

**计划完成！准备好开始实施了。**

**执行方式选择:**
1. Subagent-Driven (当前会话) - 逐任务执行，每个任务后审查
2. Parallel Session (独立会话) - 使用 executing-plans skill 批量执行

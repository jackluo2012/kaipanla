# Phase 1: 基础设施搭建实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 搭建开盘啦项目的基础设施,包括 Tauri + Rust 项目脚手架、ClickHouse 数据库、基础 API 框架

**架构:** 采用前后端分离的模块化单体架构。Tauri 作为桌面应用框架,Rust 后端提供 API 服务,ClickHouse 作为主力数据存储。

**技术栈:** Tauri 2.x, Rust 1.70+, ClickHouse 23+, tokio 异步运行时

---

## Task 1: 初始化 Tauri 项目

**目标:** 创建 Tauri 2.x 项目基础结构

**Files:**
- Create: `Cargo.toml`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/build.rs`
- Create: `package.json`
- Create: `index.html`
- Create: `.gitignore`

### Step 1: 创建项目根目录 Cargo.toml

```toml
[workspace]
members = ["src-tauri"]
resolver = "2"

[workspace.dependencies]
# 依赖版本统一管理
```

### Step 2: 创建 src-tauri/Cargo.toml

```toml
[package]
name = "kaipanla"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["devtools"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### Step 3: 创建 src-tauri/src/main.rs

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    tracing::info!("开盘啦应用启动");

    // 运行 Tauri 应用
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 4: 创建 src-tauri/src/lib.rs

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod config;
pub mod error;

pub use error::Result;
```

### Step 5: 创建 src-tauri/build.rs

```rust
fn main() {
    tauri_build::build()
}
```

### Step 6: 创建 tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "开盘啦",
  "version": "0.1.0",
  "identifier": "com.kaipanla.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "开盘啦",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self'; connect-src 'self' http://localhost:*"
    }
  },
  "bundle": {
    "active": true,
    "targets": "all"
  }
}
```

### Step 7: 创建 package.json

```json
{
  "name": "kaipanla",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "tauri": "tauri"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "vite": "^5.0.0"
  }
}
```

### Step 8: 创建 index.html

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>开盘啦</title>
</head>
<body>
    <h1>开盘啦 - 专业股票分析软件</h1>
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
</body>
</html>
```

### Step 9: 创建 src/main.js

```javascript
// 简单的前端入口,后续会使用 React/Vue
console.log('开盘啦前端启动');
document.getElementById('app').innerHTML = '<p>应用加载中...</p>';
```

### Step 10: 创建 vite.config.js

```javascript
import { defineConfig } from 'vite';

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
```

### Step 11: 创建 .gitignore

```gitignore
# Rust
/target
**/*.rs.bk
*.pdb

# Node
node_modules/
dist/
*.log

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Environment
.env
.env.local
```

### Step 12: 安装依赖并验证

```bash
# 安装 Node 依赖
npm install

# 编译 Rust 代码
cargo check

# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 13: 提交初始化代码

```bash
git add .
git commit -m "feat: 初始化 Tauri 项目结构"
```

---

## Task 2: 创建错误处理模块

**目标:** 建立统一的错误处理体系

**Files:**
- Create: `src-tauri/src/error.rs`

### Step 1: 创建 error.rs

```rust
use thiserror::Error;

/// 应用统一错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("数据库错误: {0}")]
    Database(String),

    #[error("网络错误: {0}")]
    Network(String),

    #[error("数据解析错误: {0}")]
    Parse(String),

    #[error("配置错误: {0}")]
    Config(String),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// 应用统一 Result 类型
pub type Result<T> = std::result::Result<T, AppError>;

// 为 HTTP 响应实现的错误转换
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
```

### Step 2: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 3: 提交

```bash
git add src-tauri/src/error.rs src-tauri/src/lib.rs
git commit -m "feat: 添加统一错误处理模块"
```

---

## Task 3: 创建配置管理模块

**目标:** 实现应用配置管理

**Files:**
- Create: `src-tauri/src/config.rs`
- Create: `config.example.toml`

### Step 1: 创建 config.rs

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub data_source: DataSourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub clickhouse_url: String,
    pub sqlite_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub tdx_servers: Vec<String>,
    pub update_interval_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                clickhouse_url: "http://localhost:8123".to_string(),
                sqlite_path: PathBuf::from("kaipanla.db"),
            },
            api: ApiConfig {
                host: "127.0.0.1".to_string(),
                port: 8000,
            },
            data_source: DataSourceConfig {
                tdx_servers: vec![
                    "124.71.187.122:7709".to_string(),
                    "122.51.120.217:7709".to_string(),
                ],
                update_interval_secs: 3,
            },
        }
    }
}

/// 从文件加载配置
pub fn load_config(path: &str) -> crate::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    Ok(config)
}
```

### Step 2: 更新 Cargo.toml 添加 toml 依赖

在 `src-tauri/Cargo.toml` 的 `[dependencies]` 中添加:

```toml
toml = "0.8"
```

### Step 3: 创建配置文件示例

```toml
# config.example.toml

[database]
clickhouse_url = "http://localhost:8123"
sqlite_path = "kaipanla.db"

[api]
host = "127.0.0.1"
port = 8000

[data_source]
tdx_servers = [
    "124.71.187.122:7709",
    "122.51.120.217:7709",
]
update_interval_secs = 3
```

### Step 4: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 5: 提交

```bash
git add src-tauri/src/config.rs config.example.toml src-tauri/Cargo.toml
git commit -m "feat: 添加配置管理模块"
```

---

## Task 4: 创建 Tauri Command 模块

**目标:** 定义 Tauri 命令接口

**Files:**
- Create: `src-tauri/src/cmd/mod.rs`
- Create: `src-tauri/src/cmd/quote.rs`
- Modify: `src-tauri/src/lib.rs`

### Step 1: 创建 cmd/mod.rs

```rust
pub mod quote;
```

### Step 2: 创建 cmd/quote.rs

```rust
use serde::{Deserialize, Serialize};
use tauri::State;

/// 股票实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
}

/// 获取实时行情命令
#[tauri::command]
pub async fn get_quote(code: String) -> crate::Result<Quote> {
    // TODO: 实现实际的数据获取逻辑
    Ok(Quote {
        code: code.clone(),
        name: "测试股票".to_string(),
        price: 10.5,
        change: 0.5,
        change_pct: 5.0,
        volume: 100000.0,
        amount: 1050000.0,
    })
}

/// 获取股票列表命令
#[tauri::command]
pub async fn get_stock_list() -> crate::Result<Vec<Quote>> {
    // TODO: 实现实际的数据获取逻辑
    Ok(vec![])
}
```

### Step 3: 更新 lib.rs 导出

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod config;
pub mod error;

pub use error::Result;
```

### Step 4: 更新 main.rs 注册命令

在 `tauri::Builder::default()` 后添加 `.invoke_handler()`:

```rust
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    tracing::info!("开盘啦应用启动");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::quote::get_quote,
            cmd::quote::get_stock_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 5: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 6: 提交

```bash
git add src-tauri/src/cmd/ src-tauri/src/main.rs src-tauri/src/lib.rs
git commit -m "feat: 添加 Tauri Command 接口"
```

---

## Task 5: 集成 ClickHouse 客户端

**目标:** 添加 ClickHouse 数据库支持

**Files:**
- Create: `src-tauri/src/db/mod.rs`
- Create: `src-tauri/src/db/clickhouse.rs`
- Modify: `src-tauri/Cargo.toml`

### Step 1: 添加 ClickHouse 依赖到 Cargo.toml

```toml
clickhouse-rs = "0.1"
```

### Step 2: 创建 db/mod.rs

```rust
pub mod clickhouse;

pub use clickhouse::Client;
```

### Step 3: 创建 db/clickhouse.rs

```rust
use clickhouse_rs::Pool;
use crate::config::DatabaseConfig;
use crate::{Result, AppError};

/// ClickHouse 客户端池
pub struct Client {
    pool: Pool,
}

impl Client {
    /// 创建新的 ClickHouse 客户端
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let pool = Pool::new(config.clickhouse_url.as_str());

        // 测试连接
        let mut conn = pool
            .get_handle()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // 简单查询测试连接
        let _ = conn
            .query("SELECT 1")
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        tracing::info!("ClickHouse 连接成功: {}", config.clickhouse_url);

        Ok(Self { pool })
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}
```

### Step 4: 更新 lib.rs

```rust
//! 开盘啦 - 库入口

pub mod cmd;
pub mod config;
pub mod db;
pub mod error;

pub use error::Result;
```

### Step 5: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 6: 提交

```bash
git add src-tauri/src/db/ src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat: 集成 ClickHouse 客户端"
```

---

## Task 6: 初始化 ClickHouse 数据库表

**目标:** 创建 ClickHouse 表结构

**Files:**
- Create: `migrations/001_init_tables.sql`
- Create: `src-tauri/src/db/migrations.rs`
- Modify: `src-tauri/src/db/mod.rs`

### Step 1: 创建迁移 SQL 文件

```sql
-- migrations/001_init_tables.sql

-- 日线表 (rustdx 原生格式)
CREATE TABLE IF NOT EXISTS kaipanla.factor (
    date Date,
    code FixedString(6),
    open Float64,
    high Float64,
    low Float64,
    close Float64,
    preclose Float64,
    factor Float64,
    volume Float64,
    amount Float64
) ENGINE = MergeTree()
ORDER BY (date, code);

-- 实时行情表
CREATE TABLE IF NOT EXISTS kaipanla.quote_realtime (
    datetime DateTime,
    code FixedString(6),
    price Float64,
    volume Float64,
    amount Float64,
    bids Array(Float64),
    asks Array(Float64)
) ENGINE = MergeTree()
ORDER BY (datetime, code);

-- 龙虎榜表
CREATE TABLE IF NOT EXISTS kaipanla.dragon_tiger (
    date Date,
    code FixedString(6),
    name String,
    reason String,
    broker String,
    buy_amount Float64,
    sell_amount Float64,
    net_amount Float64
) ENGINE = MergeTree()
ORDER BY (date, code);

-- 资金流向表
CREATE TABLE IF NOT EXISTS kaipanla.money_flow (
    datetime DateTime,
    code FixedString(6),
    main_inflow Float64,
    main_outflow Float64,
    retail_inflow Float64,
    retail_outflow Float64,
    net_amount Float64
) ENGINE = MergeTree()
ORDER BY (datetime, code);
```

### Step 2: 创建 db/migrations.rs

```rust
use crate::db::Client;
use crate::{Result, AppError};

impl Client {
    /// 执行数据库迁移
    pub async fn run_migrations(&self) -> Result<()> {
        let sql = include_str!("../../migrations/001_init_tables.sql");

        let mut conn = self
            .pool()
            .get_handle()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // 分割 SQL 语句并逐个执行
        for statement in sql.split(';') {
            let statement = statement.trim();
            if statement.is_empty() {
                continue;
            }

            conn.query(statement)
                .await
                .map_err(|e| AppError::Database(format!("迁移失败: {}", e)))?;
        }

        tracing::info!("数据库迁移完成");
        Ok(())
    }
}
```

### Step 3: 更新 db/mod.rs

```rust
pub mod clickhouse;
pub mod migrations;

pub use clickhouse::Client;
```

### Step 4: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 5: 提交

```bash
git add migrations/ src-tauri/src/db/
git commit -m "feat: 添加 ClickHouse 数据库迁移"
```

---

## Task 7: 创建 Docker Compose 配置

**目标:** 提供 ClickHouse 本地开发环境

**Files:**
- Create: `docker-compose.yml`

### Step 1: 创建 docker-compose.yml

```yaml
version: '3.8'

services:
  clickhouse:
    image: clickhouse/clickhouse-server:23
    container_name: kaipanla_clickhouse
    ports:
      - "8123:8123"
      - "9000:9000"
    environment:
      CLICKHOUSE_DB: kaipanla
      CLICKHOUSE_USER: default
      CLICKHOUSE_PASSWORD: ""
      CLICKHOUSE_DEFAULT_ACCESS_MANAGEMENT: 1
    volumes:
      - clickhouse_data:/var/lib/clickhouse
    ulimits:
      nofile:
        soft: 262144
        hard: 262144

volumes:
  clickhouse_data:
```

### Step 2: 创建 .env.example

```bash
# ClickHouse 配置
CLICKHOUSE_URL=http://localhost:8123
CLICKHOUSE_USER=default
CLICKHOUSE_PASSWORD=

# API 配置
API_HOST=127.0.0.1
API_PORT=8000
```

### Step 3: 测试启动 ClickHouse

```bash
# 启动 ClickHouse
docker-compose up -d

# 查看日志
docker-compose logs -f clickhouse

# 预期输出: ClickHouse starting
```

### Step 4: 测试连接

```bash
# 测试 ClickHouse 连接
curl "http://localhost:8123" --data "SELECT 1"

# 预期输出: 1
```

### Step 5: 提交

```bash
git add docker-compose.yml .env.example
git commit -m "feat: 添加 ClickHouse Docker Compose 配置"
```

---

## Task 8: 创建简单的 API 服务器框架

**目标:** 搭建 REST API 基础框架

**Files:**
- Create: `src-tauri/src/api/mod.rs`
- Create: `src-tauri/src/api/server.rs`
- Create: `src-tauri/src/api/routes.rs`
- Modify: `src-tauri/Cargo.toml`

### Step 1: 添加 Web 框架依赖

在 `src-tauri/Cargo.toml` 添加:

```toml
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }
```

### Step 2: 创建 api/mod.rs

```rust
pub mod server;
pub mod routes;

pub use server::ApiServer;
```

### Step 3: 创建 api/server.rs

```rust
use axum::Router;
use std::net::SocketAddr;
use crate::api::routes::create_router;
use crate::config::ApiConfig;
use crate::{Result, AppError};

pub struct ApiServer {
    addr: SocketAddr,
}

impl ApiServer {
    pub fn new(config: &ApiConfig) -> Self {
        let addr = format!("{}:{}", config.host, config.port)
            .parse()
            .expect("无效的地址");

        Self { addr }
    }

    pub async fn run(self) -> Result<()> {
        let app = create_router();

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        tracing::info!("API 服务器启动: http://{}", self.addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
}
```

### Step 4: 创建 api/routes.rs

```rust
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ping", get(ping))
}

/// 健康检查
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "kaipanla"
    }))
}

/// Ping 测试
async fn ping() -> impl IntoResponse {
    Json(json!({
        "message": "pong"
    }))
}
```

### Step 5: 更新 lib.rs

```rust
//! 开盘啦 - 库入口

pub mod api;
pub mod cmd;
pub mod config;
pub mod db;
pub mod error;

pub use error::Result;
```

### Step 6: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 7: 提交

```bash
git add src-tauri/src/api/ src-tauri/src/lib.rs src-tauri/Cargo.toml
git commit -m "feat: 添加 API 服务器框架"
```

---

## Task 9: 集成 API 服务器到主应用

**目标:** 在主应用中启动 API 服务器

**Files:**
- Modify: `src-tauri/src/main.rs`

### Step 1: 更新 main.rs 启动 API 服务器

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kaipanla::api::ApiServer;
use kaipanla::config;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
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
            kaipanla::cmd::quote::get_stock_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 2: 运行检查

```bash
cargo check
# 预期输出: Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Step 3: 测试运行

```bash
# 启动应用
cargo run

# 在另一个终端测试 API
curl http://127.0.0.1:8000/health

# 预期输出: {"status":"ok","service":"kaipanla"}
```

### Step 4: 提交

```bash
git add src-tauri/src/main.rs
git commit -m "feat: 集成 API 服务器到主应用"
```

---

## Task 10: 添加开发文档

**目标:** 创建开发文档

**Files:**
- Create: `docs/development.md`

### Step 1: 创建开发文档

```markdown
# 开发指南

## 环境准备

### 1. 安装依赖

**系统依赖:**
- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose

**Rust 工具链:**
```bash
rustup update
rustup component add clippy rustfmt
```

**Node 依赖:**
```bash
npm install
```

### 2. 启动 ClickHouse

```bash
# 启动 ClickHouse 容器
docker-compose up -d clickhouse

# 查看日志
docker-compose logs -f clickhouse
```

### 3. 配置

复制配置文件:
```bash
cp config.example.toml config.toml
```

根据需要修改配置。

## 开发流程

### 启动开发服务器

```bash
# 启动前端开发服务器 + Tauri
npm run tauri dev
```

### 代码检查

```bash
# Rust 代码检查
cargo check
cargo clippy

# Rust 格式化
cargo fmt

# 运行测试
cargo test
```

## 项目结构

```
kaipanla/
├── src/                   # 前端源码
├── src-tauri/            # Rust 后端
│   ├── src/
│   │   ├── api/          # API 路由
│   │   ├── cmd/          # Tauri 命令
│   │   ├── db/           # 数据库模块
│   │   ├── config.rs     # 配置管理
│   │   ├── error.rs      # 错误处理
│   │   └── main.rs       # 入口
│   ├── Cargo.toml
│   └── tauri.conf.json
├── migrations/           # 数据库迁移
├── docs/                 # 文档
└── docker-compose.yml    # Docker 配置
```

## 调试

### 查看日志

日志通过 `tracing` 输出,开发时可在终端查看。

### 数据库调试

```bash
# 连接 ClickHouse
docker exec -it kaipanla_clickhouse clickhouse-client

# 查看表
USE kaipanla;
SHOW TABLES;
```

## 常见问题

**Q: ClickHouse 连接失败?**
A: 检查 Docker 容器是否运行: `docker-compose ps`

**Q: 端口冲突?**
A: 修改 `config.toml` 中的端口配置
```

### Step 2: 提交

```bash
git add docs/development.md
git commit -m "docs: 添加开发文档"
```

---

## 验收标准

完成所有任务后,应该能够:

1. ✅ `cargo check` 通过无错误
2. ✅ `docker-compose up -d clickhouse` 成功启动 ClickHouse
3. ✅ `cargo run` 成功启动应用
4. ✅ API 健康检查返回正确响应: `curl http://127.0.0.1:8000/health`
5. ✅ Tauri 应用窗口正常打开
6. ✅ 日志正常输出

---

## 下一步

完成 Phase 1 后,继续 **Phase 2: 核心功能开发**

参考: `docs/plans/2025-12-24-kaipanla-design.md`

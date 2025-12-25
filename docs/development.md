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
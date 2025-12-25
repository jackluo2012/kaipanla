# Phase 3: 数据集成设计方案

**版本:** v0.3.0-design
**日期:** 2025-12-25
**目标:** 实现真实数据采集、存储和查询能力

---

## 核心决策

### 1. 实施策略
- **方式**: 数据优先 - 先打通真实数据源
- **范围**: 增量式 - 先实现日线数据
- **采集**: API 方式 - 使用 rustdx 从通达信服务器获取
- **初始化**: 按需初始化 - 导入最近1-3年数据
- **更新**: 实时轮询 - 每3秒更新
- **容错**: 完整方案 - 重试+服务器切换+降级
- **质量**: 全面质量 - 验证+完整性+异常检测

---

## 整体架构

### 生产者-消费者模式

```
┌─────────────────────────────────────────────┐
│         数据采集调度器 (Scheduler)          │
│  - 定时任务：交易日每3秒触发               │
│  - 交易日判断：自动识别交易时间             │
│  - 失败重试：指数退避策略                  │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│      rustdx API 客户端 (TdxClient)         │
│  - 多服务器：自动故障切换                   │
│  - 连接池：复用TCP连接                      │
│  - 健康检查：定期验证服务器可用性           │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│        数据验证器 (DataValidator)           │
│  - 格式验证：数据类型、范围检查             │
│  - 完整性验证：连续性、重复检测             │
│  - 异常检测：涨跌停、停牌识别               │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│      数据缓冲队列 (Channel Buffer)          │
│  - 容量：1000条记录                         │
│  - 批量写入：积累到100条或5秒触发           │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│     ClickHouse 批量写入器 (BatchWriter)     │
│  - 批量插入：提高性能                       │
│  - 异步写入：不阻塞采集流程                 │
│  - 错误处理：失败记录+告警                  │
└─────────────────────────────────────────────┘
```

---

## 数据流程

### 1. 初始化阶段

```
启动应用
  ↓
检查 ClickHouse 连接
  ↓
读取已采集数据的最新日期
  ↓
判断：最新日期 < 当前日期-3年？
  ↓ 是
启动后台增量导入
  - 从最新日期+1开始
  - 每次导入100只股票×30天
  - 非阻塞，应用立即可用
```

### 2. 实时采集阶段

```
交易日判断 (周一至周五 9:00-15:00)
  ↓
每3秒触发采集任务
  ↓
获取股票列表
  - 自选股
  - 沪深300成分股
  - 中证500成分股
  ↓
调用 rustdx API 获取日线数据
  ↓
数据验证
  ✓ 价格 > 0
  ✓ 成交量 >= 0
  ✓ 日期合理
  ✓ 非重复数据
  ↓
写入缓冲队列
  ↓
批量写入 ClickHouse
```

### 3. 容错流程

```
API 调用失败
  ↓
重试1: 1秒后 + 切换服务器
  ↓
失败？
  ↓ 是
重试2: 2秒后 + 再切换服务器
  ↓
失败？
  ↓ 是
重试3: 4秒后 + 最后服务器
  ↓
失败？
  ↓ 是
降级策略
  ✓ 使用最后有效数据
  ✓ 标记状态"过期"
  ✓ 记录错误日志
  ✓ 发送告警（可选）
```

---

## 数据存储设计

### ClickHouse 表优化

#### 1. factor 表优化

```sql
-- 分区优化：按月分区
ALTER TABLE kaipanla.factor
MODIFY PARTITION BY toYYYYMM(date);

-- 索引优化
ALTER TABLE kaipanla.factor
ADD INDEX idx_code (code) TYPE minmax GRANULARITY 4;

-- 数据质量字段
ALTER TABLE kaipanla.factor
ADD COLUMN data_version UInt32 DEFAULT 1,
ADD COLUMN data_source Enum('api'=1, 'file'=2, 'manual'=3) DEFAULT 'api',
ADD COLUMN quality_score Enum8('good'=1, 'suspect'=2, 'error'=3) DEFAULT 'good',
ADD COLUMN created_at DateTime DEFAULT now();
```

#### 2. collection_status 表（新增）

```sql
CREATE TABLE IF NOT EXISTS kaipanla.collection_status (
    date Date,
    code FixedString(6),
    status Enum8('success'=1, 'failed'=2, 'pending'=3),
    retry_count UInt8 DEFAULT 0,
    error_message String,
    collected_at DateTime,
    updated_at DateTime DEFAULT now()
) ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (date, code);
```

#### 3. data_quality_log 表（新增）

```sql
CREATE TABLE IF NOT EXISTS kaipanla.data_quality_log (
    log_time DateTime,
    date Date,
    code FixedString(6),
    issue_type Enum8('duplicate'=1, 'gap'=2, 'abnormal'=3, 'missing'=4),
    description String,
    severity Enum8('info'=1, 'warning'=2, 'error'=3)
) ENGINE = MergeTree()
ORDER BY (log_time, date, code);
```

### 写入策略

- **批量大小**: 100条或5秒
- **写入模式**: Async INSERT
- **并发控制**: 最多3个并发任务
- **错误处理**: 记录到 collection_status

---

## 模块设计

### 1. collector/scheduler.rs

**职责**: 数据采集调度器

**功能**:
- 交易日判断
- 定时任务调度
- 失败重试管理
- 采集任务分发

### 2. collector/fetcher.rs

**职责**: 数据采集器

**功能**:
- 调用 rustdx API
- 多服务器管理
- 连接池复用
- 健康检查

### 3. collector/validator.rs

**职责**: 数据验证器

**功能**:
- 格式验证
- 完整性检查
- 异常检测
- 质量评分

### 4. collector/writer.rs

**职责**: ClickHouse 批量写入器

**功能**:
- 批量插入
- 异步写入
- 错误处理
- 状态记录

---

## API 接口

### 1. 启动数据采集

```
POST /api/v1/collection/start
{
  "codes": ["000001", "600036"],  // 可选，默认所有股票
  "mode": "realtime"  // realtime | history
}
```

### 2. 停止数据采集

```
POST /api/v1/collection/stop
```

### 3. 查询采集状态

```
GET /api/v1/collection/status
Response:
{
  "is_running": true,
  "last_update": "2025-12-25T10:30:00Z",
  "success_count": 1234,
  "failed_count": 5,
  "servers": [
    {"host": "124.71.187.122:7709", "status": "healthy"},
    {"host": "122.51.120.217:7709", "status": "failed"}
  ]
}
```

### 4. 数据质量报告

```
GET /api/v1/data/quality?date=2025-12-25
Response:
{
  "total_records": 5000,
  "good_quality": 4985,
  "suspect": 10,
  "error": 5,
  "issues": [
    {"code": "000001", "type": "gap", "description": "Missing data 2025-12-20"}
  ]
}
```

---

## Tauri Commands

### 1. 启动采集

```rust
#[tauri::command]
async fn start_collection(codes: Option<Vec<String>>) -> Result<()>
```

### 2. 停止采集

```rust
#[tauri::command]
async fn stop_collection() -> Result<()>
```

### 3. 获取采集状态

```rust
#[tauri::command]
async fn get_collection_status() -> Result<CollectionStatus>
```

---

## 测试策略

### 1. 单元测试

- **服务器切换**: 模拟失败，验证自动切换
- **数据验证**: 正常数据、异常数据、边界值
- **重试机制**: 验证指数退避
- **降级策略**: 模拟全服务器失败

### 2. 集成测试

- **真实数据**: 连接真实通达信服务器
- **ClickHouse**: 验证数据写入
- **并发采集**: 多股票同时采集
- **长时间运行**: 稳定性测试

### 3. 性能测试

- **采集速度**: 每秒处理股票数
- **写入性能**: ClickHouse 批量写入
- **内存使用**: 缓冲队列内存占用

---

## 实施计划

### Task 1: 增强 rustdx 客户端
- 实现多服务器自动切换
- 添加连接池管理
- 实现健康检查

### Task 2: 实现数据验证器
- 格式验证
- 完整性检查
- 异常检测
- 质量评分

### Task 3: 实现采集调度器
- 交易日判断
- 定时任务调度
- 重试机制

### Task 4: 实现批量写入器
- ClickHouse 批量插入
- 异步写入
- 错误处理

### Task 5: 创建数据表
- collection_status 表
- data_quality_log 表
- 优化 factor 表

### Task 6: API 接口
- 启动/停止采集
- 查询状态
- 质量报告

### Task 7: Tauri Commands
- 集成采集命令
- 状态查询

### Task 8: 测试和文档
- 单元测试
- 集成测试
- 更新文档

---

## 风险和应对

### 风险 1: 通达信服务器不稳定
**应对**: 多服务器 + 重试 + 降级

### 风险 2: ClickHouse 写入失败
**应对**: 记录失败 + 后台重试 + 告警

### 风险 3: 数据质量问题
**应对**: 多层验证 + 质量日志 + 人工审核

### 风险 4: 性能问题
**应对**: 批量写入 + 异步处理 + 连接池

---

**设计版本**: 1.0
**最后更新**: 2025-12-25
**状态**: 待实施

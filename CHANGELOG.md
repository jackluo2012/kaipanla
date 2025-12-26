# 开盘啦 - 版本发布日志

## v0.4.0 (2025-12-26) - Phase 3 深化：真实数据集成

### 🎯 重大更新

#### 真实数据源集成
- 集成 rustdx TCP 客户端连接通达信服务器
- 实现历史K线数据获取（支持800条/批分页）
- 实现实时快照数据采集
- 多服务器自动切换和故障转移
- A股智能过滤（仅保留沪深主板）

#### 历史数据导入器
- 两阶段导入策略：最近1个月（快速可用）+ 3年历史数据（后台回填）
- 批次管理：100股票×30天/批
- 断点续传支持
- 用户取消功能
- 实时进度跟踪

#### 实时采集调度器增强
- 混合模式采集：
  - 交易时间（09:00-15:00）：3秒快照
  - 盘后时间（15:00-18:00）：1分钟数据
  - 夜间时间（18:00-09:00）：停止采集
- 分批采集：500股票/批，覆盖5000+A股
- 5秒超时保护
- 数据降级策略

#### ClickHouse 存储优化
- 优化配置管理（异步插入、批量大小、并发控制）
- 数据质量字段（data_version, data_source, quality_score）
- 查询视图（最近30天、最新快照）
- 物化视图（每日数据质量统计）

#### 监控和告警系统
- 实时指标采集（成功/失败/超时次数、平均延迟）
- 服务器健康监控
- 质量评分系统（0-100分）
- 智能告警：
  - 连续失败10次 → Error
  - 所有服务器不可用 → Critical
  - 质量分数<90% → Warning
  - 平均延迟>1000ms → Warning

### 📊 测试与质量

- 单元测试：73 passed, 0 failed, 3 ignored
- Release 编译：31.31s，2个警告
- 代码质量：所有核心功能完整测试覆盖

### 📁 文件变更

**新增文件**：
- src-tauri/src/collector/importer.rs (376行)
- src-tauri/src/collector/scheduler.rs (382行)
- src-tauri/src/collector/tdx.rs (432行真实实现)
- src-tauri/src/monitor/metrics.rs (350行)
- src-tauri/src/db/optimizer.rs (158行)
- src-tauri/src/cmd/monitor.rs (46行)
- migrations/003_add_import_tables.sql
- migrations/004_optimize_storage.sql
- docs/phase3-deepen-summary.md

**总计**: 8个新文件，1503行新增代码

### ⚡ 性能提升

- 异步数据采集，非阻塞设计
- 批量写入优化（100条/批或5秒超时）
- 并发控制（4线程）
- ClickHouse 异步插入模式

### 🐛 已知问题

- rustdx 0.4.2 不提供实时快照API，使用最新1条K线数据作为替代
- ClickHouse 索引需要在表创建时定义（生产环境建议重建表）
- 3个集成测试需要真实服务器，已被忽略

### 🔗 相关文档

- [Phase 3 深化总结](https://github.com/jackluo2012/kaipanla/blob/main/docs/phase3-deepen-summary.md)
- [Phase 3 设计文档](https://github.com/jackluo2012/kaipanla/blob/main/docs/plans/2025-12-25-phase3-data-integration-design.md)

---

## v0.3.0 (2025-12-25) - Phase 3: 数据集成设计

### 数据采集框架
- 采集调度器（交易日判断、定时任务）
- 数据验证器（格式、完整性、异常检测）
- 批量写入器（100条/批或5秒超时）
- 数据缓冲队列（1000条容量）

### ClickHouse 表结构
- collection_status 表（采集状态跟踪）
- data_quality_log 表（数据质量日志）

### API 接口
- POST /api/v1/collection/start - 启动采集
- POST /api/v1/collection/stop - 停止采集
- GET /api/v1/collection/status - 查询状态
- GET /api/v1/data/quality - 数据质量报告

### Tauri Commands
- start_collection
- stop_collection
- get_collection_status
- get_data_quality

---

## v0.2.0 (2025-12-25) - Phase 2: 基础架构

### 核心模块
- 行情服务（Quote）
- 资金流向（MoneyFlow）
- 龙虎榜（DragonTiger）
- 竞价异动（Auction）
- WebSocket 服务

### 测试
- 34个单元测试全部通过
- Debug 和 Release 编译成功

---

## 升级指南

### 从 v0.3.0 升级到 v0.4.0

1. 更新代码：
   ```bash
   git pull origin main
   ```

2. 安装新依赖：
   ```bash
   cd src-tauri
   cargo update
   ```

3. 运行迁移：
   ```bash
   clickhouse-client < migrations/003_add_import_tables.sql
   clickhouse-client < migrations/004_optimize_storage.sql
   ```

4. 重新编译：
   ```bash
   cargo build --release
   ```

5. 配置通达信服务器：
   编辑配置文件，添加服务器地址

6. 启动导入：
   - 通过 API 或 Tauri 命令启动历史数据导入
   - 系统会先导入最近1个月数据
   - 然后后台回填3年历史数据

### 配置示例

```toml
[data_source]
tdx_servers = [
    "124.71.187.122:7709",
    "122.51.120.217:7709"
]
```

---

**最后更新**: 2025-12-26

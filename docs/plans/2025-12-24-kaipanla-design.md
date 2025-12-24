# 开盘啦(KaiPanLa) - 设计文档

**项目代号:** kaipanla
**创建日期:** 2024-12-24
**版本:** 1.0
**状态:** 设计阶段

---

## 项目概述

开盘啦是一款专业的股票分析软件，专注于主力资金分析和短线交易辅助。本项目将完整克隆其核心功能，采用现代化技术栈构建高性能的桌面应用。

### 核心功能

1. **资金监控** - 实时监控大单主力资金流入流出
2. **竞价分析** - 集合竞价异动分析、狙击龙头
3. **龙虎榜** - 实时龙虎榜监测、营业部追踪
4. **数据分析** - 股票分类、区间统计、主力行为研究
5. **行情服务** - 实时行情、大盘直播、盘面情绪
6. **智能选股** - 热点挖掘、选股辅助

### 技术栈

- **前端:** Tauri + React/Vue3 + TypeScript
- **后端:** Rust (模块化单体架构)
- **数据库:** ClickHouse (主力) + SQLite (本地缓存)
- **数据源:** rustdx (通达信数据)

---

## 1. 整体架构

### 系统架构概述

**项目结构:** 采用前后端分离的模块化单体架构

**前端:** Tauri 桌面应用
- Web 技术栈（React/Vue3 + TypeScript）构建 UI
- Tauri 提供原生窗口和系统集成能力
- 通过 Tauri Command 调用 Rust 后端功能

**后端:** Rust 模块化服务
- 数据采集模块: 集成 rustdx 获取通达信数据
- API 服务模块: 提供 RESTful/WebSocket 接口
- 业务逻辑模块: 资金分析、龙虎榜、选股策略等
- 数据持久化: ClickHouse (主力) + SQLite (本地缓存)

**通信方式:**
- 桌面端 ↔ 云端: HTTP/HTTPS API + WebSocket (实时推送)
- 数据采集: 定时任务 + 实时行情监听

### 数据库设计 (ClickHouse)

**表结构:**

```sql
-- 日线表 (rustdx 原生格式)
CREATE TABLE kaipanla.factor (
    date Date,
    code FixedString(6),
    open Float64,
    high Float64,
    low Float64,
    close Float64,
    preclose Float64,
    factor Float64,  -- 复权因子
    volume Float64,
    amount Float64
) ENGINE = MergeTree()
ORDER BY (date, code);

-- 实时行情表
CREATE TABLE kaipanla.quote_realtime (
    datetime DateTime,
    code FixedString(6),
    price Float64,
    volume Float64,
    amount Float64,
    bids Array(Float64),  -- 买盘 5 档
    asks Array(Float64)   -- 卖盘 5 档
) ENGINE = MergeTree()
ORDER BY (datetime, code);

-- 龙虎榜表
CREATE TABLE kaipanla.dragon_tiger (
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
CREATE TABLE kaipanla.money_flow (
    datetime DateTime,
    code FixedString(6),
    main_inflow Float64,    -- 主力流入
    main_outflow Float64,   -- 主力流出
    retail_inflow Float64,  -- 散户流入
    retail_outflow Float64, -- 散户流出
    net_amount Float64      -- 净流入
) ENGINE = MergeTree()
ORDER BY (datetime, code);
```

**关键优势:**
- ClickHouse 列式存储，压缩率高 (1G+ CSV 压缩后 268M)
- 查询性能强 (rustdx 示例: 2s 更完当日数据)
- 集群支持，方便横向扩展
- Rust 的高性能 + ClickHouse 的分析能力

---

## 2. 数据采集与存储模块

### 数据采集架构

**核心组件:**

1. **rustdx 集成层**
   - 直接使用 `rustdx` crate 作为依赖
   - 封装为独立模块 `data-collector`
   - 支持离线数据解析 (day 文件) + 在线实时行情 (eastmoney)

2. **数据采集调度器**
   - 历史数据初始化: 启动时解析通达信 day/gbbq 文件，导入 ClickHouse
   - 实时行情更新: 交易日每 3 秒轮询一次，更新到 ClickHouse
   - 增量更新: 每日收盘后更新日线数据 (复权因子)

3. **数据流**
   - 通达信服务器 → rustdx 解析 → CSV/中间格式 → ClickHouse
   - 东方财富 API → rustdx-east → 实时行情 → 内存表 → 定期 Merge

**性能优化:**
- 批量插入 (每批 1000 条记录)
- 异步 I/O (tokio)
- 连接池复用

---

## 3. 核心业务模块

### 3.1 资金监控模块 (money-flow)

**功能:** 监控主力资金流入流出

**实现:**
- 计算大单成交 (单笔 > 100 万)
- 按股票/板块/市场聚合资金流向
- 实时资金强弱指标
- 量能预测 (基于历史量能模型)

**数据源:** rustdx 分时成交数据 + ClickHouse 聚合查询
**输出:** WebSocket 推送资金流实时数据

### 3.2 竞价分析模块 (auction)

**功能:** 集合竞价异动分析

**实现:**
- 9:15-9:25 竞价数据采集
- 竞价涨跌幅统计
- 异动筛选 (涨跌 > 5%、量比 > 2)
- 龙头股识别 (竞价强势 + 成交量大)

**输出:** 竞价异动列表 + 推送提醒

### 3.3 龙虎榜模块 (dragon-tiger)

**功能:** 实时龙虎榜数据 + 营业部追踪

**实现:**
- 爬取交易所/东方财富龙虎榜数据
- 按概念/营业部分类
- 关联个股协同操作 (同一营业部上榜多只股票)
- 用户订阅自定义组合

**存储:** ClickHouse 龙虎榜历史表

### 3.4 智能选股模块 (stock-screener)

**功能:** 多维度选股策略

**实现:**
- 技术指标过滤 (MACD、KDJ、RSI 等)
- 资金流过滤
- 龙虎榜关联
- 题材热点匹配

**策略引擎:** 可配置的选股规则 DSL

### 3.5 行情服务模块 (market-service)

**功能:** 实时行情 + 大盘直播

**实现:**
- 实时行情推送 (WebSocket)
- 大盘指数监控
- 盘面情绪指标 (涨跌家数、涨停跌停统计)
- 题材板块热度

**模块间通信:**
- 使用 Rust async/await + tokio
- 模块间通过消息传递 (tokio channels)
- 共享 ClickHouse 连接池

---

## 4. 前端架构设计

### 前端技术栈

**核心框架:**
- **UI 框架:** React 18 + TypeScript (或 Vue 3 + TypeScript)
- **状态管理:** Zustand / Pinia (轻量级)
- **路由:** React Router / Vue Router
- **UI 组件库:** Ant Design / Element Plus
- **图表库:** ECharts / TradingView Lightweight Charts (K线图)
- **实时通信:** WebSocket + 原生 EventSource

### Tauri 集成

- 窗口管理: 多窗口支持 (主窗口 + 浮动监控窗)
- 系统托盘: 最小化到托盘，行情提醒弹窗
- 本地存储: localStorage + Tauri FS API (缓存数据)
- 系统通知: 重要异动推送通知

### 前端模块划分

**1. 页面结构**
- 主页面: 行情列表 + 搜索
- 详情页: 个股 K线图 + 资金流 + 龙虎榜
- 竞价页面: 集合竞价异动列表
- 龙虎榜页面: 按概念/营业部分类
- 选股器页面: 策略配置 + 结果展示
- 监控面板: 自选股实时监控

**2. 核心组件**
- `RealtimeQuote` - 实时行情组件 (WebSocket 订阅)
- `KLineChart` - K线图 (支持缩放、十字线、指标切换)
- `MoneyFlowChart` - 资金流向图
- `AuctionList` - 竞价异动列表
- `DragonTigerTable` - 龙虎榜表格
- `StockScreener` - 选股器表单

**3. 数据层**
- API Client: 封装 HTTP/WebSocket 调用
- Query Keys: React Query / VueUse Fetch (缓存 + 请求去重)
- WebSocket Manager: 管理订阅/取消订阅，自动重连

**性能优化:**
- 虚拟滚动 (react-window / vue-virtual-scroller)
- 组件懒加载
- WebSocket 消息节流 (每秒最多更新 10 次)
- 图表数据抽样 (K线超过 1000 根时降采样)

---

## 5. API 设计与通信协议

### RESTful API 设计

**基础路径:** `/api/v1`

**核心端点:**

**行情相关**
```
GET  /stocks          - 股票列表 (搜索、筛选)
GET  /stocks/{code}   - 个股详情 (基本信息、财务数据)
GET  /quotes/{code}   - 实时行情 (5 档行情)
GET  /klines/{code}   - K线数据 (日/周/月/分钟)
GET  /moneyflow/{code} - 资金流向数据
```

**龙虎榜相关**
```
GET  /dragon-tiger    - 龙虎榜列表 (日期筛选、概念筛选)
GET  /dragon-tiger/{broker} - 营业部历史
GET  /dragon-tiger/stocks/{code} - 个股历史上榜
```

**选股相关**
```
POST /screener        - 执行选股策略
GET  /screener/{id}   - 获取策略结果
GET  /screener        - 策略模板列表
```

**竞价相关**
```
GET  /auction         - 当日竞价异动
GET  /auction/history - 历史竞价数据
```

### WebSocket 协议

**连接地址:** `ws://api.kaipanla.com/ws`

**消息格式:** JSON
```json
// 客户端订阅
{
  "action": "subscribe",
  "channel": "quote",
  "codes": ["000001", "600036"]
}

// 服务端推送
{
  "channel": "quote",
  "data": {
    "code": "000001",
    "price": 10.5,
    "change": 0.5,
    "volume": 100000,
    "timestamp": 1735012800
  }
}
```

**支持的频道:**
- `quote` - 实时行情推送
- `moneyflow` - 资金流向推送
- `auction` - 竞价异动推送
- `dragon-tiger` - 龙虎榜更新

**Rust 后端实现:**
- 使用 `tokio-tungstenite` 处理 WebSocket
- 每个连接维护订阅列表
- 广播消息到所有订阅者
- 心跳检测 (30 秒 ping/pong)

### 认证与授权

**方案:** JWT Token
- 登录后返回 JWT Token
- Token 存储在 localStorage
- 每次 API 请求在 Header 携带: `Authorization: Bearer <token>`
- Token 过期时间: 7 天

---

## 6. 错误处理与测试策略

### 错误处理体系

**分层错误处理:**

**1. 数据采集层**
- **网络错误:** 通达信服务器连接失败 → 自动切换备用服务器
- **数据解析错误:** 跳过异常记录，记录日志 + 告警
- **重试策略:** 指数退避 (1s → 2s → 4s → 8s)，最多 3 次

**2. 业务逻辑层**
- **自定义错误类型:** 使用 Rust `thiserror` 定义业务错误
- **错误传播:** 使用 `anyhow` 统一错误处理
- **降级策略:**
  - ClickHouse 查询超时 → 返回缓存数据
  - 实时行情缺失 → 使用最后有效价格

**3. API 层**
- **HTTP 状态码规范:**
  - 400 - 参数错误
  - 401 - 未认证
  - 429 - 请求限流
  - 500 - 服务器错误
- **统一响应格式:**
  ```json
  {
    "code": 0,
    "message": "success",
    "data": {...}
  }
  ```

**4. 前端层**
- **错误边界:** React Error Boundary / Vue onErrorCaptured
- **用户友好提示:** 根据错误类型显示不同提示
- **自动重连:** WebSocket 断线自动重连

### 测试策略

**1. 单元测试**
- **覆盖率目标:** 核心业务逻辑 ≥ 80%
- **工具:** Rust 内置 `cargo test`
- **Mock:** 使用 `mockito` Mock HTTP 请求
- **测试数据:** 准备样本 day/gbbq 文件 (少量股票)

**2. 集成测试**
- **ClickHouse 集成:** 使用 Docker 启动测试用 ClickHouse
- **API 测试:** 使用 `reqwest` 测试 REST API
- **WebSocket 测试:** 使用 `tokio-tungstenite` 客户端测试

**3. 端到端测试**
- **前端 E2E:** Playwright / Cypress
- **测试场景:**
  - 用户登录 → 搜索股票 → 查看 K线
  - 订阅实时行情 → 验证数据更新
  - 执行选股策略 → 验证结果

**4. 性能测试**
- **工具:** Apache Bench / k6
- **指标:**
  - API 响应时间 < 200ms (P95)
  - WebSocket 推送延迟 < 100ms
  - 并发用户数 > 1000

**5. 数据质量测试**
- **数据完整性:** 校验日线数据连续性
- **数据准确性:** 对比官方数据源 (抽样)
- **边界测试:** 涨停、跌停、停牌等特殊情况

---

## 7. 部署与运维架构

### 部署架构

**云端后端部署:**

**1. 服务器配置**
- **推荐配置:** 4 核 CPU + 16GB 内存 + 200GB SSD
- **操作系统:** Ubuntu 22.04 LTS
- **容器化:** Docker + Docker Compose

**2. 服务组件**
```yaml
services:
  # ClickHouse 数据库
  clickhouse:
    image: clickhouse/clickhouse-server:latest
    ports: ["8123:8123"]
    volumes:
      - clickhouse_data:/var/lib/clickhouse

  # Rust 后端 API
  backend:
    image: kaipanla-backend:latest
    ports: ["8000:8000"]
    environment:
      - CLICKHOUSE_URL=http://clickhouse:8123
      - DATABASE_URL=...
    depends_on: [clickhouse]

  # Nginx 反向代理
  nginx:
    image: nginx:alpine
    ports: ["80:80", "443:443"]
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
```

**3. CI/CD 流程**
- **代码仓库:** GitHub / GitLab
- **自动化构建:** GitHub Actions / GitLab CI
  - Rust: `cargo test` → `cargo build --release` → Docker 镜像
  - 前端: `npm test` → `npm run build` → Tauri 打包
- **自动部署:** 推送到 main 分支自动构建 + 部署到测试环境

### 桌面应用分发

**Tauri 打包与发布:**
- **支持平台:** Windows (msi/exe), macOS (dmg/app), Linux (deb/appimage)
- **自动更新:** Tauri 内置更新器
- **发布渠道:** GitHub Releases + 官网下载

### 监控与运维

**1. 日志管理**
- **日志框架:** Rust `tracing` + `tracing-subscriber`
- **日志格式:** JSON 结构化日志
- **日志聚合:** Loki + Grafana (可选)

**2. 监控指标**
- **系统指标:** CPU、内存、磁盘、网络
- **业务指标:**
  - API 请求量 / 响应时间 / 错误率
  - WebSocket 连接数 / 消息吞吐
  - 数据采集任务状态
- **工具:** Prometheus + Grafana

**3. 告警策略**
- **数据采集失败:** 连续 3 次失败 → 发送告警
- **API 错误率:** 5 分钟内 > 5% → 告警
- **数据库异常:** ClickHouse 连接失败 → 紧急告警
- **通知渠道:** 邮件 + 企业微信/钉钉

**4. 备份策略**
- **ClickHouse 备份:** 每日全量 + 增量
- **备份保留:** 保留最近 30 天
- **异地备份:** 可选上传到 OSS

### 成本估算

**云服务器 (按月):**
- 轻量应用服务器 (4核16G): 约 ¥300-500/月
- ClickHouse 存储 (200GB): 约 ¥50/月
- 带宽 (5Mbps): 约 ¥100/月
- **总计:** 约 ¥500-700/月

---

## 8. 项目实施计划

### 阶段划分

**Phase 1: 基础设施 (2-3 周)**
- [ ] 项目脚手架搭建 (Tauri + Rust)
- [ ] ClickHouse 部署与表结构设计
- [ ] rustdx 集成与数据采集模块
- [ ] 基础 API 框架搭建

**Phase 2: 核心功能 (4-6 周)**
- [ ] 实时行情服务
- [ ] 资金监控模块
- [ ] K线图表组件
- [ ] 龙虎榜模块
- [ ] 竞价分析模块

**Phase 3: 高级功能 (3-4 周)**
- [ ] 智能选股模块
- [ ] WebSocket 实时推送
- [ ] 用户认证与权限
- [ ] 数据分析与统计

**Phase 4: 优化与发布 (2-3 周)**
- [ ] 性能优化
- [ ] 测试覆盖
- [ ] 打包与分发
- [ ] 文档完善

### 技术风险与应对

**风险 1: 数据源稳定性**
- **应对:** 维护备用服务器列表，实现自动切换
- **备选:** 同时集成多个数据源 (通达信 + 东方财富)

**风险 2: ClickHouse 性能**
- **应对:** 合理设计分区和索引，定期优化查询
- **监控:** 建立性能监控告警

**风险 3: Tauri 桌面应用兼容性**
- **应对:** 多平台测试，提前验证兼容性
- **降级:** 提供 Web 版本作为备选

---

## 9. 参考资料

**开源项目:**
- [rustdx](https://github.com/zjp-CN/rustdx) - A股数据获取工具
- [mootdx](https://github.com/mootdx/mootdx) - 通达信数据读取接口
- [injoyai/tdx](https://github.com/injoyai/tdx) - Go语言通达信协议解析

**技术文档:**
- [Tauri 官方文档](https://tauri.app/)
- [ClickHouse 文档](https://clickhouse.com/docs)
- [Tokio 异步运行时](https://tokio.rs/)

**行业参考:**
- [开盘啦官网](https://www.kaipanla.com/)
- 通达信数据格式规范
- 量化交易最佳实践

---

## 附录

### 服务器列表 (备用)

通达信服务器列表 (来自 injoyai/tdx):
```
上海服务器:
- 124.71.187.122:7709 (华为云)
- 122.51.120.217:7709 (腾讯云)
- 111.229.247.189:7709 (腾讯云)

北京服务器:
- 121.36.54.217:7709 (华为云)
- 121.36.81.195:7709 (华为云)

广州服务器:
- 124.71.85.110:7709 (华为云)
- 139.9.51.18:7709 (华为云)
```

### 许可证

MIT License

---

**文档版本:** 1.0
**最后更新:** 2024-12-24
**维护者:** 开盘啦开发团队

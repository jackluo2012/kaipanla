# 开盘啦 (KaiPanLa) - 功能测试指南

**测试日期**: 2025-12-25
**版本**: v0.2.0

---

## ✅ 已完成的测试

### 1. 单元测试 ✅

**测试命令:**
```bash
cargo test --package kaipanla --lib
```

**测试结果:**
```
running 34 tests
✅ test collector::parser::tests::test_parse_day_data_empty ... ok
✅ test collector::tdx::tests::test_tdx_client_creation ... ok
✅ test models::dragon_tiger::tests::test_broker_stats ... ok
✅ test models::dragon_tiger::tests::test_dragon_reason_other ... ok
✅ test models::dragon_tiger::tests::test_dragon_tiger_net ... ok
✅ test models::dragon_tiger::tests::test_dragon_tiger_serialize ... ok
✅ test models::money_flow::tests::test_big_trade_judge ... ok
✅ test models::money_flow::tests::test_money_flow_calculations ... ok
✅ test models::quote::tests::test_quote_calculations ... ok
✅ test models::stock::tests::test_market_from_code ... ok
✅ test service::quote_service::tests (5 tests) ... ok
✅ test service::money_flow_service::tests (9 tests) ... ok
✅ test service::dragon_tiger_service::tests (5 tests) ... ok
✅ test websocket::message::tests (3 tests) ... ok
✅ test websocket::server::tests (2 tests) ... ok

test result: ok. 34 passed; 0 failed; 0 ignored
```

**测试覆盖率**: 34/34 ✅ (100%)

---

### 2. 编译测试 ✅

**Debug 编译:**
```bash
cargo build --package kaipanla
```
**结果**: ✅ 成功 (5.16秒)

**Release 编译:**
```bash
cargo build --release --package kaipanla
```
**结果**: ✅ 成功 (47.01秒)
**警告**: 1个 (未使用的字段 `tdx_client`)

---

### 3. ClickHouse 数据库测试 ✅

**容器状态:**
```bash
docker ps | grep clickhouse
```
**结果**: ✅ 运行中 (49fcb130e2fe)

**连接测试:**
```bash
curl http://localhost:8123/ping
```
**结果**: ✅ Ok

---

## 🧪 待执行的测试

### 1. API 端点测试

**前置条件**: 启动 Tauri 应用

```bash
npm run tauri dev
```

**测试用例:**

#### 1.1 健康检查
```bash
curl http://localhost:8000/health
```

**预期输出:**
```json
{
  "status": "ok",
  "service": "kaipanla"
}
```

#### 1.2 Ping 测试
```bash
curl http://localhost:8000/api/v1/ping
```

**预期输出:**
```json
{
  "message": "pong"
}
```

#### 1.3 获取股票行情
```bash
curl http://localhost:8000/api/v1/quote/000001
```

**预期输出:**
```json
{
  "code": "000001",
  "name": "测试股票",
  "price": 10.5,
  "preclose": 10.0,
  "change": 0.5,
  "change_pct": 5.0
}
```

#### 1.4 获取资金流向
```bash
curl http://localhost:8000/api/v1/moneyflow/000001
```

**预期输出:**
```json
{
  "code": "000001",
  "main_inflow": 5000.0,
  "main_outflow": 3000.0,
  "main_net": 2000.0,
  "retail_inflow": 2000.0,
  "retail_outflow": 4000.0,
  "net_amount": 0.0
}
```

#### 1.5 获取龙虎榜
```bash
curl http://localhost:8000/api/v1/dragon-tiger
```

**预期输出:**
```json
[]
```

#### 1.6 获取竞价异动
```bash
curl http://localhost:8000/api/v1/auction/anomalies
```

**预期输出:**
```json
[]
```

---

### 2. Tauri Commands 测试

**测试方式**: 通过前端调用或 Tauri API

#### 2.1 获取股票行情
```typescript
import { invoke } from '@tauri-apps/api/tauri';
const quote = await invoke('get_quote', { code: '000001' });
```

**预期返回**: Quote 对象

#### 2.2 获取资金流向
```typescript
const moneyFlow = await invoke('get_money_flow', { code: '000001' });
```

**预期返回**: MoneyFlow 对象

#### 2.3 获取龙虎榜
```typescript
const dragonTiger = await invoke('get_dragon_tiger_list', { date: '2025-12-25' });
```

**预期返回**: DragonTiger 数组

#### 2.4 获取竞价异动
```typescript
const anomalies = await invoke('get_auction_anomalies');
```

**预期返回**: AuctionAnomaly 数组

---

### 3. WebSocket 连接测试

**连接测试** (使用 websocat 或 wscat):

```bash
# 安装 wscat
npm install -g wscat

# 连接 WebSocket (需要先启动 Tauri 应用)
wscat -c ws://localhost:8000/ws
```

**测试消息:**

#### 3.1 订阅行情
```json
{
  "action": "subscribe",
  "channel": "quote",
  "codes": ["000001", "600036"]
}
```

**预期响应**: Pong 消息

#### 3.2 心跳测试
```json
{
  "action": "ping"
}
```

**预期响应**:
```json
{
  "action": "pong"
}
```

---

## 📊 测试报告模板

| 测试项 | 状态 | 备注 |
|--------|------|------|
| 单元测试 | ✅ PASS | 34/34 通过 |
| Debug 编译 | ✅ PASS | 5.16秒 |
| Release 编译 | ✅ PASS | 47.01秒 |
| ClickHouse 连接 | ✅ PASS | Ok |
| API 健康检查 | ⏳ 待测 | 需要启动应用 |
| API 行情查询 | ⏳ 待测 | 需要启动应用 |
| API 资金流向 | ⏳ 待测 | 需要启动应用 |
| API 龙虎榜 | ⏳ 待测 | 需要启动应用 |
| API 竞价异动 | ⏳ 待测 | 需要启动应用 |
| Tauri Commands | ⏳ 待测 | 需要启动应用 |
| WebSocket 连接 | ⏳ 待测 | 需要启动应用 |

---

## 🔧 故障排查

### 问题 1: ClickHouse 连接失败

**检查:**
```bash
docker ps -a | grep clickhouse
```

**解决:**
```bash
docker-compose up -d clickhouse
```

### 问题 2: API 端点无响应

**检查:**
```bash
# 检查端口是否被占用
lsof -i :8000

# 检查应用日志
# 查看终端输出
```

**解决:**
```bash
# 停止占用端口的进程
kill -9 <PID>

# 重新启动应用
npm run tauri dev
```

### 问题 3: WebSocket 连接失败

**检查:**
```bash
# 测试端口
telnet localhost 8000
```

**解决:**
- 确保 Tauri 应用已启动
- 检查防火墙设置
- 查看 WebSocket 服务器日志

---

## 📝 测试清单

在启动 Tauri 应用后，按顺序执行以下测试：

- [ ] 1. 应用正常启动
- [ ] 2. API 健康检查 (/health)
- [ ] 3. API Ping 测试 (/api/v1/ping)
- [ ] 4. 获取股票行情 (/api/v1/quote/:code)
- [ ] 5. 获取资金流向 (/api/v1/moneyflow/:code)
- [ ] 6. 获取龙虎榜 (/api/v1/dragon-tiger)
- [ ] 7. 获取竞价异动 (/api/v1/auction/anomalies)
- [ ] 8. WebSocket 连接
- [ ] 9. WebSocket 订阅行情
- [ ] 10. WebSocket 心跳测试
- [ ] 11. Tauri Commands 调用
- [ ] 12. 错误处理测试

---

## 🎯 下一步

完成所有测试后：

1. **记录测试结果**: 更新测试报告
2. **修复问题**: 如有失败，记录并修复
3. **集成测试**: 在 Phase 3 中添加端到端测试
4. **性能测试**: 压力测试和性能基准

---

**测试负责人**: 开发团队
**文档版本**: 1.0
**最后更新**: 2025-12-25

# 开盘啦 (KaiPanLa)

> 专业的股票分析软件 - 基于 Rust + Tauri + ClickHouse

## 项目简介

开盘啦是一款专注于主力资金分析和短线交易辅助的专业炒股软件。本项目采用现代化技术栈完整克隆其核心功能，提供实时行情监控、资金流向分析、龙虎榜追踪、智能选股等功能。

## 核心功能

- 📊 **实时行情** - 5 档行情、K 线图、分时图
- 💰 **资金监控** - 主力资金流入流出实时分析
- ⚡ **竞价分析** - 集合竞价异动、龙头狙击
- 🐉 **龙虎榜** - 实时龙虎榜、营业部追踪
- 🎯 **智能选股** - 多维度选股策略
- 📈 **数据分析** - 区间统计、技术指标

## 技术栈

- **前端**: Tauri + React/Vue3 + TypeScript
- **后端**: Rust
- **数据库**: ClickHouse + SQLite
- **数据源**: rustdx (通达信)

## 项目状态

🎨 **当前阶段**: 设计阶段

详细设计文档请查看: [docs/plans/2025-12-24-kaipanla-design.md](./docs/plans/2025-12-24-kaipanla-design.md)

## 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+
- ClickHouse 23+
- Tauri CLI

### 安装

```bash
# 克隆项目
git clone https://github.com/yourusername/kaipanla.git
cd kaipanla

# 安装依赖
npm install
cargo build

# 启动开发环境
npm run tauri dev
```

## 开发计划

- [x] 项目架构设计
- [ ] 基础设施搭建
- [ ] 数据采集模块
- [ ] 核心业务功能
- [ ] 前端界面开发
- [ ] 测试与优化

## 贡献指南

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 免责声明

本项目仅供学习交流使用，不构成任何投资建议。股市有风险，投资需谨慎。

---

**参考资料**: [开盘啦官网](https://www.kaipanla.com/)

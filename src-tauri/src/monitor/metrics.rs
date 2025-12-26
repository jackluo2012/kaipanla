//! 采集监控和告警系统
//!
//! 实时监控数据采集状态，提供指标采集和告警功能

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// 采集统计指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMetrics {
    // 基本统计
    pub total_stocks: usize,           // 总股票数
    pub success_count: u64,            // 成功次数
    pub failed_count: u64,             // 失败次数
    pub timeout_count: u64,            // 超时次数

    // 性能指标
    pub avg_latency_ms: f64,           // 平均延迟（毫秒）
    pub last_update: DateTime<Utc>,    // 最后更新时间
    pub uptime_secs: u64,              // 运行时长（秒）

    // 服务器状态
    pub servers: Vec<ServerHealth>,    // 服务器健康状态

    // 数据质量
    pub quality_score: f64,            // 质量分数（0-100）
    pub data_freshness_secs: u64,      // 数据新鲜度（秒）
}

/// 服务器健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHealth {
    pub addr: String,                  // 服务器地址
    pub is_healthy: bool,              // 是否健康
    pub last_check: DateTime<Utc>,     // 最后检查时间
    pub fail_count: usize,             // 失败次数
    pub avg_latency_ms: f64,           // 平均延迟
}

/// 告警级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,       // 信息
    Warning,    // 警告
    Error,      // 错误
    Critical,   // 严重
}

/// 告警信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub context: String,              // 上下文信息
}

/// 监控器
pub struct CollectorMonitor {
    // 原子计数器（高性能）
    success_count: Arc<AtomicU64>,
    failed_count: Arc<AtomicU64>,
    timeout_count: Arc<AtomicU64>,

    // 延迟统计
    latency_samples: Arc<RwLock<Vec<f64>>>,

    // 服务器状态
    servers: Arc<RwLock<Vec<ServerHealth>>>,

    // 启动时间
    start_time: DateTime<Utc>,

    // 股票总数
    total_stocks: AtomicUsize,
}

impl CollectorMonitor {
    /// 创建新的监控器
    pub fn new(servers: Vec<String>) -> Self {
        let server_health = servers.into_iter()
            .map(|addr| ServerHealth {
                addr,
                is_healthy: true,
                last_check: Utc::now(),
                fail_count: 0,
                avg_latency_ms: 0.0,
            })
            .collect();

        Self {
            success_count: Arc::new(AtomicU64::new(0)),
            failed_count: Arc::new(AtomicU64::new(0)),
            timeout_count: Arc::new(AtomicU64::new(0)),
            latency_samples: Arc::new(RwLock::new(Vec::with_capacity(1000))),
            servers: Arc::new(RwLock::new(server_health)),
            start_time: Utc::now(),
            total_stocks: AtomicUsize::new(0),
        }
    }

    /// 记录成功采集
    pub fn record_success(&self) {
        self.success_count.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录失败采集
    pub fn record_failure(&self) {
        self.failed_count.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录超时
    pub fn record_timeout(&self) {
        self.timeout_count.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录延迟
    pub async fn record_latency(&self, latency_ms: f64) {
        let mut samples = self.latency_samples.write().await;
        samples.push(latency_ms);

        // 保持最多1000个样本
        if samples.len() > 1000 {
            samples.remove(0);
        }
    }

    /// 计算平均延迟
    pub async fn get_avg_latency(&self) -> f64 {
        let samples = self.latency_samples.read().await;
        if samples.is_empty() {
            return 0.0;
        }

        let sum: f64 = samples.iter().sum();
        sum / samples.len() as f64
    }

    /// 更新服务器状态
    pub async fn update_server_health(&self, addr: String, is_healthy: bool, latency_ms: f64) {
        let mut servers = self.servers.write().await;

        for server in servers.iter_mut() {
            if server.addr == addr {
                server.is_healthy = is_healthy;
                server.last_check = Utc::now();

                if is_healthy {
                    server.fail_count = 0;
                    // 更新平均延迟
                    server.avg_latency_ms = (server.avg_latency_ms * 0.9 + latency_ms * 0.1);
                } else {
                    server.fail_count += 1;
                }

                tracing::debug!(
                    "服务器 {} 状态更新: healthy={}, latency={}ms",
                    addr, is_healthy, latency_ms
                );
                return;
            }
        }

        // 如果找不到对应服务器，添加新的
        servers.push(ServerHealth {
            addr,
            is_healthy,
            last_check: Utc::now(),
            fail_count: if is_healthy { 0 } else { 1 },
            avg_latency_ms: latency_ms,
        });
    }

    /// 设置股票总数
    pub fn set_total_stocks(&self, count: usize) {
        self.total_stocks.store(count, Ordering::SeqCst);
    }

    /// 获取当前指标
    pub async fn get_metrics(&self) -> CollectionMetrics {
        let success_count = self.success_count.load(Ordering::SeqCst);
        let failed_count = self.failed_count.load(Ordering::SeqCst);
        let timeout_count = self.timeout_count.load(Ordering::SeqCst);
        let total_stocks = self.total_stocks.load(Ordering::SeqCst);

        let avg_latency_ms = self.get_avg_latency().await;
        let servers = self.servers.read().await.clone();

        let now = Utc::now();
        let uptime_secs = (now - self.start_time).num_seconds() as u64;

        // 计算质量分数
        let total_attempts = success_count + failed_count + timeout_count;
        let quality_score = if total_attempts > 0 {
            let success_rate = success_count as f64 / total_attempts as f64;
            success_rate * 100.0
        } else {
            100.0
        };

        // 数据新鲜度（最后更新时间到现在）
        let data_freshness_secs = if uptime_secs > 0 {
            uptime_secs.min(300) // 最多显示5分钟
        } else {
            0
        };

        CollectionMetrics {
            total_stocks,
            success_count,
            failed_count,
            timeout_count,
            avg_latency_ms,
            last_update: now,
            uptime_secs,
            servers,
            quality_score,
            data_freshness_secs,
        }
    }

    /// 检查告警条件
    pub async fn check_alerts(&self) -> Vec<Alert> {
        let metrics = self.get_metrics().await;
        let mut alerts = Vec::new();

        // 告警规则 1: 连续失败10次
        if metrics.failed_count >= 10 {
            alerts.push(Alert {
                level: AlertLevel::Error,
                message: format!("连续失败 {} 次", metrics.failed_count),
                timestamp: Utc::now(),
                context: "collection".to_string(),
            });
        }

        // 告警规则 2: 所有服务器不健康
        let all_unhealthy = metrics.servers.iter()
            .all(|s| !s.is_healthy);

        if all_unhealthy && !metrics.servers.is_empty() {
            alerts.push(Alert {
                level: AlertLevel::Critical,
                message: "所有服务器不可用".to_string(),
                timestamp: Utc::now(),
                context: "servers".to_string(),
            });
        }

        // 告警规则 3: 数据质量异常率>10%
        if metrics.quality_score < 90.0 && metrics.success_count + metrics.failed_count > 100 {
            alerts.push(Alert {
                level: AlertLevel::Warning,
                message: format!("数据质量分数: {:.1}%", metrics.quality_score),
                timestamp: Utc::now(),
                context: "quality".to_string(),
            });
        }

        // 告警规则 4: 平均延迟过高
        if metrics.avg_latency_ms > 1000.0 {
            alerts.push(Alert {
                level: AlertLevel::Warning,
                message: format!("平均延迟过高: {:.1}ms", metrics.avg_latency_ms),
                timestamp: Utc::now(),
                context: "performance".to_string(),
            });
        }

        alerts
    }

    /// 重置统计计数
    pub fn reset_counters(&self) {
        self.success_count.store(0, Ordering::SeqCst);
        self.failed_count.store(0, Ordering::SeqCst);
        self.timeout_count.store(0, Ordering::SeqCst);

        tracing::info!("监控计数器已重置");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor_creation() {
        let servers = vec![
            "124.71.187.122:7709".to_string(),
            "122.51.120.217:7709".to_string(),
        ];

        let monitor = CollectorMonitor::new(servers);
        assert_eq!(monitor.servers.read().await.len(), 2);
    }

    #[tokio::test]
    async fn test_record_metrics() {
        let monitor = CollectorMonitor::new(vec!["localhost:7709".to_string()]);

        monitor.record_success();
        monitor.record_success();
        monitor.record_failure();

        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.success_count, 2);
        assert_eq!(metrics.failed_count, 1);
    }

    #[tokio::test]
    async fn test_latency_tracking() {
        let monitor = CollectorMonitor::new(vec!["localhost:7709".to_string()]);

        monitor.record_latency(10.0).await;
        monitor.record_latency(20.0).await;
        monitor.record_latency(30.0).await;

        let avg = monitor.get_avg_latency().await;
        assert_eq!(avg, 20.0);
    }

    #[tokio::test]
    async fn test_alert_conditions() {
        let monitor = CollectorMonitor::new(vec!["localhost:7709".to_string()]);

        // 测试失败告警
        for _ in 0..10 {
            monitor.record_failure();
        }

        let alerts = monitor.check_alerts().await;
        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].level, AlertLevel::Error);
    }

    #[tokio::test]
    async fn test_server_health_update() {
        let monitor = CollectorMonitor::new(vec!["localhost:7709".to_string()]);

        monitor.update_server_health("localhost:7709".to_string(), false, 0.0).await;

        let servers = monitor.servers.read().await;
        assert_eq!(servers[0].is_healthy, false);
        assert_eq!(servers[0].fail_count, 1);
    }
}

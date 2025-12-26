//! 监控和告警模块

pub mod metrics;

pub use metrics::{
    CollectorMonitor,
    CollectionMetrics,
    ServerHealth,
    Alert,
    AlertLevel,
};

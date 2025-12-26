//! ClickHouse 存储优化器
//!
//! 提供查询优化、批量写入、索引管理等功能

use crate::db::Client;
use tracing::info;

/// ClickHouse 优化配置
#[derive(Debug, Clone)]
pub struct OptimizeConfig {
    pub async_insert: bool,           // 异步插入
    pub wait_for_async_insert: bool,  // 等待异步插入完成
    pub max_insert_threads: u32,      // 最大插入线程数
    pub max_insert_block_size: u32,   // 最大插入块大小（字节）
    pub batch_size: usize,            // 批量插入行数
    pub batch_timeout_secs: u64,      // 批量超时（秒）
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            async_insert: true,
            wait_for_async_insert: false,
            max_insert_threads: 4,
            max_insert_block_size: 1048576, // 1MB
            batch_size: 100,
            batch_timeout_secs: 5,
        }
    }
}

/// ClickHouse 优化器
pub struct ClickHouseOptimizer {
    config: OptimizeConfig,
}

impl ClickHouseOptimizer {
    /// 创建新的优化器
    pub fn new(_client: &Client) -> Self {
        Self {
            config: OptimizeConfig::default(),
        }
    }

    /// 设置优化配置
    pub fn with_config(mut self, config: OptimizeConfig) -> Self {
        self.config = config;
        self
    }

    /// 获取优化配置
    pub fn config(&self) -> &OptimizeConfig {
        &self.config
    }

    /// 生成优化后的 SQL 语句前缀
    pub fn get_insert_settings_sql(&self) -> String {
        let mut settings = Vec::new();

        if self.config.async_insert {
            settings.push("async_insert = 1".to_string());
        }

        if !self.config.wait_for_async_insert {
            settings.push("wait_for_async_insert = 0".to_string());
        }

        settings.push(format!("max_insert_threads = {}", self.config.max_insert_threads));
        settings.push(format!("max_insert_block_size = {}", self.config.max_insert_block_size));

        if !settings.is_empty() {
            format!("SETTINGS {}", settings.join(", "))
        } else {
            String::new()
        }
    }

    /// 记录优化配置
    pub fn log_config(&self) {
        info!("ClickHouse 优化配置:");
        info!("  - 异步插入: {}", self.config.async_insert);
        info!("  - 批量大小: {} 行", self.config.batch_size);
        info!("  - 批量超时: {} 秒", self.config.batch_timeout_secs);
        info!("  - 最大插入线程: {}", self.config.max_insert_threads);
    }
}

/// 表统计信息
#[derive(Debug, Clone)]
pub struct TableStats {
    pub table_name: String,
    pub size: String,      // 格式化后的大小（如 "1.23 GiB"）
    pub rows: u64,         // 总行数
    pub parts: u32,        // 分区数
}

/// 查询统计信息
#[derive(Debug, Clone)]
pub struct QueryStats {
    pub query: String,
    pub duration_ms: u64,
    pub read_rows: u64,
    pub written_rows: u64,
}

/// 表健康状态
#[derive(Debug, Clone)]
pub struct TableHealth {
    pub table_name: String,
    pub is_healthy: bool,
    pub health_score: u32,  // 0-100
    pub issues: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_config_default() {
        let config = OptimizeConfig::default();
        assert_eq!(config.async_insert, true);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.batch_timeout_secs, 5);
    }

    #[test]
    fn test_insert_settings_sql() {
        let config = crate::config::DatabaseConfig {
            clickhouse_url: "localhost:8123".to_string(),
            sqlite_path: "/tmp/test.db".into(),
        };

        // 创建一个运行时的测试
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt.block_on(async {
            Client::new(&config).await.unwrap()
        });

        let optimizer = ClickHouseOptimizer::new(&client);

        let sql = optimizer.get_insert_settings_sql();
        assert!(sql.contains("async_insert = 1"));
        assert!(sql.contains("wait_for_async_insert = 0"));
    }

    #[test]
    fn test_custom_config() {
        let config = OptimizeConfig {
            async_insert: false,
            batch_size: 200,
            ..Default::default()
        };

        assert_eq!(config.async_insert, false);
        assert_eq!(config.batch_size, 200);
    }
}

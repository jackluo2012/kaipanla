use clickhouse_rs::Pool;
use crate::config::DatabaseConfig;
use crate::error::Result;

/// ClickHouse 客户端池
pub struct Client {
    pool: Pool,
}

impl Client {
    /// 创建新的 ClickHouse 客户端
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let pool = Pool::new(config.clickhouse_url.as_str());

        // 测试连接
        let _conn = pool.get_handle();

        tracing::info!("ClickHouse 连接成功: {}", config.clickhouse_url);

        Ok(Self { pool })
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}
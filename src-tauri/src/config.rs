use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub data_source: DataSourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub clickhouse_url: String,
    pub sqlite_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub tdx_servers: Vec<String>,
    pub update_interval_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                clickhouse_url: "http://localhost:8123".to_string(),
                sqlite_path: PathBuf::from("kaipanla.db"),
            },
            api: ApiConfig {
                host: "127.0.0.1".to_string(),
                port: 8000,
            },
            data_source: DataSourceConfig {
                tdx_servers: vec![
                    "124.71.187.122:7709".to_string(),
                    "122.51.120.217:7709".to_string(),
                ],
                update_interval_secs: 3,
            },
        }
    }
}

/// 从文件加载配置
pub fn load_config(path: &str) -> crate::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| crate::error::AppError::Config(e.to_string()))?;
    Ok(config)
}

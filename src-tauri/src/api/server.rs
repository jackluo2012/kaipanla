use std::net::SocketAddr;
use crate::api::routes::create_router;
use crate::config::ApiConfig;
use crate::error::{Result, AppError};

pub struct ApiServer {
    addr: SocketAddr,
}

impl ApiServer {
    pub fn new(config: &ApiConfig) -> Self {
        let addr = format!("{}:{}", config.host, config.port)
            .parse()
            .expect("无效的地址");

        Self { addr }
    }

    pub async fn run(self) -> Result<()> {
        let app = create_router();

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        tracing::info!("API 服务器启动: http://{}", self.addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
}
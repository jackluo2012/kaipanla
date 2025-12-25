use thiserror::Error;

/// 应用统一错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("数据库错误: {0}")]
    Database(String),

    #[error("网络错误: {0}")]
    Network(String),

    #[error("数据解析错误: {0}")]
    Parse(String),

    #[error("配置错误: {0}")]
    Config(String),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

/// 应用统一 Result 类型
pub type Result<T> = std::result::Result<T, AppError>;

// 为 HTTP 响应实现的错误转换
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

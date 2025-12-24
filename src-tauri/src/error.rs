//! 错误处理模块 (Task 2)

use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("通用错误: {0}")]
    Custom(String),
}

/// 实现 serde::Serialize 以便可以跨 FFI 边界传递错误
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// 应用结果类型
pub type Result<T> = std::result::Result<T, AppError>;

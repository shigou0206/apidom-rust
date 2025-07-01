use std::fmt;

/// 路径片段类型
pub type Path = Vec<String>;

/// 字段错误，包含路径和错误消息
#[derive(Debug, Clone)]
pub struct FieldError {
    pub path: Path,
    pub message: String,
}

impl FieldError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            path: Vec::new(),
            message: message.into(),
        }
    }

    pub fn with_path(mut self, path: Path) -> Self {
        self.path = path;
        self
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "{} at {}", self.message, self.path.join("."))
        }
    }
}

impl std::error::Error for FieldError {}

/// 通用结果别名
pub type Result<T> = std::result::Result<T, FieldError>;

/// 语义验证错误，沿用 FieldError 以保持统一
pub type ValidationError = FieldError; 
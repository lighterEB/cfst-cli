use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    FetchFailed(String),
    ParseFailed(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFailed(msg) => write!(f, "请求失败：{}", msg),
            Self::ParseFailed(msg) => write!(f, "解析失败：{}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

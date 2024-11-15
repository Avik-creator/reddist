pub mod types;

#[derive(Debug)]
pub enum RespError{
    InvalidBulkString(String),
    InvalidSimpleString(String),
    Other(String),
}

impl std::fmt::Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespError::Other(msg) => msg.as_str().fmt(f),
            RespError::InvalidBulkString(msg) => msg.as_str().fmt(f),
            RespError::InvalidSimpleString(msg) => msg.as_str().fmt(f),
        }
    }
}
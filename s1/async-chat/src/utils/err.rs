use std::error::Error;

/// todo 使用anyhow定义error, 如果实现一种通用的error方案， 这里为什么需要'static？
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

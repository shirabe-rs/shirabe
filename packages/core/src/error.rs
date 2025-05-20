use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrameworkError {
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError), // 如果使用 config-rs
    #[error("WebSocket 连接错误: {0}")]
    WebSocketConnection(String),
    #[error("WebSocket 错误: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    // #[error("HTTP 请求错误: {0}")]
    // Http(#[from] reqwest::Error), // 如果使用 reqwest
    #[error("JSON 序列化/反序列化错误: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Satori API 错误: code={code}, message={message}")]
    SatoriApi { code: i32, message: String },
    #[error("Satori 事件解析错误: {0}")]
    EventParsing(String),
    #[error("鉴权失败: {0}")]
    AuthFailed(String),
    #[error("插件错误: {0}")]
    Plugin(String),
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("URL 解析错误: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("内部错误: {0}")]
    Internal(String),
}

pub type FrameworkResult<T> = Result<T, FrameworkError>;

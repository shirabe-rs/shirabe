use tracing_subscriber::{EnvFilter, fmt};

/// 初始化日志系统
///
/// # Panics
///
/// 如果日志系统初始化失败，则会 panic。
pub fn init_logger() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_target(true) // 显示日志来源模块
        .with_line_number(true) // 显示日志来源行号
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracing subscriber");
}

/// 一个简单的函数，用于演示日志记录
#[allow(dead_code)]
fn example_log_usage() {
    tracing::info!("This is an info message from the logger.");
    tracing::warn!("This is a warning message.");
    tracing::error!("This is an error message.");
    tracing::debug!("This is a debug message, will only show if RUST_LOG includes this level.");
    tracing::trace!("This is a trace message, very verbose.");

    let data = "some important data";
    tracing::info!(data, "Processing data");
}

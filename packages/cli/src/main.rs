use shirabe_utils::log::init_logger; // 导入日志初始化函数

fn main() {
    init_logger(); // 初始化日志系统

    tracing::info!("CLI application started."); // 使用 tracing 记录日志

    println!("Hello, world!");

    tracing::info!("CLI application finished.");
}

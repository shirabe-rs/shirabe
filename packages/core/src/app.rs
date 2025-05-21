use crate::{
    adapter::Adapter,
    bot::Bot,
    context::{Context, state::EventSystemSharedState},
    error::FrameworkResult,
    plugin::Plugin,
};
use shirabe_utils::log;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

// 事件应用主结构
pub struct App {
    config: config::Config,
    adapters: Vec<Arc<dyn Adapter>>,
    plugins: Vec<Box<dyn Plugin>>,
    shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl App {
    pub fn new() -> Self {
        App {
            config: config::Config::default(),
            adapters: Vec::new(),
            plugins: Vec::new(),
            shared_state: Arc::new(RwLock::new(EventSystemSharedState::default())),
        }
    }

    /// 获取根事件上下文
    pub fn context(&self) -> Context {
        Context::new_root(Arc::clone(&self.shared_state))
    }

    /// 加载适配器
    pub fn load_adapter(&mut self, adapter: impl Adapter + 'static) -> &mut Self {
        self.adapters.push(Arc::new(adapter));
        self
    }

    /// 加载插件
    pub fn load_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// 启动应用
    pub fn run(&mut self) -> FrameworkResult<()> {
        log::init_logger();
        tracing::info!("启动应用……");

        let rt = Runtime::new().unwrap();
        // 加载配置文件
        self.config = config::Config::builder()
            .add_source(config::File::new("config", config::FileFormat::Toml))
            .build()?;

        // 获取应用根上下文
        let app_root_context = Arc::new(self.context());
        rt.block_on(async {
            // --- 插件加载 ---
            if !self.plugins.is_empty() {
                tracing::info!("开始加载插件 (on_load)...");
                for plugin in &self.plugins {
                    let ctx_clone = Arc::clone(&app_root_context);
                    tracing::debug!("调用插件 '{}' 的 on_load 方法", plugin.name());
                    if let Err(e) = plugin.on_load().await {
                        tracing::error!("插件 '{}' 的 on_load 方法执行失败: {}", plugin.name(), e);
                    }
                    match plugin.apply(ctx_clone).await {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::error!("插件{}启动失败: {:?}", plugin.name(), e)
                        }
                    }
                }
                tracing::debug!("所有插件启动完成。");
            }
            // --- 插件加载结束 ---

            // --- 从adapters中加载适配器并启动bots ---
            let mut bot_handlers = vec![];
            if self.adapters.is_empty() {
                tracing::warn!("没有加载任何适配器。应用将不会启动任何 Bot 或处理事件。");
            }

            for adapter_arc in &self.adapters {
                let bot_context = Arc::clone(&app_root_context); // 每个 bot 共享根应用上下文的某些部分
                let current_adapter_instance = Arc::clone(adapter_arc);

                // 创建 Bot 实例
                let bot = Arc::new(Bot::new(bot_context, current_adapter_instance));

                // 将 Bot 实例注册到应用上下文中
                // 这样插件或其他服务可以通过应用上下文访问到所有 Bot 实例
                {
                    let bots_arc_mutex = Arc::clone(&app_root_context.bots);
                    let mut bots_guard = bots_arc_mutex.lock().unwrap();
                    bots_guard.push(Arc::clone(&bot));
                }

                let bot_for_task = Arc::clone(&bot);
                let platform = bot_for_task.platform.clone();
                tracing::info!(
                    "使用 {} 适配器启动机器人 (Platform: {})",
                    adapter_arc.get_name(),
                    platform
                );

                // 为每个 Bot 启动一个异步任务
                let handle = tokio::spawn(async move {
                    if let Err(e) = bot_for_task.start().await {
                        tracing::error!("Bot {} 启动失败：{}", platform, e);
                    } else {
                        tracing::info!("Bot {} 已停止", platform);
                    }
                });
                bot_handlers.push(handle);
            }

            // 等待所有 Bot 任务完成
            if !bot_handlers.is_empty() {
                tracing::info!("所有 Bot 任务已派生，应用正在运行。等待 Bot 任务完成...");
                for handle in bot_handlers {
                    if let Err(e) = handle.await {
                        tracing::error!("一个 Bot 任务执行时发生错误: {:?}", e);
                    }
                }
                tracing::info!("所有 Bot 任务已完成。");
            } else if self.adapters.is_empty() {
                tracing::info!("由于没有加载适配器，所以没有 Bot 启动。");
            } else {
                tracing::info!("适配器已加载，但没有 Bot 任务被创建或成功启动。");
            }
        });

        tracing::info!("应用已关闭");

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

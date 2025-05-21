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
        let mut bot_handlers = vec![];
        // 从adapters中加载适配器
        if self.adapters.is_empty() {
            tracing::warn!("没有加载任何适配器。应用将不会加载任何插件")
        }

        for adapter_arc in &self.adapters {
            let bot_context = Arc::clone(&app_root_context);
            let current_adapter_instance = Arc::clone(adapter_arc);

            let bot = Arc::new(Bot::new(bot_context, current_adapter_instance));
            let bots_arc_mutex = Arc::clone(&app_root_context.bots);
            let mut bots_guard = bots_arc_mutex.lock().unwrap();
            bots_guard.push(Arc::clone(&bot));
            let bot_for_task = Arc::clone(&bot);
            let platform = bot_for_task.platform.clone();
            tracing::info!(
                "使用{}适配器启动机器人: {}",
                adapter_arc.get_name(),
                bot_for_task.platform
            );
            let handle = rt.spawn(async move {
                if let Err(e) = bot_for_task.start().await {
                    tracing::error!("Bot{}启动失败：{}", platform, e);
                } else {
                    tracing::info!("Bot{}已停止", platform);
                }
            });
            bot_handlers.push(handle);
        }
        if !bot_handlers.is_empty() {
            tracing::info!("所有bot已启动，应用已启动");
            rt.block_on(async {
                for handle in bot_handlers {
                    if let Err(e) = handle.await {
                        tracing::error!("一个bot任务发生错误: {:?}", e);
                    }
                }
            })
        } else {
            tracing::info!("没有bot启动。关闭应用……");
        }

        tracing::info!("应用已关闭");

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

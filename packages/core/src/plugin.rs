use crate::bot::Bot;
use crate::error::FrameworkResult;
use crate::session::Session;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;

// TODO: 完善插件系统
/// 插件特性，定义了插件的基本行为。
///
/// 插件应该能够响应会话事件，并拥有一个唯一的名称。
#[async_trait]
pub trait Plugin: Send + Sync + Debug {
    /// 返回插件的唯一名称。
    fn name(&self) -> &'static str;

    /// 当新的会话（通常由传入事件触发）创建时调用此方法。
    ///
    /// 插件可以检查会话内容并决定是否以及如何响应该事件。
    ///
    /// # 参数
    ///
    /// * `session` - 一个 `Arc<Session>`，代表当前的事件上下文。
    ///
    /// # 返回
    ///
    /// 如果处理成功，返回 `Ok(())`，否则返回一个错误。
    async fn apply(&self, session: Arc<Session>) -> FrameworkResult<()>;

    // 可选：插件加载时调用的方法，用于一次性设置。
    async fn on_load(&self, bot: Arc<Bot>) -> FrameworkResult<()>;

    // 可选：机器人关闭时调用的方法，用于清理工作。
    // async fn on_unload(&self, bot: Arc<Bot>) -> FrameworkResult<()>;
}

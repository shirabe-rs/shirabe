use std::any::Any;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

use super::{filter::ContextFilter, state::EventSystemSharedState};
use crate::session::Session;

pub type ListenerId = Uuid;

// 事件参数将是 Box<dyn Any + Send + Sync> 的切片
// 监听器负责向下转型 (downcasting)。
// Option<&Session> 是可选的上下文会话。
pub type ListenerCallback =
    Box<dyn FnMut(Option<&Session>, &[Box<dyn Any + Send + Sync>]) + Send + Sync>;
pub type BailCallback = Box<
    dyn FnMut(Option<&Session>, &[Box<dyn Any + Send + Sync>]) -> Option<Box<dyn Any + Send + Sync>>
        + Send
        + Sync,
>;

// ListenerAction 枚举，用于存储不同类型的回调逻辑
// 注意：FnMut 回调如果捕获了可变状态，并且监听器被 Arc 共享，
// 则需要 Mutex 来保证线程安全。
pub enum ListenerAction {
    On(Mutex<ListenerCallback>),
    Once(Mutex<Option<ListenerCallback>>), // Option 用于“取出”回调，实现一次性
    Bail(Mutex<BailCallback>),             // Bail监听器是持久的，除非显式移除
}

pub struct RegisteredListener {
    pub id: ListenerId,
    pub filter: ContextFilter,  // 注册时此监听器关联的上下文过滤器
    pub action: ListenerAction, // 监听器具体行为 (On, Once, Bail 及其回调)
}

// 用于移除监听器的句柄
pub struct ListenerHandle {
    pub id: ListenerId,
    pub shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl ListenerHandle {
    pub fn dispose(self) {
        // 调用此方法移除监听器
        let mut state = self.shared_state.write().unwrap();
        state.remove_listener(self.id);
    }
}

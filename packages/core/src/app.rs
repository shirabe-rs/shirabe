use crate::context::{Context, state::EventSystemSharedState};
use std::sync::{Arc, RwLock};

// 事件应用主结构
pub struct App {
    shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl App {
    pub fn new() -> Self {
        App {
            shared_state: Arc::new(RwLock::new(EventSystemSharedState::default())),
        }
    }

    // 获取根事件上下文
    pub fn context(&self) -> Context {
        Context::new_root(Arc::clone(&self.shared_state))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

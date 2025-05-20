use std::collections::HashMap;
use std::sync::Arc; // Mutex 用于回调的内部可变性

use super::listener::{ListenerId, RegisteredListener};

// 共享状态，存储所有事件的监听器
#[derive(Default)]
pub struct EventSystemSharedState {
    // 事件名 -> 该事件的所有监听器列表
    pub listeners_by_event: HashMap<String, Vec<Arc<RegisteredListener>>>,
    // 监听器ID -> 监听器，用于通过ID快速移除
    pub listeners_by_id: HashMap<ListenerId, Arc<RegisteredListener>>,
}

impl EventSystemSharedState {
    pub fn add_listener(&mut self, event_name: String, listener: Arc<RegisteredListener>) {
        self.listeners_by_event
            .entry(event_name)
            .or_default()
            .push(Arc::clone(&listener)); // 将监听器的 Arc 引用存入列表
        self.listeners_by_id.insert(listener.id, listener); // 同时存入 ID 映射
    }

    pub fn remove_listener(&mut self, id_to_remove: ListenerId) -> bool {
        if self.listeners_by_id.remove(&id_to_remove).is_some() {
            // 如果成功从 ID 映射中移除，也需要从事件名映射的列表中移除
            for listeners in self.listeners_by_event.values_mut() {
                listeners.retain(|l| l.id != id_to_remove);
            }
            true
        } else {
            false
        }
    }
}

pub mod filter;
pub mod listener;
pub mod state;

// TODO: 完善上下文系统
use std::any::Any;
use std::sync::{Arc, Mutex, RwLock}; // Mutex 用于回调的内部可变性
use uuid::Uuid; // Mutex 用于回调的内部可变性

use crate::bot::Bot;
use crate::context::{
    filter::ContextFilter,
    listener::{ListenerAction, ListenerHandle, ListenerId, RegisteredListener},
    state::EventSystemSharedState,
};
use crate::session::Session;

// 事件上下文
#[derive(Clone)]
pub struct Context {
    /// 当前应用的全部机器人实例
    pub bots: Vec<Arc<Bot>>,
    /// 当前上下文的过滤器设置
    pub current_filter: ContextFilter,
    /// 对共享状态的引用
    pub shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl Context {
    pub fn new_root(shared_state: Arc<RwLock<EventSystemSharedState>>) -> Self {
        Context {
            bots: Vec::new(),
            current_filter: ContextFilter::new(), // 根上下文的过滤器是空的
            shared_state,
        }
    }

    // --- 上下文派生方法 ---
    pub fn user(&self, user_id: &str) -> Self {
        // 创建只针对特定用户的子上下文
        Context {
            bots: self.bots.clone(),
            current_filter: self.current_filter.clone().user(user_id),
            shared_state: Arc::clone(&self.shared_state),
        }
    }
    pub fn guild(&self, guild_id: &str) -> Self {
        // 创建只针对特定群组的子上下文
        Context {
            bots: self.bots.clone(),
            current_filter: self.current_filter.clone().guild(guild_id),
            shared_state: Arc::clone(&self.shared_state),
        }
    }
    pub fn platform(&self, platform: &str) -> Self {
        // 创建只针对特定平台的子上下文
        Context {
            bots: self.bots.clone(),
            current_filter: self.current_filter.clone().platform(platform),
            shared_state: Arc::clone(&self.shared_state),
        }
    }
    pub fn private(&self) -> Self {
        // 创建只针对私聊的子上下文
        Context {
            bots: self.bots.clone(),
            current_filter: self.current_filter.clone().private(),
            shared_state: Arc::clone(&self.shared_state),
        }
    }
    pub fn group(&self) -> Self {
        // 创建只针对群聊的子上下文 (与 private 相对)
        Context {
            bots: self.bots.clone(),
            current_filter: self.current_filter.clone().group(),
            shared_state: Arc::clone(&self.shared_state),
        }
    }
    // --- 上下文派生方法结束 ---

    fn register_listener_internal(
        &self,
        event_name: &str,
        action: ListenerAction,
    ) -> ListenerHandle {
        let id = Uuid::new_v4(); // 生成唯一ID
        let listener = Arc::new(RegisteredListener {
            id,
            filter: self.current_filter.clone(), // 监听器关联到当前上下文的过滤器副本
            action,
        });
        let mut state = self.shared_state.write().unwrap(); // 获取共享状态的写锁
        state.add_listener(event_name.to_string(), Arc::clone(&listener));
        ListenerHandle {
            id,
            shared_state: Arc::clone(&self.shared_state),
        }
    }

    // 注册普通监听器
    pub fn on<F>(&self, event_name: &str, callback: F) -> ListenerHandle
    where
        F: FnMut(Option<&Session>, &[Box<dyn Any + Send + Sync>]) + Send + Sync + 'static,
    {
        self.register_listener_internal(
            event_name,
            ListenerAction::On(Mutex::new(Box::new(callback))),
        )
    }

    // 注册一次性监听器
    pub fn once<F>(&self, event_name: &str, callback: F) -> ListenerHandle
    where
        F: FnMut(Option<&Session>, &[Box<dyn Any + Send + Sync>]) + Send + Sync + 'static,
    {
        self.register_listener_internal(
            event_name,
            ListenerAction::Once(Mutex::new(Some(Box::new(callback)))),
        )
    }

    // 注册可熔断监听器
    pub fn bail<F>(&self, event_name: &str, callback: F) -> ListenerHandle
    where
        F: FnMut(
                Option<&Session>,
                &[Box<dyn Any + Send + Sync>],
            ) -> Option<Box<dyn Any + Send + Sync>>
            + Send
            + Sync
            + 'static,
    {
        self.register_listener_internal(
            event_name,
            ListenerAction::Bail(Mutex::new(Box::new(callback))),
        )
    }

    /// 发射事件
    /// # Arguments
    ///
    /// `session_context`: 可选的会话上下文，用于过滤器匹配
    /// `event_name`: 事件名称
    /// `args`: 事件参数列表
    ///
    /// # Returns
    /// 如果某个 bail 监听器熔断了，则返回其返回值
    pub fn emit(
        &self,
        session_context: Option<&Session>,
        event_name: &str,
        args: &[Box<dyn Any + Send + Sync>],
    ) -> Option<Box<dyn Any + Send + Sync>> {
        let state_read_guard = self.shared_state.read().unwrap(); // 获取读锁
        let listeners_for_event_arcs = match state_read_guard.listeners_by_event.get(event_name) {
            Some(listeners) => listeners.clone(), // 克隆 Arc 列表，以便在锁外操作
            None => return None,                  // 没有此事件的监听器
        };
        drop(state_read_guard); // 释放读锁，避免长时间持有

        let mut ids_of_once_listeners_fired: Vec<ListenerId> = Vec::new(); // 存储已触发的 Once 监听器ID

        for listener_arc in listeners_for_event_arcs {
            //  检查上下文过滤器是否匹配
            let should_run = match session_context {
                Some(s_ctx) => listener_arc.filter.matches_session(s_ctx),
                None => listener_arc.filter.matches_generic(), // 对于无会话的通用事件
            };

            if !should_run {
                continue; // 过滤器不匹配，跳过此监听器
            }

            //  根据监听器类型执行回调
            match &listener_arc.action {
                ListenerAction::On(cb_mutex) => {
                    // 对于 On 监听器，获取其回调的锁并执行
                    // 使用 try_lock 更好，以防死锁（如果回调内部又 emit 同步事件）
                    if let Ok(mut cb_guard) = cb_mutex.try_lock() {
                        (*cb_guard)(session_context, args);
                    } else {
                        // 处理无法获取锁的情况，例如打印警告
                        eprintln!(
                            "[事件系统警告] 无法获取 On 监听器 {} 的锁，可能存在重入或竞争。",
                            listener_arc.id
                        );
                    }
                }
                ListenerAction::Once(cb_mutex_opt) => {
                    let mut opt_cb_guard = cb_mutex_opt.lock().unwrap();
                    if let Some(mut cb) = opt_cb_guard.take() {
                        // 尝试取出回调
                        // 成功取出，表示这是第一次执行
                        drop(opt_cb_guard); // 在调用回调前释放锁
                        cb(session_context, args);
                        ids_of_once_listeners_fired.push(listener_arc.id); // 记录此ID，稍后移除
                    }
                    // 如果 opt_cb_guard.take() 返回 None，说明回调已被取走，不再执行
                }
                ListenerAction::Bail(cb_mutex) => {
                    if let Ok(mut cb_guard) = cb_mutex.try_lock() {
                        let bail_result = (*cb_guard)(session_context, args);
                        if bail_result.is_some() {
                            // Bail 监听器返回了 Some 值，表示熔断
                            // 在返回前，清理掉在此 Bail 之前已触发的 Once 监听器
                            if !ids_of_once_listeners_fired.is_empty() {
                                let mut state_write_guard = self.shared_state.write().unwrap();
                                for id in &ids_of_once_listeners_fired {
                                    state_write_guard.remove_listener(*id);
                                }
                            }
                            return bail_result; // 立即返回熔断结果
                        }
                    } else {
                        eprintln!(
                            "[事件系统警告] 无法获取 Bail 监听器 {} 的锁，可能存在重入或竞争。",
                            listener_arc.id
                        );
                    }
                }
            }
        }

        // 循环结束后，统一清理所有已触发的 Once 监听器
        if !ids_of_once_listeners_fired.is_empty() {
            let mut state_write_guard = self.shared_state.write().unwrap();
            for id in ids_of_once_listeners_fired {
                state_write_guard.remove_listener(id);
            }
        }

        None // 没有监听器熔断
    }
}

use crate::bot::Bot;
// // TODO: 完善上下文系统
use crate::session::Session;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock}; // Mutex 用于回调的内部可变性
use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContextFilter {
    user_ids: Option<HashSet<String>>,
    guild_ids: Option<HashSet<String>>,
    platforms: Option<HashSet<String>>,
    is_private: Option<bool>, // true: 仅私聊, false: 仅群聊, None: 两者皆可
}

impl ContextFilter {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn user(mut self, user_id: &str) -> Self {
        self.user_ids
            .get_or_insert_with(HashSet::new)
            .insert(user_id.to_string());
        self
    }
    pub fn guild(mut self, guild_id: &str) -> Self {
        self.guild_ids
            .get_or_insert_with(HashSet::new)
            .insert(guild_id.to_string());
        self
    }
    pub fn platform(mut self, platform: &str) -> Self {
        self.platforms
            .get_or_insert_with(HashSet::new)
            .insert(platform.to_string());
        self
    }
    pub fn private(mut self) -> Self {
        self.is_private = Some(true);
        self
    }
    pub fn group(mut self) -> Self {
        self.is_private = Some(false);
        self
    }

    // 检查过滤器是否匹配给定的 Session
    pub fn matches_session(&self, session: &Session) -> bool {
        if let Some(users) = &self.user_ids {
            if !users.contains(&session.user_id) {
                return false;
            }
        }
        if let Some(guilds) = &self.guild_ids {
            let gid = &session.guild_id;
            if !guilds.contains(gid) {
                return false; // 群聊ID不匹配
            }
        }
        if let Some(platforms) = &self.platforms {
            if !platforms.contains(&session.platform) {
                return false;
            }
        }
        if let Some(is_private_filter) = self.is_private {
            if is_private_filter != session.is_direct {
                return false;
            }
        }
        true
    }
    // 检查过滤器是否匹配没有 Session 上下文的通用事件
    pub fn matches_generic(&self) -> bool {
        // 如果过滤器指定了用户/群组/私聊等会话相关的属性，
        // 那么对于没有会话上下文的通用事件，它通常不应该匹配。
        // 这里的逻辑可以根据具体需求调整。
        // 例如，只基于平台的过滤器可能对通用事件仍然有效。
        self.user_ids.is_none() &&
        self.guild_ids.is_none() &&
        self.platforms.is_none() && // 平台相关的过滤器可能对通用事件依然有意义
        self.is_private.is_none()
    }
}

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
enum ListenerAction {
    On(Mutex<ListenerCallback>),
    Once(Mutex<Option<ListenerCallback>>), // Option 用于“取出”回调，实现一次性
    Bail(Mutex<BailCallback>),             // Koishi的Bail监听器是持久的，除非显式移除
}

struct RegisteredListener {
    id: ListenerId,
    filter: ContextFilter,  // 注册时此监听器关联的上下文过滤器
    action: ListenerAction, // 监听器具体行为 (On, Once, Bail 及其回调)
}

// 共享状态，存储所有事件的监听器
#[derive(Default)]
struct EventSystemSharedState {
    // 事件名 -> 该事件的所有监听器列表
    listeners_by_event: HashMap<String, Vec<Arc<RegisteredListener>>>,
    // 监听器ID -> 监听器，用于通过ID快速移除
    listeners_by_id: HashMap<ListenerId, Arc<RegisteredListener>>,
}

impl EventSystemSharedState {
    fn add_listener(&mut self, event_name: String, listener: Arc<RegisteredListener>) {
        self.listeners_by_event
            .entry(event_name)
            .or_default()
            .push(Arc::clone(&listener)); // 将监听器的 Arc 引用存入列表
        self.listeners_by_id.insert(listener.id, listener); // 同时存入 ID 映射
    }

    fn remove_listener(&mut self, id_to_remove: ListenerId) -> bool {
        if self.listeners_by_id.remove(&id_to_remove).is_some() {
            // 如果成功从 ID 映射中移除，也需要从事件名映射的列表中移除
            // 这个操作效率较低 (O(N_listeners_for_event))，但为了正确性是必要的
            // 更复杂的结构可以优化此操作
            for listeners in self.listeners_by_event.values_mut() {
                listeners.retain(|l| l.id != id_to_remove);
            }
            true
        } else {
            false
        }
    }
}

// 用于移除监听器的句柄
pub struct ListenerHandle {
    id: ListenerId,
    shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl ListenerHandle {
    pub fn dispose(self) {
        // 调用此方法移除监听器
        let mut state = self.shared_state.write().unwrap();
        state.remove_listener(self.id);
    }
}

// 事件上下文，类似于 Koishi 的 ctx
#[derive(Clone)]
pub struct Context {
    /// 当前应用的全部机器人实例
    bots: Vec<Arc<Bot>>,
    /// 当前上下文的过滤器设置
    current_filter: ContextFilter,
    /// 对共享状态的引用
    shared_state: Arc<RwLock<EventSystemSharedState>>,
}

impl Context {
    fn new_root(shared_state: Arc<RwLock<EventSystemSharedState>>) -> Self {
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

    // 发射事件
    // session_context: 可选的会话上下文，用于过滤器匹配
    // event_name: 事件名称
    // args: 事件参数列表
    // 返回值: 如果某个 bail 监听器熔断了，则返回其返回值
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

// --- 使用示例 ---
#[derive(Debug)] // 仅用于打印
#[allow(dead_code)]
struct MyCustomEventArgs {
    message: String,
    count: i32,
}

#[cfg(test)]
mod test {
    use crate::context::App;

    #[test]
    fn test_context() {
        let app = App::new();
        let root_ctx = app.context(); // 获取根上下文

        // 监听器 1: 通用消息记录器 (ON 类型)
        let _logger_handle = root_ctx.on("message", |session_opt, args| {
            if let Some(session) = session_opt {
                println!(
                    "[ON Logger] 收到消息，来自用户 {}: 内容 '{}'",
                    session.user_id, session.content
                );
            }
            // 检查是否有自定义参数
            if !args.is_empty() {
                if let Some(custom_data) = args[0].downcast_ref::<String>() {
                    // 尝试向下转型为 String
                    println!("[ON Logger] 附带的自定义数据: {}", custom_data);
                }
            }
        });

        // 监听器 2: 特定用户 "user123" 的一次性消息监听 (ONCE 类型)
        let specific_user_ctx = root_ctx.user("user123"); // 创建针对 user123 的子上下文
        let _once_handle = specific_user_ctx.once("message", |session_opt, _args| {
            if let Some(session) = session_opt {
                if session.content.starts_with("!secret_command") {
                    println!(
                        "[ONCE user123] 用户 {} 执行了 !secret_command。此监听器现已移除。",
                        session.user_id
                    );
                }
            }
        });

        // 监听器 3: 管理员 "admin001" 在 "discord" 平台的熔断操作 (BAIL 类型)
        let admin_ctx = root_ctx.user("admin001").platform("discord");
        let _bail_handle = admin_ctx.bail("admin_action", |session_opt, args| {
            if let Some(session) = session_opt {
                // 确保会话存在且匹配 admin_ctx 的过滤器
                println!(
                    "[BAIL admin001 @ {}] 检查 admin_action...",
                    session.platform
                );
            }
            if let Some(action_name) = args.first().and_then(|a| a.downcast_ref::<String>()) {
                if action_name == "shutdown_server" {
                    println!("[BAIL admin001] 检测到 'shutdown_server' 操作！执行熔断。");
                    return Some(Box::new("服务器关闭指令已由管理员发起".to_string())); // 返回 Some 值以熔断
                }
            }
            None // 不熔断，返回 None
        });

        // 监听器 4: 另一个通用的 admin_action 监听器，如果前面的 BAIL 监听器熔断了，这个将不会执行
        let _general_admin_action_handle =
            root_ctx
                .platform("discord")
                .on("admin_action", |_session_opt, args| {
                    if let Some(action_name) = args.first().and_then(|a| a.downcast_ref::<String>())
                    {
                        println!("[ON Discord Admin] 通用 admin_action 事件: {}", action_name);
                    }
                });

        // --- 模拟发射事件 ---
        // println!("\n--- 开始发射事件 ---");
        //
        // // 准备一些会话实例
        // let session_user_A = Session::new();
        // let session_user123_private = Session::new("user123", None, "onebot", "!secret_command 执行它"); // user123 的私聊
        // let session_admin_discord =
        //     Session::new("admin001", Some("guild_main"), "discord", "管理员消息");
        //
        // println!("\n1. 为 user_A (Telegram, 群聊 group_X) 发射 'message' 事件:");
        // let custom_arg_for_msg: Vec<Box<dyn Any + Send + Sync>> =
        //     vec![Box::new("附加信息1".to_string())];
        // root_ctx.emit(Some(&session_user_A), "message", &custom_arg_for_msg);
        //
        // println!("\n2. 为 user123 (OneBot, 私聊) 发射 'message' 事件 (触发 !secret_command):");
        // root_ctx.emit(Some(&session_user123_private), "message", &[]);
        //
        // println!("\n3. 再次为 user123 (OneBot, 私聊) 发射 'message' 事件 (ONCE 监听器应已移除):");
        // root_ctx.emit(Some(&session_user123_private), "message", &[]);
        //
        // println!("\n4. 为 admin001 (Discord) 发射 'admin_action' 事件 (非熔断操作 'kick_user'):");
        // let args_kick: Vec<Box<dyn Any + Send + Sync>> = vec![Box::new("kick_user".to_string())];
        // let bail_result1 = root_ctx.emit(Some(&session_admin_discord), "admin_action", &args_kick);
        // if let Some(res_any) = bail_result1 {
        //     println!(
        //         "'admin_action' (kick_user) 熔断结果: {:?}",
        //         res_any.downcast_ref::<String>()
        //     );
        // } else {
        //     println!("'admin_action' (kick_user) 未熔断。");
        // }
        //
        // println!("\n5. 为 admin001 (Discord) 发射 'admin_action' 事件 (熔断操作 'shutdown_server'):");
        // let args_shutdown: Vec<Box<dyn Any + Send + Sync>> =
        //     vec![Box::new("shutdown_server".to_string())];
        // let bail_result2 = root_ctx.emit(Some(&session_admin_discord), "admin_action", &args_shutdown);
        // if let Some(result_any) = bail_result2 {
        //     if let Some(result_str) = result_any.downcast_ref::<String>() {
        //         // 尝试向下转型为String
        //         println!(
        //             "'admin_action' (shutdown_server) 成功熔断，结果: '{}'",
        //             result_str
        //         );
        //     } else {
        //         println!("'admin_action' (shutdown_server) 成功熔断，但结果不是字符串。");
        //     }
        // } else {
        //     println!("'admin_action' (shutdown_server) 未熔断 (这不符合预期，请检查逻辑)。");
        // }
        //
        // println!("\n6. 发射一个没有 Session 上下文的通用事件 'app_lifecycle':");
        // let lifecycle_args: Vec<Box<dyn Any + Send + Sync>> = vec![
        //     Box::new("系统初始化完成".to_string()),
        //     Box::new(MyCustomEventArgs {
        //         message: "生命周期参数".to_string(),
        //         count: 100,
        //     }),
        // ];
        // // 注册一个监听器来接收这个通用事件
        // let _lifecycle_listener = root_ctx.on("app_lifecycle", |_session, args| {
        //     // _session 会是 None
        //     if let Some(status) = args.get(0).and_then(|a| a.downcast_ref::<String>()) {
        //         println!("[ON app_lifecycle] 状态: {}", status);
        //     }
        //     if let Some(custom_obj) = args
        //         .get(1)
        //         .and_then(|a| a.downcast_ref::<MyCustomEventArgs>())
        //     {
        //         println!("[ON app_lifecycle] 自定义对象: {:?}", custom_obj);
        //     }
        // });
        // root_ctx.emit(None, "app_lifecycle", &lifecycle_args);
        //
        // println!("\n7. 测试监听器移除 (dispose):");
        // let handle_to_dispose = root_ctx.on("temp_event", |_, _| {
        //     println!("[ON temp_event] 临时事件触发!")
        // });
        // root_ctx.emit(None, "temp_event", &[]); // 应该会触发
        // handle_to_dispose.dispose(); // 移除监听器
        // println!("移除了 temp_event 的监听器。");
        // root_ctx.emit(None, "temp_event", &[]); // 应该不会再触发了
    }
}

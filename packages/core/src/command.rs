use crate::context::filter::ContextFilter;
use crate::error::FrameworkResult;
use crate::session::Session;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

/// 解析后的命令参数和选项。
#[derive(Debug, Clone, Default)]
pub struct ParsedArgs {
    /// 位置参数。
    pub arguments: Vec<String>,
    /// 提供的选项，例如 --option value。
    /// 对于标志（没有值的选项），其值可能为空字符串或占位符。
    pub options: HashMap<String, String>, // 选项名称 -> 选项值
}

/// 命令执行的异步动作的类型别名。
pub type CommandAction = Box<
    dyn Fn(
            Arc<Session>,
            ParsedArgs,
        ) -> Pin<Box<dyn Future<Output = FrameworkResult<()>> + Send + Sync>>
        + Send
        + Sync,
>;

/// 代表一个机器人指令。
pub struct Command {
    /// 指令的主要名称。
    pub name: String,
    /// 指令的别名列表。
    pub aliases: Vec<String>,
    /// 指令功能的简要描述。
    pub description: Option<String>,
    /// 指令注册时关联的上下文过滤器。
    pub filter: ContextFilter,
    /// 指令执行时的异步动作。
    pub action: CommandAction,
    // TODO: 为帮助信息生成和验证添加参数和选项定义的字段
    // pub arg_defs: Vec<CommandArgumentDef>,
    // pub opt_defs: Vec<CommandOptionDef>,
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("aliases", &self.aliases)
            .field("description", &self.description)
            .field("filter", &self.filter)
            .field("action", &"Box<dyn Fn(...)>") // 不打印闭包本身
            .finish()
    }
}

/// 管理并执行指令。
#[derive(Default, Debug)]
pub struct CommandRegistry {
    /// 存储指令，将指令名称/别名映射到指令定义。
    pub commands: HashMap<String, Arc<Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个指令。
    /// 指令的主要名称及其所有别名都将被注册。
    pub fn register(&mut self, command: Command) -> FrameworkResult<()> {
        let command_arc = Arc::new(command);
        // 注册主要名称
        if self.commands.contains_key(&command_arc.name) {
            // 可以返回错误或记录警告
            tracing::warn!("指令 {} 已注册，将被覆盖。", command_arc.name);
        }
        self.commands
            .insert(command_arc.name.clone(), Arc::clone(&command_arc));

        // 注册别名
        for alias in &command_arc.aliases {
            if self.commands.contains_key(alias) {
                tracing::warn!(
                    "指令 {} 的别名 {} 已注册或与另一指令冲突，将被覆盖。",
                    command_arc.name,
                    alias
                );
            }
            self.commands
                .insert(alias.clone(), Arc::clone(&command_arc));
        }
        Ok(())
    }

    /// 解析消息并执行相应的指令（如果找到）。
    ///
    /// # 参数
    ///
    /// * `session`: 当前会话，提供上下文和机器人访问。
    /// * `message_content`: 消息的原始文本内容。
    /// * `prefixes`: 命令前缀集合。
    ///
    /// # 返回
    ///
    /// * `Ok(true)` 如果找到了指令并尝试执行。
    /// * `Ok(false)` 如果没有找到指令（例如，没有前缀匹配或未知指令）。
    /// * `Err(FrameworkError)` 如果在指令执行期间发生错误。
    pub async fn parse_and_execute(
        &self,
        session: Arc<Session>,
        message_content: &str,
        // TODO: 从配置中获取前缀
        prefixes: &[&str], // 允许传入前缀
    ) -> FrameworkResult<bool> {
        let mut potential_command_text: Option<&str> = None;

        for prefix in prefixes {
            if message_content.starts_with(prefix) {
                potential_command_text =
                    Some(message_content.trim_start_matches(prefix).trim_start());
                break;
            }
        }

        if potential_command_text.is_none() {
            return Ok(false); // 不是指令 (没有匹配的前缀)
        }

        let text = potential_command_text.unwrap();
        if text.is_empty() {
            return Ok(false); // 只有前缀，没有指令名称
        }

        let parts: Vec<&str> = text.split_whitespace().collect();
        let command_name = parts[0];

        if let Some(command_arc) = self.commands.get(command_name) {
            // 检查指令自身注册时绑定的过滤器
            if !command_arc.filter.matches_session(&session) {
                tracing::trace!(
                    "指令 {} 找到，但其上下文过滤器不匹配当前会话。",
                    command_name
                );
                return Ok(false); // 指令的上下文过滤器不匹配
            }

            tracing::debug!("正在执行指令: {}", command_name);

            let mut parsed_args = ParsedArgs::default();
            let mut i = 1; // 从指令名称后的第一个部分开始
            while i < parts.len() {
                let part = parts[i];
                if part.starts_with("--") {
                    let option_name = part.trim_start_matches("--").to_string();
                    if i + 1 < parts.len() {
                        let next_part = parts[i + 1];
                        // 值不能以 '-' 开头 (除非它是负数等特定情况，但这里简化)
                        if !next_part.starts_with('-') && !next_part.is_empty() {
                            // 主要修改在这里
                            parsed_args
                                .options
                                .insert(option_name, next_part.to_string());
                            i += 1; // 消耗选项值部分
                        } else {
                            // 下一个 token 是另一个选项，或没有值，当前长选项是标志
                            parsed_args.options.insert(option_name, String::new());
                        }
                    } else {
                        // 没有更多 token 了，当前长选项是标志
                        parsed_args.options.insert(option_name, String::new());
                    }
                } else if part.starts_with('-') && part.len() > 1 && !part.starts_with("--") {
                    // 短选项逻辑
                    for (idx, char_val) in part.char_indices() {
                        if idx == 0 {
                            continue;
                        }
                        parsed_args
                            .options
                            .insert(char_val.to_string(), String::new());
                    }
                } else {
                    // 参数
                    parsed_args.arguments.push(part.to_string());
                }
                i += 1;
            }

            // 执行指令的动作
            (command_arc.action)(session, parsed_args).await?;
            Ok(true) // 指令找到并尝试执行
        } else {
            tracing::trace!("未知指令: {}", command_name);
            Ok(false) // 未知指令
        }
    }
}

/// 用于链式构建和注册指令的构建器。
pub struct CommandBuilder {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
    filter: ContextFilter, // 从调用 command() 的上下文中捕获
    action: Option<CommandAction>,
    registry: Arc<RwLock<CommandRegistry>>, // 指向共享的指令注册表
}

impl CommandBuilder {
    /// 创建一个新的 CommandBuilder。
    /// 通常由 `Context::command()` 调用。
    pub fn new(
        name: String,
        filter: ContextFilter,
        registry: Arc<RwLock<CommandRegistry>>,
    ) -> Self {
        CommandBuilder {
            name,
            aliases: Vec::new(),
            description: None,
            filter,
            action: None,
            registry,
        }
    }

    /// 为指令添加一个别名。
    pub fn alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }

    /// 为指令设置描述。
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// 设置指令的执行动作。
    pub fn action<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Arc<Session>, ParsedArgs) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = FrameworkResult<()>> + Send + Sync + 'static,
    {
        self.action = Some(Box::new(move |session, args| Box::pin(f(session, args))));
        self
    }

    /// 构建并注册指令。
    pub fn register(self) -> FrameworkResult<()> {
        let action = self.action.ok_or_else(|| {
            crate::error::FrameworkError::Command(format!("指令 '{}' 没有定义 action", self.name))
        })?;

        let command = Command {
            name: self.name.clone(),
            aliases: self.aliases,
            description: self.description,
            filter: self.filter,
            action,
        };

        let mut registry_guard = self.registry.write().map_err(|_| {
            crate::error::FrameworkError::Internal("无法获取 CommandRegistry 的写锁".to_string())
        })?;

        registry_guard.register(command)?;
        tracing::info!("指令 '{}' 已注册", self.name);
        Ok(())
    }
}

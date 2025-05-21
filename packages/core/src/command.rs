use crate::error::FrameworkResult;
use crate::session::Session;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// TODO:完善命令系统
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
// Command 没有派生 Debug，因为 CommandAction 不是 Debug。
// 如果需要，可以手动实现 Debug，并排除 action 字段。
pub struct Command {
    /// 指令的主要名称。
    pub name: String,
    /// 指令的别名列表。
    pub aliases: Vec<String>,
    /// 指令功能的简要描述。
    pub description: Option<String>,
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
            .field("action", &"Box<dyn Fn(...)>") // 不打印闭包本身
            .finish()
    }
}

/// 管理并执行指令。
#[derive(Default, Debug)]
pub struct CommandRegistry {
    /// 存储指令，将指令名称/别名映射到指令定义。
    commands: HashMap<String, Arc<Command>>,
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
    ) -> FrameworkResult<bool> {
        //     let prefixes = &session.bot().config.prefix;
        //     let mut potential_command_text: Option<&str> = None;

        //     for prefix in prefixes {
        //         if message_content.starts_with(prefix) {
        //             potential_command_text =
        //                 Some(message_content.trim_start_matches(prefix).trim_start());
        //             break;
        //         }
        //     }

        //     if potential_command_text.is_none() {
        //         return Ok(false); // 不是指令
        //     }

        //     let text = potential_command_text.unwrap();
        //     if text.is_empty() {
        //         return Ok(false); // 只有前缀，没有指令名称
        //     }

        //     let parts: Vec<&str> = text.split_whitespace().collect();
        //     let command_name = parts[0];

        //     if let Some(command_arc) = self.commands.get(command_name) {
        //         tracing::debug!("正在执行指令: {}", command_name);

        //         let mut parsed_args = ParsedArgs::default();
        //         // let remaining_parts = &parts[1..]; // 这行是旧的，下面修复了索引
        //         let mut i = 1; // 从指令名称后的第一个部分开始
        //         while i < parts.len() {
        //             let part = parts[i];
        //             if part.starts_with("--") {
        //                 let option_name = part.trim_start_matches("--").to_string();
        //                 if i + 1 < parts.len() && !parts[i + 1].starts_with("--") {
        //                     // 带值的选项
        //                     parsed_args
        //                         .options
        //                         .insert(option_name, parts[i + 1].to_string());
        //                     i += 1; // 消耗选项值部分
        //                 } else {
        //                     // 标志选项（无值）或最后一个选项
        //                     parsed_args.options.insert(option_name, String::new()); // 或其他占位符
        //                 }
        //             } else {
        //                 // 参数
        //                 parsed_args.arguments.push(part.to_string());
        //             }
        //             i += 1;
        //         }

        //         // 执行指令的动作
        //         (command_arc.action)(session, parsed_args).await?;
        //         Ok(true) // 指令找到并尝试执行
        //     } else {
        //         tracing::trace!("未知指令: {}", command_name);
        //         Ok(false) // 未知指令
        //     }
        Ok(false)
    }
}

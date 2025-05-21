# Shirabe Core

[![Crates.io](httpsa://img.shields.io/crates/v/shirabe-core.svg)](https://crates.io/crates/shirabe-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Repository](https://img.shields.io/badge/repository-GitHub-blue.svg)](https://github.com/shirabe-rs/shirabe)

Shirabe Core 是一个用于 Rust 语言的 Satori 机器人框架核心库。它提供了与 Satori 协议交互以及开发模块化机器人应用所需的核心组件和抽象。

## 主要特性

*   **事件驱动**: 基于 `App` 和 `Context` 的事件处理机制。
*   **适配器系统**: 通过 `Adapter` trait 实现与不同聊天平台的集成。`WSClient` trait 为 WebSocket 适配器提供了便利。
*   **机器人抽象**: `Bot` 结构体代表一个通过适配器连接的机器人实例。
*   **会话管理**: `Session` 封装了传入事件的上下文，并提供了便捷的交互方法。
*   **命令系统**: 通过 `CommandRegistry`、`CommandBuilder` 和 `Command` 结构轻松定义和管理机器人指令。
*   **插件化架构**: 使用 `Plugin` trait 来扩展框架功能。
*   **富消息元素**: 通过 `MessageElement` 枚举支持 Satori 协议的多种消息格式。
*   **配置管理**: 集成了简单的配置文件加载。
*   **异步支持**: 全面基于 `async/await` 和 `tokio`。

## 核心组件

*   **`App`**: 应用程序的主入口和协调器，负责加载适配器、插件，并启动机器人实例。
*   **`Context`**: 事件处理的核心，管理事件监听器、命令注册表、机器人实例以及上下文过滤器。它允许创建作用于特定用户、群组或平台的子上下文。
*   **`Adapter`**: 一个 trait，定义了与特定聊天平台通信的接口。你需要为每个希望支持的平台实现此 trait。
*   **`Bot`**: 代表一个机器人实例。它通过一个 `Adapter` 实例连接到特定平台，并与 `Context` 关联。
*   **`Session`**: 代表一个由传入事件（如新消息）触发的会话。它封装了事件的详细信息（如发送者、频道、消息内容等），并提供了与用户交互的方法（如发送回复）。
*   **`CommandRegistry` 和 `CommandBuilder`**: 用于定义、注册和解析用户指令。`CommandBuilder` 提供了链式 API 来构建指令。
*   **`Plugin`**: 一个 trait，允许你创建可重用的模块来扩展机器人的功能。插件可以注册监听器、命令等。
*   **`MessageElement`**: 枚举类型，用于构建和解析 Satori 协议支持的富文本消息，如文本、图片、提及、引用等。

## 基本用法示例

以下是一个如何设置应用、加载适配器并定义一个简单命令的概念性示例：

```rust
use shirabe_core::app::App;
use shirabe_core::adapter::Adapter;
use shirabe_core::bot::Bot;
use shirabe_core::context::Context;
use shirabe_core::session::Session;
use shirabe_core::message::MessageElement;
use shirabe_core::error::FrameworkResult;
use shirabe_core::types::{Channel, Guild, GuildMember, GuildRole, Login, LoginStatus, Message, User}; // For Adapter methods
use async_trait::async_trait;
use std::sync::Arc;

// 为示例创建一个虚拟适配器
#[derive(Debug)]
struct MyPlatformAdapter;

#[async_trait]
impl Adapter for MyPlatformAdapter {
    fn get_name(&self) -> String { "my-platform".to_string() }
    async fn connect(&self, _bot: Arc<Bot>) { println!("MyPlatformAdapter connected!"); }
    async fn disconnect(&self, _bot: Arc<Bot>) { println!("MyPlatformAdapter disconnected!"); }

    async fn send_message(&self, channel_id: &str, elements: &[MessageElement]) -> FrameworkResult<Vec<String>> {
        println!("MyPlatformAdapter: Sending to channel {}: {:?}", channel_id, elements);
        Ok(vec!["mock_message_id".to_string()])
    }

    // 为使示例完整，其他 Adapter 方法也需要实现 (通常返回 Ok(()) 或 unimplemented!())
    async fn create_reaction( &self, _: &str, _: &str, _: &str,) -> FrameworkResult<()> { Ok(()) }
    async fn delete_reaction( &self, _: &str, _: &str, _: &str, _: &str,) -> FrameworkResult<()> { Ok(()) }
    async fn clear_reaction( &self, _: &str, _: &str, _: &str,) -> FrameworkResult<()> { Ok(()) }
    async fn get_reaction_list( &self, _: &str, _: &str, _: &str, _: Option<&str>,) -> FrameworkResult<Vec<User>> { Ok(vec![]) }
    async fn get_channel(&self, _: &str) -> FrameworkResult<Channel> { unimplemented!() }
    async fn get_channel_list( &self, _: &str, _: Option<&str>,) -> FrameworkResult<Vec<Channel>> { unimplemented!() }
    async fn create_channel(&self, _: &str, _: Channel) -> FrameworkResult<Channel> { unimplemented!() }
    async fn update_channel(&self, _: &str, _: Channel) -> FrameworkResult<()> { unimplemented!() }
    async fn delete_channel(&self, _: &str) -> FrameworkResult<()> { unimplemented!() }
    async fn create_direct_channel(&self, _: &str) -> FrameworkResult<Channel> { unimplemented!() }
    async fn set_guild_member_role( &self, _: &str, _: &str, _: &str,) -> FrameworkResult<()> { unimplemented!() }
    async fn unset_guild_member_role( &self, _: &str, _: &str, _: &str,) -> FrameworkResult<()> { unimplemented!() }
    async fn get_guild_member_role_list( &self, _: &str, _: Option<&str>,) -> FrameworkResult<Vec<GuildRole>> { unimplemented!() }
    async fn create_guild_role( &self, _: &str, _: &str,) -> FrameworkResult<GuildRole> { unimplemented!() }
    async fn update_guild_role( &self, _: &str, _: &str, _: GuildRole,) -> FrameworkResult<()> { unimplemented!() }
    async fn delete_guild_role(&self, _: &str, _: &str) -> FrameworkResult<()> { unimplemented!() }
    async fn send_private_message( &self, _: &str, _: &str, _: &[MessageElement],) -> FrameworkResult<Vec<String>> { unimplemented!() }
    async fn get_message(&self, _: &str, _: &str) -> FrameworkResult<Message> { unimplemented!() }
    async fn delete_message(&self, _: &str, _: &str) -> FrameworkResult<()> { unimplemented!() }
    async fn update_message( &self, _: &str, _: &str, _: &[MessageElement],) -> FrameworkResult<()> { unimplemented!() }
    async fn get_message_list( &self, _: &str, _: Option<&str>, _: Option<&str>,) -> FrameworkResult<Vec<Message>> { unimplemented!() }
    async fn get_user(&self, _: &str) -> FrameworkResult<User> { unimplemented!() }
    async fn get_friends(&self, _: Option<&str>) -> FrameworkResult<Vec<User>> { unimplemented!() }
    async fn handle_friend_request( &self, _: &str, _: bool, _: Option<&str>,) -> FrameworkResult<()> { unimplemented!() }
    async fn get_guild(&self, _: &str) -> FrameworkResult<Guild> { unimplemented!() }
    async fn get_guilds(&self, _: Option<&str>) -> FrameworkResult<Vec<Guild>> { unimplemented!() }
    async fn handle_guild_invite( &self, _: &str, _: bool, _: Option<&str>,) -> FrameworkResult<()> { unimplemented!() }
    async fn get_guild_member(&self, _: &str, _: &str) -> FrameworkResult<GuildMember> { unimplemented!() }
    async fn get_guild_members( &self, _: &str, _: Option<&str>,) -> FrameworkResult<Vec<GuildMember>> { unimplemented!() }
    async fn kick_guild_member( &self, _: &str, _: &str, _: Option<bool>,) -> FrameworkResult<()> { unimplemented!() }
    async fn mute_guild_member( &self, _: &str, _: &str, _: Option<u64>, _: &str,) -> FrameworkResult<()> { unimplemented!() }
    async fn handle_guild_request( &self, _: &str, _: bool, _: Option<&str>,) -> FrameworkResult<()> { unimplemented!() }
    async fn get_login(&self) -> FrameworkResult<Login> { unimplemented!() }
}

fn main() -> FrameworkResult<()> {
    // 1. 创建应用实例
    let mut app = App::new();

    // 2. 加载适配器
    // 你需要实现一个具体的 Adapter (例如 MyPlatformAdapter)
    app.load_adapter(MyPlatformAdapter);

    // 3. 获取根上下文并定义指令
    let ctx = app.context();
    ctx.command("ping")
        .description("Replies with pong!")
        .action(|session: Arc<Session>, _args| Box::pin(async move {
            // 构建回复消息
            let reply_elements = vec![MessageElement::Text { text: "pong!".to_string() }];
            // 通过 session 发送消息
            session.send(&reply_elements).await?;
            Ok(())
        }))
        .register()?;

    // 4. (可选) 加载插件
    // app.load_plugin(MyPlugin::new());

    // 5. 运行应用 (这将阻塞直到应用停止)
    // 注意：实际应用中，你可能需要处理来自 app.run() 的 Result
    if let Err(e) = app.run() {
        eprintln!("Application ended with error: {}", e);
    }

    Ok(())
}

use crate::{
    bot::Bot,
    context::Context,
    error::FrameworkResult,
    message::MessageElement,
    types::{Channel, Guild, GuildMember, GuildRole, Login, LoginStatus, Message, User},
};
use async_trait::async_trait;
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use url::Url;

/// 适配器 trait
/// 适配器负责与具体的聊天平台进行通信。
#[async_trait]
pub trait Adapter: Send + Sync + std::fmt::Debug {
    /// 获取适配器的名称
    fn get_name(&self) -> String;

    /// 连接到聊天平台并开始接收事件
    /// 此方法会接收一个 Arc<Bot> 的引用，以便适配器可以将事件传递给 Bot，
    /// 或者通过 Bot 调用其他服务。
    async fn connect(&self, bot: Arc<Bot>);

    /// 断开与聊天平台的连接
    async fn disconnect(&self, bot: Arc<Bot>);

    /// 向特定消息添加某个特定表态
    async fn create_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
    ) -> FrameworkResult<()>;

    /// 从特定消息删除某个用户添加的特定表态
    async fn delete_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
        user_id: &str,
    ) -> FrameworkResult<()>;

    /// 从特定消息清除某个特定表态
    async fn clear_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
    ) -> FrameworkResult<()>;

    /// 获取添加特定消息的特定表态的用户列表
    async fn get_reaction_list(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<User>>;

    /// 获取频道信息
    async fn get_channel(&self, channel_id: &str) -> FrameworkResult<Channel>;

    /// 获取某个群组的频道列表
    async fn get_channel_list(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<Channel>>;

    /// 创建群组频道
    async fn create_channel(&self, guild_id: &str, data: Channel) -> FrameworkResult<Channel>;

    /// 修改群组频道
    async fn update_channel(&self, channel_id: &str, data: Channel) -> FrameworkResult<()>;

    /// 删除群组频道
    async fn delete_channel(&self, channel_id: &str) -> FrameworkResult<()>;

    /// 创建私聊频道
    async fn create_direct_channel(&self, user_id: &str) -> FrameworkResult<Channel>;

    /// 设置群组内用户的角色
    async fn set_guild_member_role(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> FrameworkResult<()>;

    /// 取消群组内用户的角色
    async fn unset_guild_member_role(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> FrameworkResult<()>;

    /// 获取群组内用户的角色列表
    async fn get_guild_member_role_list(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<GuildRole>>;

    /// 创建群组角色
    async fn create_guild_role(
        &self,
        guild_id: &str,
        role_name: &str,
    ) -> FrameworkResult<GuildRole>;

    /// 修改群组角色
    async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        role: GuildRole,
    ) -> FrameworkResult<()>;

    /// 删除群组角色
    async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> FrameworkResult<()>;

    /// 向特定频道发送消息
    /// 返回消息ID列表
    async fn send_message(
        &self,
        channel_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<Vec<String>>;

    /// 向特定用户发送私信
    async fn send_private_message(
        &self,
        user_id: &str,
        guild_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<Vec<String>>;

    /// 获取特定消息
    async fn get_message(&self, channel_id: &str, message_id: &str) -> FrameworkResult<Message>;

    /// 撤回特定消息
    async fn delete_message(&self, channel_id: &str, message_id: &str) -> FrameworkResult<()>;

    /// 修改特定消息
    async fn update_message(
        &self,
        channel_id: &str,
        message_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<()>;

    /// 获取频道消息列表
    async fn get_message_list(
        &self,
        channel_id: &str,
        next: Option<&str>,
        directory: Option<&str>,
    ) -> FrameworkResult<Vec<Message>>;

    /// 获取用户信息
    async fn get_user(&self, user_id: &str) -> FrameworkResult<User>;

    /// 获取机器人的好友列表
    async fn get_friends(&self, next: Option<&str>) -> FrameworkResult<Vec<User>>;

    /// 处理好友请求
    async fn handle_friend_request(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()>;

    /// 获取群组信息
    async fn get_guild(&self, guild_id: &str) -> FrameworkResult<Guild>;

    /// 获取机器人加入的群组列表
    async fn get_guilds(&self, next: Option<&str>) -> FrameworkResult<Vec<Guild>>;

    /// 处理来自群组的邀请
    async fn handle_guild_invite(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()>;

    /// 获取群成员信息
    async fn get_guild_member(&self, guild_id: &str, user_id: &str)
    -> FrameworkResult<GuildMember>;

    /// 获取群成员列表
    async fn get_guild_members(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<GuildMember>>;

    /// 将某个用户踢出群组
    async fn kick_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        permanent: Option<bool>,
    ) -> FrameworkResult<()>;

    /// 禁言某个用户
    async fn mute_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        duration: Option<u64>,
        reason: &str,
    ) -> FrameworkResult<()>;

    /// 处理加群请求
    async fn handle_guild_request(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()>;

    /// 获取登陆状态
    async fn get_login(&self) -> FrameworkResult<Login>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WSClientConfig<C> {
    retry_lazy: u64,
    retry_times: u64,
    retry_interval: u64,
    _extend: Option<C>,
}

#[async_trait]
pub trait WSClient<C>: Adapter
where
    C: for<'de> Deserialize<'de> + Send,
{
    /// 获取适配器的上下文
    fn ctx(&self) -> Context;

    /// 获取适配器下的Bot实例
    fn bot(&self) -> Arc<Bot>;

    /// 获取适配器的WebSocket实例
    fn socket(&self) -> Option<WebSocketStream<MaybeTlsStream<TcpStream>>>;

    /// 获取适配器的配置
    fn config(&self) -> WSClientConfig<C>;

    /// 根据Bot实例生成一个WebSocket对象
    async fn prepare(&self) -> FrameworkResult<(WebSocketStream<MaybeTlsStream<TcpStream>>, Url)>;

    /// WebSocket连接成功后建立的回调函数
    async fn accept(&self);

    /// 设置status
    fn set_status(&self, status: LoginStatus);

    /// 获取适配器的状态
    fn get_active(&self) -> bool;

    async fn start(&self) {
        let mut retry_count = 0;
        let ws_config = self.config();

        loop {
            if !self.get_active() {
                tracing::debug!(
                    "Adapter {} is not active, stopping connection attempts.",
                    self.get_name()
                );
                self.set_status(LoginStatus::Offline);
                return;
            }

            tracing::debug!(
                "Adapter {} (attempt {}): Trying to connect...",
                self.get_name(),
                retry_count + 1
            );

            let mut socket_stream = match self.prepare().await {
                Ok((stream, _url)) => {
                    self.set_status(LoginStatus::Online);
                    tracing::info!("Adapter {} connected successfully.", self.get_name());
                    if retry_count > 0 {
                        retry_count = 0;
                    }
                    self.accept().await;
                    stream
                }
                Err(e) => {
                    tracing::warn!(
                        "Adapter {} failed to prepare connection: {}",
                        self.get_name(),
                        e
                    );
                    let timeout = if retry_count >= ws_config.retry_times {
                        if ws_config.retry_lazy == 0 {
                            tracing::error!(
                                "Adapter {} reached max retry attempts ({}) and no lazy retry configured. Stopping.",
                                self.get_name(),
                                ws_config.retry_times
                            );
                            self.set_status(LoginStatus::Offline);
                            return;
                        }
                        if retry_count == ws_config.retry_times {
                            tracing::warn!(
                                "Adapter {} reached max retry attempts. Falling back to lazy retry ({}ms).",
                                self.get_name(),
                                ws_config.retry_lazy
                            );
                        }
                        ws_config.retry_lazy
                    } else {
                        ws_config.retry_interval
                    };

                    retry_count += 1;
                    self.set_status(LoginStatus::Reconnect);
                    tracing::info!(
                        "Adapter {} will retry connection in {}ms (attempt {}).",
                        self.get_name(),
                        timeout,
                        retry_count
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
                    continue;
                }
            };

            tracing::debug!("Adapter {} listening for messages.", self.get_name());
            while let Some(message_result) = socket_stream.next().await {
                if !self.get_active() {
                    tracing::info!(
                        "Adapter {} became inactive while listening. Closing connection.",
                        self.get_name()
                    );
                    let _ = socket_stream.close(None).await; // Attempt to close gracefully
                    self.set_status(LoginStatus::Offline);
                    return;
                }

                match message_result {
                    Ok(msg) => {
                        if msg.is_close() {
                            tracing::info!(
                                "Adapter {} received WebSocket Close frame. Connection closed by peer.",
                                self.get_name(),
                            );
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "Adapter {} error while receiving message: {}. Attempting to reconnect.",
                            self.get_name(),
                            e
                        );
                        break;
                    }
                }
            }

            if !self.get_active() {
                tracing::info!(
                    "Adapter {} became inactive after message loop. Not reconnecting.",
                    self.get_name()
                );
                self.set_status(LoginStatus::Offline);
                return;
            }

            tracing::warn!(
                "Adapter {} disconnected or encountered an error in message loop. Preparing to reconnect.",
                self.get_name()
            );

            let timeout = if retry_count >= ws_config.retry_times {
                if ws_config.retry_lazy == 0 {
                    tracing::error!(
                        "Adapter {} reached max retry attempts ({}) for disconnection and no lazy retry. Stopping.",
                        self.get_name(),
                        ws_config.retry_times
                    );
                    self.set_status(LoginStatus::Offline);
                    return;
                }
                if retry_count == ws_config.retry_times {
                    tracing::warn!(
                        "Adapter {} reached max retry attempts for disconnection. Falling back to lazy retry ({}ms).",
                        self.get_name(),
                        ws_config.retry_lazy
                    );
                }
                ws_config.retry_lazy
            } else {
                ws_config.retry_interval
            };

            retry_count += 1;
            self.set_status(LoginStatus::Reconnect);
            tracing::info!(
                "Adapter {} will retry connection in {}ms (attempt {}).",
                self.get_name(),
                timeout,
                retry_count
            );
            tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
        }
    }

    async fn stop(&self) -> FrameworkResult<()> {
        if let Some(mut socket) = self.socket() {
            socket.close(None).await?;
        }
        self.set_status(LoginStatus::Offline);
        tracing::info!("适配器 {} 已停止。", self.get_name());
        Ok(())
    }
}

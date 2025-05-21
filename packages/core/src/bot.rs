use crate::adapter::Adapter;
use crate::context::Context;
use crate::error::FrameworkResult;
use crate::message::MessageElement;
use crate::types::*;
use std::sync::Arc;

/// Bot 结构体，代表一个机器人实例
pub struct Bot {
    /// Bot所属的适配器实例
    pub adapter: Arc<dyn Adapter>,
    /// Bot配置
    // pub config: Arc<C>,
    /// Bot 所在的Context实例
    pub ctx: Arc<Context>,
    /// Bot的所在平台名称
    pub platform: String,
    /// Bot在平台上的 ID
    pub self_id: String,
    /// Bot的登录状态
    pub state: LoginStatus,
    /// Bot的用户信息
    pub user: User,
}

impl Bot {
    /// 创建一个新的 Bot 实例
    ///
    /// # Arguments
    ///
    /// * `ctx` - Bot 的应用上下文 (Arc<Context>)
    /// * `config` - 机器人的基础配置
    /// * `adapter_instance` - 实现了 Adapter trait 的适配器实例
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Bot` 实例
    pub fn new(ctx: Arc<Context>, adapter_instance: Arc<dyn Adapter>) -> Self {
        let platform = adapter_instance.get_name();
        Bot {
            adapter: adapter_instance,
            // config: Arc::new(config),
            ctx,
            platform,
            self_id: String::new(),
            state: LoginStatus::Offline,
            user: User::default(),
        }
    }

    /// 启动适配器
    ///
    /// # Arguments
    ///
    /// * `self_arc` - 对 Bot 自身的 Arc 引用，用于传递给适配器。
    ///
    /// # Returns
    ///
    /// 如果适配器成功启动，返回 `Ok(())`，否则返回遇到的错误
    pub async fn start(self: Arc<Self>) -> FrameworkResult<()> {
        tracing::info!("尝试启动适配器: {}", self.platform);
        self.adapter.connect(self.clone()).await; // self 在这里是 Arc<Bot>
        tracing::info!("适配器 {} 已启动。", self.platform);
        Ok(())
    }

    /// 停止所有已注册的适配器
    ///
    /// # Arguments
    ///
    /// * `self_arc` - 对 Bot 自身的 Arc 引用，用于传递给适配器。
    ///
    /// # Returns
    ///
    /// 如果配器成功停止，返回 `Ok(())`，否则返回遇到的错误
    pub async fn stop(self: Arc<Self>) -> FrameworkResult<()> {
        tracing::info!("尝试停止适配器: {}", self.platform);
        self.adapter.disconnect(self.clone()).await;
        tracing::info!("适配器 {} 已停止。", self.platform);
        Ok(())
    }

    /// 修改登录状态为在线
    pub fn online(&mut self) {
        self.state = LoginStatus::Online
    }

    /// 修改登录状态为离线
    pub fn offline(&mut self) {
        self.state = LoginStatus::Offline
    }

    /// 向特定消息添加某个特定表态
    pub async fn create_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .create_reaction(message_id, channel_id, emoji)
            .await
    }

    /// 从特定消息删除某个用户添加的特定表态
    pub async fn delete_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
        user_id: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .delete_reaction(message_id, channel_id, emoji, user_id)
            .await
    }

    /// 从特定消息清除某个特定表态
    pub async fn clear_reaction(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .clear_reaction(message_id, channel_id, emoji)
            .await
    }

    /// 获取添加特定消息的特定表态的用户列表
    pub async fn get_reaction_list(
        &self,
        message_id: &str,
        channel_id: &str,
        emoji: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<User>> {
        self.adapter
            .get_reaction_list(message_id, channel_id, emoji, next)
            .await
    }

    /// 获取频道信息
    pub async fn get_channel(&self, channel_id: &str) -> FrameworkResult<Channel> {
        self.adapter.get_channel(channel_id).await
    }

    /// 获取某个群组的频道列表
    pub async fn get_channel_list(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<Channel>> {
        self.adapter.get_channel_list(guild_id, next).await
    }

    /// 创建群组频道
    pub async fn create_channel(&self, guild_id: &str, data: Channel) -> FrameworkResult<Channel> {
        self.adapter.create_channel(guild_id, data).await
    }

    /// 修改群组频道
    pub async fn update_channel(&self, channel_id: &str, data: Channel) -> FrameworkResult<()> {
        self.adapter.update_channel(channel_id, data).await
    }

    /// 删除群组频道
    pub async fn delete_channel(&self, channel_id: &str) -> FrameworkResult<()> {
        self.adapter.delete_channel(channel_id).await
    }

    /// 创建私聊频道
    pub async fn create_direct_channel(&self, user_id: &str) -> FrameworkResult<Channel> {
        self.adapter.create_direct_channel(user_id).await
    }

    /// 设置群组内用户的角色
    pub async fn set_guild_member_role(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .set_guild_member_role(guild_id, user_id, role_id)
            .await
    }

    /// 取消群组内用户的角色
    pub async fn unset_guild_member_role(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .unset_guild_member_role(guild_id, user_id, role_id)
            .await
    }

    /// 获取群组内用户的角色列表
    pub async fn get_guild_member_role_list(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<GuildRole>> {
        self.adapter
            .get_guild_member_role_list(guild_id, next)
            .await
    }

    /// 创建群组角色
    pub async fn create_guild_role(
        &self,
        guild_id: &str,
        role_name: &str,
    ) -> FrameworkResult<GuildRole> {
        self.adapter.create_guild_role(guild_id, role_name).await
    }

    /// 修改群组角色
    pub async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        role: GuildRole,
    ) -> FrameworkResult<()> {
        self.adapter
            .update_guild_role(guild_id, role_id, role)
            .await
    }

    /// 删除群组角色
    pub async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> FrameworkResult<()> {
        self.adapter.delete_guild_role(guild_id, role_id).await
    }

    /// 向特定频道发送消息
    pub async fn send_message(
        &self,
        channel_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<Vec<String>> {
        self.adapter.send_message(channel_id, elements).await
    }

    /// 向特定用户发送私信
    pub async fn send_private_message(
        &self,
        user_id: &str,
        guild_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<Vec<String>> {
        self.adapter
            .send_private_message(user_id, guild_id, elements)
            .await
    }

    /// 获取特定消息
    pub async fn get_message(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> FrameworkResult<Message> {
        self.adapter.get_message(channel_id, message_id).await
    }

    /// 撤回特定消息
    pub async fn delete_message(&self, channel_id: &str, message_id: &str) -> FrameworkResult<()> {
        self.adapter.delete_message(channel_id, message_id).await
    }

    /// 修改特定消息
    pub async fn update_message(
        &self,
        channel_id: &str,
        message_id: &str,
        elements: &[MessageElement],
    ) -> FrameworkResult<()> {
        self.adapter
            .update_message(channel_id, message_id, elements)
            .await
    }

    /// 获取频道消息列表
    pub async fn get_message_list(
        &self,
        channel_id: &str,
        next: Option<&str>,
        directory: Option<&str>,
    ) -> FrameworkResult<Vec<Message>> {
        self.adapter
            .get_message_list(channel_id, next, directory)
            .await
    }

    /// 向多个频道广播消息
    pub async fn broadcast(
        &self,
        channels: Vec<String>,
        elements: &[MessageElement],
    ) -> FrameworkResult<()> {
        for channel in channels {
            self.adapter
                .send_message(channel.as_str(), elements)
                .await?;
        }
        Ok(())
    }

    /// 获取用户信息
    pub async fn get_user(&self, user_id: &str) -> FrameworkResult<User> {
        self.adapter.get_user(user_id).await
    }

    /// 获取机器人的好友列表
    pub async fn get_friend_list(&self, next: Option<&str>) -> FrameworkResult<Vec<User>> {
        self.adapter.get_friends(next).await
    }

    /// 处理好友请求
    pub async fn handle_friend_request(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()> {
        self.adapter
            .handle_friend_request(message_id, accept, comment)
            .await
    }

    /// 获取群组信息
    pub async fn get_guild(&self, guild_id: &str) -> FrameworkResult<Guild> {
        self.adapter.get_guild(guild_id).await
    }

    /// 获取机器人加入的群组列表
    pub async fn get_guild_list(&self, next: Option<&str>) -> FrameworkResult<Vec<Guild>> {
        self.adapter.get_guilds(next).await
    }

    /// 处理来自群组的邀请
    pub async fn handle_guild_invite(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()> {
        self.adapter
            .handle_guild_invite(message_id, accept, comment)
            .await
    }

    /// 获取群成员信息
    pub async fn get_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
    ) -> FrameworkResult<GuildMember> {
        self.adapter.get_guild_member(guild_id, user_id).await
    }

    /// 获取群成员列表
    pub async fn get_guild_member_list(
        &self,
        guild_id: &str,
        next: Option<&str>,
    ) -> FrameworkResult<Vec<GuildMember>> {
        self.adapter.get_guild_members(guild_id, next).await
    }

    /// 将某个用户踢出群组
    pub async fn kick_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        permanent: Option<bool>,
    ) -> FrameworkResult<()> {
        self.adapter
            .kick_guild_member(guild_id, user_id, permanent)
            .await
    }

    /// 禁言某个用户
    pub async fn mute_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        duration: Option<u64>,
        reason: &str,
    ) -> FrameworkResult<()> {
        self.adapter
            .mute_guild_member(guild_id, user_id, duration, reason)
            .await
    }

    /// 处理加群请求
    pub async fn handle_guild_request(
        &self,
        message_id: &str,
        accept: bool,
        comment: Option<&str>,
    ) -> FrameworkResult<()> {
        self.adapter
            .handle_guild_request(message_id, accept, comment)
            .await
    }

    /// 获取登陆状态
    pub async fn get_login(&self) -> FrameworkResult<Login> {
        self.adapter.get_login().await
    }
}

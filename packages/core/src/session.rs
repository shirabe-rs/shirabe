use serde::Deserialize;

use crate::bot::Bot;
use crate::context::Context;
use crate::error::FrameworkResult;
use crate::types::{Channel, ChannelType, Guild, GuildMember, GuildRole, Login, Message, User};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct SessionEvent {
    /// 事件 ID
    pub id: i64,
    /// 事件类型
    #[serde(rename = "type")]
    pub ty: String,
    /// 接受者的平台名称
    pub platform: String,
    /// 接受者的平台 ID
    pub self_id: String,
    /// 事件发生的时间戳
    pub timestamp: i64,
    /// 事件所属的频道
    pub channel: Channel,
    /// 事件所属的群组
    pub guild: Guild,
    /// 事件的登录信息
    pub login: Login,
    /// 事件的目标成员
    pub member: GuildMember,
    /// 事件的消息
    pub message: Message,
    /// 事件的操作者
    pub operator: User,
    /// 事件的目标角色
    pub role: GuildRole,
    /// 事件的目标用户
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    /// 用户对象
    pub user: User,
    /// 成员在群组中的昵称
    pub nick: Option<String>,
    /// 成员在群组中的头像链接
    pub avatar: Option<String>,
    /// 用户 ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 是否为机器人
    pub is_bot: Option<bool>,
    /// 加入时间
    #[serde(rename = "joined_at")]
    pub joined_at_ms: Option<i64>,
}

/// 表示一个会话，封装了事件上下文并提供了便捷的交互方法。
#[derive(Debug)]
pub struct Session {
    /// 当前会话的应用上下文。
    pub app: Arc<Context>,
    /// 当前会话绑定的机器人实例。
    pub bot: Arc<Bot>,
    /// 当前会话绑定的频道数据
    pub channel: Channel,
    /// 会话事件数据
    pub event: SessionEvent,
    /// 当前会话绑定的用户数据
    pub user: User,
    pub author: Author,
    pub channel_id: String,
    pub channel_name: String,
    pub content: String,
    // pub elements: Vec<MessageElement>,
    pub guild_id: String,
    pub guild_name: String,
    pub id: String,
    pub is_direct: bool,
    pub message_id: String,
    pub platform: String,
    pub quote: Option<Box<Message>>,
    pub self_id: String,
    pub timestamp: i64,
    pub type_: String,
    pub user_id: String,
}

impl Session {
    /// 创建一个新的会话实例。
    ///
    /// # 参数
    ///
    /// * `bot` - `Bot` 实例的 `Arc` 引用。
    /// * `event` - 触发会话的 `SatoriEvent`。
    pub fn new(bot: Arc<Bot>, event: SessionEvent) -> Self {
        let app = bot.ctx.clone();
        let channel = event.channel.clone();
        let user = event.user.clone();
        let nick = match event.member.nick.clone() {
            Some(nick) => Some(nick),
            None => event.user.name.clone(),
        };
        let avatar = match event.member.avatar.clone() {
            Some(avatar) => Some(avatar),
            None => event.user.avatar.clone(),
        };
        let author = Author {
            user: event.user.clone(),
            nick,
            avatar,
            id: event.user.id.clone(),
            name: event.user.name.clone(),
            is_bot: event.user.is_bot,
            joined_at_ms: event.member.joined_at_ms,
        };
        let channel_id = event.channel.id.clone();
        let channel_name = event.channel.name.clone();
        let content = event.message.content.clone();
        // let elements = event.message.elements.clone();
        let guild_id = event.guild.id.clone();
        let guild_name = event.guild.name.clone();
        let id = event.id.to_string();
        let is_direct = event.channel.ty == ChannelType::Direct;
        let message_id = event.message.id.to_string();
        let platform = event.platform.clone();
        let quote = event.message.quote.clone();
        let self_id = event.self_id.clone();
        let timestamp = event.timestamp;
        let type_ = event.ty.clone();
        let user_id = user.id.clone();

        Session {
            app,
            bot,
            channel,
            event,
            user,
            author,
            channel_id,
            channel_name,
            content,
            // elements,
            guild_id,
            guild_name,
            id,
            is_direct,
            message_id,
            platform,
            quote,
            self_id,
            timestamp,
            type_,
            user_id,
        }
    }

    /// 在当前上下文发送消息
    pub async fn send(&self, message: String) -> FrameworkResult<Vec<String>> {
        self.bot.send_message(&self.channel_id, &message).await
    }
}

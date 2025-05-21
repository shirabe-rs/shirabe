use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SatoriEvent {
    // 消息事件
    #[serde(rename = "message-created")]
    MessageCreated(EventPayload),
    #[serde(rename = "message-updated")]
    MessageUpdated(EventPayload),
    #[serde(rename = "message-deleted")]
    MessageDeleted(EventPayload),

    // 群组事件
    #[serde(rename = "guild-added")]
    GuildAdded(EventPayload),
    #[serde(rename = "guild-updated")]
    GuildUpdated(EventPayload),
    #[serde(rename = "guild-removed")]
    GuildRemoved(EventPayload),
    #[serde(rename = "guild-request")]
    GuildRequest(EventPayload),

    // 群组成员事件
    #[serde(rename = "guild-member-added")]
    GuildMemberAdded(EventPayload),
    #[serde(rename = "guild-member-updated")]
    GuildMemberUpdated(EventPayload),
    #[serde(rename = "guild-member-removed")]
    GuildMemberRemoved(EventPayload),
    #[serde(rename = "guild-member-request")]
    GuildMemberRequest(EventPayload),

    // 群组角色事件
    #[serde(rename = "guild-role-created")]
    GuildRoleCreated(EventPayload),
    #[serde(rename = "guild-role-updated")]
    GuildRoleUpdated(EventPayload),
    #[serde(rename = "guild-role-deleted")]
    GuildRoleDeleted(EventPayload),

    // 表情回应事件
    #[serde(rename = "reaction-added")]
    ReactionAdded(EventPayload),
    #[serde(rename = "reaction-removed")]
    ReactionRemoved(EventPayload),

    // 登录事件
    #[serde(rename = "login-added")]
    LoginAdded(EventPayload),
    #[serde(rename = "login-removed")]
    LoginRemoved(EventPayload),
    #[serde(rename = "login-updated")]
    LoginUpdated(EventPayload),

    // 好友请求事件
    #[serde(rename = "friend-request")]
    FriendRequest(EventPayload),

    // 交互事件
    #[serde(rename = "interaction/button")]
    InteractionButton(EventPayload),
    #[serde(rename = "interaction/command")]
    InteractionCommand(EventPayload),

    #[serde(other)] // 捕获所有未知的事件类型
    Unknown,
}

impl SatoriEvent {
    pub fn event_type_str(&self) -> &'static str {
        match self {
            SatoriEvent::MessageCreated(_) => "message-created",
            SatoriEvent::MessageUpdated(_) => "message-updated",
            SatoriEvent::MessageDeleted(_) => "message-deleted",
            SatoriEvent::GuildAdded(_) => "guild-added",
            SatoriEvent::GuildUpdated(_) => "guild-updated",
            SatoriEvent::GuildRemoved(_) => "guild-removed",
            SatoriEvent::GuildRequest(_) => "guild-request",
            SatoriEvent::GuildMemberAdded(_) => "guild-member-added",
            SatoriEvent::GuildMemberUpdated(_) => "guild-member-updated",
            SatoriEvent::GuildMemberRemoved(_) => "guild-member-removed",
            SatoriEvent::GuildMemberRequest(_) => "guild-member-request",
            SatoriEvent::GuildRoleCreated(_) => "guild-role-created",
            SatoriEvent::GuildRoleUpdated(_) => "guild-role-updated",
            SatoriEvent::GuildRoleDeleted(_) => "guild-role-deleted",
            SatoriEvent::ReactionAdded(_) => "reaction-added",
            SatoriEvent::ReactionRemoved(_) => "reaction-removed",
            SatoriEvent::LoginAdded(_) => "login-added",
            SatoriEvent::LoginRemoved(_) => "login-removed",
            SatoriEvent::LoginUpdated(_) => "login-updated",
            SatoriEvent::FriendRequest(_) => "friend-request",
            SatoriEvent::InteractionButton(_) => "interaction/button",
            SatoriEvent::InteractionCommand(_) => "interaction/command",
            SatoriEvent::Unknown => "unknown",
        }
    }

    /// 从任何包含它的 SatoriEvent 变体中获取通用 EventPayload 的辅助方法。
    pub fn common_payload(&self) -> Option<&EventPayload> {
        match self {
            SatoriEvent::MessageCreated(p)
            | SatoriEvent::MessageUpdated(p)
            | SatoriEvent::MessageDeleted(p)
            | SatoriEvent::GuildAdded(p)
            | SatoriEvent::GuildUpdated(p)
            | SatoriEvent::GuildRemoved(p)
            | SatoriEvent::GuildRequest(p)
            | SatoriEvent::GuildMemberAdded(p)
            | SatoriEvent::GuildMemberUpdated(p)
            | SatoriEvent::GuildMemberRemoved(p)
            | SatoriEvent::GuildMemberRequest(p)
            | SatoriEvent::GuildRoleCreated(p)
            | SatoriEvent::GuildRoleUpdated(p)
            | SatoriEvent::GuildRoleDeleted(p)
            | SatoriEvent::ReactionAdded(p)
            | SatoriEvent::ReactionRemoved(p)
            | SatoriEvent::LoginAdded(p)
            | SatoriEvent::LoginRemoved(p)
            | SatoriEvent::LoginUpdated(p)
            | SatoriEvent::FriendRequest(p)
            | SatoriEvent::InteractionButton(p)
            | SatoriEvent::InteractionCommand(p) => Some(p),
            SatoriEvent::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    /// 序列号
    pub sn: i64,
    /// 事件的时间戳
    pub timestamp: i64,
    /// 登录信息
    pub login: Login,
    /// 交互指令
    pub argv: Option<Argv>,
    /// 交互按钮
    pub button: Option<Button>,
    /// 事件所属的频道
    pub channel: Option<Channel>,
    /// 事件所属的群组
    pub guild: Option<Guild>,
    /// 事件所属的目标成员
    pub member: Option<GuildMember>,
    /// 事件的消息
    pub message: Option<Message>,
    /// 事件的操作者
    pub operator: Option<User>,
    /// 事件的目标角色
    pub role: Option<GuildRole>,
    /// 事件的目标用户
    pub user: Option<User>,
}

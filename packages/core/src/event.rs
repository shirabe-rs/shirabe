use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShirabeEvent {
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

impl ShirabeEvent {
    pub fn event_type_str(&self) -> &'static str {
        match self {
            ShirabeEvent::MessageCreated(_) => "message-created",
            ShirabeEvent::MessageUpdated(_) => "message-updated",
            ShirabeEvent::MessageDeleted(_) => "message-deleted",
            ShirabeEvent::GuildAdded(_) => "guild-added",
            ShirabeEvent::GuildUpdated(_) => "guild-updated",
            ShirabeEvent::GuildRemoved(_) => "guild-removed",
            ShirabeEvent::GuildRequest(_) => "guild-request",
            ShirabeEvent::GuildMemberAdded(_) => "guild-member-added",
            ShirabeEvent::GuildMemberUpdated(_) => "guild-member-updated",
            ShirabeEvent::GuildMemberRemoved(_) => "guild-member-removed",
            ShirabeEvent::GuildMemberRequest(_) => "guild-member-request",
            ShirabeEvent::GuildRoleCreated(_) => "guild-role-created",
            ShirabeEvent::GuildRoleUpdated(_) => "guild-role-updated",
            ShirabeEvent::GuildRoleDeleted(_) => "guild-role-deleted",
            ShirabeEvent::ReactionAdded(_) => "reaction-added",
            ShirabeEvent::ReactionRemoved(_) => "reaction-removed",
            ShirabeEvent::LoginAdded(_) => "login-added",
            ShirabeEvent::LoginRemoved(_) => "login-removed",
            ShirabeEvent::LoginUpdated(_) => "login-updated",
            ShirabeEvent::FriendRequest(_) => "friend-request",
            ShirabeEvent::InteractionButton(_) => "interaction/button",
            ShirabeEvent::InteractionCommand(_) => "interaction/command",
            ShirabeEvent::Unknown => "unknown",
        }
    }

    /// 从任何包含它的 SatoriEvent 变体中获取通用 EventPayload 的辅助方法。
    pub fn common_payload(&self) -> Option<&EventPayload> {
        match self {
            ShirabeEvent::MessageCreated(p)
            | ShirabeEvent::MessageUpdated(p)
            | ShirabeEvent::MessageDeleted(p)
            | ShirabeEvent::GuildAdded(p)
            | ShirabeEvent::GuildUpdated(p)
            | ShirabeEvent::GuildRemoved(p)
            | ShirabeEvent::GuildRequest(p)
            | ShirabeEvent::GuildMemberAdded(p)
            | ShirabeEvent::GuildMemberUpdated(p)
            | ShirabeEvent::GuildMemberRemoved(p)
            | ShirabeEvent::GuildMemberRequest(p)
            | ShirabeEvent::GuildRoleCreated(p)
            | ShirabeEvent::GuildRoleUpdated(p)
            | ShirabeEvent::GuildRoleDeleted(p)
            | ShirabeEvent::ReactionAdded(p)
            | ShirabeEvent::ReactionRemoved(p)
            | ShirabeEvent::LoginAdded(p)
            | ShirabeEvent::LoginRemoved(p)
            | ShirabeEvent::LoginUpdated(p)
            | ShirabeEvent::FriendRequest(p)
            | ShirabeEvent::InteractionButton(p)
            | ShirabeEvent::InteractionCommand(p) => Some(p),
            ShirabeEvent::Unknown => None,
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

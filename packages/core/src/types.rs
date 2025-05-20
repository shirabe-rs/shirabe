use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct User {
    /// 用户 ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户昵称
    pub nick: Option<String>,
    /// 用户头像链接
    pub avatar: Option<String>,
    /// 是否为机器人
    pub is_bot: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// 消息ID
    pub id: String,
    /// 消息内容
    pub content: String,
    /// 消息所在的频道对象
    pub channel: Option<Channel>,
    /// 消息所在的群组对象
    pub guild: Option<Guild>,
    /// 群组成员对象
    pub member: Option<GuildMember>,
    /// 回复的消息对象
    pub quote: Option<Box<Message>>,
    /// 用户对象
    pub user: Option<User>, // 发送者
    /// 消息发送时间的时间戳
    #[serde(rename = "created_at")]
    pub created_at_ms: Option<i64>,
    /// 消息修改时间的时间戳
    #[serde(rename = "updated_at")]
    pub updated_at_ms: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    /// 频道 ID
    pub id: String,
    /// 频道类型
    #[serde(rename = "type")]
    pub ty: ChannelType,
    /// 频道名称
    pub name: String,
    /// 父频道ID
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum ChannelType {
    /// 文本频道
    Text = 0,
    /// 私聊频道
    Direct = 1,
    /// 分类频道
    Category = 2,
    /// 语音频道
    Voice = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    /// 群组 ID
    pub id: String,
    /// 群组名称
    pub name: String,
    /// 群组头像链接
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuildMember {
    /// 用户对象
    pub user: Option<User>,
    /// 成员在群组中的昵称
    pub nick: Option<String>,
    /// 成员在群组中的头像链接
    pub avatar: Option<String>,
    /// 加入时间
    #[serde(rename = "joined_at")]
    pub joined_at_ms: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuildRole {
    /// 角色 ID
    pub id: String,
    /// 角色名称
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Argv {
    /// 指令名称
    pub name: String,
    /// 参数
    pub argument: Vec<String>,
    /// 选项
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Button {
    /// 按钮 ID
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    /// 序列号
    pub sn: i64,
    /// 平台名称
    pub platform: Option<String>,
    /// 用户对象
    pub user: Option<User>,
    /// 登录状态
    pub status: LoginStatus,
    /// 适配器名称
    pub adapter: String,
    /// 平台特性列表
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[repr(u8)]
pub enum LoginStatus {
    /// 离线
    Offline = 0,
    /// 在线
    Online = 1,
    /// 正在连接
    Connect = 2,
    /// 正在断开连接
    Disconnect = 3,
    /// 正在重新连接
    Reconnect = 4,
}

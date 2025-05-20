use crate::event::ShirabeEvent;
use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum OpCode {
    /// 事件
    Event = 0,
    /// 心跳
    Ping = 1,
    /// 心跳响应
    Pong = 2,
    /// 鉴权
    Identify = 3,
    /// 鉴权成功
    Ready = 4,
    /// 元信息更新
    Meta = 5,
}

#[derive(Debug, Clone, Deserialize, Serialize)] // Identify 通常是客户端序列化
pub struct IdentifyData {
    /// 鉴权令牌
    pub token: Option<String>,
    /// 序列号
    pub sn: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyData {
    /// 登录信息
    pub logins: Vec<Login>,
    /// 代理路由列表
    pub proxy_urls: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PingData;

#[derive(Debug, Clone, Deserialize, Serialize)] // Pong 通常是客户端序列化
pub struct PongData;

#[derive(Debug, Clone, Deserialize)]
pub struct MetaData {
    /// 代理路由列表
    pub proxy_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    #[serde(flatten)]
    pub data: ShirabeEvent,
}

#[derive(Debug, Deserialize)]
pub struct SatoriFrame {
    /// 操作码
    pub op: OpCode,
    /// 数据体
    pub body: Option<Value>,
}

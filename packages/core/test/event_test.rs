#[cfg(test)]
mod test {
    use serde_json::{self, json};

    #[test]
    /// 测试 EventData 的 JSON 反序列化
    fn test_event_data_schema() {
        use crate::connection::EventData;

        let _: EventData = serde_json::from_value(json!({
            "type": "message-created", // 事件类型
            "sn": 123, // 序列号
            "timestamp": 123456789, // 时间戳
            "login": { // 登录信息
                "sn": 123, // 序列号
                "platform": "test_platform", // 平台名称
                "user": null, // 用户对象
                "status": "Connect", // 登录状态
                "adapter": "test_adapter", // 适配器名称
                "features": [] // 平台特性列表
            },
            "argv": null, // 交互指令
            "button": null, // 交互按钮
            "channel": null, // 事件所属的频道
            "guild": null, // 事件所属的群组
            "member": null, // 事件所属的目标成员
            "message": null, // 事件的消息
            "operator": null, // 事件的操作者
            "role": null, // 事件的目标角色
            "user": null // 事件的目标用户
        }))
        .unwrap();
    }
}

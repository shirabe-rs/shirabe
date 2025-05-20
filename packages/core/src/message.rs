use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageElement {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "at")]
    At {
        id: String,
        name: Option<String>,
        role: Option<String>,
    },
    #[serde(rename = "img")]
    Image {
        src: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    // TODO: Satori 支持的所有消息元素类型
}

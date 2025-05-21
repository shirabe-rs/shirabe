use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageElement {
    /// 文本
    #[serde(rename = "text")]
    Text {
        /// 消息文本
        #[serde(rename = "content")]
        text: String,
    },
    /// 提及用户
    #[serde(rename = "at")]
    At {
        /// 目标用户的 ID
        id: String,
        /// 目标用户的名称
        name: Option<String>,
        /// 目标角色
        role: Option<String>,
        #[serde(rename = "type")]
        at_type: Option<String>, // all 表示 @全体成员，here 表示 @在线成员
    },

    /// 提及频道
    #[serde(rename = "sharp")]
    Sharp {
        /// 目标频道的 ID
        id: String,
        /// 目标频道的名称
        name: Option<String>,
    },

    /// 图片
    #[serde(rename = "img")]
    Image {
        /// 资源的 URL
        src: String,
        /// 资源的名称
        title: Option<String>,
        /// 图片宽度 (像素)
        width: Option<u32>,
        /// 图片高度 (像素)
        height: Option<u32>,
        /// 是否使用已缓存的文件
        cache: Option<bool>,
        /// 下载文件的最长时间 (毫秒)
        timeout: Option<String>,
    },
    #[serde(rename = "audio")]
    /// 音频
    Audio {
        /// 资源的 URL
        src: String,
        /// 资源的名称
        title: Option<String>,
        /// 音频长度 (秒)
        duration: Option<u64>,
        /// 音频封面 URL
        poster: Option<String>,
        /// 是否使用已缓存的文件
        cache: Option<bool>,
        /// 下载文件的最长时间 (毫秒)
        timeout: Option<String>,
    },
    /// 视频
    #[serde(rename = "video")]
    Video {
        /// 资源的 URL
        src: String,
        /// 资源的名称
        title: Option<String>,
        /// 视频长度 (秒)
        duration: Option<u64>,
        /// 视频封面 URL
        poster: Option<String>,
        /// 视频宽度 (像素)
        width: Option<u32>,
        /// 视频高度 (像素)
        height: Option<u32>,
        /// 是否使用已缓存的文件
        cache: Option<bool>,
        /// 下载文件的最长时间 (毫秒)
        timeout: Option<String>,
    },
    /// 文件
    #[serde(rename = "file")]
    File {
        /// 资源的 URL
        src: String,
        /// 资源的名称
        name: Option<String>,
        /// 缩略图封面 URL
        poster: Option<String>,
        /// 是否使用已缓存的文件
        cache: Option<bool>,
        /// 下载文件的最长时间 (毫秒)
        timeout: Option<String>,
    },
    /// 引用
    #[serde(rename = "quote")]
    Quote {
        /// 引用的消息ID
        id: String,
        /// 是否为合并转发
        forward: Option<bool>,
        /// 子元素
        children: Vec<MessageElement>,
    },
    /// 作者
    #[serde(rename = "author")]
    Author {
        /// 用户 ID
        id: String,
        /// 昵称
        name: Option<String>,
        /// 头像 URL
        avatar: Option<String>,
    },
    /// 消息
    #[serde(rename = "message")]
    Message {
        /// 消息的 ID
        id: Option<String>,
        /// 是否为转发消息
        forward: Option<bool>,
        /// 消息的子元素
        children: Vec<MessageElement>,
    },

    // 修饰元素
    /// 粗体
    #[serde(rename = "strong")]
    Bold { children: Vec<MessageElement> },
    /// 斜体
    #[serde(rename = "em")]
    Italic { children: Vec<MessageElement> },
    /// 下划线
    #[serde(rename = "u")]
    Underline { children: Vec<MessageElement> },
    /// 删除线
    #[serde(rename = "s")]
    Strikethrough { children: Vec<MessageElement> },
    /// 剧透
    #[serde(rename = "spl")]
    Spoiler { children: Vec<MessageElement> },
    /// 代码
    #[serde(rename = "code")]
    Code { children: Vec<MessageElement> },
    /// 上标
    #[serde(rename = "sup")]
    Superscript { children: Vec<MessageElement> },
    /// 下标
    #[serde(rename = "sub")]
    Subscript { children: Vec<MessageElement> },

    // 排版元素
    /// 换行
    #[serde(rename = "br")]
    LineBreak,
    /// 段落
    #[serde(rename = "p")]
    Paragraph { children: Vec<MessageElement> },
    /// 链接
    #[serde(rename = "a")]
    Link {
        /// 	链接的 URL
        href: String,
        children: Vec<MessageElement>,
    },
    // #[serde(rename = "hr")]
    // HorizontalRule,

    // 列表元素
    #[serde(rename = "li")]
    ListItem { children: Vec<MessageElement> },
    #[serde(rename = "ul")]
    UnorderedList { children: Vec<MessageElement> },
    #[serde(rename = "ol")]
    OrderedList {
        start: Option<u64>,
        reversed: Option<bool>,
        #[serde(rename = "type")]
        list_type: Option<String>, // e.g., "1", "a", "A", "i", "I"
        children: Vec<MessageElement>,
    },

    // 表格元素
    #[serde(rename = "table")]
    Table { children: Vec<MessageElement> }, // children: thead?, tbody, tfoot?
    #[serde(rename = "thead")]
    TableHead { children: Vec<MessageElement> }, // children: tr
    #[serde(rename = "tbody")]
    TableBody { children: Vec<MessageElement> }, // children: tr
    #[serde(rename = "tfoot")]
    TableFoot { children: Vec<MessageElement> }, // children: tr
    #[serde(rename = "tr")]
    TableRow { children: Vec<MessageElement> }, // children: th | td
    #[serde(rename = "th")]
    TableHeader { children: Vec<MessageElement> },
    #[serde(rename = "td")]
    TableCell { children: Vec<MessageElement> },

    // 交互元素
    #[serde(rename = "button")]
    Button {
        /// 按钮的 ID
        id: Option<String>,
        /// 按钮的样式
        theme: Option<String>, // e.g., "primary", "secondary", "success", "danger", "warning", "info"
        /// 按钮的链接
        href: Option<String>,
        /// 按钮的文本
        text: Option<String>,
        /// 是否禁用按钮
        disabled: Option<bool>,
        children: Vec<MessageElement>,
    },

    // HTML-like 元素
    #[serde(rename = "span")]
    Span {
        style: Option<String>,
        children: Vec<MessageElement>,
    },
    #[serde(rename = "div")]
    Div {
        style: Option<String>,
        children: Vec<MessageElement>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_complex_message() {
        let msg = vec![
            MessageElement::Author {
                id: "user123".to_string(),
                name: Some("Alice".to_string()),
                avatar: Some("http://example.com/alice.png".to_string()),
            },
            MessageElement::Text {
                text: "Hello, ".to_string(),
            },
            MessageElement::Bold {
                children: vec![MessageElement::Text {
                    text: "world".to_string(),
                }],
            },
            MessageElement::Image {
                src: "http://example.com/image.png".to_string(),
                title: None,
                width: Some(100),
                height: Some(100),
                cache: None,
                timeout: None,
            },
        ];

        let serialized = serde_json::to_string_pretty(&msg).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_serialize_quote_message() {
        let quote_content = MessageElement::Quote {
            id: "prev_msg_id_123".to_string(),
            forward: None,
            children: vec![
                MessageElement::Author {
                    id: "user456".to_string(),
                    name: Some("Bob".to_string()),
                    avatar: None,
                },
                MessageElement::Text {
                    text: "This was the original message.".to_string(),
                },
            ],
        };

        let main_message = vec![
            MessageElement::Text {
                text: "Replying to Bob: ".to_string(),
            },
            quote_content,
            MessageElement::Text {
                text: "What do you think?".to_string(),
            },
        ];

        let serialized = serde_json::to_string_pretty(&main_message).unwrap();
        println!("{}", serialized);
    }
}

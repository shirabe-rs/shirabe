use crate::session::Session;
use std::collections::HashSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContextFilter {
    pub user_ids: Option<HashSet<String>>,
    pub guild_ids: Option<HashSet<String>>,
    pub platforms: Option<HashSet<String>>,
    pub is_private: Option<bool>, // true: 仅私聊, false: 仅群聊, None: 两者皆可
}

impl ContextFilter {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn user(mut self, user_id: &str) -> Self {
        self.user_ids
            .get_or_insert_with(HashSet::new)
            .insert(user_id.to_string());
        self
    }
    pub fn guild(mut self, guild_id: &str) -> Self {
        self.guild_ids
            .get_or_insert_with(HashSet::new)
            .insert(guild_id.to_string());
        self
    }
    pub fn platform(mut self, platform: &str) -> Self {
        self.platforms
            .get_or_insert_with(HashSet::new)
            .insert(platform.to_string());
        self
    }
    pub fn private(mut self) -> Self {
        self.is_private = Some(true);
        self
    }
    pub fn group(mut self) -> Self {
        self.is_private = Some(false);
        self
    }

    // 检查过滤器是否匹配给定的 Session
    pub fn matches_session(&self, session: &Session) -> bool {
        if let Some(users) = &self.user_ids {
            if !users.contains(&session.user_id) {
                return false;
            }
        }
        if let Some(guilds) = &self.guild_ids {
            let gid = &session.guild_id;
            if !guilds.contains(gid) {
                return false; // 群聊ID不匹配
            }
        }
        if let Some(platforms) = &self.platforms {
            if !platforms.contains(&session.platform) {
                return false;
            }
        }
        if let Some(is_private_filter) = self.is_private {
            if is_private_filter != session.is_direct {
                return false;
            }
        }
        true
    }
    // 检查过滤器是否匹配没有 Session 上下文的通用事件
    pub fn matches_generic(&self) -> bool {
        // 如果过滤器指定了用户/群组/私聊等会话相关的属性，
        // 那么对于没有会话上下文的通用事件，它通常不应该匹配。
        // 这里的逻辑可以根据具体需求调整。
        // 例如，只基于平台的过滤器可能对通用事件仍然有效。
        self.user_ids.is_none() &&
        self.guild_ids.is_none() &&
        self.platforms.is_none() && // 平台相关的过滤器可能对通用事件依然有意义
        self.is_private.is_none()
    }
}

#[cfg(test)]
mod tests {
    use shirabe_core::adapter::Adapter;
    use shirabe_core::bot::Bot;
    use shirabe_core::context::{Context, listener::ListenerAction, state::EventSystemSharedState};
    use shirabe_core::error::FrameworkResult;
    use shirabe_core::session::{Session, SessionEvent};
    use shirabe_core::types::{
        Channel, ChannelType, Guild, GuildMember, GuildRole, Login, LoginStatus, Message, User,
    };

    use async_trait::async_trait;
    use std::any::Any;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex, RwLock};
    use uuid::Uuid;

    // --- Mock Adapter 实例 ---
    #[derive(Debug)]
    struct MockAdapter {
        name: String,
        self_id: String,
    }

    #[async_trait]
    impl Adapter for MockAdapter {
        fn get_name(&self) -> String {
            self.name.clone()
        }

        async fn connect(&self, _bot: Arc<Bot>) {}
        async fn disconnect(&self, _bot: Arc<Bot>) {}

        async fn create_reaction(
            &self,
            _message_id: &str,
            _channel_id: &str,
            _emoji: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn delete_reaction(
            &self,
            _message_id: &str,
            _channel_id: &str,
            _emoji: &str,
            _user_id: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn clear_reaction(
            &self,
            _message_id: &str,
            _channel_id: &str,
            _emoji: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_reaction_list(
            &self,
            _message_id: &str,
            _channel_id: &str,
            _emoji: &str,
            _next: Option<&str>,
        ) -> FrameworkResult<Vec<User>> {
            Ok(vec![])
        }
        async fn get_channel(&self, channel_id: &str) -> FrameworkResult<Channel> {
            Ok(Channel {
                id: channel_id.to_string(),
                ty: ChannelType::Text,
                name: "mock_channel_name".to_string(),
                parent_id: None,
            })
        }
        async fn get_channel_list(
            &self,
            _guild_id: &str,
            _next: Option<&str>,
        ) -> FrameworkResult<Vec<Channel>> {
            Ok(vec![])
        }
        async fn create_channel(&self, _guild_id: &str, data: Channel) -> FrameworkResult<Channel> {
            Ok(data)
        }
        async fn update_channel(&self, _channel_id: &str, _data: Channel) -> FrameworkResult<()> {
            Ok(())
        }
        async fn delete_channel(&self, _channel_id: &str) -> FrameworkResult<()> {
            Ok(())
        }
        async fn create_direct_channel(&self, user_id: &str) -> FrameworkResult<Channel> {
            Ok(Channel {
                id: format!("dm_{}", user_id),
                ty: ChannelType::Direct,
                name: "mock_direct_channel_name".to_string(),
                parent_id: None,
            })
        }
        async fn set_guild_member_role(
            &self,
            _guild_id: &str,
            _user_id: &str,
            _role_id: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn unset_guild_member_role(
            &self,
            _guild_id: &str,
            _user_id: &str,
            _role_id: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_guild_member_role_list(
            &self,
            _guild_id: &str,
            _next: Option<&str>,
        ) -> FrameworkResult<Vec<GuildRole>> {
            Ok(vec![])
        }
        async fn create_guild_role(
            &self,
            _guild_id: &str,
            role_name: &str,
        ) -> FrameworkResult<GuildRole> {
            Ok(GuildRole {
                id: "mock_role_id".to_string(),
                name: Some(role_name.to_string()),
            })
        }
        async fn update_guild_role(
            &self,
            _guild_id: &str,
            _role_id: &str,
            _role: GuildRole,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn delete_guild_role(&self, _guild_id: &str, _role_id: &str) -> FrameworkResult<()> {
            Ok(())
        }
        async fn send_message(
            &self,
            _channel_id: &str,
            _content: &str,
        ) -> FrameworkResult<Vec<String>> {
            Ok(vec!["mock_sent_message_id".to_string()])
        }
        async fn send_private_message(
            &self,
            _user_id: &str,
            _content: &str,
            _guild_id: &str,
        ) -> FrameworkResult<Vec<String>> {
            Ok(vec![])
        }
        async fn get_message(
            &self,
            _channel_id: &str,
            message_id: &str,
        ) -> FrameworkResult<Message> {
            Ok(Message {
                id: message_id.to_string(),
                content: "mock content".to_string(),
                channel: None,
                guild: None,
                member: None,
                quote: None,
                user: None,
                created_at_ms: Some(0),
                updated_at_ms: Some(0),
            })
        }
        async fn delete_message(
            &self,
            _channel_id: &str,
            _message_id: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn update_message(
            &self,
            _channel_id: &str,
            _message_id: &str,
            _content: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_message_list(
            &self,
            _channel_id: &str,
            _next: Option<&str>,
            _directory: Option<&str>,
        ) -> FrameworkResult<Vec<Message>> {
            Ok(vec![])
        }
        async fn get_user(&self, user_id: &str) -> FrameworkResult<User> {
            Ok(User {
                id: user_id.to_string(),
                ..Default::default()
            })
        }
        async fn get_friends(&self, _next: Option<&str>) -> FrameworkResult<Vec<User>> {
            Ok(vec![])
        }
        async fn handle_friend_request(
            &self,
            _message_id: &str,
            _accept: bool,
            _comment: Option<&str>,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_guild(&self, guild_id: &str) -> FrameworkResult<Guild> {
            Ok(Guild {
                id: guild_id.to_string(),
                name: "mock_guild_name".to_string(),
                avatar: None,
            })
        }
        async fn get_guilds(&self, _next: Option<&str>) -> FrameworkResult<Vec<Guild>> {
            Ok(vec![])
        }
        async fn handle_guild_invite(
            &self,
            _message_id: &str,
            _accept: bool,
            _comment: Option<&str>,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_guild_member(
            &self,
            _guild_id: &str,
            user_id: &str,
        ) -> FrameworkResult<GuildMember> {
            Ok(GuildMember {
                user: Some(User {
                    id: user_id.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        }
        async fn get_guild_members(
            &self,
            _guild_id: &str,
            _next: Option<&str>,
        ) -> FrameworkResult<Vec<GuildMember>> {
            Ok(vec![])
        }
        async fn kick_guild_member(
            &self,
            _guild_id: &str,
            _user_id: &str,
            _permanent: Option<bool>,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn mute_guild_member(
            &self,
            _guild_id: &str,
            _user_id: &str,
            _duration: Option<u64>,
            _reason: &str,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn handle_guild_request(
            &self,
            _message_id: &str,
            _accept: bool,
            _comment: Option<&str>,
        ) -> FrameworkResult<()> {
            Ok(())
        }
        async fn get_login(&self) -> FrameworkResult<Login> {
            Ok(Login {
                sn: 0,
                platform: Some(self.name.clone()),
                user: Some(User {
                    id: self.self_id.clone(),
                    name: Some("MockBotUser".to_string()),
                    is_bot: Some(true),
                    ..Default::default()
                }),
                status: LoginStatus::Online,
                adapter: self.name.clone(),
                features: vec![],
            })
        }
    }

    fn create_shared_state() -> Arc<RwLock<EventSystemSharedState>> {
        Arc::new(RwLock::new(EventSystemSharedState::default()))
    }

    fn create_minimal_session_event(
        user_id: &str,
        guild_id: Option<&str>,
        channel_id: &str,
        platform: &str,
        channel_type: ChannelType,
        bot_self_id: &str,
    ) -> SessionEvent {
        SessionEvent {
            id: Uuid::new_v4().to_u128_le() as i64, // 使用uuid_v4作为唯一的event id
            ty: "message.created".to_string(),
            platform: platform.to_string(),
            self_id: bot_self_id.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            channel: Channel {
                id: channel_id.to_string(),
                ty: channel_type,
                name: "test_channel".to_string(),
                parent_id: None,
            },
            guild: Guild {
                id: guild_id.unwrap_or("").to_string(),
                name: "test_guild".to_string(),
                avatar: None,
            },
            login: Login {
                sn: 0,
                platform: Some(platform.to_string()),
                user: Some(User {
                    id: bot_self_id.to_string(),
                    name: Some("TestBotLoginUser".to_string()),
                    is_bot: Some(true),
                    ..Default::default()
                }),
                status: LoginStatus::Online,
                adapter: platform.to_string(),
                features: vec![],
            },
            member: GuildMember {
                user: Some(User {
                    id: user_id.to_string(),
                    name: Some("test_user_member".to_string()),
                    ..Default::default()
                }),
                nick: Some("TestUserNick".to_string()),
                avatar: None,
                joined_at_ms: Some(chrono::Utc::now().timestamp_millis()),
            },
            message: Message {
                id: Uuid::new_v4().to_string(),
                content: "test message".to_string(),
                channel: Some(Channel {
                    id: channel_id.to_string(),
                    ty: channel_type,
                    name: "test_channel".to_string(),
                    parent_id: None,
                }),
                guild: guild_id.map(|gid| Guild {
                    id: gid.to_string(),
                    name: "test_guild".to_string(),
                    avatar: None,
                }),
                member: Some(GuildMember {
                    user: Some(User {
                        id: user_id.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                quote: None,
                user: Some(User {
                    id: user_id.to_string(),
                    name: Some("test_user_message_sender".to_string()),
                    ..Default::default()
                }),
                created_at_ms: Some(chrono::Utc::now().timestamp_millis()),
                updated_at_ms: Some(chrono::Utc::now().timestamp_millis()),
            },
            operator: User {
                id: user_id.to_string(),
                name: Some("test_operator".to_string()),
                ..Default::default()
            },
            role: Default::default(),
            user: User {
                id: user_id.to_string(),
                name: Some("test_user_event_actor".to_string()),
                avatar: None,
                is_bot: Some(false),
                nick: None,
            },
        }
    }

    fn create_mock_session(
        app_ctx: Arc<Context>, // App的根上下文
        event: SessionEvent,
    ) -> Session {
        let bot_platform_ctx = app_ctx.platform(&event.platform);

        let mock_adapter = Arc::new(MockAdapter {
            name: event.platform.clone(),
            self_id: event.self_id.clone(),
        });

        let mut bot_instance =
            Bot::new(Arc::new(bot_platform_ctx), mock_adapter as Arc<dyn Adapter>);
        bot_instance.self_id = event.self_id.clone();
        bot_instance.platform = event.platform.clone();
        bot_instance.user = User {
            id: event.self_id.clone(),
            name: Some("MockBotUserSession".to_string()),
            is_bot: Some(true),
            ..Default::default()
        };
        bot_instance.state = LoginStatus::Online;

        Session::new(Arc::new(bot_instance), event)
    }

    #[test]
    fn test_new_root_context() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let bots_guard = ctx.bots.lock().unwrap();
        assert!(bots_guard.is_empty());
        assert!(ctx.current_filter.user_ids.is_none());
        assert!(ctx.current_filter.guild_ids.is_none());
        assert!(ctx.current_filter.platforms.is_none());
        assert!(ctx.current_filter.is_private.is_none());
    }

    #[test]
    fn test_context_derivation() {
        let shared_state = create_shared_state();
        let root_ctx = Context::new_root(Arc::clone(&shared_state));

        let user_ctx = root_ctx.user("user123");
        assert_eq!(
            user_ctx.current_filter.user_ids,
            Some(HashSet::from(["user123".to_string()]))
        );

        let guild_ctx = root_ctx.guild("guild456");
        assert_eq!(
            guild_ctx.current_filter.guild_ids,
            Some(HashSet::from(["guild456".to_string()]))
        );

        let platform_ctx = root_ctx.platform("discord");
        assert_eq!(
            platform_ctx.current_filter.platforms,
            Some(HashSet::from(["discord".to_string()]))
        );

        let private_ctx = root_ctx.private();
        assert_eq!(private_ctx.current_filter.is_private, Some(true));

        let group_ctx = root_ctx.group();
        assert_eq!(group_ctx.current_filter.is_private, Some(false));

        let complex_ctx = root_ctx.user("user1").platform("slack").guild("guild1");
        assert_eq!(
            complex_ctx.current_filter.user_ids,
            Some(HashSet::from(["user1".to_string()]))
        );
        assert_eq!(
            complex_ctx.current_filter.platforms,
            Some(HashSet::from(["slack".to_string()]))
        );
        assert_eq!(
            complex_ctx.current_filter.guild_ids,
            Some(HashSet::from(["guild1".to_string()]))
        );
    }

    #[test]
    fn test_on_listener() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let call_count = Arc::new(Mutex::new(0));

        let cc_clone = Arc::clone(&call_count);
        let _handle = ctx.on("test_event", move |_session, _args| {
            let mut count = cc_clone.lock().unwrap();
            *count += 1;
        });

        ctx.emit("test_event", None, &[]);
        ctx.emit("test_event", None, &[]);

        assert_eq!(*call_count.lock().unwrap(), 2);

        let state = shared_state.read().unwrap();
        assert!(state.listeners_by_event.contains_key("test_event"));
        assert_eq!(state.listeners_by_event.get("test_event").unwrap().len(), 1);
    }

    #[test]
    fn test_once_listener() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let call_count = Arc::new(Mutex::new(0));

        let cc_clone = Arc::clone(&call_count);
        let _handle = ctx.once("test_event_once", move |_session, _args| {
            let mut count = cc_clone.lock().unwrap();
            *count += 1;
        });

        ctx.emit("test_event_once", None, &[]);
        ctx.emit("test_event_once", None, &[]);

        assert_eq!(*call_count.lock().unwrap(), 1);

        let state = shared_state.read().unwrap();
        assert!(
            !state.listeners_by_event.contains_key("test_event_once")
                || state
                    .listeners_by_event
                    .get("test_event_once")
                    .map_or(true, |v| v.is_empty())
        );
    }

    #[test]
    fn test_bail_listener() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let bail_called = Arc::new(Mutex::new(false));
        let subsequent_called = Arc::new(Mutex::new(false));

        let bc_clone = Arc::clone(&bail_called);
        let _h1 = ctx.bail("test_event_bail", move |_session, _args| {
            let mut called = bc_clone.lock().unwrap();
            *called = true;
            Some(Box::new("bailed_value".to_string()) as Box<dyn Any + Send + Sync>)
        });

        let sc_clone = Arc::clone(&subsequent_called);
        let _h2 = ctx.on("test_event_bail", move |_session, _args| {
            let mut called = sc_clone.lock().unwrap();
            *called = true;
        });

        let result = ctx.emit("test_event_bail", None, &[]);

        assert_eq!(*bail_called.lock().unwrap(), true);
        assert_eq!(*subsequent_called.lock().unwrap(), false);

        assert!(result.is_some());
        if let Some(value) = result {
            if let Some(s_val) = value.downcast_ref::<String>() {
                assert_eq!(s_val, "bailed_value");
            } else {
                panic!("Bail value was not a string");
            }
        }
    }

    #[test]
    fn test_bail_listener_no_bail() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let bail_called_count = Arc::new(Mutex::new(0));
        let subsequent_called_count = Arc::new(Mutex::new(0));

        let bcc_clone = Arc::clone(&bail_called_count);
        let _h1 = ctx.bail("test_event_nobail", move |_session, _args| {
            let mut called = bcc_clone.lock().unwrap();
            *called += 1;
            None
        });

        let scc_clone = Arc::clone(&subsequent_called_count);
        let _h2 = ctx.on("test_event_nobail", move |_session, _args| {
            let mut called = scc_clone.lock().unwrap();
            *called += 1;
        });

        let result = ctx.emit("test_event_nobail", None, &[]);
        assert_eq!(*bail_called_count.lock().unwrap(), 1);
        assert_eq!(*subsequent_called_count.lock().unwrap(), 1);
        assert!(result.is_none());

        let result2 = ctx.emit("test_event_nobail", None, &[]);
        assert_eq!(*bail_called_count.lock().unwrap(), 2);
        assert_eq!(*subsequent_called_count.lock().unwrap(), 2);
        assert!(result2.is_none());
    }

    #[test]
    fn test_listener_filtering_with_session() {
        let app_shared_state = create_shared_state();
        let app_ctx = Arc::new(Context::new_root(Arc::clone(&app_shared_state)));
        let bot_self_id_for_test = "test_bot_filter_session";

        let user_specific_called = Arc::new(Mutex::new(false));
        let guild_specific_called = Arc::new(Mutex::new(false));
        let platform_specific_called = Arc::new(Mutex::new(false));
        let generic_called = Arc::new(Mutex::new(false));

        let user_ctx = app_ctx.user("user1");
        let usc_clone = Arc::clone(&user_specific_called);
        let _h_user = user_ctx.on("event_filtered", move |_session, _args| {
            *usc_clone.lock().unwrap() = true;
        });

        let guild_ctx = app_ctx.guild("guild1");
        let gsc_clone = Arc::clone(&guild_specific_called);
        let _h_guild = guild_ctx.on("event_filtered", move |_session, _args| {
            *gsc_clone.lock().unwrap() = true;
        });

        let platform_ctx = app_ctx.platform("platform1");
        let psc_clone = Arc::clone(&platform_specific_called);
        let _h_platform = platform_ctx.on("event_filtered", move |_session, _args| {
            *psc_clone.lock().unwrap() = true;
        });

        let gc_clone = Arc::clone(&generic_called);
        let _h_generic = app_ctx.on("event_filtered", move |_session, _args| {
            *gc_clone.lock().unwrap() = true;
        });

        let event1 = create_minimal_session_event(
            "user1",
            Some("guild1"),
            "channel1",
            "platform1",
            ChannelType::Text,
            bot_self_id_for_test,
        );
        let session1 = create_mock_session(Arc::clone(&app_ctx), event1);
        app_ctx.emit("event_filtered", Some(&session1), &[]);

        assert!(
            *user_specific_called.lock().unwrap(),
            "User1 listener should have been called for session1"
        );
        assert!(
            *guild_specific_called.lock().unwrap(),
            "Guild1 listener should have been called for session1"
        );
        assert!(
            *platform_specific_called.lock().unwrap(),
            "Platform1 listener should have been called for session1"
        );
        assert!(
            *generic_called.lock().unwrap(),
            "Generic listener should have been called for session1"
        );

        *user_specific_called.lock().unwrap() = false;
        *guild_specific_called.lock().unwrap() = false;
        *platform_specific_called.lock().unwrap() = false;
        *generic_called.lock().unwrap() = false;

        let event2 = create_minimal_session_event(
            "user2",
            Some("guild2"),
            "channel2",
            "platform1",
            ChannelType::Text,
            bot_self_id_for_test,
        );
        let session2 = create_mock_session(Arc::clone(&app_ctx), event2);
        app_ctx.emit("event_filtered", Some(&session2), &[]);

        assert!(
            !*user_specific_called.lock().unwrap(),
            "User1 listener should NOT be called for session2"
        );
        assert!(
            !*guild_specific_called.lock().unwrap(),
            "Guild1 listener should NOT be called for session2"
        );
        assert!(
            *platform_specific_called.lock().unwrap(),
            "Platform1 listener should be called for session2"
        );
        assert!(
            *generic_called.lock().unwrap(),
            "Generic listener should be called for session2"
        );

        *platform_specific_called.lock().unwrap() = false;
        *generic_called.lock().unwrap() = false;

        let private_ctx = app_ctx.user("user1").platform("platform1").private();
        let private_called = Arc::new(Mutex::new(false));
        let pc_clone = Arc::clone(&private_called);
        let _h_private = private_ctx.on("event_filtered", move |_, _| {
            *pc_clone.lock().unwrap() = true;
        });

        let event3 = create_minimal_session_event(
            "user1",
            None,
            "dm_channel",
            "platform1",
            ChannelType::Direct,
            bot_self_id_for_test,
        );
        let session3 = create_mock_session(Arc::clone(&app_ctx), event3);
        app_ctx.emit("event_filtered", Some(&session3), &[]);

        assert!(
            *private_called.lock().unwrap(),
            "Private listener for user1/platform1 should be called for session3"
        );
        assert!(
            *platform_specific_called.lock().unwrap(),
            "Platform1 listener should be called for session3"
        );
        assert!(
            *generic_called.lock().unwrap(),
            "Generic listener should be called for session3"
        );

        *user_specific_called.lock().unwrap() = false;
        *guild_specific_called.lock().unwrap() = false;
        *platform_specific_called.lock().unwrap() = false;
        *generic_called.lock().unwrap() = false;
        *private_called.lock().unwrap() = false;

        let generic_called_no_session = Arc::new(Mutex::new(false));
        let gcn_clone = Arc::clone(&generic_called_no_session);
        let _h_generic_no_session = app_ctx.on("event_no_session", move |_session, _args| {
            *gcn_clone.lock().unwrap() = true;
        });

        let user_specific_no_session_called = Arc::new(Mutex::new(false));
        let usnsc_clone = Arc::clone(&user_specific_no_session_called);
        let user_ctx_for_no_session_test = app_ctx.user("user_for_no_session_test");
        let _h_user_no_session =
            user_ctx_for_no_session_test.on("event_no_session", move |_, _| {
                *usnsc_clone.lock().unwrap() = true;
            });

        app_ctx.emit("event_no_session", None, &[]);
        assert!(
            *generic_called_no_session.lock().unwrap(),
            "Generic listener (no session filter) should be called for emit(None, ...)"
        );
        assert!(
            !*user_specific_no_session_called.lock().unwrap(),
            "User-specific listener should NOT be called for emit(None, ...) because its filter requires session data"
        );
    }

    #[test]
    fn test_listener_handle_dispose_removes_listener() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));
        let call_count = Arc::new(Mutex::new(0));

        let cc_clone = Arc::clone(&call_count);
        let handle = ctx.on("event_handle_dispose", move |_session, _args| {
            let mut count = cc_clone.lock().unwrap();
            *count += 1;
        });

        ctx.emit("event_handle_dispose", None, &[]);
        assert_eq!(*call_count.lock().unwrap(), 1);

        {
            let state = shared_state.read().unwrap();
            assert_eq!(
                state
                    .listeners_by_event
                    .get("event_handle_dispose")
                    .unwrap()
                    .len(),
                1
            );
        }

        handle.dispose();

        ctx.emit("event_handle_dispose", None, &[]);
        assert_eq!(*call_count.lock().unwrap(), 1);

        {
            let state = shared_state.read().unwrap();
            assert!(
                !state
                    .listeners_by_event
                    .contains_key("event_handle_dispose")
                    || state
                        .listeners_by_event
                        .get("event_handle_dispose")
                        .map_or(true, |v| v.is_empty())
            );
        }
    }

    #[test]
    fn test_bail_and_once_interaction() {
        let shared_state = create_shared_state();
        let ctx = Context::new_root(Arc::clone(&shared_state));

        let once_fired_before_bail = Arc::new(Mutex::new(false));
        let bail_fired = Arc::new(Mutex::new(false));
        let once_fired_after_bail = Arc::new(Mutex::new(false));

        let ofbb_clone = Arc::clone(&once_fired_before_bail);
        let _h_once_before = ctx.once("bail_interaction_event", move |_, _| {
            *ofbb_clone.lock().unwrap() = true;
        });

        let bf_clone = Arc::clone(&bail_fired);
        let h_bail = ctx.bail("bail_interaction_event", move |_, _| {
            *bf_clone.lock().unwrap() = true;
            Some(Box::new("bailed".to_string()) as Box<dyn Any + Send + Sync>)
        });

        let ofab_clone = Arc::clone(&once_fired_after_bail);
        let _h_once_after = ctx.once("bail_interaction_event", move |_, _| {
            *ofab_clone.lock().unwrap() = true;
        });

        let result = ctx.emit("bail_interaction_event", None, &[]);

        assert!(result.is_some(), "Emit should return the bailed value");
        assert!(
            *once_fired_before_bail.lock().unwrap(),
            "Once listener before bail should have fired"
        );
        assert!(
            *bail_fired.lock().unwrap(),
            "Bail listener should have fired"
        );
        assert!(
            !*once_fired_after_bail.lock().unwrap(),
            "Once listener after bail should NOT have fired"
        );

        {
            let state = shared_state.read().unwrap();
            let listeners = state
                .listeners_by_event
                .get("bail_interaction_event")
                .unwrap();
            let once_listener_count = listeners
                .iter()
                .filter(|l| matches!(l.action, ListenerAction::Once(_)))
                .count();
            assert_eq!(
                once_listener_count, 1,
                "Only the once listener that did not fire should remain"
            );
            let bail_listener_count = listeners
                .iter()
                .filter(|l| matches!(l.action, ListenerAction::Bail(_)))
                .count();
            assert_eq!(bail_listener_count, 1, "The bail listener should remain");
        }

        h_bail.dispose();

        *once_fired_before_bail.lock().unwrap() = false;
        *bail_fired.lock().unwrap() = false;

        let result2 = ctx.emit("bail_interaction_event", None, &[]);
        assert!(result2.is_none(), "Emit should not bail this time");
        assert!(
            *once_fired_after_bail.lock().unwrap(),
            "The second once listener should have fired now"
        );

        {
            let state_after_second_emit = shared_state.read().unwrap();
            if let Some(listeners_after_second_emit) = state_after_second_emit
                .listeners_by_event
                .get("bail_interaction_event")
            {
                let once_listener_count_after_second_emit = listeners_after_second_emit
                    .iter()
                    .filter(|l| matches!(l.action, ListenerAction::Once(_)))
                    .count();
                assert_eq!(
                    once_listener_count_after_second_emit, 0,
                    "The second once listener should have fired and been removed"
                );
                let bail_listener_count = listeners_after_second_emit
                    .iter()
                    .filter(|l| matches!(l.action, ListenerAction::Bail(_)))
                    .count();
                assert_eq!(
                    bail_listener_count, 0,
                    "The bail listener should have been removed"
                );
            } else {
            }
        }
    }
}

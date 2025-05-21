#[cfg(test)]
mod tests {
    use shirabe_core::bot::Bot;
    use shirabe_core::command::{
        Command, CommandAction, CommandBuilder, CommandRegistry, ParsedArgs,
    };
    use shirabe_core::context::Context;
    use shirabe_core::context::filter::ContextFilter;
    use shirabe_core::context::state::EventSystemSharedState;
    use shirabe_core::error::{FrameworkError, FrameworkResult};
    use shirabe_core::message::MessageElement;
    use shirabe_core::session::{Session, SessionEvent};
    use shirabe_core::types::{
        Channel, ChannelType, Guild, GuildMember, GuildRole, Login, LoginStatus,
        Message as FrameworkMessage, User,
    };
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, RwLock};

    fn create_test_session(
        platform: &str,
        user_id: &str,
        guild_id: &str,
        channel_id: &str,
        is_direct: bool,
        message_text_content: &str,
        message_elements: Vec<MessageElement>,
    ) -> Arc<Session> {
        let shared_state = Arc::new(RwLock::new(EventSystemSharedState::default()));
        let app_context = Arc::new(Context::new_root(shared_state));

        #[derive(Debug)]
        struct DummyAdapter;
        #[async_trait::async_trait]
        impl shirabe_core::adapter::Adapter for DummyAdapter {
            fn get_name(&self) -> String {
                "dummy".to_string()
            }
            async fn connect(&self, _bot: Arc<Bot>) {}
            async fn disconnect(&self, _bot: Arc<Bot>) {}
            async fn create_reaction(&self, _: &str, _: &str, _: &str) -> FrameworkResult<()> {
                Ok(())
            }
            async fn delete_reaction(
                &self,
                _: &str,
                _: &str,
                _: &str,
                _: &str,
            ) -> FrameworkResult<()> {
                Ok(())
            }
            async fn clear_reaction(&self, _: &str, _: &str, _: &str) -> FrameworkResult<()> {
                Ok(())
            }
            async fn get_reaction_list(
                &self,
                _: &str,
                _: &str,
                _: &str,
                _: Option<&str>,
            ) -> FrameworkResult<Vec<User>> {
                Ok(vec![])
            }
            async fn get_channel(&self, _: &str) -> FrameworkResult<Channel> {
                unimplemented!()
            }
            async fn get_channel_list(
                &self,
                _: &str,
                _: Option<&str>,
            ) -> FrameworkResult<Vec<Channel>> {
                unimplemented!()
            }
            async fn create_channel(&self, _: &str, _: Channel) -> FrameworkResult<Channel> {
                unimplemented!()
            }
            async fn update_channel(&self, _: &str, _: Channel) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn delete_channel(&self, _: &str) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn create_direct_channel(&self, _: &str) -> FrameworkResult<Channel> {
                unimplemented!()
            }
            async fn set_guild_member_role(
                &self,
                _: &str,
                _: &str,
                _: &str,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn unset_guild_member_role(
                &self,
                _: &str,
                _: &str,
                _: &str,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn get_guild_member_role_list(
                &self,
                _: &str,
                _: Option<&str>,
            ) -> FrameworkResult<Vec<GuildRole>> {
                unimplemented!()
            }
            async fn create_guild_role(&self, _: &str, _: &str) -> FrameworkResult<GuildRole> {
                unimplemented!()
            }
            async fn update_guild_role(
                &self,
                _: &str,
                _: &str,
                _: GuildRole,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn delete_guild_role(&self, _: &str, _: &str) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn send_message(
                &self,
                _channel_id: &str,
                _elements: &[MessageElement],
            ) -> FrameworkResult<Vec<String>> {
                Ok(vec![])
            }
            async fn send_private_message(
                &self,
                _: &str,
                _: &str,
                _: &[MessageElement],
            ) -> FrameworkResult<Vec<String>> {
                unimplemented!()
            }
            async fn get_message(&self, _: &str, _: &str) -> FrameworkResult<FrameworkMessage> {
                unimplemented!()
            }
            async fn delete_message(&self, _: &str, _: &str) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn update_message(
                &self,
                _: &str,
                _: &str,
                _: &[MessageElement],
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn get_message_list(
                &self,
                _: &str,
                _: Option<&str>,
                _: Option<&str>,
            ) -> FrameworkResult<Vec<FrameworkMessage>> {
                unimplemented!()
            }
            async fn get_user(&self, _: &str) -> FrameworkResult<User> {
                unimplemented!()
            }
            async fn get_friends(&self, _: Option<&str>) -> FrameworkResult<Vec<User>> {
                unimplemented!()
            }
            async fn handle_friend_request(
                &self,
                _: &str,
                _: bool,
                _: Option<&str>,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn get_guild(&self, _: &str) -> FrameworkResult<Guild> {
                unimplemented!()
            }
            async fn get_guilds(&self, _: Option<&str>) -> FrameworkResult<Vec<Guild>> {
                unimplemented!()
            }
            async fn handle_guild_invite(
                &self,
                _: &str,
                _: bool,
                _: Option<&str>,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn get_guild_member(&self, _: &str, _: &str) -> FrameworkResult<GuildMember> {
                unimplemented!()
            }
            async fn get_guild_members(
                &self,
                _: &str,
                _: Option<&str>,
            ) -> FrameworkResult<Vec<GuildMember>> {
                unimplemented!()
            }
            async fn kick_guild_member(
                &self,
                _: &str,
                _: &str,
                _: Option<bool>,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn mute_guild_member(
                &self,
                _: &str,
                _: &str,
                _: Option<u64>,
                _: &str,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn handle_guild_request(
                &self,
                _: &str,
                _: bool,
                _: Option<&str>,
            ) -> FrameworkResult<()> {
                unimplemented!()
            }
            async fn get_login(&self) -> FrameworkResult<Login> {
                unimplemented!()
            }
        }
        let adapter = Arc::new(DummyAdapter);

        let bot = Arc::new(Bot::new(app_context, adapter));
        let event = SessionEvent {
            id: 1,
            ty: "message.created".to_string(),
            platform: platform.to_string(),
            self_id: "test_bot_id".to_string(),
            timestamp: 0,
            channel: Channel {
                id: channel_id.to_string(),
                ty: if is_direct {
                    ChannelType::Direct
                } else {
                    ChannelType::Text
                },
                name: "test_channel".to_string(),
                parent_id: None,
            },
            guild: Guild {
                id: guild_id.to_string(),
                name: "test_guild".to_string(),
                avatar: None,
            },
            login: Login {
                // Add dummy Login
                sn: 0,
                platform: Some(platform.to_string()),
                user: Some(User {
                    id: "test_bot_id".to_string(),
                    ..Default::default()
                }),
                status: LoginStatus::Online,
                adapter: "dummy".to_string(),
                features: vec![],
            },
            member: Default::default(), // GuildMember
            message: FrameworkMessage {
                id: "test_msg_id".to_string(),
                content: message_text_content.to_string(),
                elements: message_elements,
                channel: Some(Channel {
                    id: channel_id.to_string(),
                    ty: if is_direct {
                        ChannelType::Direct
                    } else {
                        ChannelType::Text
                    },
                    name: "test_channel".to_string(),
                    parent_id: None,
                }),
                guild: Some(Guild {
                    id: guild_id.to_string(),
                    name: "test_guild".to_string(),
                    avatar: None,
                }),
                member: None,
                quote: None,
                user: Some(User {
                    id: user_id.to_string(),
                    name: Some("test_user".to_string()),
                    ..Default::default()
                }),
                created_at_ms: Some(0),
                updated_at_ms: Some(0),
            },
            operator: Default::default(),
            role: Default::default(),
            user: User {
                id: user_id.to_string(),
                name: Some("test_user".to_string()),
                ..Default::default()
            },
        };
        Arc::new(Session::new(bot, event))
    }

    fn create_empty_action() -> CommandAction {
        Box::new(|_session, _args| Box::pin(async { Ok(()) }))
    }

    #[test]
    fn test_register_command() {
        let mut registry = CommandRegistry::new();
        let cmd = Command {
            name: "testcmd".to_string(),
            aliases: vec!["tc".to_string()],
            description: None,
            filter: ContextFilter::new(),
            action: create_empty_action(),
        };
        registry.register(cmd).unwrap();

        assert!(registry.commands.contains_key("testcmd"));
        assert!(registry.commands.contains_key("tc"));
        assert_eq!(registry.commands.get("testcmd").unwrap().name, "testcmd");
        assert_eq!(registry.commands.get("tc").unwrap().name, "testcmd");
    }

    #[test]
    fn test_register_overwrite_command() {
        let mut registry = CommandRegistry::new();
        let cmd1 = Command {
            name: "testcmd".to_string(),
            aliases: vec![],
            description: Some("First version".to_string()),
            filter: ContextFilter::new(),
            action: create_empty_action(),
        };
        registry.register(cmd1).unwrap();
        assert_eq!(
            registry.commands.get("testcmd").unwrap().description,
            Some("First version".to_string())
        );

        let cmd2 = Command {
            name: "testcmd".to_string(),
            aliases: vec![],
            description: Some("Second version".to_string()),
            filter: ContextFilter::new(),
            action: create_empty_action(),
        };
        registry.register(cmd2).unwrap();
        assert_eq!(
            registry.commands.get("testcmd").unwrap().description,
            Some("Second version".to_string())
        );
    }

    #[tokio::test]
    async fn test_parse_and_execute_no_prefix() {
        let registry = CommandRegistry::new();
        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "testcmd arg1",
            vec![],
        );
        let prefixes = ["/", "!"];
        let executed = registry
            .parse_and_execute(session, "testcmd arg1", &prefixes)
            .await
            .unwrap();
        assert!(!executed);
    }

    #[tokio::test]
    async fn test_parse_and_execute_only_prefix() {
        let registry = CommandRegistry::new();
        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/",
            vec![],
        );
        let prefixes = ["/", "!"];
        let executed = registry
            .parse_and_execute(session, "/", &prefixes)
            .await
            .unwrap();
        assert!(!executed);
    }

    #[tokio::test]
    async fn test_parse_and_execute_unknown_command() {
        let registry = CommandRegistry::new();
        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/unknown",
            vec![],
        );
        let prefixes = ["/", "!"];
        let executed = registry
            .parse_and_execute(session, "/unknown", &prefixes)
            .await
            .unwrap();
        assert!(!executed);
    }

    #[tokio::test]
    async fn test_parse_and_execute_simple_command() {
        let mut registry = CommandRegistry::new();
        let executed_flag = Arc::new(Mutex::new(false));
        let flag_clone = Arc::clone(&executed_flag);

        let cmd = Command {
            name: "ping".to_string(),
            aliases: vec![],
            description: None,
            filter: ContextFilter::new(),
            action: Box::new(move |_session, _args| {
                let mut flag = flag_clone.lock().unwrap();
                *flag = true;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/ping",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry
            .parse_and_execute(session, "/ping", &prefixes)
            .await
            .unwrap();

        assert!(executed);
        assert!(*executed_flag.lock().unwrap());
    }

    #[tokio::test]
    async fn test_parse_and_execute_command_with_args() {
        let mut registry = CommandRegistry::new();
        let received_args = Arc::new(Mutex::new(Vec::new()));
        let args_clone = Arc::clone(&received_args);

        let cmd = Command {
            name: "echo".to_string(),
            aliases: vec![],
            description: None,
            filter: ContextFilter::new(),
            action: Box::new(move |_session, args| {
                let mut received = args_clone.lock().unwrap();
                *received = args.arguments;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "!echo hello world",
            vec![],
        );
        let prefixes = ["!"];
        let executed = registry
            .parse_and_execute(session, "!echo hello world", &prefixes)
            .await
            .unwrap();

        assert!(executed);
        assert_eq!(
            *received_args.lock().unwrap(),
            vec!["hello".to_string(), "world".to_string()]
        );
    }

    #[tokio::test]
    async fn test_parse_and_execute_command_with_options() {
        let mut registry = CommandRegistry::new();
        let received_options = Arc::new(Mutex::new(HashMap::new()));
        let options_clone = Arc::clone(&received_options);

        let cmd = Command {
            name: "greet".to_string(),
            aliases: vec![],
            description: None,
            filter: ContextFilter::new(),
            action: Box::new(move |_session, args| {
                let mut received = options_clone.lock().unwrap();
                *received = args.options;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/greet --name Alice --loud",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry
            .parse_and_execute(session, "/greet --name Alice --loud", &prefixes)
            .await
            .unwrap();

        assert!(executed);
        let options = received_options.lock().unwrap();
        assert_eq!(options.get("name"), Some(&"Alice".to_string()));
        assert_eq!(options.get("loud"), Some(&"".to_string()));
    }

    #[tokio::test]
    async fn test_parse_and_execute_command_with_short_options() {
        let mut registry = CommandRegistry::new();
        let received_options = Arc::new(Mutex::new(HashMap::new()));
        let options_clone = Arc::clone(&received_options);

        let cmd = Command {
            name: "shortgreet".to_string(),
            aliases: vec![],
            description: None,
            filter: ContextFilter::new(),
            action: Box::new(move |_session, args| {
                let mut received = options_clone.lock().unwrap();
                *received = args.options;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/shortgreet -v -x arg1 --mode test -abc",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry
            .parse_and_execute(
                session,
                "/shortgreet -v -x arg1 --mode test -abc",
                &prefixes,
            )
            .await
            .unwrap();

        assert!(executed);
        let options = received_options.lock().unwrap();
        assert_eq!(options.get("v"), Some(&"".to_string()));
        assert_eq!(options.get("x"), Some(&"".to_string()));
        assert_eq!(options.get("a"), Some(&"".to_string()));
        assert_eq!(options.get("b"), Some(&"".to_string()));
        assert_eq!(options.get("c"), Some(&"".to_string()));
        assert_eq!(options.get("mode"), Some(&"test".to_string()));
    }

    #[tokio::test]
    async fn test_parse_and_execute_command_with_mixed_args_and_options() {
        let mut registry = CommandRegistry::new();
        let received_data = Arc::new(Mutex::new(ParsedArgs::default()));
        let data_clone = Arc::clone(&received_data);

        let cmd = Command {
            name: "mix".to_string(),
            aliases: vec![],
            description: None,
            filter: ContextFilter::new(),
            action: Box::new(move |_session, args| {
                let mut received = data_clone.lock().unwrap();
                *received = args;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/mix arg1 --opt1 val1 arg2 --flag -s arg3",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry
            .parse_and_execute(
                session,
                "/mix arg1 --opt1 val1 arg2 --flag -s arg3",
                &prefixes,
            )
            .await
            .unwrap();

        assert!(executed);
        let parsed_args = received_data.lock().unwrap();
        assert_eq!(
            parsed_args.arguments,
            vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()]
        );
        assert_eq!(parsed_args.options.get("opt1"), Some(&"val1".to_string()));
        assert_eq!(parsed_args.options.get("flag"), Some(&"".to_string()));
        assert_eq!(parsed_args.options.get("s"), Some(&"".to_string()));
    }

    #[tokio::test]
    async fn test_command_builder_register() {
        let shared_state = Arc::new(RwLock::new(EventSystemSharedState::default()));
        let registry_arc = Arc::clone(&shared_state.read().unwrap().command_registry);

        let executed_flag = Arc::new(Mutex::new(false));
        let flag_clone = Arc::clone(&executed_flag);

        CommandBuilder::new(
            "builtcmd".to_string(),
            ContextFilter::new(),
            Arc::clone(&registry_arc),
        )
        .alias("bc")
        .description("A built command")
        .action(move |_session, _args| {
            let mut flag = flag_clone.lock().unwrap();
            *flag = true;
            Box::pin(async { Ok(()) })
        })
        .register()
        .unwrap();

        let registry_guard = registry_arc.read().unwrap();
        assert!(registry_guard.commands.contains_key("builtcmd"));
        assert!(registry_guard.commands.contains_key("bc"));
        assert_eq!(
            registry_guard.commands.get("builtcmd").unwrap().description,
            Some("A built command".to_string())
        );

        // Test execution
        let session = create_test_session(
            "test_platform",
            "user1",
            "guild1",
            "channel1",
            false,
            "/builtcmd",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry_guard
            .parse_and_execute(session, "/builtcmd", &prefixes)
            .await
            .unwrap();

        assert!(executed);
        assert!(*executed_flag.lock().unwrap());
    }

    #[test]
    fn test_command_builder_no_action_error() {
        let shared_state = Arc::new(RwLock::new(EventSystemSharedState::default()));
        let registry_arc = Arc::clone(&shared_state.read().unwrap().command_registry);

        let result = CommandBuilder::new(
            "noactioncmd".to_string(),
            ContextFilter::new(),
            registry_arc,
        )
        .description("This command has no action")
        .register();

        assert!(result.is_err());
        if let Err(FrameworkError::Command(msg)) = result {
            assert_eq!(msg, "指令 'noactioncmd' 没有定义 action");
        } else {
            panic!("Expected FrameworkError::Command");
        }
    }

    #[tokio::test]
    async fn test_command_context_filter_match() {
        let mut registry = CommandRegistry::new();
        let executed_flag = Arc::new(Mutex::new(false));
        let flag_clone = Arc::clone(&executed_flag);

        let cmd_filter = ContextFilter::new()
            .user("user123")
            .platform("test_platform");
        let cmd = Command {
            name: "filteredcmd".to_string(),
            aliases: vec![],
            description: None,
            filter: cmd_filter,
            action: Box::new(move |_session, _args| {
                let mut flag = flag_clone.lock().unwrap();
                *flag = true;
                Box::pin(async { Ok(()) })
            }),
        };
        registry.register(cmd).unwrap();

        let session_match = create_test_session(
            "test_platform",
            "user123",
            "guild1",
            "channel1",
            false,
            "/filteredcmd",
            vec![],
        );
        let prefixes = ["/"];
        let executed = registry
            .parse_and_execute(Arc::clone(&session_match), "/filteredcmd", &prefixes)
            .await
            .unwrap();
        assert!(executed);
        assert!(*executed_flag.lock().unwrap());

        *executed_flag.lock().unwrap() = false;

        let session_no_match_user = create_test_session(
            "test_platform",
            "user456",
            "guild1",
            "channel1",
            false,
            "/filteredcmd",
            vec![],
        );
        let executed = registry
            .parse_and_execute(session_no_match_user, "/filteredcmd", &prefixes)
            .await
            .unwrap();
        assert!(!executed);
        assert!(!*executed_flag.lock().unwrap());

        let session_no_match_platform = create_test_session(
            "other_platform",
            "user123",
            "guild1",
            "channel1",
            false,
            "/filteredcmd",
            vec![],
        );
        let executed = registry
            .parse_and_execute(session_no_match_platform, "/filteredcmd", &prefixes)
            .await
            .unwrap();
        assert!(!executed);
        assert!(!*executed_flag.lock().unwrap());
    }
}

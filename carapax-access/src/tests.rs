use crate::*;
use carapax::prelude::*;
use futures::Future;
use serde_json::{from_value, json};

struct MockPolicy {
    result: bool,
}

impl AccessPolicy for MockPolicy {
    fn is_granted(&self, _context: &mut Context, _update: &Update) -> AccessPolicyFuture {
        self.result.into()
    }
}

#[test]
fn test_handler() {
    let update: Update = from_value(json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                "text": "test middleware"
            }
        }
    ))
    .unwrap();
    for &result in &[true, false] {
        let policy = MockPolicy { result };
        let handler = AccessHandler::new(policy);
        let mut context = Context::default();
        let middleware_result = handler.handle(&mut context, &update).wait().unwrap();
        if result {
            assert_eq!(middleware_result, HandlerResult::Continue);
        } else {
            assert_eq!(middleware_result, HandlerResult::Stop);
        }
    }
}

#[test]
fn test_in_memory_policy() {
    macro_rules! check_access {
        ($rules:expr, $updates:expr) => {{
            for rules in $rules {
                let policy = InMemoryAccessPolicy::new(rules);
                for (flag, update) in $updates {
                    let update: Update = from_value(update).unwrap();
                    let mut context = Context::default();
                    let is_granted = policy.is_granted(&mut context, &update).wait().unwrap();
                    assert_eq!(is_granted, flag);
                }
            }
        }};
    }

    check_access!(
        vec![
            vec![AccessRule::allow_user(1)],
            vec![AccessRule::allow_user("username1")],
            vec![AccessRule::deny_user(2), AccessRule::allow_all()],
            vec![AccessRule::deny_user("username2"), AccessRule::allow_all()],
        ],
        vec![
            (
                true,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                        "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                        "text": "test allowed for user #1"
                    }
                })
            ),
            (
                false,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                        "chat": {"id": 2, "type": "private", "first_name": "test", "username": "username2"},
                        "text": "test denied for user #2"
                    }
                })
            )
        ]
    );

    check_access!(
        vec![
            vec![AccessRule::allow_chat(1)],
            vec![AccessRule::allow_chat("username1")],
            vec![AccessRule::deny_chat(2), AccessRule::allow_all()],
            vec![AccessRule::deny_chat("username2"), AccessRule::allow_all()],
        ],
        vec![
            (
                true,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 111, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test allowed for chat #1"
                    }
                })
            ),
            (
                false,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 111, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 2, "type": "supergroup", "title": "test", "username": "username2"},
                        "text": "test denied for chat #2"
                    }
                })
            )
        ]
    );

    check_access!(
        vec![
            vec![AccessRule::allow_chat_user(1, 2)],
            vec![AccessRule::allow_chat_user("username1", 2)],
            vec![AccessRule::allow_chat_user(1, "username2")],
            vec![AccessRule::allow_chat_user("username1", "username2")],
            vec![
                AccessRule::deny_chat_user(1, 3),
                AccessRule::deny_chat_user(4, 3),
                AccessRule::allow_all()
            ],
            vec![
                AccessRule::deny_chat_user("username1", "username3"),
                AccessRule::deny_chat_user(4, 3),
                AccessRule::allow_all()
            ],
        ],
        vec![
            (
                true,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test allowed for user in chat"
                    }
                })
            ),
            (
                false,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test denied for user in chat"
                    }
                })
            ),
            (
                false,
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                        "chat": {"id": 4, "type": "supergroup", "title": "test", "username": "username4"},
                        "text": "test denied for chat and user"
                    }
                })
            )
        ]
    );
}

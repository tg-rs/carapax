use crate::*;
use carapax::prelude::*;
use futures::Future;
use serde_json::from_str;

struct MockPolicy {
    result: bool,
}

impl AccessPolicy<()> for MockPolicy {
    fn is_granted(&mut self, _: &mut (), _update: &Update) -> AccessPolicyFuture {
        self.result.into()
    }
}

#[test]
fn test_middleware() {
    let update: Update = from_str(
        r#"{
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                "text": "test middleware"
            }
        }"#,
    )
    .unwrap();
    for &result in &[true, false] {
        let policy = MockPolicy { result };
        let mut middleware = AccessMiddleware::new(policy);
        let middleware_result = middleware.before(&mut (), &update).wait().unwrap();
        if result {
            assert_eq!(middleware_result, MiddlewareResult::Continue);
        } else {
            assert_eq!(middleware_result, MiddlewareResult::Stop);
        }
    }
}

#[test]
fn test_in_memory_policy() {
    macro_rules! check_access {
        ($rules:expr, $updates:expr) => {{
            for rules in $rules {
                let mut policy = InMemoryAccessPolicy::new(rules);
                for (flag, update) in $updates {
                    let update: Update = from_str(update).unwrap();
                    let is_granted = policy.is_granted(&mut (), &update).wait().unwrap();
                    assert_eq!(is_granted, *flag);
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
        &[
            (
                true,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                        "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                        "text": "test allowed for user #1"
                    }
                }"#
            ),
            (
                false,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                        "chat": {"id": 2, "type": "private", "first_name": "test", "username": "username2"},
                        "text": "test denied for user #2"
                    }
                }"#
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
        &[
            (
                true,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 111, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test allowed for chat #1"
                    }
                }"#
            ),
            (
                false,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 111, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 2, "type": "supergroup", "title": "test", "username": "username2"},
                        "text": "test denied for chat #2"
                    }
                }"#
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
        &[
            (
                true,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 1,
                        "date": 0,
                        "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test allowed for user in chat"
                    }
                }"#
            ),
            (
                false,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test denied for user in chat"
                    }
                }"#
            ),
            (
                false,
                r#"{
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 1,
                        "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                        "chat": {"id": 4, "type": "supergroup", "title": "test", "username": "username4"},
                        "text": "test denied for chat and user"
                    }
                }"#
            )
        ]
    );
}

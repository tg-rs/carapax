use crate::access::rules::AccessRule;
use async_trait::async_trait;
use tgbot::types::Update;

/// An access policy
///
/// Decides whether update is allowed or not
#[async_trait]
pub trait AccessPolicy<C> {
    /// Return true if update is allowed and false otherwise
    async fn is_granted(&mut self, context: &C, update: &Update) -> bool;
}

/// In-memory access policy
///
/// Stores all rules in a Vec
///
/// If there are no rules found for update, [is_granted()] will return false
/// You can use [allow_all()] as a last rule in order to change this behaviour
///
/// [is_granted()]: trait.AccessPolicy.html#tymethod.is_granted
/// [allow_all()]: struct.AccessRule.html#method.allow_all
#[derive(Default)]
pub struct InMemoryAccessPolicy {
    rules: Vec<AccessRule>,
}

impl InMemoryAccessPolicy {
    /// Creates a new policy
    pub fn new(rules: Vec<AccessRule>) -> Self {
        InMemoryAccessPolicy { rules }
    }

    /// Adds a rule to the end of the list
    pub fn push_rule(mut self, rule: AccessRule) -> Self {
        self.rules.push(rule);
        self
    }
}

#[async_trait]
impl<C> AccessPolicy<C> for InMemoryAccessPolicy
where
    C: Send + Sync,
{
    async fn is_granted(&mut self, _context: &C, update: &Update) -> bool {
        let mut result = false;
        for rule in &self.rules {
            if rule.accepts(&update) {
                result = rule.is_granted();
                log::info!("Found rule: {:?}", rule);
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn in_memory_policy() {
        let mut policy = InMemoryAccessPolicy::default();
        assert!(policy.rules.is_empty());
        policy = policy.push_rule(AccessRule::allow_user(1));
        assert_eq!(policy.rules.len(), 1);

        macro_rules! check_access {
            ($rules:expr, $updates:expr) => {{
                for rules in $rules {
                    let mut policy = InMemoryAccessPolicy::new(rules);
                    for (flag, update) in $updates {
                        let update: Update = serde_json::from_value(update).unwrap();
                        let is_granted = policy.is_granted(&(), &update).await;
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
                    serde_json::json!({
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
                    serde_json::json!({
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
                    serde_json::json!({
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
                    serde_json::json!({
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
                    serde_json::json!({
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
                    serde_json::json!({
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
                    serde_json::json!({
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
}

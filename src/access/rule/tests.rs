use crate::types::Update;

use super::*;

#[test]
fn access_rule_new() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let principal_chat = Principal::from(ChatId::from(1));
    let principal_user = Principal::from(UserId::from(1));

    let rule = AccessRule::new(principal_user.clone(), true);
    assert_eq!(rule.principal, principal_user);
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::new(principal_chat.clone(), false);
    assert_eq!(rule.principal, principal_chat);
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

#[test]
fn access_rule_allow_deny() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let principal_chat = Principal::from(ChatId::from(1));
    let principal_user = Principal::from(UserId::from(1));

    let rule = AccessRule::allow(principal_user.clone());
    assert_eq!(rule.principal, principal_user);
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::deny(principal_chat.clone());
    assert_eq!(rule.principal, principal_chat);
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

#[test]
fn access_rule_principal_all() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let rule = AccessRule::allow_all();
    assert_eq!(rule.principal, Principal::All);
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::deny_all();
    assert_eq!(rule.principal, Principal::All);
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

#[test]
fn access_rule_principal_user() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let principal_user = Principal::from(UserId::from(1));

    let rule = AccessRule::allow_user(1);
    assert_eq!(rule.principal, principal_user);
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::deny_user(1);
    assert_eq!(rule.principal, principal_user);
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

#[test]
fn access_rule_principal_chat() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let principal_chat = Principal::from(ChatId::from(1));

    let rule = AccessRule::allow_chat(1);
    assert_eq!(rule.principal, principal_chat);
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::deny_chat(1);
    assert_eq!(rule.principal, principal_chat);
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

#[test]
fn access_rule_principal_chat_user() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 1,
            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
            "text": "test"
        }
    }))
    .unwrap();

    let rule = AccessRule::allow_chat_user(1, 1);
    assert_eq!(rule.principal, Principal::from((ChatId::from(1), UserId::from(1))));
    assert!(rule.is_granted());
    assert!(rule.accepts(&update));

    let rule = AccessRule::deny_chat_user(1, 1);
    assert_eq!(rule.principal, Principal::from((ChatId::from(1), UserId::from(1))));
    assert!(!rule.is_granted());
    assert!(rule.accepts(&update));
}

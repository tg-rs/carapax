use crate::types::Update;

use super::*;

#[tokio::test]
async fn in_memory_access_policy() {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(1)]);

    let update_granted: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input_granted = HandlerInput::from(update_granted);
    assert!(policy.is_granted(input_granted).await.unwrap());

    let update_forbidden: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 2, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input_forbidden = HandlerInput::from(update_forbidden);
    assert!(!policy.is_granted(input_forbidden).await.unwrap());
}

use std::{error::Error, fmt};

use futures_util::future::{err, ok, Ready};

use crate::{
    core::HandlerInput,
    types::{Integer, Update},
};

use super::*;

#[derive(Debug)]
struct ErrorMock;

impl Error for ErrorMock {}

impl fmt::Display for ErrorMock {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "error")
    }
}

#[derive(Clone)]
struct PolicyMock;

impl AccessPolicy for PolicyMock {
    type Error = ErrorMock;
    type Future = Ready<Result<bool, Self::Error>>;

    fn is_granted(&self, input: HandlerInput) -> Self::Future {
        match input.update.get_user().map(|user| Integer::from(user.id)) {
            Some(1) => ok(true),
            Some(2) => ok(false),
            Some(_) => err(ErrorMock),
            None => err(ErrorMock),
        }
    }
}

#[tokio::test]
async fn access_predicate() {
    let policy = PolicyMock;
    let predicate = AccessPredicate::new(policy);

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
    let result = predicate.handle(input_granted).await;
    assert!(result.unwrap());

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
    let result = predicate.handle(input_forbidden).await;
    assert!(!result.unwrap());

    let update_error: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 3, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input_error = HandlerInput::from(update_error);
    let result = predicate.handle(input_error).await;
    assert!(result.is_err());
}

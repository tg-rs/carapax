use std::{error::Error, fmt, sync::Arc};

use tokio::sync::Mutex;

use crate::{core::handler::HandlerInput, types::Update};

use super::*;

#[derive(Clone)]
struct Condition {
    value: Arc<Mutex<bool>>,
}

#[derive(Debug)]
struct ExampleError;

impl fmt::Display for ExampleError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Example error")
    }
}

impl Error for ExampleError {}

impl ErrorHandler for Condition {
    async fn handle(&self, err: HandlerError) -> HandlerError {
        let value = self.value.clone();
        *value.lock().await = true;
        err
    }
}

async fn handler(_: ()) -> Result<(), ExampleError> {
    Err(ExampleError)
}

fn create_update() -> Update {
    serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "message": {
            "message_id": 1111,
            "date": 0,
            "from": {"id": 1, "is_bot": false, "first_name": "test"},
            "chat": {"id": 1, "type": "private", "first_name": "test"},
            "text": "test message from private chat"
        }
    }))
    .unwrap()
}

#[tokio::test]
async fn error_decorator() {
    let condition = Condition {
        value: Arc::new(Mutex::new(false)),
    };
    let handler = ErrorDecorator::new(condition.clone(), handler);
    let update = create_update();
    let input = HandlerInput::from(update);
    let result = handler.handle(input).await;
    assert!(result.is_err());
    assert!(*condition.value.lock().await)
}
